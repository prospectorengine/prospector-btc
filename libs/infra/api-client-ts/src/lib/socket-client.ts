/**
 * =================================================================
 * APARATO: NEURAL SOCKET CLIENT (V2.0 - HEIMDALL INTEGRATED)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (L4)
 * RESPONSABILIDAD: ENLACE BI-DIRECCIONAL RESILIENTE (WS)
 *
 * VISION HIPER-HOLÍSTICA:
 * 1. ZERO ANY POLICY: Tipado estricto en eventos y comandos.
 * 2. OBSERVABILIDAD: Registro de latido y reconexión vía Heimdall.
 * 3. RESILIENCIA: Backoff exponencial para protección de red.
 * =================================================================
 */

import { createLogger } from "@prospector/heimdall-ts";

const logger = createLogger("Infra:NeuralSocket");

type NeuralHandler = (data: string) => void;

interface SocketOptions {
  url: string;
  token?: string;
  onMessage: NeuralHandler;
  onError?: (error: unknown) => void;
  onOpen?: () => void;
  onClose?: () => void;
}

export class NeuralSocket {
  private socket: WebSocket | null = null;
  private retry_count = 0;
  private is_intentionally_closed = false;
  private heartbeat_handle: ReturnType<typeof setInterval> | null = null;

  constructor(private options: SocketOptions) {
    this.establish_uplink();
  }

  private establish_uplink(): void {
    if (this.is_intentionally_closed) return;

    const target_url = this.options.url.replace(/^http/, 'ws');

    // Inyectamos el token vía Sec-WebSocket-Protocol (Estándar de bypass de headers en navegador)
    this.socket = new WebSocket(
      target_url,
      this.options.token ? ["access_token", this.options.token] : undefined
    );

    this.socket.onopen = () => {
      this.retry_count = 0;
      this.start_heartbeat();
      this.options.onOpen?.();
      logger.info("Uplink synchronized. Neural link is ACTIVE.");
    };

    this.socket.onmessage = (event: MessageEvent<string>) => {
      if (typeof event.data === "string") {
        this.options.onMessage(event.data);
      }
    };

    this.socket.onerror = (error: Event) => {
      logger.error("Physical layer error detected in WebSocket tunnel.");
      this.options.onError?.(error);
    };

    this.socket.onclose = () => {
      this.stop_heartbeat();
      this.options.onClose?.();
      if (!this.is_intentionally_closed) {
        this.initiate_recovery_sequence();
      }
    };
  }

  private initiate_recovery_sequence(): void {
    const backoff_delay = Math.min(1000 * Math.pow(2, this.retry_count++), 30000);
    logger.warn(`Neural Link severed. Re-negotiating in ${backoff_delay}ms...`);
    setTimeout(() => this.establish_uplink(), backoff_delay);
  }

  private start_heartbeat(): void {
    this.stop_heartbeat();
    this.heartbeat_handle = setInterval(() => {
      if (this.socket?.readyState === WebSocket.OPEN) {
        // Pulso lógico para mantener abiertos Proxies y Balanceadores (Nginx/Render)
        this.socket.send(JSON.stringify({ t: "PING", ts: Date.now() }));
      }
    }, 30000);
  }

  private stop_heartbeat(): void {
    if (this.heartbeat_handle) {
      clearInterval(this.heartbeat_handle);
      this.heartbeat_handle = null;
    }
  }

  /**
   * Envía una directiva de mando al Orquestador L3.
   * ✅ SOBERANÍA DE TIPOS: Prohíbe el envío de datos no estructurados.
   */
  public send_command(command: Record<string, unknown>): void {
    if (this.socket?.readyState === WebSocket.OPEN) {
      this.socket.send(JSON.stringify(command));
    } else {
      logger.error("Command Dispatch Aborted: Uplink is currently offline.");
    }
  }

  public close(): void {
    this.is_intentionally_closed = true;
    this.stop_heartbeat();
    if (this.socket) {
      this.socket.close();
      this.socket = null;
    }
    logger.info("Neural Link terminated by Architect.");
  }
}

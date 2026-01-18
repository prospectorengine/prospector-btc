// INICIO DEL ARCHIVO [tools/provisioner/src/lib/mechanics/sentinel.ts]
/**
 * =================================================================
 * APARATO: SENTINEL VIGILANCE (V6.0 - BUFFERED UPLINK)
 * CLASIFICACIÓN: OPS MECHANICS (ESTRATO L6)
 * RESPONSABILIDAD: SUPERVIVENCIA Y GARANTÍA DE ENTREGA DE LOGS
 *
 * VISION HIPER-HOLÍSTICA:
 * Evolución resiliente del centinela. Implementa un buffer FIFO en memoria
 * para acumular trazas si el Orquestador no responde (backpressure),
 * garantizando cero pérdida de observabilidad durante el arranque en frío.
 * =================================================================
 */

import { Page } from "playwright";
import axios from "axios";
import { config } from "../../config";

interface BufferedLogEntry {
  node_index: number;
  message: string;
  level: "INFO" | "WARN" | "CRITICAL";
  timestamp: string;
}

export class Sentinel {
  private surveillance_heartbeat_interval: NodeJS.Timeout | null = null;
  private log_transmission_buffer: BufferedLogEntry[] = [];
  private is_flushing_buffer: boolean = false;
  private consecutive_network_failures: number = 0;

  constructor(
    private playwright_page: Page,
    private worker_node_identifier: string,
    private node_sequence_index: number,
    private terminal_log_prefix: string
  ) {}

  /**
   * Encola una traza de navegación y dispara el intento de vaciado asíncrono.
   * No bloquea el flujo principal de ejecución de Playwright.
   */
  public async emitTrace(message: string, level: "INFO" | "WARN" | "CRITICAL" = "INFO"): Promise<void> {
    const log_entry: BufferedLogEntry = {
      node_index: this.node_sequence_index,
      message: `${this.terminal_log_prefix} ${message}`,
      level,
      timestamp: new Date().toISOString(),
    };

    // Inserción en memoria (Alta velocidad)
    this.log_transmission_buffer.push(log_entry);

    // Intento de transmisión no bloqueante
    void this.attempt_buffer_flush();

    // Espejo en consola local para debug de CI/CD
    console.log(log_entry.message);
  }

  /**
   * Motor de vaciado de logs con lógica de reintento exponencial.
   */
  private async attempt_buffer_flush(): Promise<void> {
    if (this.is_flushing_buffer || this.log_transmission_buffer.length === 0 || !config.ORCHESTRATOR_URL) {
      return;
    }

    this.is_flushing_buffer = true;

    try {
      // Tomamos una instantánea del buffer actual para intentar enviarlo
      const batch_to_send = [...this.log_transmission_buffer];

      // Enviamos el lote más antiguo primero (FIFO Preservation)
      const entry = batch_to_send[0];

      await axios.post(
        `${config.ORCHESTRATOR_URL}/api/v1/admin/provisioning/log`,
        entry,
        {
          headers: { Authorization: `Bearer ${config.WORKER_AUTH_TOKEN}` },
          timeout: 5000,
        }
      );

      // Éxito: Removemos del buffer real y reseteamos contadores de fallo
      this.log_transmission_buffer.shift();
      this.consecutive_network_failures = 0;

      // Si quedan logs, recursión inmediata para vaciar rápido
      if (this.log_transmission_buffer.length > 0) {
        this.is_flushing_buffer = false;
        void this.attempt_buffer_flush();
        return;
      }

    } catch (network_error) {
      this.consecutive_network_failures++;

      // Backoff exponencial simple: si fallamos mucho, esperamos más antes de reintentar
      const backoff_ms = Math.min(1000 * Math.pow(2, this.consecutive_network_failures), 10000);

      if (this.consecutive_network_failures % 5 === 0) {
         console.warn(`${this.terminal_log_prefix} ⚠️ Telemetry Uplink Unstable. Buffer size: ${this.log_transmission_buffer.length}`);
      }

      // Reintentamos después del backoff
      setTimeout(() => {
        this.is_flushing_buffer = false;
        void this.attempt_buffer_flush();
      }, backoff_ms);

      return; // Salimos para respetar el timeout
    }

    this.is_flushing_buffer = false;
  }

  /**
   * Captura y transmite el estado visual del worker al Panóptico (L5).
   */
  public async captureFrame(status: "running" | "error" | "captcha"): Promise<void> {
    if (!config.ORCHESTRATOR_URL) return;
    try {
      const screenshot_buffer = await this.playwright_page.screenshot({ type: "jpeg", quality: 30 });
      const base64_image_data = `data:image/jpeg;base64,${screenshot_buffer.toString("base64")}`;

      await axios.post(
        `${config.ORCHESTRATOR_URL}/api/v1/visual/snapshot`,
        {
          worker_identifier: this.worker_node_identifier,
          operational_status: status,
          snapshot_base64_data: base64_image_data,
          captured_at_timestamp: new Date().toISOString(),
        },
        {
          headers: { Authorization: `Bearer ${config.WORKER_AUTH_TOKEN}` },
          timeout: 8000, // Timeout extendido para carga útil pesada (Imagen)
        }
      );
    } catch {
      // Fallo silencioso en video para no saturar el canal de logs críticos
    }
  }

  public startHeartbeat(): void {
    this.emitTrace("💓 Heartbeat pulse activated. Swarm surveillance online.", "INFO");
    this.surveillance_heartbeat_interval = setInterval(
      () => this.captureFrame("running"),
      60000
    );
  }

  public stop(): void {
    if (this.surveillance_heartbeat_interval) {
      clearInterval(this.surveillance_heartbeat_interval);
      this.surveillance_heartbeat_interval = null;
    }
    // Intento final de vaciar el buffer antes de morir (Best Effort)
    if (this.log_transmission_buffer.length > 0) {
       console.log(`${this.terminal_log_prefix} 🛑 Dumping ${this.log_transmission_buffer.length} remaining traces locally.`);
    }
  }

  public async triggerKillSwitch(reason: string): Promise<void> {
    await this.emitTrace(`💀 KILL-SWITCH ACTIVATED: ${reason}`, "CRITICAL");
    if (!config.ORCHESTRATOR_URL) return;
    try {
      await axios.post(
        `${config.ORCHESTRATOR_URL}/api/v1/admin/identities/revoke`,
        { email: "managed_identity_via_id", worker_id: this.worker_node_identifier },
        {
          headers: { Authorization: `Bearer ${config.WORKER_AUTH_TOKEN}` },
          timeout: 5000,
        }
      );
    } catch (e: any) {
      console.error(`${this.terminal_log_prefix} Kill-switch transmission failed: ${e.message}`);
    }
  }
}
// FIN DEL ARCHIVO [tools/provisioner/src/lib/mechanics/sentinel.ts]

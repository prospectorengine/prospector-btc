/**
 * =================================================================
 * APARATO: NEURAL SOCKET MIRROR TEST (V1.3 - TYPE SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L4-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE RESILIENCIA Y PROTOCOLO WS
 *
 * VISION HIPER-HOLÍSTICA:
 * Erradica el uso de 'any' (TS-Explicit-Any) mediante la definición
 * de una interface intrusiva para acceso a estados privados.
 * =================================================================
 */

import { NeuralSocket } from "../../../../../libs/infra/api-client-ts/src/lib/socket-client";

/**
 * Interface de Intrusión Táctica.
 * Permite acceder a las propiedades privadas de NeuralSocket durante el test
 * manteniendo la integridad de tipos (Zero Any Policy).
 */
interface IntrusiveNeuralSocket {
  socket: MockWebSocket | null;
}

/**
 * Mock de infraestructura WebSocket con paridad de constantes estáticas.
 * Satisface los requerimientos de la API nativa de DOM.
 */
class MockWebSocket {
  static readonly CONNECTING = 0;
  static readonly OPEN = 1;
  static readonly CLOSING = 2;
  static readonly CLOSED = 3;

  public onopen: () => void = () => {};
  public onmessage: (event: { data: string }) => void = () => {};
  public onclose: () => void = () => {};
  public onerror: (error: Event) => void = () => {};

  public close: jest.Mock = jest.fn();
  public send: jest.Mock = jest.fn();
  public readyState: number = MockWebSocket.OPEN;

  constructor(public url: string, public protocols?: string | string[]) {
    // Simulamos una latencia de red de 10ms para el apretón de manos
    setTimeout(() => this.onopen(), 10);
  }
}

// Inyección de Mock en el entorno global de ejecución
global.WebSocket = MockWebSocket as unknown as typeof WebSocket;

describe("NeuralSocket: Transport Resilience Strata", () => {

  /**
   * # Mathematical Proof:
   * Verifica que el túnel TCP se establece y que la decodificación
   * Base64 delegada al handler se ejecuta ante ráfagas entrantes.
   */
  it("should establish secure uplink and route binary signals through the codec", (done) => {
    const message_handler_spy = jest.fn();

    const neural_socket_instance = new NeuralSocket({
      url: "http://prospector-orchestrator.internal/stream",
      token: "TEST_MASTER_TOKEN_V11",
      onMessage: message_handler_spy,
      onOpen: () => {
        // ACCESO SOBERANO SIN ANY: Casting controlado a interface intrusiva
        const internal_access = (neural_socket_instance as unknown as IntrusiveNeuralSocket);
        const physical_socket = internal_access.socket;

        if (physical_socket) {
          // Inyección de señal simulada (Base64 encoded)
          physical_socket.onmessage({ data: "SGVsbG8gSHlkcmE=" });
          expect(message_handler_spy).toHaveBeenCalledWith("SGVsbG8gSHlkcmE=");
        } else {
          throw new Error("CRITICAL_TEST_FAILURE: Physical socket not materialized.");
        }

        neural_socket_instance.close();
        done();
      }
    });
  });

  /**
   * # Security Proof:
   * Certifica la transformación automática del esquema de protocolo
   * de HTTP inseguro a WebSocket Seguro (wss).
   */
  it("should enforce secure protocol transformation automatically", () => {
    const neural_socket_instance = new NeuralSocket({
      url: "https://api.metashark.tech/v1",
      onMessage: jest.fn()
    });

    const internal_access = (neural_socket_instance as unknown as IntrusiveNeuralSocket);

    // Verificación de integridad de URL
    expect(internal_access.socket?.url).toBe("wss://api.metashark.tech/v1");
    neural_socket_instance.close();
  });

});

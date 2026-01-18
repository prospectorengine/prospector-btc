// INICIO DEL ARCHIVO [tests/mirror/libs/infra/api_client_ts/socket_client.test.ts]
/**
 * =================================================================
 * APARATO: NEURAL SOCKET MIRROR TEST (V1.2 - TYPE SAFE)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar la resiliencia del transporte WebSocket.
 *
 * # Diff Justification:
 * + Definición de 'IntrusiveNeuralSocket' para acceso tipado a privados.
 * + Reemplazo de 'any' por 'unknown' + tipos explícitos en el Mock.
 * + Inyección de constantes estáticas en MockWebSocket para paridad con DOM.
 * =================================================================
 */

import { NeuralSocket } from "../../../../../libs/infra/api-client-ts/src/lib/socket-client";

// Interfaz para acceder a propiedades privadas durante el test (White-box)
interface IntrusiveNeuralSocket {
  socket: MockWebSocket | null;
}

// Mock del entorno WebSocket Global con constantes estáticas requeridas
class MockWebSocket {
  static readonly CONNECTING = 0;
  static readonly OPEN = 1;
  static readonly CLOSING = 2;
  static readonly CLOSED = 3;

  onopen: () => void = () => {};
  onmessage: (event: { data: string }) => void = () => {};
  onclose: () => void = () => {};
  close: jest.Mock = jest.fn();
  send: jest.Mock = jest.fn();
  readyState: number = 1; // OPEN

  constructor(public url: string) {
    // Simulamos conexión exitosa inmediata
    setTimeout(() => this.onopen(), 10);
  }
}

// ✅ FIX: Casting seguro de doble estrato para satisfacer el entorno global
global.WebSocket = MockWebSocket as unknown as typeof WebSocket;

describe("NeuralSocket Transport Engine", () => {
  it("should establish uplink and route base64 messages", (done) => {
    const messageSpy = jest.fn();

    const socket = new NeuralSocket({
      url: "http://localhost:3000/stream",
      token: "TEST_TACTICAL_TOKEN",
      onMessage: messageSpy,
      onOpen: () => {
        // ✅ FIX: Acceso tipado a la propiedad privada 'socket'
        const internalSocket = (socket as unknown as IntrusiveNeuralSocket).socket;

        if (internalSocket) {
          // Simular inyección de payload desde el Orquestador
          internalSocket.onmessage({ data: "SGVsbG8gTmV1cmFsIExpbms=" });
          expect(messageSpy).toHaveBeenCalledWith("SGVsbG8gTmV1cmFsIExpbms=");
        } else {
          fail("CRITICAL_TEST_FAILURE: Internal socket not initialized.");
        }

        socket.close();
        done();
      }
    });
  });

  it("should transform http protocol to secure ws protocol", () => {
    const socket = new NeuralSocket({
      url: "https://api.prospector.com/v1",
      onMessage: jest.fn()
    });

    // ✅ FIX: Acceso tipado seguro
    const internalSocket = (socket as unknown as IntrusiveNeuralSocket).socket;

    if (internalSocket) {
      expect(internalSocket.url).toBe("wss://api.prospector.com/v1");
    } else {
      fail("CRITICAL_TEST_FAILURE: Internal socket unreachable.");
    }
  });
});
// FIN DEL ARCHIVO [tests/mirror/libs/infra/api_client_ts/socket_client.test.ts]

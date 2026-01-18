/**
 * =================================================================
 * APARATO: COLAB CONTROLLER IGNITION TEST (V42.5 - SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE (ESTRATO L6-MIRROR)
 * RESPONSABILIDAD: CERTIFICACIÓN DE DESPLIEGUE Y TELEMETRÍA C2
 *
 * VISION HIPER-HOLÍSTICA:
 * Valida que el controlador de Google Colab orqueste correctamente
 * la secuencia de ignición, inyectando el payload y activando
 * los centinelas de vigilancia.
 * =================================================================
 */

// ✅ REPARACIÓN TS2307: Corrección de profundidad de ruta (4 niveles hasta la raíz)
import { ColabController } from "../../../../tools/provisioner/src/lib/colab";
import { Page } from "playwright";

describe("ColabController: Ignition Sequence Strata", () => {
  /**
   * # Mathematical Proof:
   * Verifica que la instanciación del controlador vincule correctamente
   * el índice de nodo con la identidad de red, garantizando que
   * la telemetría enviada al Orquestador sea unívoca.
   */
  it("should initialize with correct node identification and telemetry prefix", () => {
    // 1. SETUP: Mock de página de Playwright nivelado
    const mockPage = {
      locator: jest.fn(),
      keyboard: { press: jest.fn() },
      evaluate: jest.fn(),
      on: jest.fn(),
    } as unknown as Page;

    const nodeIndex = 7;
    const operatorEmail = "architector@prospector.io";

    // 2. EXECUTION
    const controller = new ColabController(mockPage, nodeIndex, operatorEmail);

    // 3. VALIDATION: Acceso a propiedad privada vía casting para auditoría de integridad
    const internalNodeId = (controller as any).worker_node_identifier;
    expect(internalNodeId).toBe(`hydra-node-${nodeIndex}`);
  });

  /**
   * # Safety Check:
   * Valida que el proceso de despliegue aborte y reporte el fallo
   * ante la ausencia de elementos críticos en el DOM de Google.
   */
  it("should collapse and emit critical trace when Monaco Editor is missing", async () => {
    // 1. SETUP: Mock que simula timeout de selector
    const mockPage = {
      locator: jest.fn().mockReturnValue({
        first: jest.fn().mockReturnValue({
          waitFor: jest.fn().mockRejectedValue(new Error("TIMEOUT_WAITING_FOR_MONACO")),
        }),
      }),
      screenshot: jest.fn().mockResolvedValue(Buffer.from([])),
    } as unknown as Page;

    const controller = new ColabController(mockPage, 1, "test@prospector.io");

    // 2. EXECUTION & VALIDATION
    // Se espera que el método deploy capture el error y lo propague tras emitir la traza.
    await expect(controller.deploy("MASTER_KEY_V12")).rejects.toThrow();
  });
});

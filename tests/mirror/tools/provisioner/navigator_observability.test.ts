/**
 * =================================================================
 * APARATO: NAVIGATOR OBSERVABILITY MIRROR TEST (V1.1 - FIXED)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * RESPONSABILIDAD: VALIDACIÓN DE TELEMETRÍA EN EL PROVISIONADOR
 * =================================================================
 */

// ✅ REPARACIÓN TS2307: Ruta corregida subiendo 4 niveles hasta la raíz del monorepo
import { ColabNavigator } from "../../../../tools/provisioner/src/lib/mechanics/navigator";

describe("ColabNavigator Observability Strata", () => {
  it("should trigger sentinel.emitTrace during the approach sequence", async () => {
    // 1. SETUP: Mock de dependencias niveladas
    const mockSentinel = {
      emitTrace: jest.fn().mockResolvedValue(undefined)
    };

    const mockPage = {
      goto: jest.fn().mockResolvedValue(undefined)
    } as any;

    const navigator = new ColabNavigator(
      mockPage,
      null,
      mockSentinel as any
    );

    // 2. EXECUTION
    await navigator.approachTarget();

    // 3. VALIDATION (EVIDENCIA)
    // ✅ REPARACIÓN TS2304: 'expect' y 'jest' ahora reconocidos vía tsconfig.json
    expect(mockPage.goto).toHaveBeenCalled();
    expect(mockSentinel.emitTrace).toHaveBeenCalledWith(
      expect.stringContaining("Navigating to Google Colab"),
      expect.any(String)
    );
  });
});

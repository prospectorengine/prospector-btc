/**
 * =================================================================
 * APARATO: API CONTRACTS BARREL CERTIFIER (V1.2 - ALIAS FIXED)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L2-MIRROR
 * RESPONSABILIDAD: VALIDACIÓN DE EXPOSICIÓN NOMINAL SOBERANA
 * =================================================================
 */

// ✅ REPARACIÓN TS2307: Se sustituye la ruta relativa por el Alias de Estrato.
// Esto utiliza la configuración de 'paths' de tsconfig.base.json.
import * as Contracts from "@prospector/api-contracts";

describe("API Contracts Barrel: Zenith Integrity Audit", () => {
  it("should expose the full Command & Control nomenclature", () => {
    // Verificación de existencia nominal de los miembros clave
    expect(Contracts.CommandDirectiveSchema).toBeDefined();
    expect(Contracts.KnowledgeModuleSchema).toBeDefined();
  });

  it("should maintain strict naming parity with Rust DTOs", () => {
    const mock_directive: Contracts.CommandDirective = {
      action: "HaltSwarm",
      payload: { reason: "UNIT_TEST_OVERRIDE" }
    };

    const validation = Contracts.CommandDirectiveSchema.safeParse(mock_directive);
    expect(validation.success).toBe(true);
  });
});

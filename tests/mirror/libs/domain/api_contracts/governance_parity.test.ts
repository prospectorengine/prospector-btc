// INICIO DEL ARCHIVO [tests/mirror/libs/domain/api_contracts/governance_parity.test.ts]
/**
 * =================================================================
 * APARATO: GOVERNANCE CONTRACT PARITY TEST (V1.1 - FIELD CHECK)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar la estructura del payload de gobernanza e identidad.
 * =================================================================
 */

import { IdentityGovernanceSchema, IdentitySchema } from "@prospector/api-contracts";

describe("Identity Governance Parity Audit", () => {

    it("should validate a correct governance payload", () => {
        const validPayload = {
            email: "operator@prospector.io",
            reason: "ZOMBIE_WORKER_CLEANUP"
        };

        const result = IdentityGovernanceSchema.safeParse(validPayload);
        expect(result.success).toBe(true);
    });

    it("should reject invalid email formats", () => {
        const invalidPayload = {
            email: "not-an-email",
            reason: "TEST"
        };

        const result = IdentityGovernanceSchema.safeParse(invalidPayload);
        expect(result.success).toBe(false);
    });

    it("should validate IdentitySchema with optional lease fields", () => {
        // Simulamos una respuesta de DB que incluye leased_until
        const identityFromDb = {
            id: "123e4567-e89b-12d3-a456-426614174000",
            platform: "google_colab",
            email: "test@example.com",
            credentials_json: "{}",
            user_agent: "Mozilla/5.0",
            usage_count: 5,
            last_used_at: new Date().toISOString(),
            created_at: new Date().toISOString(),
            status: "active",
            leased_until: new Date().toISOString() // Campo opcional nuevo
        };

        const result = IdentitySchema.safeParse(identityFromDb);
        expect(result.success).toBe(true);
        if (result.success) {
            expect(result.data.leased_until).toBeDefined();
        }
    });

    it("should allow payload without optional reason", () => {
        const minimalPayload = {
            email: "minimal@prospector.io"
        };

        const result = IdentityGovernanceSchema.safeParse(minimalPayload);
        expect(result.success).toBe(true);
    });
});
// FIN DEL ARCHIVO [tests/mirror/libs/domain/api_contracts/governance_parity.test.ts]

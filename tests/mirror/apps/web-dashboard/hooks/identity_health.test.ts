// INICIO DEL ARCHIVO [tests/mirror/apps/web-dashboard/hooks/identity_health.test.ts]
/**
 * =================================================================
 * APARATO: IDENTITY HEALTH LOGIC TEST (V1.1 - PATH FIXED)
 * CLASIFICACIÓN: UNIT TEST
 * OBJETIVO: Certificar cálculo de TTL y clasificación de riesgo.
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa resolución de rutas mediante alias '@/' para garantizar
 * la localización correcta del motor de lógica en el monorepo.
 * =================================================================
 */

// ✅ FIX: Uso de alias soberano configurado en tests/tsconfig.json
import { IdentityHealthEngine } from "@/lib/utils/identity-health";
import { getUnixTime, addDays, subDays } from "date-fns";

describe("Identity Health Engine (Logic Core)", () => {

  it("should classify future cookies (> 30 days) as HEALTHY", () => {
    const futureDate = addDays(new Date(), 45);
    const cookies = [{ name: "SID", expirationDate: getUnixTime(futureDate) }];

    const result = IdentityHealthEngine.analyze(cookies);

    expect(result.status).toBe("HEALTHY");
    expect(result.ttl_days).toBeGreaterThan(40);
    expect(result.risk_factors).toHaveLength(0);
  });

  it("should classify expiring cookies (< 7 days) as CRITICAL", () => {
    const nearFuture = addDays(new Date(), 3);
    const cookies = [{ name: "SID", expirationDate: getUnixTime(nearFuture) }];

    const result = IdentityHealthEngine.analyze(cookies);

    expect(result.status).toBe("CRITICAL");
    expect(result.ttl_days).toBeLessThanOrEqual(3);
    expect(result.risk_factors).toContain(expect.stringContaining("CRITICAL_TTL_LOW"));
  });

  it("should detect expired cookies", () => {
    const pastDate = subDays(new Date(), 1);
    const cookies = [{ name: "SID", expirationDate: getUnixTime(pastDate) }];

    const result = IdentityHealthEngine.analyze(cookies);

    expect(result.status).toBe("CRITICAL");
    expect(result.risk_factors).toContain("SESSION_EXPIRED");
  });

  it("should handle missing expiration data gracefully", () => {
    const cookies = [{ name: "SID", value: "session-only" }];

    const result = IdentityHealthEngine.analyze(cookies);

    expect(result.status).toBe("UNKNOWN");
    expect(result.ttl_days).toBeNull();
  });
});
// FIN DEL ARCHIVO [tests/mirror/apps/web-dashboard/hooks/identity_health.test.ts]

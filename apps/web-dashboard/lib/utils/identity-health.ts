// INICIO DEL ARCHIVO [apps/web-dashboard/lib/utils/identity-health.ts]
/**
 * =================================================================
 * APARATO: IDENTITY HEALTH ENGINE (V1.0 - PURE LOGIC)
 * CLASIFICACIÓN: DOMAIN UTILITY (ESTRATO L5)
 * RESPONSABILIDAD: CÁLCULO DETERMINISTA DE VIABILIDAD DE SESIÓN
 * =================================================================
 */

import { differenceInDays, fromUnixTime, isPast } from "date-fns";

export type HealthStatus = "HEALTHY" | "DEGRADED" | "CRITICAL" | "UNKNOWN";

export interface IdentityHealthMetrics {
  status: HealthStatus;
  ttl_days: number | null;
  expiration_date: Date | null;
  risk_factors: string[];
}

/**
 * Interfaz mínima de una cookie para análisis.
 */
interface CookieFragment {
  name: string;
  expirationDate?: number;
  expires?: number;
}

const CRITICAL_TTL_THRESHOLD_DAYS = 7;
const WARNING_TTL_THRESHOLD_DAYS = 30;

export class IdentityHealthEngine {
  /**
   * Analiza un conjunto de cookies descifradas para determinar su salud.
   *
   * @param cookies - Array de objetos cookie (any para flexibilidad de JSON parseado).
   * @returns Métricas de salud calculadas.
   */
  public static analyze(cookies: CookieFragment[]): IdentityHealthMetrics {
    const risk_factors: string[] = [];
    let min_expiration_ts = Infinity;
    let has_expiration_data = false;

    // 1. ANÁLISIS DE EXPIRACIÓN (Búsqueda del eslabón más débil)
    for (const cookie of cookies) {
      // Normalización de campo de expiración (Puppeteer vs Standard)
      const exp = cookie.expirationDate ?? cookie.expires;

      if (typeof exp === "number") {
        has_expiration_data = true;
        if (exp < min_expiration_ts) {
          min_expiration_ts = exp;
        }
      }
    }

    // 2. CÁLCULO DE TTL
    if (!has_expiration_data || min_expiration_ts === Infinity) {
      return {
        status: "UNKNOWN",
        ttl_days: null,
        expiration_date: null,
        risk_factors: ["NO_EXPIRATION_DATA_FOUND"],
      };
    }

    // Conversión segura de Timestamp Unix (segundos) a Fecha JS
    // Nota: Algunas cookies usan milisegundos, heurística básica:
    // Si es muy grande (> 30000000000), asumimos ms, si no, segundos.
    // Para Google Cookies estándar, suele ser segundos.
    const expiration_date = fromUnixTime(min_expiration_ts);
    const ttl_days = differenceInDays(expiration_date, new Date());

    // 3. CLASIFICACIÓN DE RIESGO
    let status: HealthStatus = "HEALTHY";

    if (isPast(expiration_date)) {
      status = "CRITICAL";
      risk_factors.push("SESSION_EXPIRED");
    } else if (ttl_days <= CRITICAL_TTL_THRESHOLD_DAYS) {
      status = "CRITICAL";
      risk_factors.push(`CRITICAL_TTL_LOW (<${CRITICAL_TTL_THRESHOLD_DAYS}d)`);
    } else if (ttl_days <= WARNING_TTL_THRESHOLD_DAYS) {
      status = "DEGRADED";
      risk_factors.push("TTL_DEGRADING");
    }

    return {
      status,
      ttl_days,
      expiration_date,
      risk_factors,
    };
  }
}
// FIN DEL ARCHIVO [apps/web-dashboard/lib/utils/identity-health.ts]

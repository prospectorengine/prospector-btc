// INICIO DEL ARCHIVO [apps/web-dashboard/lib/utils/cookie-purifier.ts]
/**
 * =================================================================
 * APARATO: ELITE COOKIE PURIFIER (V2.0)
 * CLASIFICACIÓN: UTILITY (ESTRATO L5)
 * RESPONSABILIDAD: SANITIZACIÓN QUIRÚRGICA DE CREDENCIALES
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la matriz de integridad para distinguir entre cookies
 * de identidad (Core), cookies de consistencia (Anti-Ban) y basura
 * de rastreo (Analytics).
 * =================================================================
 */

import { z } from "zod";

// --- CONSTANTES ESTRATÉGICAS ---

/**
 * EL ANILLO DE PODER:
 * Estas son las únicas cookies necesarias para una sesión soberana.
 * Cualquier otra cookie es ruido o vector de rastreo innecesario.
 */
const GOLDEN_KEYS = new Set([
  // Core Identity (Quién eres)
  "SID", "HSID", "SSID",
  // API Authorization (Permisos de XHR)
  "APISID", "SAPISID",
  // Secure Context (HTTPS & Cross-Site)
  "__Secure-1PSID", "__Secure-3PSID",
  "__Secure-1PAPISID", "__Secure-3PAPISID",
  // Consistency & Timestamping (CRÍTICO PARA ANTI-BAN 2025)
  "__Secure-1PSIDTS", "__Secure-1PSIDCC",
  // Environment Context (Preferencias y Anti-Captcha)
  "NID", "AEC", "1P_JAR"
]);

const ALLOWED_DOMAINS = [".google.com", "google.com", ".colab.research.google.com"];

// --- CONTRATOS DE TIPADO ---

const RawCookieSchema = z.object({
  name: z.string(),
  value: z.string(),
  domain: z.string().optional(),
  path: z.string().optional(),
  expirationDate: z.number().optional(),
  expires: z.number().optional(), // Soporte para formato Puppeteer
  httpOnly: z.boolean().optional(),
  secure: z.boolean().optional(),
  sameSite: z.string().optional(),
}).passthrough();

export interface PlaywrightCookie {
  name: string;
  value: string;
  domain: string;
  path: string;
  secure: boolean;
  httpOnly: boolean;
  sameSite: "Strict" | "Lax" | "None";
  expirationDate?: number;
}

export interface PurificationReport {
  isValidStructure: boolean;
  totalInputCookies: number;
  validCookiesCount: number;
  purgedGarbageCount: number;
  missingCriticalKeys: string[];
  cleanCookies: PlaywrightCookie[];
  status: "OPTIMAL" | "DEGRADED" | "CRITICAL" | "INVALID";
}

// --- LÓGICA DEL NÚCLEO ---

/**
 * Analiza, filtra y normaliza un JSON crudo de cookies.
 * Genera un reporte forense para la UI antes de la inyección.
 *
 * @param rawJson String JSON pegado por el usuario.
 */
export function analyzeAndPurify(rawJson: string): PurificationReport {
  let parsed: unknown[];

  // 1. VALIDACIÓN ESTRUCTURAL (PARSING)
  try {
    parsed = JSON.parse(rawJson);
    if (!Array.isArray(parsed)) throw new Error("Not an array");
  } catch {
    return createEmptyReport("INVALID");
  }

  // 2. FILTRADO QUIRÚRGICO
  const validCookies: PlaywrightCookie[] = [];
  let garbageCount = 0;

  for (const item of parsed) {
    const parseResult = RawCookieSchema.safeParse(item);

    if (!parseResult.success) {
      garbageCount++;
      continue;
    }

    const cookie = parseResult.data;

    // Filtro por Nombre (Whitelist Estricta)
    if (!GOLDEN_KEYS.has(cookie.name)) {
      garbageCount++;
      continue;
    }

    // Filtro por Dominio (Seguridad de Origen)
    const domain = cookie.domain || "";
    const isGoogle = ALLOWED_DOMAINS.some(d => domain.endsWith(d));
    if (!isGoogle) {
      garbageCount++;
      continue;
    }

    // Normalización al Estándar Playwright
    validCookies.push({
      name: cookie.name,
      value: cookie.value,
      domain: cookie.domain || ".google.com",
      path: cookie.path || "/",
      secure: true, // Forzamos HTTPS para Google
      httpOnly: cookie.httpOnly ?? false,
      sameSite: "None", // Crítico para contextos cross-site en Colab
      expirationDate: cookie.expirationDate || cookie.expires
    });
  }

  // 3. AUDITORÍA DE INTEGRIDAD (Búsqueda de llaves maestras)
  // SID y HSID son fundamentales para la sesión base.
  // __Secure-1PSID es fundamental para contextos seguros.
  const foundKeys = new Set(validCookies.map(c => c.name));
  const criticalMissing = ["SID", "HSID", "__Secure-1PSID"].filter(k => !foundKeys.has(k));

  // 4. DETERMINACIÓN DE ESTADO
  let status: PurificationReport["status"] = "OPTIMAL";

  if (criticalMissing.length > 0) {
    status = "CRITICAL";
  } else if (validCookies.length < 5) {
    // Si tenemos las críticas pero faltan muchas secundarias (ej. NID, 1PSIDTS), es degradado
    status = "DEGRADED";
  }

  return {
    isValidStructure: true,
    totalInputCookies: parsed.length,
    validCookiesCount: validCookies.length,
    purgedGarbageCount: garbageCount,
    missingCriticalKeys: criticalMissing,
    cleanCookies: validCookies,
    status
  };
}

/**
 * Wrapper simplificado para uso directo (Legacy Support).
 * Retorna solo el array limpio, lanzando error si es inválido.
 */
export function purifyCookies(rawJson: string): PlaywrightCookie[] {
  const report = analyzeAndPurify(rawJson);
  if (!report.isValidStructure) {
    throw new Error("INVALID_JSON_FORMAT");
  }
  return report.cleanCookies;
}

function createEmptyReport(status: PurificationReport["status"]): PurificationReport {
  return {
    isValidStructure: false,
    totalInputCookies: 0,
    validCookiesCount: 0,
    purgedGarbageCount: 0,
    missingCriticalKeys: ["ALL"],
    cleanCookies: [],
    status
  };
}
// FIN DEL ARCHIVO [apps/web-dashboard/lib/utils/cookie-purifier.ts]

// INICIO DEL ARCHIVO [apps/web-dashboard/lib/utils/cookie-cleaner.ts]
/**
 * =================================================================
 * APARATO: ELITE COOKIE CLEANER (V2.0 - GOLDEN PATTERN)
 * CLASIFICACIÓN: UTILITY (ESTRATO L5)
 * RESPONSABILIDAD: SANITIZACIÓN QUIRÚRGICA Y DIAGNÓSTICO DE IDENTIDAD
 *
 * NOTA DE TRANSICIÓN: Este aparato reemplaza la lógica V1 básica.
 * Mantiene compatibilidad regresiva con 'purifyCookies' mientras
 * expone 'analyzeIdentity' para el nuevo UI de Gobernanza.
 * =================================================================
 */

import { z } from "zod";

// --- 1. CONSTANTES ESTRATÉGICAS (THE RING OF POWER) ---

/**
 * Solo estas cookies son necesarias para una sesión soberana en Colab/Kaggle.
 * Cualquier otra (analytics, ads, tracking) es ruido y riesgo.
 */
const GOLDEN_KEYS = new Set([
  // NÚCLEO DE IDENTIDAD (Quién eres)
  "SID", "HSID", "SSID",
  // AUTORIZACIÓN DE API (Permisos XHR)
  "APISID", "SAPISID",
  // CONTEXTO SEGURO (HTTPS & Cross-Site)
  "__Secure-1PSID", "__Secure-3PSID",
  "__Secure-1PAPISID", "__Secure-3PAPISID",
  // CONSISTENCIA TEMPORAL (Crítico Anti-Ban 2026)
  "__Secure-1PSIDTS", "__Secure-1PSIDCC",
  // CONTEXTO DE ENTORNO (Anti-Captcha)
  "NID", "AEC", "1P_JAR"
]);

const ALLOWED_DOMAINS = [".google.com", "google.com", ".colab.research.google.com"];

// --- 2. CONTRATOS DE TIPADO ---

const RawCookieSchema = z.object({
  name: z.string(),
  value: z.string(),
  domain: z.string().optional(),
  path: z.string().optional(),
  expirationDate: z.number().optional(),
  expires: z.number().optional(), // Soporte legacy para Puppeteer/EditThisCookie
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

// --- 3. MOTORES DE LÓGICA ---

/**
 * MOTOR DE ANÁLISIS FORENSE (V2.0)
 * Analiza, filtra y normaliza un JSON crudo. Genera un reporte para la UI.
 *
 * @param rawJson String JSON pegado por el usuario.
 */
export function analyzeIdentity(rawJson: string): PurificationReport {
  let parsed: unknown[];

  // A. Validación Estructural
  try {
    parsed = JSON.parse(rawJson);
    if (!Array.isArray(parsed)) throw new Error("Not an array");
  } catch {
    return createEmptyReport("INVALID");
  }

  // B. Filtrado Quirúrgico
  const validCookies: PlaywrightCookie[] = [];
  let garbageCount = 0;

  for (const item of parsed) {
    const parseResult = RawCookieSchema.safeParse(item);

    if (!parseResult.success) {
      garbageCount++; // Estructura de objeto inválida
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

    // C. Normalización al Estándar Playwright
    validCookies.push({
      name: cookie.name,
      value: cookie.value,
      domain: cookie.domain || ".google.com",
      path: cookie.path || "/",
      secure: true, // Forzamos HTTPS para Google siempre
      httpOnly: cookie.httpOnly ?? false,
      sameSite: "None", // Crítico para contextos cross-site en Colab
      expirationDate: cookie.expirationDate || cookie.expires
    });
  }

  // D. Auditoría de Integridad (Búsqueda de llaves maestras)
  const foundKeys = new Set(validCookies.map(c => c.name));
  // Estas 3 son obligatorias para que Google siquiera reconozca la sesión
  const criticalMissing = ["SID", "HSID", "__Secure-1PSID"].filter(k => !foundKeys.has(k));

  // E. Determinación de Estado
  let status: PurificationReport["status"] = "OPTIMAL";

  if (criticalMissing.length > 0) {
    status = "CRITICAL"; // Faltan llaves maestras
  } else if (validCookies.length < 5) {
    status = "DEGRADED"; // Tenemos lo básico, pero falta contexto (riesgo de captcha)
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
 * WRAPPER DE COMPATIBILIDAD (LEGACY SUPPORT)
 * Mantiene el contrato anterior para que 'identity-injector.tsx' no se rompa
 * mientras migramos la UI.
 *
 * @deprecated Usar analyzeIdentity() para obtener feedback visual.
 */
export function purifyCookies(rawJson: string): PlaywrightCookie[] {
  const report = analyzeIdentity(rawJson);

  // Si es inválido o crítico, lanzamos error para detener la inyección antigua
  if (report.status === "INVALID") {
    throw new Error("INVALID_JSON_FORMAT"); // Mensaje esperado por la UI vieja
  }

  // En V2, somos benevolentes: si faltan llaves, permitimos la inyección
  // pero el Worker probablemente fallará. La UI nueva usará analyzeIdentity
  // para advertir al usuario antes.
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
// FIN DEL ARCHIVO [apps/web-dashboard/lib/utils/cookie-cleaner.ts]

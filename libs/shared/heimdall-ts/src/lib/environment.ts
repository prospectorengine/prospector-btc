// INICIO DEL ARCHIVO [libs/shared/heimdall-ts/src/lib/environment.ts]
/**
 * =================================================================
 * APARATO: ENVIRONMENT SENSOR (V1.2 - UNIVERSAL COMPAT)
 * RESPONSABILIDAD: DETECCIÓN SEGURA DEL CONTEXTO DE EJECUCIÓN
 * =================================================================
 */

// ✅ FIX: Uso de globalThis para evitar error TS2304 'window'
export const isBrowser = typeof globalThis.window !== "undefined";
export const isProduction = process.env.NODE_ENV === "production";

/**
 * Obtiene un timestamp de alta resolución seguro para cualquier entorno.
 */
export function getMonotonicTime(): number {
  if (isBrowser && typeof performance !== "undefined") {
    return performance.now();
  }
  // Fallback para Node.js o entornos sin performance API
  return Date.now();
}
// FIN DEL ARCHIVO [libs/shared/heimdall-ts/src/lib/environment.ts]

// INICIO DEL ARCHIVO [libs/shared/heimdall-ts/src/lib/transports.ts]
/**
 * =================================================================
 * APARATO: LOG TRANSPORTS (V2.1 - FULL IMPLEMENTATION)
 * CLASIFICACIÓN: SHARED UTIL (ESTRATO L4/L5)
 * RESPONSABILIDAD: ENRUTAMIENTO Y FORMATEO DE SEÑALES DE TELEMETRÍA
 *
 * VISION HIPER-HOLÍSTICA:
 * Integra transportes para Consola (Dev), JSON (Server Prod) y
 * Beacon Remoto (Panopticon Uplink). Resuelve TS2304 y TS6133.
 * =================================================================
 */

import { LogEntry, LogSeverity } from "./types";
import { isProduction, isBrowser } from "./environment";

// Variables de estado para el enlace táctico (Singleton Module Pattern)
let UPLINK_ENDPOINT: string | null = null;
let SESSION_TOKEN: string | null = null;

/**
 * Mapa de estilos CSS para la consola del navegador (Dev Mode).
 * Consume LogSeverity para satisfacer el linter (TS6133).
 */
const STYLES: Record<LogSeverity, string> = {
  debug: "color: #a1a1aa", // Zinc-400
  info: "color: #3b82f6",  // Blue-500
  warn: "color: #eab308",  // Yellow-500
  error: "color: #ef4444; font-weight: bold", // Red-500
  critical: "background: #ef4444; color: white; font-weight: bold; padding: 2px 4px; border-radius: 2px",
};

/**
 * Inicializador de configuración para el transporte remoto.
 * Debe ser llamado al inicio de la app (Providers o Layout).
 */
export function configureHeimdallUplink(endpoint: string, token: string) {
  UPLINK_ENDPOINT = endpoint;
  SESSION_TOKEN = token;
}

/**
 * Emite el log a la consola del navegador con estilos visuales.
 * Utilizado en entornos de desarrollo local para legibilidad humana.
 */
function browserTransport(entry: LogEntry): void {
  const method = (entry.severity === "error" || entry.severity === "critical") ? "error" : "log";
  const style = STYLES[entry.severity] || STYLES.info;

  // eslint-disable-next-line no-console
  console[method](
    `%c[${entry.context}] %c${entry.message}`,
    "color: #71717a; font-weight: bold", // Context style (Zinc-500)
    style, // Severity style
    entry.metadata || ""
  );
}

/**
 * Emite el log como JSON estructurado en una sola línea.
 * Estándar para ingestión en CloudWatch, Datadog o Vercel Logs.
 */
function serverJsonTransport(entry: LogEntry): void {
  // eslint-disable-next-line no-console
  console.log(JSON.stringify({
    ts: entry.timestamp,
    lvl: entry.severity,
    ctx: entry.context,
    msg: entry.message,
    meta: entry.metadata
  }));
}

/**
 * Envía logs críticos al Orquestador mediante fetch (Fire-and-Forget).
 * Actúa como el "Nervio Óptico" del sistema Panóptico.
 */
function remoteBeaconTransport(entry: LogEntry): void {
  // 1. Filtro de Ruido: Solo enviamos WARN+ para no saturar el canal
  if (!UPLINK_ENDPOINT || !["warn", "error", "critical"].includes(entry.severity)) return;

  // 2. Prevención de Bucle: Evitamos que un error de red genere otro log de red infinito
  if (entry.context.includes("UPLINK_FAULT") || entry.context.includes("C2_Dispatch")) return;

  const payload = {
    id: crypto.randomUUID(),
    timestamp: entry.timestamp,
    stratum: isBrowser ? "L5_VIEW" : "L4_API",
    severity: entry.severity.toUpperCase(),
    message: `[${entry.context}] ${entry.message}`,
    metadata: entry.metadata,
    trace_id: entry.metadata?.trace_id as string | undefined
  };

  // 3. Disparo Asíncrono (Keepalive asegura entrega al cerrar pestaña)
  fetch(`${UPLINK_ENDPOINT}/telemetry/ingest`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "Authorization": `Bearer ${SESSION_TOKEN}`
    },
    body: JSON.stringify(payload),
    keepalive: true
  }).catch(() => { /* Protocolo Fail-Silent: Si falla, no generamos ruido local */ });
}

/**
 * ENRUTADOR PRINCIPAL DE SEÑALES (DISPATCHER)
 * Decide el destino del log basado en el entorno y la configuración.
 */
export function dispatchLog(entry: LogEntry): void {
  if (isProduction) {
    // EN PRODUCCIÓN:
    // 1. JSON estructurado para el recolector de logs de la infraestructura (Vercel/Docker)
    serverJsonTransport(entry);
    // 2. Uplink táctico para el Dashboard (si está configurado)
    remoteBeaconTransport(entry);
  } else {
    // EN DESARROLLO:
    if (typeof window === 'undefined') {
        // Entorno Node.js (Terminal): Usamos formato legible
        // eslint-disable-next-line no-console
        console.log(`[${entry.timestamp}] ${entry.severity.toUpperCase()} [${entry.context}] ${entry.message}`);
    } else {
        // Entorno Navegador (DevTools): Usamos estilos CSS
        browserTransport(entry);
    }
  }
}
// FIN DEL ARCHIVO [libs/shared/heimdall-ts/src/lib/transports.ts]

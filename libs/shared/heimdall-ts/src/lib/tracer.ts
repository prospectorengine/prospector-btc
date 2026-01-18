// INICIO DEL ARCHIVO [libs/shared/heimdall-ts/src/lib/tracer.ts]
/**
 * =================================================================
 * APARATO: CHRONOS TRACER (V2.1 - TYPE FIXED)
 * RESPONSABILIDAD: MEDICIÓN DE INTERVALOS DE OPERACIÓN
 * =================================================================
 */

import { getMonotonicTime, isBrowser, isProduction } from "./environment";
import { SovereignLogger } from "./logger";
// ✅ FIX: Importar el tipo correcto
import { AuditMetadata } from "./types";

export class OperationSpan {
  private start_time: number;
  private label: string;
  private logger: SovereignLogger;

  constructor(logger: SovereignLogger, label: string) {
    this.logger = logger;
    this.label = label;
    this.start_time = getMonotonicTime();

    if (isBrowser && !isProduction) {
      // eslint-disable-next-line no-console
      console.groupCollapsed(`%c ⏱️ [START] ${label}`, "color: #a8a29e; font-weight: bold;");
    }
  }

  /**
   * Cierra la traza exitosamente.
   * ✅ FIX: Tipado estricto AuditMetadata para evitar error TS2345
   */
  public ok(metadata?: AuditMetadata): void {
    const duration = getMonotonicTime() - this.start_time;
    const enrichedMeta = { ...metadata, duration_ms: duration };
    this.finalize("info", `✅ [END] ${this.label} completed in ${duration.toFixed(2)}ms`, enrichedMeta);
  }

  /**
   * Cierra la traza con error.
   */
  public fail(error: Error | unknown): void {
    const duration = getMonotonicTime() - this.start_time;
    const errorMeta: AuditMetadata = error instanceof Error
      ? { message: error.message, stack: error.stack, duration_ms: duration }
      : { error: String(error), duration_ms: duration };

    this.finalize("error", `❌ [FAIL] ${this.label} failed after ${duration.toFixed(2)}ms`, errorMeta);
  }

  private finalize(level: "info" | "error", msg: string, meta?: AuditMetadata): void {
    if (isBrowser && !isProduction) {
      // eslint-disable-next-line no-console
      console.groupEnd();
    }

    if (level === "info") this.logger.info(msg, meta);
    else this.logger.error(msg, meta);
  }
}
// FIN DEL ARCHIVO [libs/shared/heimdall-ts/src/lib/tracer.ts]

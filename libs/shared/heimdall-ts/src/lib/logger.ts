
// INICIO DEL ARCHIVO [libs/shared/heimdall-ts/src/lib/logger.ts]
/**
 * =================================================================
 * APARATO: SOVEREIGN LOGGER CORE (V2.0)
 * RESPONSABILIDAD: FACHADA PÚBLICA DE OBSERVABILIDAD
 * =================================================================
 */

import { LogSeverity, AuditMetadata } from "./types";
import { dispatchLog } from "./transports";
import { OperationSpan } from "./tracer";

export class SovereignLogger {
  private readonly context: string;

  constructor(context: string) {
    this.context = context;
  }

  /**
   * Inicia el rastreo de una operación de larga duración.
   * Retorna un objeto Span para cerrar la operación.
   */
  public track(operation_name: string): OperationSpan {
    return new OperationSpan(this, `${this.context}:${operation_name}`);
  }

  private log(severity: LogSeverity, message: string, metadata?: AuditMetadata): void {
    dispatchLog({
      timestamp: new Date().toISOString(),
      severity,
      context: this.context,
      message,
      metadata
    });
  }

  public debug(msg: string, meta?: AuditMetadata) { this.log("debug", msg, meta); }
  public info(msg: string, meta?: AuditMetadata) { this.log("info", msg, meta); }
  public warn(msg: string, meta?: AuditMetadata) { this.log("warn", msg, meta); }
  public error(msg: string, meta?: AuditMetadata) { this.log("error", msg, meta); }
  public critical(msg: string, meta?: AuditMetadata) { this.log("critical", msg, meta); }
}
// FIN DEL ARCHIVO [libs/shared/heimdall-ts/src/lib/logger.ts]

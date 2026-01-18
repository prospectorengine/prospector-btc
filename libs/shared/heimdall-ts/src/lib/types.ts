// INICIO DEL ARCHIVO [libs/shared/heimdall-ts/src/lib/types.ts]
// Re-visitamos types.ts para asegurar que AuditMetadata sea compatible con Index Signature
/**
 * =================================================================
 * APARATO: HEIMDALL TYPES (V1.1 - FLEXIBLE)
 * =================================================================
 */

export type LogSeverity = "debug" | "info" | "warn" | "error" | "critical";

// ✅ FIX: Record<string, unknown> permite cualquier propiedad adicional
export interface AuditMetadata extends Record<string, unknown> {
  stratum_identifier?: string;
  operation_id?: string;
  duration_ms?: number;
  error_code?: string;
  user_id?: string;
}

export interface LogEntry {
  timestamp: string;
  severity: LogSeverity;
  context: string;
  message: string;
  metadata?: AuditMetadata;
}
// FIN DEL ARCHIVO [libs/shared/heimdall-ts/src/lib/types.ts]

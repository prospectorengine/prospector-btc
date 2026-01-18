/**
 * =================================================================
 * APARATO: DOMAIN UNIFIED SCHEMAS (V84.0 - SILICON AWARE)
 * CLASIFICACIÓN: DOMAIN CONTRACTS (ESTRATO L2)
 * RESPONSABILIDAD: FUENTE ÚNICA DE VERDAD (SSoT) PARA EL ENLACE NEURAL
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. HARDWARE AWARENESS: Inyección de 'supports_avx2' en el esquema de métricas
 *    para certificar la capacidad de aceleración vectorial en el Dashboard L5.
 * 2. TYPE SOVEREIGNTY: Sincronización bit-perfecta con el modelo perimetral
 *    de Rust (telemetry.rs), eliminando discrepancias en la desincronización de la Tríada.
 * 3. ZERO ABBREVIATIONS: Mantenimiento de nomenclatura nominal descriptiva.
 * 4. HYGIENE: Documentación técnica nivel Tesis Doctoral en cada esquema atómico.
 *
 * # Mathematical Proof (Neural Synchronization):
 * El esquema garantiza que el flujo binario des-serializado por el NeuralCodec
 * cumpla estrictamente con la topología de datos exigida por los Hooks reactivos,
 * previniendo estados 'undefined' en el Panóptico.
 * =================================================================
 */

import { z } from "zod";

/**
 * -----------------------------------------------------------------
 * ESTRATO 1: INFRAESTRUCTURA & DIAGNÓSTICO (L6)
 * -----------------------------------------------------------------
 */

/**
 * Niveles de salud operativa para subsistemas de persistencia y red.
 */
export const IntegrityStatusSchema = z.enum([
  'OPERATIONAL',
  'DEGRADED',
  'CRITICAL'
]);
export type IntegrityStatus = z.infer<typeof IntegrityStatusSchema>;

/**
 * Reporte de integridad de subsistemas internos (Auditor, Inspector, Dumper).
 */
export const SystemIntegrityReportSchema = z.object({
  identifier: z.string().uuid().optional(),
  apparatus_name: z.string(),
  status: IntegrityStatusSchema,
  metrics: z.record(z.string(), z.unknown()),
  detected_at_timestamp: z.string().datetime(),
});
export type SystemIntegrityReport = z.infer<typeof SystemIntegrityReportSchema>;

/**
 * -----------------------------------------------------------------
 * ESTRATO 2: HARDWARE & SILICIO (L1)
 * -----------------------------------------------------------------
 */

/**
 * Telemetría detallada de hardware de un nodo individual.
 * ✅ NIVELACIÓN V84.0: Inyección de capacidad de aceleración vectorial.
 */
export const NodeHardwareMetricsSchema = z.object({
  cpu_frequency_megahertz: z.number().nonnegative(),
  cpu_load_percentage: z.number().min(0).max(100),
  cpu_temperature_celsius: z.number(),
  ram_usage_megabytes: z.number().nonnegative(),
  is_thermal_throttling_active: z.boolean().describe("Indica si el procesador está limitando su potencia por calor"),
  /**
   * Identificador de soporte para instrucciones Advanced Vector Extensions 2.
   * Crítico para la discriminación de nodos ELITE en el SequentialEngine.
   */
  supports_avx2: z.boolean().default(false),
});
export type NodeHardwareMetrics = z.infer<typeof NodeHardwareMetricsSchema>;

/**
 * -----------------------------------------------------------------
 * ESTRATO 3: VIGILANCIA & TELEMETRÍA (L3)
 * -----------------------------------------------------------------
 */

/**
 * Latido de vida de un nodo del enjambre.
 */
export const WorkerHeartbeatSchema = z.object({
  worker_identifier: z.string(),
  hostname_identity: z.string(),
  current_hashrate: z.number().int().nonnegative(),
  active_job_identifier: z.string().uuid().nullable(),
  timestamp_utc: z.string().datetime(),
  hardware_metrics: NodeHardwareMetricsSchema,
});
export type WorkerHeartbeat = z.infer<typeof WorkerHeartbeatSchema>;

/**
 * Instantánea visual y operativa para el Panóptico.
 */
export const WorkerSnapshotSchema = z.object({
  worker_identifier: z.string(),
  operational_status: z.enum(["running", "captcha", "error", "idle"]),
  snapshot_base64_data: z.string(),
  captured_at_timestamp: z.string().datetime(),
  hardware_metrics: NodeHardwareMetricsSchema.optional(),
});
export type WorkerSnapshot = z.infer<typeof WorkerSnapshotSchema>;

/**
 * -----------------------------------------------------------------
 * ESTRATO 4: AUDITORÍA & MISIÓN (L2)
 * -----------------------------------------------------------------
 */

/**
 * Reporte final certificado de una misión de búsqueda.
 */
export const AuditReportSchema = z.object({
  job_mission_identifier: z.string().uuid(),
  worker_node_identifier: z.string(),
  computational_effort_volume: z.string(),
  execution_duration_milliseconds: z.number().nonnegative(),
  final_mission_status: z.string(),
  audit_footprint_checkpoint: z.string(),
  completed_at_timestamp: z.string().datetime(),
});
export type AuditReport = z.infer<typeof AuditReportSchema>;

/**
 * Segmento para el mapa de calor espacial (Curva Elíptica).
 */
export const SwarmHeatmapSegmentSchema = z.object({
  normalized_start_position: z.number().min(0).max(1),
  intensity_weight: z.number().min(0).max(1),
  mission_identifier: z.string().uuid(),
});
export type SwarmHeatmapSegment = z.infer<typeof SwarmHeatmapSegmentSchema>;

/**
 * Resumen global de métricas del sistema (Pulse).
 */
export const SystemMetricsSchema = z.object({
  active_nodes_count: z.number().int().nonnegative(),
  cumulative_global_hashrate: z.number().nonnegative(),
  active_missions_in_flight: z.number().int().nonnegative(),
  timestamp_ms: z.number().positive(),
});
export type SystemMetrics = z.infer<typeof SystemMetricsSchema>;

/**
 * -----------------------------------------------------------------
 * ESTRATO 5: OBSERVABILIDAD DE MANDO C2 (V80.0)
 * -----------------------------------------------------------------
 */

/**
 * Registro de navegación del automatizador (Playwright).
 */
export const ProvisioningLogSchema = z.object({
  node_index: z.number(),
  message: z.string(),
  level: z.enum(["INFO", "WARN", "CRITICAL"]),
  timestamp: z.string().datetime(),
});
export type ProvisioningLog = z.infer<typeof ProvisioningLogSchema>;

/**
 * Estado del Escudo de Baneo basado en inventario de Bóveda.
 */
export const BanShieldStatusSchema = z.object({
  identities_in_vault: z.number(),
  safe_node_capacity: z.number(),
  is_ignition_authorized: z.boolean(),
  restriction_reason: z.string().optional(),
});
export type BanShieldStatus = z.infer<typeof BanShieldStatusSchema>;

/**
 * -----------------------------------------------------------------
 * ESTRATO 6: OBSERVABILIDAD UNIFICADA (V81.0 - PANOPTICON)
 * -----------------------------------------------------------------
 */

export const LogSeveritySchema = z.enum(["DEBUG", "INFO", "WARN", "ERROR", "CRITICAL"]);
export type LogSeverity = z.infer<typeof LogSeveritySchema>;

export const SystemStrataSchema = z.enum([
  "L1_CORE",      // Matemática y Criptografía
  "L2_STRATEGY",  // Lógica de Negocio
  "L3_ORCH",      // Orquestador y Estado
  "L4_API",       // Capa de Adaptadores
  "L5_VIEW",      // Interfaz de Usuario
  "L6_OPS"        // Infraestructura y Scripts
]);
export type SystemStrata = z.infer<typeof SystemStrataSchema>;

/**
 * Estructura canónica para el "Heimdall Unified Stream".
 * Centraliza logs de Rust, TypeScript, Python y SQL.
 */
export const SystemLogSchema = z.object({
  id: z.string().uuid(),
  timestamp: z.string().datetime(),
  stratum: SystemStrataSchema,
  severity: LogSeveritySchema,
  message: z.string(),
  // Metadatos flexibles para contexto técnico (Stack trace, Latency, UserID)
  metadata: z.record(z.string(), z.unknown()).optional(),
  trace_id: z.string().optional()
});
export type SystemLog = z.infer<typeof SystemLogSchema>;

/**
 * -----------------------------------------------------------------
 * UNIÓN DISCRIMINADA SOBERANA (RealTimeEvent)
 * SSoT para la comunicación SSE/WS Orquestador -> Dashboard.
 * -----------------------------------------------------------------
 */
export type RealTimeEvent =
  | { t: "sp"; p: SystemMetrics }
  | { t: "ac"; p: AuditReport }
  | { t: "sh"; p: SwarmHeatmapSegment[] }
  | { t: "ir"; p: SystemIntegrityReport }
  | { t: "cc"; p: { target_bitcoin_address: string; discovery_node: string } }
  | { t: "ad"; p: { drift_gap_count: number; total_tactical_count: number } }
  | { t: "vr"; p: { worker_identifier: string; operational_status: string; system_timestamp: number } }
  | { t: "pl"; p: ProvisioningLog }
  | { t: "bs"; p: BanShieldStatus }
  | { t: "sl"; p: SystemLog };

export const RealTimeEventSchema = z.discriminatedUnion("t", [
  z.object({ t: z.literal("sp"), p: SystemMetricsSchema }),
  z.object({ t: z.literal("ac"), p: AuditReportSchema }),
  z.object({ t: z.literal("sh"), p: z.array(SwarmHeatmapSegmentSchema) }),
  z.object({ t: z.literal("ir"), p: SystemIntegrityReportSchema }),
  z.object({
    t: z.literal("cc"),
    p: z.object({ target_bitcoin_address: z.string(), discovery_node: z.string() })
  }),
  z.object({
    t: z.literal("ad"),
    p: z.object({ drift_gap_count: z.number(), total_tactical_count: z.number() })
  }),
  z.object({
    t: z.literal("vr"),
    p: z.object({ worker_identifier: z.string(), operational_status: z.string(), system_timestamp: z.number() })
  }),
  z.object({ t: z.literal("pl"), p: ProvisioningLogSchema }),
  z.object({ t: z.literal("bs"), p: BanShieldStatusSchema }),
  z.object({ t: z.literal("sl"), p: SystemLogSchema }),
]);

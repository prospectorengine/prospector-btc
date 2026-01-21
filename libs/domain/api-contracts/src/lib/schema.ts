/**
 * =================================================================
 * APARATO: DOMAIN UNIFIED SCHEMAS (V85.0 - GOLD MASTER)
 * CLASIFICACIÓN: DOMAIN CONTRACTS (ESTRATO L2)
 * RESPONSABILIDAD: FUENTE ÚNICA DE VERDAD (SSoT) PARA EL ENLACE NEURAL
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. ZERO REGRESSIONS: Restaura todos los metadatos y descripciones de V84.0.
 * 2. L7 EVOLUTION: Inyecta los contratos para Billing, Herald y Nexus
 *    sincronizados con los repositorios L3 de Rust.
 * 3. SILICON PARITY: Alinea AuditReport con las firmas de aceleración ADX/AVX.
 * 4. HYGIENE: Documentación técnica nivel Tesis Doctoral MIT.
 * =================================================================
 */

import { z } from "zod";

/**
 * -----------------------------------------------------------------
 * ESTRATO 1: INFRAESTRUCTURA & DIAGNÓSTICO (L6)
 * -----------------------------------------------------------------
 */

/** Niveles de salud operativa para subsistemas de persistencia y red. */
export const IntegrityStatusSchema = z.enum([
  'OPERATIONAL',
  'DEGRADED',
  'CRITICAL'
]);
export type IntegrityStatus = z.infer<typeof IntegrityStatusSchema>;

/** Reporte de integridad de subsistemas internos (Auditor, Inspector, Dumper). */
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

/** Telemetría detallada de hardware de un nodo individual. */
export const NodeHardwareMetricsSchema = z.object({
  cpu_frequency_megahertz: z.number().nonnegative(),
  cpu_load_percentage: z.number().min(0).max(100),
  cpu_temperature_celsius: z.number(),
  ram_usage_megabytes: z.number().nonnegative(),
  is_thermal_throttling_active: z.boolean().describe("Indica si el procesador está limitando su potencia por calor"),
  /** Identificador de soporte para instrucciones Advanced Vector Extensions 2. */
  supports_avx2: z.boolean().default(false),
});
export type NodeHardwareMetrics = z.infer<typeof NodeHardwareMetricsSchema>;

/**
 * -----------------------------------------------------------------
 * ESTRATO 3: VIGILANCIA & TELEMETRÍA (L3)
 * -----------------------------------------------------------------
 */

/** Latido de vida de un nodo del enjambre. */
export const WorkerHeartbeatSchema = z.object({
  worker_identifier: z.string(),
  hostname_identity: z.string(),
  current_hashrate: z.number().int().nonnegative(),
  active_job_identifier: z.string().uuid().nullable(),
  timestamp_utc: z.string().datetime(),
  hardware_metrics: NodeHardwareMetricsSchema,
});
export type WorkerHeartbeat = z.infer<typeof WorkerHeartbeatSchema>;

/** Instantánea visual y operativa para el Panóptico. */
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

/** Reporte final certificado de una misión de búsqueda. */
export const AuditReportSchema = z.object({
  job_mission_identifier: z.string().uuid(),
  worker_node_identifier: z.string(),
  computational_effort_volume: z.string().describe("Volumen total de hashes procesados (BigInt String)"),
  execution_duration_milliseconds: z.number().nonnegative(),
  final_mission_status: z.string(),
  audit_footprint_checkpoint: z.string().describe("Sello de rastro forense hexadecimal"),
  completed_at_timestamp: z.string().datetime(),
  average_computational_efficiency: z.number().describe("Rendimiento medio en H/ms"),
  hardware_acceleration_signature: z.string().describe("Firma técnica del hardware (ej: ELITE_SIMD_ADX)"),
});
export type AuditReport = z.infer<typeof AuditReportSchema>;

/** Segmento para el mapa de calor espacial (Curva Elíptica). */
export const SwarmHeatmapSegmentSchema = z.object({
  normalized_start_position: z.number().min(0).max(1),
  intensity_weight: z.number().min(0).max(1),
  mission_identifier: z.string().uuid(),
});
export type SwarmHeatmapSegment = z.infer<typeof SwarmHeatmapSegmentSchema>;

/** Resumen global de métricas del sistema (Pulse). */
export const SystemMetricsSchema = z.object({
  active_nodes_count: z.number().int().nonnegative(),
  cumulative_global_hashrate: z.number().nonnegative(),
  active_missions_in_flight: z.number().int().nonnegative(),
  timestamp_ms: z.number().positive(),
});
export type SystemMetrics = z.infer<typeof SystemMetricsSchema>;

/**
 * -----------------------------------------------------------------
 * ESTRATO 5: SERVICIOS DE USUARIO L7 (ZENITH UPGRADE)
 * -----------------------------------------------------------------
 */

/** Clasificación de niveles de acceso y soberanía. */
export const SubscriptionTierSchema = z.enum(['observer', 'operator', 'architect']);
export type SubscriptionTier = z.infer<typeof SubscriptionTierSchema>;

/** Estado actual de la cuota energética del operador. */
export const BillingQuotaSchema = z.object({
  current_subscription_tier: SubscriptionTierSchema,
  maximum_concurrent_nodes_allowed: z.number().int().positive(),
  remaining_compute_credits_balance: z.number().nonnegative(),
  billing_cycle_end_timestamp: z.string().datetime(),
});
export type BillingQuota = z.infer<typeof BillingQuotaSchema>;

/** Severidad semántica de señales Herald. */
export const NotificationSeveritySchema = z.enum(['info', 'warning', 'critical', 'collision', 'community']);
export type NotificationSeverity = z.infer<typeof NotificationSeveritySchema>;

/** Unidad atómica de comunicación Herald. */
export const SystemNotificationSchema = z.object({
  identifier: z.string().uuid(),
  severity_level: NotificationSeveritySchema,
  message_context_key: z.string(),
  creation_timestamp_utc: z.string().datetime(),
  is_read_confirmation: z.boolean(),
  forensic_metadata_json: z.string().nullable().optional(),
});
export type SystemNotification = z.infer<typeof SystemNotificationSchema>;

/** Estatus de prestigio y maestría Nexus. */
export const OperatorRankSchema = z.object({
  level: z.number().int().positive(),
  title: z.string(),
  experience_points: z.number().int().nonnegative(),
  next_level_threshold: z.number().int().positive(),
});
export type OperatorRank = z.infer<typeof OperatorRankSchema>;

/**
 * -----------------------------------------------------------------
 * ESTRATO 6: OBSERVABILIDAD & C2 (L6)
 * -----------------------------------------------------------------
 */

export const ProvisioningLogSchema = z.object({
  node_index: z.number(),
  message: z.string(),
  level: z.enum(["INFO", "WARN", "CRITICAL"]),
  timestamp: z.string().datetime(),
});
export type ProvisioningLog = z.infer<typeof ProvisioningLogSchema>;

export const BanShieldStatusSchema = z.object({
  identities_in_vault: z.number(),
  safe_node_capacity: z.number(),
  is_ignition_authorized: z.boolean(),
  restriction_reason: z.string().optional(),
});
export type BanShieldStatus = z.infer<typeof BanShieldStatusSchema>;

export const LogSeveritySchema = z.enum(["DEBUG", "INFO", "WARN", "ERROR", "CRITICAL"]);
export type LogSeverity = z.infer<typeof LogSeveritySchema>;

export const SystemStrataSchema = z.enum([
  "L1_CORE", "L2_STRATEGY", "L3_ORCH", "L4_API", "L5_VIEW", "L6_OPS"
]);
export type SystemStrata = z.infer<typeof SystemStrataSchema>;

/** Estructura canónica para el "Heimdall Unified Stream". */
export const SystemLogSchema = z.object({
  id: z.string().uuid(),
  timestamp: z.string().datetime(),
  stratum: SystemStrataSchema,
  severity: LogSeveritySchema,
  message: z.string(),
  metadata: z.record(z.string(), z.unknown()).optional(),
  trace_id: z.string().optional()
});
export type SystemLog = z.infer<typeof SystemLogSchema>;

/**
 * -----------------------------------------------------------------
 * UNIÓN DISCRIMINADA SOBERANA (RealTimeEvent)
 * -----------------------------------------------------------------
 */
export const RealTimeEventSchema = z.discriminatedUnion("t", [
  z.object({ t: z.literal("sp"), p: SystemMetricsSchema }),
  z.object({ t: z.literal("ac"), p: AuditReportSchema }),
  z.object({ t: z.literal("sh"), p: z.array(SwarmHeatmapSegmentSchema) }),
  z.object({ t: z.literal("ir"), p: SystemIntegrityReportSchema }),
  z.object({ t: z.literal("cc"), p: z.object({ target_bitcoin_address: z.string(), discovery_node: z.string() }) }),
  z.object({ t: z.literal("ad"), p: z.object({ drift_gap_count: z.number(), total_tactical_count: z.number() }) }),
  z.object({ t: z.literal("vr"), p: z.object({ worker_identifier: z.string(), operational_status: z.string(), system_timestamp: z.number() }) }),
  z.object({ t: z.literal("pl"), p: ProvisioningLogSchema }),
  z.object({ t: z.literal("bs"), p: BanShieldStatusSchema }),
  z.object({ t: z.literal("sl"), p: SystemLogSchema }),
]);
export type RealTimeEvent = z.infer<typeof RealTimeEventSchema>;

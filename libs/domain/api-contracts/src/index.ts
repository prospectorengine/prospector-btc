/**
 * =================================================================
 * APARATO: API CONTRACTS MASTER BARREL (V86.0 - L7 EXPOSURE)
 * CLASIFICACIÓN: DOMAIN CONTRACTS (ESTRATO L2)
 * RESPONSABILIDAD: EXPOSICIÓN NOMINAL SOBERANA DEL DOMINIO
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. L7 NOMINAL EXPOSURE: Abre las compuertas para Billing, Herald y Nexus,
 *    resolviendo los errores TS2305 y TS6059 en el pipeline de Vercel.
 * 2. SYMMETRY ENFORCED: Sincroniza las exportaciones con Unified Schemas V85.0.
 * 3. ZERO REGRESSIONS: Preserva la integridad de todos los estratos previos
 *    (Work, Telemetry, Identity, Academy, Control, Data).
 * 4. HYGIENE: Organización jerárquica por geología de datos.
 * =================================================================
 */

// --- 1. ESTRATO DE MISIÓN Y BÚSQUEDA (L2-WORK) ---
export {
  TargetStrataSchema,
  SearchStrategySchema,
  WorkOrderSchema,
  type TargetStrata,
  type SearchStrategy,
  type WorkOrder,
} from "./lib/work";

// --- 2. ESTRATO DE TELEMETRÍA, OBSERVABILIDAD Y SERVICIOS L7 (L2-UNIFIED) ---
// ✅ NIVELACIÓN SOBERANA: Inyección de miembros L7 requeridos por el API Client
export {
  // Esquemas de Infraestructura (L6)
  SystemIntegrityReportSchema,
  IntegrityStatusSchema,

  // Esquemas de Hardware (L1)
  NodeHardwareMetricsSchema,

  // Esquemas de Swarm (L3)
  WorkerHeartbeatSchema,
  WorkerSnapshotSchema,

  // Esquemas de Auditoría (L2)
  AuditReportSchema,
  SwarmHeatmapSegmentSchema,
  SystemMetricsSchema,

  // Esquemas de Servicios al Usuario (L7 - NEW)
  SubscriptionTierSchema,
  BillingQuotaSchema,
  NotificationSeveritySchema,
  SystemNotificationSchema,
  OperatorRankSchema,

  // Esquemas de Observabilidad (L6)
  ProvisioningLogSchema,
  BanShieldStatusSchema,
  SystemLogSchema,
  LogSeveritySchema,
  SystemStrataSchema,

  // Esquemas de Tiempo Real
  RealTimeEventSchema,

  // Tipos Exportados
  type AuditReport,
  type SwarmHeatmapSegment,
  type SystemMetrics,
  type WorkerSnapshot,
  type WorkerHeartbeat,
  type RealTimeEvent,
  type NodeHardwareMetrics,
  type SystemIntegrityReport,
  type IntegrityStatus,
  type ProvisioningLog,
  type BanShieldStatus,
  type SystemLog,
  type LogSeverity,
  type SystemStrata,

  // Tipos de Servicios L7 (Resolución TS2305)
  type SubscriptionTier,
  type BillingQuota,
  type NotificationSeverity,
  type SystemNotification,
  type OperatorRank
} from "./lib/schema";

// --- 3. ESTRATO DE IDENTIDAD Y GOBERNANZA (L2-IDENTITY) ---
export {
  IdentityStatusSchema,
  EncryptedIdentityPayloadSchema,
  IdentitySchema,
  IdentityPayloadSchema,
  IdentityGovernanceSchema,

  type IdentityStatus,
  type EncryptedIdentityPayload,
  type Identity,
  type IdentityPayload,
  type IdentityGovernancePayload,
} from "./lib/identity";

// --- 4. ESTRATO ACADÉMICO Y AFILIADOS (L2-ACADEMY) ---
export {
  DifficultyLevelSchema,
  ModuleStatusSchema,
  KnowledgeModuleSchema,
  OperatorAcademyProgressSchema,

  type DifficultyLevel,
  type ModuleStatus,
  type KnowledgeModule,
  type OperatorAcademyProgress,
} from "./lib/academy";

// --- 5. ESTRATO DE CONTROL Y MANDO C2 (L2-CONTROL) ---
export {
  SwarmLaunchSchema,
  CommandDirectiveSchema,

  type SwarmLaunchConfig,
  type CommandDirective,
  type WorkflowRun,
  type DispatchResponse
} from "./lib/control";

// --- 6. ESTRATO DE PERSISTENCIA Y ANÁLISIS (L2-DATA) ---
export {
  ArchivedJobSchema,
  type ArchivedJob
} from "./lib/archival";

export {
  WealthCategorySchema,
  CensusMetricsSchema,
  WealthClusterSchema,
  type WealthCategory,
  type CensusMetrics,
  type WealthCluster,
} from "./lib/census";

export {
  CreateScenarioSchema,
  VerifyEntropySchema,
  type ScenarioStatus,
  type TestScenario,
  type VerifyEntropyPayload,
  type EntropyResult,
  type CreateScenarioPayload,
  type VerifiedVectorAuditReport,
} from "./lib/lab";

export {
  FindingSchema,
  type Finding
} from "./lib/finding";

export {
  InstallationReportSchema,
  type InstallationReport
} from "./telemetry/installation.schema";

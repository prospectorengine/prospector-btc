/**
 * =================================================================
 * APARATO: API CONTRACTS MASTER BARREL (V85.1 - C2 FIXED)
 * CLASIFICACIÓN: DOMAIN CONTRACTS (ESTRATO L2)
 * RESPONSABILIDAD: EXPOSICIÓN NOMINAL SOBERANA DEL DOMINIO
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

// --- 2. ESTRATO DE TELEMETRÍA Y OBSERVABILIDAD (L2-TELEMETRY) ---
export {
  AuditReportSchema,
  SwarmHeatmapSegmentSchema,
  SystemMetricsSchema,
  WorkerSnapshotSchema,
  WorkerHeartbeatSchema,
  RealTimeEventSchema,
  SystemIntegrityReportSchema,
  IntegrityStatusSchema,
  ProvisioningLogSchema,
  BanShieldStatusSchema,
  SystemLogSchema,
  LogSeveritySchema,
  SystemStrataSchema,

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
  type SystemStrata
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
  type DispatchResponse // ✅ ENLACE CERTIFICADO
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

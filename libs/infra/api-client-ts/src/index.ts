/**
 * =================================================================
 * APARATO: API CLIENT MASTER BARREL (V75.5 - L7 ALIGNMENT)
 * CLASIFICACIÓN: INFRASTRUCTURE FACADE (ESTRATO L4)
 * RESPONSABILIDAD: EXPOSICIÓN NOMINAL SOBERANA DEL SISTEMA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. L7 FAÇADE EXPOSURE: Sella el error de compilación de Vercel al
 *    exportar nominalmente billingApi, heraldApi y nexusApi.
 * 2. SYMMETRY ENFORCED: Sincroniza la visibilidad de los adaptadores
 *    tácticos con los requisitos del Dashboard Zenith (L5).
 * 3. NOMINAL PURITY: Mantenimiento de la jerarquía de contratos L2.
 * 4. ZERO REGRESSIONS: Preserva la integridad de los túneles WebSocket
 *    y los hooks reactivos de telemetría.
 * =================================================================
 */

// --- ESTRATO 1: CONTRATOS DE DOMINIO (L2) ---
// SSoT: Fuente única de verdad para esquemas y tipos Zod.
export * from "@prospector/api-contracts";

// --- ESTRATO 2: ADAPTADORES TÁCTICOS Y SERVICIOS L7 (L4) ---
// RESOLUCIÓN BUILD ERR: Inyección de fachadas de servicios al usuario.
export {
  apiClient,
  nextApiClient,
  neuralOracle,
  billingApi,
  heraldApi,
  nexusApi
} from "./lib/client";

export { adminApi } from "./lib/admin";
export { controlApi } from "./lib/control";

/**
 * ESTRATO DE LABORATORIO Y CERTIFICACIÓN
 */
export {
  labApi,
  type CertificationIgnitionResponse
} from "./lib/lab";

// --- ESTRATO 3: TRANSPORTE NEURAL Y CODECS ---
export { NeuralSocket } from "./lib/socket-client";
export { NeuralCodec } from "./lib/neural-codec";

// --- ESTRATO 4: MOTOR ESTRATÉGICO (ENGINE B - SUPABASE) ---
export {
  strategicArchive,
  strategicCensus,
  supabase
} from "@prospector/infra-supabase";

// --- ESTRATO 5: SINAPSIS REACTIVA (HOOKS L5) ---
export {
  useNeuralLink,
  useRealTimeTelemetry,
  type NeuralLinkInterface,
  type ArchivalSynchronizationDrift
} from "./lib/hooks-rt";

export {
  useSystemTelemetry,
  type TelemetryMetricsSummary
} from "./lib/hooks";

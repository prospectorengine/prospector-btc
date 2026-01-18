/**
 * =================================================================
 * APARATO: API CLIENT MASTER BARREL (V75.1 - PATH ALIGNED)
 * CLASIFICACIÓN: INFRASTRUCTURE FACADE (ESTRATO L4)
 * RESPONSABILIDAD: EXPOSICIÓN NOMINAL SOBERANA DEL SISTEMA
 * =================================================================
 */

// --- 1. ESTRATOS DE CONTRATOS DE DOMINIO (L2) ---
export * from "@prospector/api-contracts";

// --- 2. ADAPTADORES DE INFRAESTRUCTURA TÁCTICA (L4) ---
export { apiClient, nextApiClient, neuralOracle } from "./lib/client";
export { adminApi } from "./lib/admin";
export { controlApi } from "./lib/control";

/**
 * ESTRATO DE LABORATORIO
 */
export {
  labApi,
  type CertificationIgnitionResponse
} from "./lib/lab";

// ✅ RESOLUCIÓN TS2307: Ruta corregida a './lib/neural-codec'
export { NeuralSocket } from "./lib/socket-client";
export { NeuralCodec } from "./lib/neural-codec";

// --- 3. MOTOR ESTRATÉGICO (ENGINE B) ---
export {
  strategicArchive,
  strategicCensus,
  supabase
} from "@prospector/infra-supabase";

// --- 4. SINAPSIS NEURAL (HOOKS REACTIVOS L5) ---
// ✅ RESOLUCIÓN TS2305: 'useRealTimeTelemetry' ahora exportado nominalmente en hooks-rt.ts
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

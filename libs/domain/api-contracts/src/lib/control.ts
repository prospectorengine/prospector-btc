/**
 * =================================================================
 * APARATO: SWARM CONTROL CONTRACTS (V2.3 - DISPATCH FIXED)
 * CLASIFICACIÓN: DOMAIN CONTRACTS (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DE ÓRDENES TÁCTICAS Y RESPUESTAS C2
 * =================================================================
 */

import { z } from "zod";

/**
 * Esquema de Directiva Táctica (Upstream).
 * Sincronizado con el Serde Tag de Rust.
 */
export const CommandDirectiveSchema = z.discriminatedUnion("action", [
  z.object({
    action: z.literal("HaltSwarm"),
    payload: z.object({ reason: z.string() }),
  }),
  z.object({
    action: z.literal("IgniteSwarm"),
  }),
  z.object({
    action: z.literal("PurgeLedger"),
  }),
  z.object({
    action: z.literal("SetGlobalStrategy"),
    payload: z.object({ strategy: z.string() }),
  }),
]);

export type CommandDirective = z.infer<typeof CommandDirectiveSchema>;

export const SwarmLaunchSchema = z.object({
  worker_count: z.number().min(1).max(50).default(30),
  shard_count: z.number().min(1).max(20).default(5),
  ref: z.string().default("main"),
});

export type SwarmLaunchConfig = z.infer<typeof SwarmLaunchSchema>;

export interface WorkflowRun {
  id: number;
  name: string;
  status: "queued" | "in_progress" | "completed" | "failure" | "cancelled";
  conclusion: string | null;
  created_at: string;
  html_url: string;
  run_number: number;
}

/**
 * RESPUESTA SOBERANA DE DESPACHO
 * ✅ REPARACIÓN TS2305: Interface exportada explícitamente.
 */
export interface DispatchResponse {
  success: boolean;
  message_code: "IGNITION_ACCEPTED" | "AUTH_FAILURE" | "CONFIG_ERROR" | "IGNITION_REJECTED" | "C2_SYSTEM_COLLAPSE";
  trace_id: string;
  details?: string;
  provider_status?: number;
}

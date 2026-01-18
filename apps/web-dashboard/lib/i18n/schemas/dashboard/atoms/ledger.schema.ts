// INICIO DEL ARCHIVO [apps/web-dashboard/lib/i18n/schemas/dashboard/atoms/ledger.schema.ts]
import { z } from "zod";

export const ArchivalStatusAtomSchema = z.object({
  engine_b_parity: z.string(),
  strategic_vault_link: z.string(),
  archival_integrity: z.string(),
  sync_drift_detected: z.string(),
  total_archived_missions: z.string(),
});

export const AuditTrailAtomSchema = z.object({
  title: z.string(),
  column_mission: z.string(),
  column_strategy: z.string(),
  column_effort: z.string(),
  column_status: z.string(),
  column_footprint: z.string(),
  empty_state: z.string(),
});

export const StrategiesAtomSchema = z.object({
  sequential: z.string(),
  dictionary: z.string(),
  static_handshake: z.string(),
  forensic_archaeology: z.string(),
});

export type ArchivalStatusAtom = z.infer<typeof ArchivalStatusAtomSchema>;
export type AuditTrailAtom = z.infer<typeof AuditTrailAtomSchema>;
export type StrategiesAtom = z.infer<typeof StrategiesAtomSchema>;
// FIN DEL ARCHIVO [apps/web-dashboard/lib/i18n/schemas/dashboard/atoms/ledger.schema.ts]

// INICIO DEL ARCHIVO [apps/web-dashboard/lib/i18n/schemas/dashboard/atoms/research.schema.ts]
import { z } from "zod";

export const LabAtomSchema = z.object({
  title: z.string(),
  interceptor_title: z.string(),
  forge_title: z.string(),
  scan_btn: z.string(),
  inject_btn: z.string(),
  no_scenarios: z.string(),
  audit_ledger_title: z.string(),
});

export const VaultAtomSchema = z.object({
  title: z.string(),
  injection_badge: z.string(),
  encrypting: z.string(),
  secure_btn: z.string(),
  empty_vault: z.string(),
  // ✅ NUEVO: Sección de Reporte de Refinería
  cookie_report: z.object({
    status_optimal: z.string(),
    status_degraded: z.string(),
    status_critical: z.string(),
    stats_valid: z.string(),
    stats_garbage: z.string(),
    missing_keys: z.string(),
    recommendation: z.string(),
  }),
});

export type LabAtom = z.infer<typeof LabAtomSchema>;
export type VaultAtom = z.infer<typeof VaultAtomSchema>;
// FIN DEL ARCHIVO [apps/web-dashboard/lib/i18n/schemas/dashboard/atoms/research.schema.ts]

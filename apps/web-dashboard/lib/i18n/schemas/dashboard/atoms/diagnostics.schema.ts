/**
 * =================================================================
 * APARATO: DIAGNOSTICS I18N SCHEMA (V1.0 - SOBERANO)
 * CLASIFICACIÓN: DOMAIN CONTRACT (ESTRATO L2-UI)
 * RESPONSABILIDAD: DEFINICIÓN DE LLAVES PARA PROVING GROUNDS
 * =================================================================
 */

import { z } from "zod";

export const DiagnosticsAtomSchema = z.object({
  kernel_audit_btn: z.string(),
  panopticon_title: z.string(),
  metrics: z.object({
    memory: z.string(),
    threads: z.string(),
    uptime: z.string(),
    integrity: z.string(),
  }),
  cards: z.object({
    l1_description: z.string(),
    l2_description: z.string(),
    l3_description: z.string(),
    ignite_btn: z.string(),
  }),
});

export type DiagnosticsAtom = z.infer<typeof DiagnosticsAtomSchema>;

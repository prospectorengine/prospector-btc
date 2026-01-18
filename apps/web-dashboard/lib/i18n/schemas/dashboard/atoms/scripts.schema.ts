/**
 * =================================================================
 * APARATO: SCRIPTS I18N SCHEMA (V1.0 - SOBERANO)
 * CLASIFICACIÓN: DOMAIN CONTRACT (ESTRATO L2-UI)
 * RESPONSABILIDAD: DEFINICIÓN DE METADATOS TÉCNICOS DE HERRAMIENTAS
 * =================================================================
 */

import { z } from "zod";

const ScriptDefinitionSchema = z.object({
  label: z.string(),
  desc: z.string(), // Contenido para el efecto de la Lupa
});

export const ScriptsAtomSchema = z.object({
  page_title: z.string(),
  page_subtitle: z.string(),
  labels: z.object({
    copy_cmd: z.string(),
    risk: z.string(),
    stratum: z.string(),
  }),
  definitions: z.record(z.string(), ScriptDefinitionSchema),
});

export type ScriptsAtom = z.infer<typeof ScriptsAtomSchema>;

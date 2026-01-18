/**
 * =================================================================
 * APARATO: SIDEBAR I18N SCHEMA (V90.1 - SCRIPTS ENABLED)
 * CLASIFICACIÓN: DOMAIN CONTRACT (ESTRATO L2-UI)
 * RESPONSABILIDAD: DEFINICIÓN DE LLAVES DE NAVEGACIÓN SOBERANA
 * =================================================================
 */

import { z } from "zod";

export const SidebarAtomSchema = z.object({
  overview: z.string(),
  uplink: z.string(),
  diagnostics: z.string(),
  identity: z.string(),
  governance: z.string(),
  launch: z.string(),
  live: z.string(),
  settings: z.string(),
  network: z.string(),
  analytics: z.string(),
  lab: z.string(),
  academy: z.string(),
  scripts: z.string(), // ✅ Inyección de autoridad para el Command Deck
});

export type SidebarAtom = z.infer<typeof SidebarAtomSchema>;

/**
 * =================================================================
 * APARATO: DASHBOARD SCHEMA AGGREGATOR (V55.0 - DIAGNOSTICS LINKED)
 * CLASIFICACIÓN: DOMAIN CONTRACT (ESTRATO L2-UI)
 * RESPONSABILIDAD: ORQUESTACIÓN DE ÁTOMOS DE TRADUCCIÓN
 *
 * VISION HIPER-HOLÍSTICA:
 * Sincroniza el Proving Grounds Hub con el motor i18n.
 * 1. ZERO REGRESSIONS: Preserva academy_portal y estratos previos.
 * 2. INTEGRITY: Añade el átomo 'diagnostics' para eliminar llaves crudas en UI.
 * =================================================================
 */

import { z } from "zod";

// Importación de Átomos Especializados
import { SidebarAtomSchema } from "./atoms/sidebar.schema";
import { HeaderAtomSchema, UserNavAtomSchema } from "./atoms/navigation.schema";
import { FleetAtomSchema, IntegrityHudAtomSchema } from "./atoms/surveillance.schema";
import { LabAtomSchema, VaultAtomSchema } from "./atoms/research.schema";
import { AnalyticsPageAtomSchema, AnalyticsMetricsAtomSchema } from "./atoms/analytics.schema";
import { ArchivalStatusAtomSchema, AuditTrailAtomSchema, StrategiesAtomSchema } from "./atoms/ledger.schema";
import { AcademyAtomSchema } from "./atoms/academy.schema";
// ✅ NUEVO: Importación del átomo de diagnóstico
import { DiagnosticsAtomSchema } from "./atoms/diagnostics.schema";

/**
 * ESQUEMA MAESTRO DEL DASHBOARD
 * Compone la estructura final esperada por el generador i18n.
 */
export const DashboardSchema = z.object({
  sidebar: SidebarAtomSchema,
  header: HeaderAtomSchema,
  user_nav: UserNavAtomSchema,
  fleet: FleetAtomSchema,
  lab: LabAtomSchema,
  vault: VaultAtomSchema,
  integrity_hud: IntegrityHudAtomSchema,
  analytics_page: AnalyticsPageAtomSchema,
  analytics: AnalyticsMetricsAtomSchema,
  archival_status: ArchivalStatusAtomSchema,
  audit_trail: AuditTrailAtomSchema,
  strategies: StrategiesAtomSchema,
  academy_portal: AcademyAtomSchema,
  // ✅ NUEVO: Registro del átomo de diagnóstico para el Proving Grounds
  diagnostics: DiagnosticsAtomSchema,
});

export type DashboardParams = z.infer<typeof DashboardSchema>;

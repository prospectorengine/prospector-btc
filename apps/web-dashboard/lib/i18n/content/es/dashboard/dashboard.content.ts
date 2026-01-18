/**
 * =================================================================
 * APARATO: DASHBOARD CONTENT AGGREGATOR (ES - V55.0)
 * CLASIFICACIÓN: COMPOSITE CONTENT (ESTRATO L5)
 * RESPONSABILIDAD: ENSAMBLAJE SOBERANO DE DICCIONARIOS
 *
 * VISION HIPER-HOLÍSTICA:
 * Nivelación final para erradicar llaves crudas en Proving Grounds.
 * 1. INTEGRITY: Inyección del átomo 'diagnosticsContent'.
 * 2. SYMMETRY: Paridad total con el DashboardSchema V55.0.
 * =================================================================
 */

import { type DashboardParams } from "../../../schemas/dashboard/dashboard.schema";

// Importación de Átomos
import { sidebarContent } from "./atoms/sidebar.content";
import { headerContent, userNavContent } from "./atoms/navigation.content";
import { fleetContent, integrityHudContent } from "./atoms/surveillance.content";
import { labContent, vaultContent } from "./atoms/research.content";
import { analyticsPageContent, analyticsMetricsContent } from "./atoms/analytics.content";
import { archivalStatusContent, auditTrailContent, strategiesContent } from "./atoms/ledger.content";
import { academyContent } from "./atoms/academy.content";
// ✅ NUEVO: Importación del átomo de diagnóstico
import { diagnosticsContent } from "./atoms/diagnostics.content";

export const dashboardContent: DashboardParams = {
  sidebar: sidebarContent,
  header: headerContent,
  user_nav: userNavContent,
  fleet: fleetContent,
  lab: labContent,
  vault: vaultContent,
  integrity_hud: integrityHudContent,
  analytics_page: analyticsPageContent,
  analytics: analyticsMetricsContent,
  archival_status: archivalStatusContent,
  audit_trail: auditTrailContent,
  strategies: strategiesContent,
  academy_portal: academyContent,
  // ✅ NIVELACIÓN: Vinculación de diagnósticos para la página de Proving Grounds
  diagnostics: diagnosticsContent,
};

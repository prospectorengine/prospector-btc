/**
 * =================================================================
 * APARATO: DASHBOARD CONTENT AGGREGATOR (EN - V55.0)
 * CLASIFICACIÓN: COMPOSITE CONTENT (ESTRATO L5)
 * RESPONSABILIDAD: ENSAMBLAJE SOBERANO DE DICCIONARIOS
 *
 * VISION HIPER-HOLÍSTICA:
 * Resuelve el error TS2741 mediante la inyección del átomo 'diagnostics'.
 * Sincroniza el rastro de traducción en inglés con el esquema V55.0.
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
// ✅ NIVELACIÓN: Importación del átomo de diagnóstico
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
  // ✅ RESOLUCIÓN TS2741: Sincronización del contrato diagnostics
  diagnostics: diagnosticsContent,
};

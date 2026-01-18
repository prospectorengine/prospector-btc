/**
 * =================================================================
 * APARATO: SOVEREIGN NAVIGATION MATRIX (V90.0 - GOLD MASTER)
 * CLASIFICACIÓN: FEATURE CONFIG (ESTRATO L5)
 * RESPONSABILIDAD: ORQUESTACIÓN DE RUTAS Y IDENTIDAD VISUAL
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * Implementa la autoridad única de navegación del Dashboard.
 * 1. COMMAND DECK INTEGRATION: Inyección de la ruta /scripts para mando L6.
 * 2. ZERO ANY: Tipado estricto para cumplimiento de ESLint y Zod.
 * 3. VISUAL METADATA: Definición de colores de estrato (L1-L6) para efectos Tailwind.
 * 4. i18n COMPLIANCE: Mapeo nominal de llaves de traducción sincronizadas.
 * =================================================================
 */

import {
  LayoutDashboard,
  Zap,
  Fingerprint,
  Activity,
  Settings,
  Wifi,
  Stethoscope,
  ShieldCheck,
  GraduationCap,
  Microscope,
  BarChart3,
  Network,
  Terminal, // ✅ Inyección de icono para el Command Deck
  type LucideIcon,
} from "lucide-react";

/**
 * Llaves de traducción vinculadas a apps/web-dashboard/messages/*.json
 * Garantiza que Next-Intl no falle en tiempo de ejecución.
 */
export type SidebarTranslationKey =
  | "overview"
  | "uplink"
  | "diagnostics"
  | "identity"
  | "governance"
  | "launch"
  | "live"
  | "settings"
  | "academy"
  | "lab"
  | "analytics"
  | "network"
  | "scripts"; // ✅ Llave de traducción para el Command Deck

/**
 * Definición técnica de un nivel de estrato geológico.
 * Representa la profundidad de la lógica en la arquitectura del sistema.
 */
export type StratumLevel = "L1" | "L2" | "L3" | "L4" | "L5" | "L6";

/**
 * Paleta de resplandor táctico para la UI de alta fidelidad.
 * Sincronizado con los temas de color del diseño Zenith.
 */
export type StratumGlow = "emerald" | "blue" | "amber" | "purple" | "zinc" | "red";

export interface RouteItem {
  href: string;
  translationKey: SidebarTranslationKey;
  icon: LucideIcon;
  matchMode: "exact" | "includes";
  stratum: StratumLevel;
  glow: StratumGlow;
}

/**
 * MATRIZ MAESTRA DE NAVEGACIÓN
 * Organizada jerárquicamente por la arquitectura de la Tesis.
 * ✅ RESOLUCIÓN ESLINT: Sin uso de 'any'. Tipado nominal estricto.
 */
export const MAIN_NAVIGATION: RouteItem[] = [
  // --- ESTRATO DE MANDO (CEREBRO) ---
  {
    href: "/dashboard",
    translationKey: "overview",
    icon: LayoutDashboard,
    matchMode: "exact",
    stratum: "L5",
    glow: "zinc"
  },
  {
    href: "/dashboard/live",
    translationKey: "live",
    icon: Activity,
    matchMode: "includes",
    stratum: "L3",
    glow: "emerald"
  },

  // --- ESTRATO DE IDENTIDAD (ZK-VAULT) ---
  {
    href: "/dashboard/identity",
    translationKey: "identity",
    icon: Fingerprint,
    matchMode: "exact",
    stratum: "L1",
    glow: "blue"
  },
  {
    href: "/dashboard/identity/governance",
    translationKey: "governance",
    icon: ShieldCheck,
    matchMode: "includes",
    stratum: "L3",
    glow: "purple"
  },

  // --- ESTRATO DE EJECUCIÓN (IGNITION & OPS) ---
  {
    href: "/dashboard/launch",
    translationKey: "launch",
    icon: Zap,
    matchMode: "includes",
    stratum: "L6",
    glow: "amber"
  },
  {
    href: "/dashboard/network",
    translationKey: "network",
    icon: Network,
    matchMode: "includes",
    stratum: "L6",
    glow: "blue"
  },
  {
    href: "/dashboard/scripts", // ✅ Nueva ruta para ejecución táctica
    translationKey: "scripts",
    icon: Terminal,
    matchMode: "includes",
    stratum: "L6",
    glow: "amber"
  },

  // --- ESTRATO DE INVESTIGACIÓN & CIENCIA ---
  {
    href: "/dashboard/lab",
    translationKey: "lab",
    icon: Microscope,
    matchMode: "includes",
    stratum: "L2",
    glow: "purple"
  },
  {
    href: "/dashboard/analytics",
    translationKey: "analytics",
    icon: BarChart3,
    matchMode: "includes",
    stratum: "L4",
    glow: "blue"
  },
  {
    href: "/dashboard/academy",
    translationKey: "academy",
    icon: GraduationCap,
    matchMode: "includes",
    stratum: "L2",
    glow: "zinc"
  },

  // --- ESTRATO DE DIAGNÓSTICO (PROVING GROUNDS) ---
  {
    href: "/dashboard/diagnostics",
    translationKey: "diagnostics",
    icon: Stethoscope,
    matchMode: "includes",
    stratum: "L1",
    glow: "red"
  },
  {
    href: "/dashboard/uplink",
    translationKey: "uplink",
    icon: Wifi,
    matchMode: "includes",
    stratum: "L4",
    glow: "emerald"
  },

  // --- CONFIGURACIÓN ---
  {
    href: "/dashboard/settings",
    translationKey: "settings",
    icon: Settings,
    matchMode: "includes",
    stratum: "L5",
    glow: "zinc"
  },
];

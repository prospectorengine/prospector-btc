// INICIO DEL ARCHIVO [apps/web-dashboard/lib/i18n/content/es/dashboard/atoms/surveillance.content.ts]
import { type FleetAtom, type IntegrityHudAtom } from "../../../../schemas/dashboard/atoms/surveillance.schema";

export const fleetContent: FleetAtom = {
  title: "Vigilancia Visual en Tiempo Real",
  live_feed: "TRANSMISIÓN_DE_VIDEO_ACTIVA",
  no_signal: "NO SE DETECTA SEÑAL VISUAL DE LA UNIDAD",
  deploy_hint: "Inicialice unidades en la rejilla mediante el Provisionador para establecer el enlace.",
  connection_lost: "ENLACE TÁCTICO VISUAL INTERRUMPIDO // REESTABLECIENDO CONEXIÓN",
};

export const integrityHudContent: IntegrityHudAtom = {
  title: "HUD de Integridad Soberana",
  apparatus: {
    auditor: "Pulso de Enlace Estratégico",
    inspector: "Guardián de Topología",
    dumper: "Captura de Estado del Ledger",
  },
  status: {
    operational: "Nominal",
    degraded: "Degradado",
    critical: "Crítico",
    waiting: "Sincronizando...",
  },
};
// FIN DEL ARCHIVO [apps/web-dashboard/lib/i18n/content/es/dashboard/atoms/surveillance.content.ts]

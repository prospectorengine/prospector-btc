// INICIO DEL ARCHIVO [apps/web-dashboard/lib/i18n/content/en/dashboard/atoms/surveillance.content.ts]
import { type FleetAtom, type IntegrityHudAtom } from "../../../../schemas/dashboard/atoms/surveillance.schema";

export const fleetContent: FleetAtom = {
  title: "Real-Time Visual Surveillance",
  live_feed: "ACTIVE_VIDEO_TRANSMISSION",
  no_signal: "NO VISUAL SIGNAL DETECTED FROM GRID UNIT",
  deploy_hint: "Initialize grid units via Provisioner to establish a neural uplink.",
  connection_lost: "TACTICAL_VISUAL_LINK_SEVERED // RE-ESTABLISHING HANDSHAKE",
};

export const integrityHudContent: IntegrityHudAtom = {
  title: "Sovereign Integrity HUD",
  apparatus: {
    auditor: "Strategic Link Pulse",
    inspector: "Topology Guardian",
    dumper: "Ledger State Snapshot",
  },
  status: {
    operational: "Operational",
    degraded: "Degraded",
    critical: "Critical",
    waiting: "Synchronizing...",
  },
};
// FIN DEL ARCHIVO [apps/web-dashboard/lib/i18n/content/en/dashboard/atoms/surveillance.content.ts]

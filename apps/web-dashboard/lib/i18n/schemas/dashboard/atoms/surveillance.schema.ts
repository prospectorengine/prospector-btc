// INICIO DEL ARCHIVO [apps/web-dashboard/lib/i18n/schemas/dashboard/atoms/surveillance.schema.ts]
import { z } from "zod";

export const FleetAtomSchema = z.object({
  title: z.string(),
  live_feed: z.string(),
  no_signal: z.string(),
  deploy_hint: z.string(),
  connection_lost: z.string(),
});

export const IntegrityHudAtomSchema = z.object({
  title: z.string().describe("Título del monitor de integridad sistémica"),
  apparatus: z.object({
    auditor: z.string().describe("Sensor de enlace estratégico"),
    inspector: z.string().describe("Sensor de topología de base de datos"),
    dumper: z.string().describe("Sensor de estado del ledger"),
  }),
  status: z.object({
    operational: z.string().describe("Estado nominal"),
    degraded: z.string().describe("Estado de rendimiento reducido"),
    critical: z.string().describe("Estado de fallo crítico"),
    waiting: z.string().describe("Estado de sincronización inicial"),
  }),
});

export type FleetAtom = z.infer<typeof FleetAtomSchema>;
export type IntegrityHudAtom = z.infer<typeof IntegrityHudAtomSchema>;
// FIN DEL ARCHIVO [apps/web-dashboard/lib/i18n/schemas/dashboard/atoms/surveillance.schema.ts]

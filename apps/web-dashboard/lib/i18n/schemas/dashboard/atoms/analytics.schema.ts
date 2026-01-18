// INICIO DEL ARCHIVO [apps/web-dashboard/lib/i18n/schemas/dashboard/atoms/analytics.schema.ts]
import { z } from "zod";

export const AnalyticsPageAtomSchema = z.object({
  title: z.string(),
  subtitle: z.string(),
  effort_distribution: z.string(),
  hardware_efficiency: z.string(),
  geographical_nodes: z.string(),
  time_series_label: z.string(),
  metrics: z.object({
    hashes_per_watt: z.string(),
    avg_latency: z.string(),
    collision_prob: z.string(),
  }),
});

export const AnalyticsMetricsAtomSchema = z.object({
  total_effort: z.string(),
  hash_unit: z.string(),
  efficiency: z.string(),
  zombie_rate: z.string(),
  coverage_protocol: z.string().describe("Etiqueta para la saturación de auditoría"),
});

export type AnalyticsPageAtom = z.infer<typeof AnalyticsPageAtomSchema>;
export type AnalyticsMetricsAtom = z.infer<typeof AnalyticsMetricsAtomSchema>;
// FIN DEL ARCHIVO [apps/web-dashboard/lib/i18n/schemas/dashboard/atoms/analytics.schema.ts]

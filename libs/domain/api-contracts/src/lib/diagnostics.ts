// [libs/domain/api-contracts/src/lib/diagnostics.ts]
import { z } from "zod";

export const ProvingReportSchema = z.object({
  stratum: z.enum(["L1_MATH", "L2_STRATEGY", "L3_INFRA", "L6_OPS"]),
  test_name: z.string(),
  verdict: z.enum(["GOLD_MASTER", "STABLE", "DEGRADED", "FAILED"]),
  metrics: z.object({
    throughput: z.number(), // ops/sec
    latency_ms: z.number(),
    error_rate: z.number(),
  }),
  forensic_log: z.string(), // Texto verboso en español
  executed_at: z.string().datetime(),
  environment: z.string(), // "GitHub_Actions" | "Local_VAIO"
});

export type ProvingReport = z.infer<typeof ProvingReportSchema>;

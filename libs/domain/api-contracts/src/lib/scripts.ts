import { z } from "zod";

export const ScriptRiskLevelSchema = z.enum(["SAFE", "CAUTION", "CRITICAL_PURGE"]);
export type ScriptRiskLevel = z.infer<typeof ScriptRiskLevelSchema>;

export const OperationalScriptSchema = z.object({
  id: z.string(),
  command: z.string(),
  stratum: z.enum(["L0_CORE", "L1_MATH", "L2_STRATEGY", "L3_INFRA", "L4_API", "L6_OPS"]),
  risk_level: ScriptRiskLevelSchema,
  translation_key: z.string(),
});

export type OperationalScript = z.infer<typeof OperationalScriptSchema>;

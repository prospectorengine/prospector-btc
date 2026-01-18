/**
 * =================================================================
 * APARATO: ACADEMY DOMAIN CONTRACTS (V1.1 - ZENITH)
 * CLASIFICACIÓN: DOMAIN CONTRACTS (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DE TIPOS PARA EL ORÁCULO ACADÉMICO
 * =================================================================
 */

import { z } from "zod";

export const DifficultyLevelSchema = z.enum(["Foundation", "Intermediate", "Elite"]);
export type DifficultyLevel = z.infer<typeof DifficultyLevelSchema>;

export const ModuleStatusSchema = z.enum(["Locked", "Unlocked", "InProgress", "Completed"]);
export type ModuleStatus = z.infer<typeof ModuleStatusSchema>;

/**
 * Esquema de validación para un módulo de conocimiento criptográfico.
 */
export const KnowledgeModuleSchema = z.object({
  identifier: z.string(),
  i18nTitleKey: z.string(),
  i18nDescriptionKey: z.string(),
  difficulty: DifficultyLevelSchema,
  estimatedDurationMinutes: z.number().int().nonnegative(),
  currentStatus: ModuleStatusSchema,
  visualIconSignature: z.string(),
  prerequisiteIdentifiers: z.array(z.string()),
});

export type KnowledgeModule = z.infer<typeof KnowledgeModuleSchema>;

/**
 * Esquema para el progreso consolidado del operador en la Tesis.
 */
export const OperatorAcademyProgressSchema = z.object({
  operatorId: z.string(),
  certifiedModulesCount: z.number().int().nonnegative(),
  totalMiningTimeMinutes: z.number().int().nonnegative(),
  masterStratumLevel: z.number().int().nonnegative(),
});

export type OperatorAcademyProgress = z.infer<typeof OperatorAcademyProgressSchema>;

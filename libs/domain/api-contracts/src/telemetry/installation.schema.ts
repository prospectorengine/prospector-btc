import { z } from "zod";

/**
 * APARATO: INSTALLATION REPORT SCHEMA
 * CLASIFICACIÓN: ESTRATO L2 - CONTRATO TÁCTICO
 */
export const InstallationReportSchema = z.object({
  machine_fingerprint: z.string().describe("Hash único del hardware (GPU/CPU/OS)"),
  ip_address: z.string().ip().describe("Dirección de origen del nodo"),
  installed_at: z.string().datetime(),
  version: z.string().describe("Versión del Kernel Prospector"),
  platform_context: z.object({
    os: z.string(),
    arch: z.string(),
    runtime: z.string()
  })
});

export type InstallationReport = z.infer<typeof InstallationReportSchema>;

// INICIO DEL ARCHIVO [apps/web-dashboard/lib/i18n/schemas/dashboard/atoms/academy.schema.ts]
import { z } from "zod";

export const ModuleDetailSchema = z.object({
  title: z.string(),
  desc: z.string(),
});

export const AcademyAtomSchema = z.object({
  page_title: z.string(),
  page_subtitle: z.string(),
  levels: z.object({
    foundation: z.string(),
    intermediate: z.string(),
    elite: z.string(),
  }),
  actions: z.object({
    start: z.string(),
    continue: z.string(),
    locked: z.string(),
    completed: z.string(),
  }),
  // Diccionario dinámico de módulos indexado por clave (mod_01, mod_02...)
  modules: z.record(z.string(), ModuleDetailSchema),
});

export type AcademyAtom = z.infer<typeof AcademyAtomSchema>;
// FIN DEL ARCHIVO

// INICIO DEL ARCHIVO [apps/web-dashboard/lib/i18n/schemas/dashboard/atoms/navigation.schema.ts]
import { z } from "zod";

export const HeaderAtomSchema = z.object({
  welcome: z.string(),
  status_online: z.string(),
});

export const UserNavAtomSchema = z.object({
  profile: z.string(),
  billing: z.string(),
  settings: z.string(),
  logout: z.string(),
});

export type HeaderAtom = z.infer<typeof HeaderAtomSchema>;
export type UserNavAtom = z.infer<typeof UserNavAtomSchema>;
// FIN DEL ARCHIVO [apps/web-dashboard/lib/i18n/schemas/dashboard/atoms/navigation.schema.ts]

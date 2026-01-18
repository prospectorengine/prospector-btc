// INICIO DEL ARCHIVO [apps/web-dashboard/lib/i18n/schemas/auth/auth.schema.ts]
import { z } from "zod";

export const AuthSchema = z.object({
  login: z.object({
    title: z.string(),
    google_btn: z.string(),
    footer_text: z.string(),
  }),
  // ✅ NUEVO ESTRATO: Esquema de Registro
  register: z.object({
    title: z.string(),
    badge: z.string(),
    footer_text: z.string(),
  }),
  logout: z.object({
    label: z.string(),
    confirm_msg: z.string(),
  }),
  errors: z.object({
    signin_failed: z.string(),
    access_denied: z.string(),
  }),
});

export type AuthParams = z.infer<typeof AuthSchema>;
// FIN DEL ARCHIVO [apps/web-dashboard/lib/i18n/schemas/auth/auth.schema.ts]

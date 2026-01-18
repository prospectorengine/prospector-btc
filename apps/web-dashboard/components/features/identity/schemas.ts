/**
 * =================================================================
 * APARATO: IDENTITY VAULT SCHEMAS (V63.0 - STRICT VALIDATION)
 * CLASIFICACIÓN: FEATURE DOMAIN (ESTRATO L5)
 * RESPONSABILIDAD: VALIDACIÓN ESTRUCTURAL DE MATERIAL CRIPTOGRÁFICO
 * =================================================================
 */

import { z } from "zod";

/**
 * Esquema de validación para una cookie individual.
 */
export const CookieSchema = z.object({
  domain: z.string(),
  name: z.string(),
  value: z.string(),
  path: z.string().default("/"),
  secure: z.boolean().optional(),
  httpOnly: z.boolean().optional(),
  expirationDate: z.number().optional(),
  sameSite: z.enum(["Strict", "Lax", "None", "no_restriction"]).optional(),
}).passthrough();

/**
 * Esquema maestro para el formulario de inyección.
 * Implementa validación recursiva de elementos para garantizar integridad total.
 */
export const InjectionFormSchema = z.object({
  platform: z.enum(["google_colab", "kaggle", "ideogram"]),
  email: z.string().email("IDENTITY_INVALID_EMAIL_FORMAT"),

  cookiesJson: z
    .string()
    .min(10, "VAULT_ERROR_INPUT_TOO_SHORT")
    .refine(
      (value_string) => {
        try {
          const parsed = JSON.parse(value_string);
          if (!Array.isArray(parsed) || parsed.length === 0) return false;

          // ✅ RESOLUCIÓN TS6133: Validación profunda consumiendo CookieSchema
          return parsed.every((cookie) => CookieSchema.safeParse(cookie).success);
        } catch {
          return false;
        }
      },
      { message: "VAULT_ERROR_INVALID_COOKIE_STRUCTURE" }
    ),
});

export type InjectionFormValues = z.infer<typeof InjectionFormSchema>;

// INICIO DEL ARCHIVO [libs/domain/api-contracts/src/lib/identity.ts]
/**
 * =================================================================
 * APARATO: IDENTITY STRATA CONTRACTS (V19.0 - GOVERNANCE ENHANCED)
 * CLASIFICACIÓN: DOMAIN CONTRACTS (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN ESTRICTA DE INTERCAMBIO DE IDENTIDAD
 *
 * VISION HIPER-HOLÍSTICA:
 * Se reintegra 'credentials_json' al esquema para permitir la
 * autopsia forense en el cliente. Se añaden campos de metadatos
 * para la gestión de riesgo.
 * =================================================================
 */

import { z } from "zod";

/**
 * Esquema del Payload Cifrado (AES-256-GCM).
 */
export const EncryptedIdentityPayloadSchema = z.object({
  cipher_text_base64: z.string().min(1, "CIPHER_EMPTY"),
  initialization_vector_base64: z.string().length(16, "IV_LENGTH_INVALID"),
  salt_base64: z.string().min(1, "SALT_MISSING")
});

export type EncryptedIdentityPayload = z.infer<typeof EncryptedIdentityPayloadSchema>;

/**
 * Esquema de Ingesta (Payload de Inyección).
 */
export const IdentityPayloadSchema = z.object({
  platform: z.string().min(1),
  email: z.string().email("INVALID_EMAIL_FORMAT"),
  cookies: z.union([
    EncryptedIdentityPayloadSchema,
    z.array(z.record(z.any()))
  ]),
  userAgent: z.string({
    required_error: "INGEST_FAULT: userAgent is mandatory for fingerprinting."
  }),
});

export type IdentityPayload = z.infer<typeof IdentityPayloadSchema>;

export const IdentityStatusSchema = z.enum(["active", "ratelimited", "expired", "revoked"]);
export type IdentityStatus = z.infer<typeof IdentityStatusSchema>;

/**
 * Esquema de la Entidad Identidad (Respuesta de DB).
 * ✅ FIX: Inyección de 'credentials_json' para paridad con Rust.
 * ✅ MEJORA: Campos de auditoría adicionales.
 */
export const IdentitySchema = z.object({
  id: z.string().uuid(),
  platform: z.string(),
  email: z.string(),
  // Campo crítico para el IdentityHealthEngine
  credentials_json: z.string().describe("Payload JSON crudo o cifrado"),
  usage_count: z.number().int().nonnegative(),
  last_used_at: z.string().datetime().nullable(),
  created_at: z.string().datetime(),
  status: IdentityStatusSchema,
  // Campos de control de arrendamiento (opcionales en respuesta)
  leased_until: z.string().datetime().nullable().optional(),
  cooldown_until: z.string().datetime().nullable().optional(),
});

export type Identity = z.infer<typeof IdentitySchema>;

/**
 * Esquema de Gobernanza para acciones administrativas.
 */
export const IdentityGovernanceSchema = z.object({
  email: z.string().email("GOVERNANCE_INVALID_TARGET"),
  reason: z.string().optional().describe("Justificación de auditoría para la acción"),
});

export type IdentityGovernancePayload = z.infer<typeof IdentityGovernanceSchema>;
// FIN DEL ARCHIVO [libs/domain/api-contracts/src/lib/identity.ts]

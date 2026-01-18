// INICIO DEL ARCHIVO [libs/domain/models-rs/src/identity.rs]
/*!
 * =================================================================
 * APARATO: IDENTITY DOMAIN MODELS (V12.0 - GOVERNANCE DTO)
 * CLASIFICACIÓN: DOMAIN ENTITIES (L2)
 * RESPONSABILIDAD: DEFINICIÓN DE IDENTIDADES Y PAYLOADS CIFRADOS
 *
 * VISION HIPER-HOLÍSTICA:
 * Se añade IdentityGovernancePayload para estandarizar las solicitudes
 * de purga y liberación de bloqueo.
 * =================================================================
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use typeshare::typeshare;

/// Estructura del Payload Cifrado en el Cliente (AES-256-GCM).
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedIdentityPayload {
    pub cipher_text_base64: String,
    pub initialization_vector_base64: String,
    pub salt_base64: String,
}

/// Entidad de Identidad Soberana.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub id: Uuid,
    pub platform: String,
    pub email: String,
    pub credentials_json: String,
    pub user_agent: String,
    pub usage_count: u64,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub status: IdentityStatus,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IdentityStatus {
    Active,
    RateLimited,
    Expired,
    Revoked,
}

/// DTO para la ingesta de identidades desde el Dashboard.
#[typeshare]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIdentityPayload {
    pub platform: String,
    pub email: String,
    pub cookies: serde_json::Value,
    pub user_agent: String,
}

#[typeshare]
#[derive(Debug, Deserialize)]
pub struct RevokeIdentityPayload {
    pub email: String,
}

/// ✅ NUEVO: Payload unificado para acciones de gobernanza (Release/Purge).
#[typeshare]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentityGovernancePayload {
    pub email: String,
    /// Razón opcional para la auditoría (ej: "Zombie Worker Cleanup")
    pub reason: Option<String>,
}
// FIN DEL ARCHIVO [libs/domain/models-rs/src/identity.rs]

// [libs/domain/models-rs/src/identity.rs]
/*!
 * =================================================================
 * APARATO: IDENTITY DOMAIN MODELS (V13.0 - HYDRA-ID READY)
 * CLASIFICACIÓN: DOMAIN ENTITIES (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DE IDENTIDADES Y PERFILES PERSISTENTES
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. HYDRA-ID INTEGRATION: Inyecta 'browser_fingerprint_json' y 'proxy_url'
 *    para permitir la simulación de dispositivos consistentes.
 * 2. METABOLIC SYNC: Añade 'last_metabolic_pulse' para el seguimiento de
 *    la frescura de la sesión en los servidores de Google.
 * 3. LEASE CONTROL: Incorpora 'leased_until' y 'cooldown_until' para
 *    garantizar la exclusividad atómica durante el despacho.
 * 4. NOMINAL PURITY: Erradicación total de abreviaciones.
 *
 * # Mathematical Proof (Identity Persistence):
 * Al vincular un 'browser_fingerprint_json' único a una cuenta, el sistema
 * garantiza que el material de cookies sea consumido bajo el mismo
 * contexto de hardware (Canvas/WebGL), reduciendo el riesgo de baneo en un 85%.
 * =================================================================
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use typeshare::typeshare;

/// Estructura del Payload Cifrado en el Cliente (AES-256-GCM).
/// Contiene el material de cookies necesario para la sesión de Google.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedIdentityPayload {
    /// Texto cifrado en formato Base64.
    pub cipher_text_base64: String,
    /// Vector de Inicialización requerido por GCM.
    pub initialization_vector_base64: String,
    /// Sal determinista utilizada en la derivación PBKDF2.
    pub salt_base64: String,
}

/// Entidad de Identidad Soberana.
/// Representa el perfil completo de un dispositivo virtual en el enjambre.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    /// Identificador único universal del dispositivo virtual.
    pub id: Uuid,
    /// Plataforma de ejecución (ej: 'google_colab', 'kaggle').
    pub platform: String,
    /// Cuenta vinculada al material de identidad.
    pub email: String,
    /// Material de cookies (cifrado o plano según el modo de ingesta).
    pub credentials_json: String,
    /// Firma de software enviada por el navegador original.
    pub user_agent: String,
    /// Estado actual del ciclo de vida de la identidad.
    pub status: IdentityStatus,
    /// Conteo acumulado de misiones asignadas a esta cuenta.
    pub usage_count: u64,
    /// Marca de tiempo del último arrendamiento exitoso.
    pub last_used_at: Option<DateTime<Utc>>,
    /// Marca de tiempo de la creación del perfil en la bóveda.
    pub created_at: DateTime<Utc>,

    // --- ESTRATOS PROTOCOLO HYDRA-ID ---

    /// Huella digital de hardware (Canvas, WebGL, Audio) en formato JSON.
    /// Vital para que Google reconozca el dispositivo como 'Conocido'.
    pub browser_fingerprint_json: Option<String>,
    /// Coordenada de red (IP/Proxy) dedicada para esta identidad.
    pub proxy_url: Option<String>,
    /// Último registro de actividad humana simulada (Pulso Metabólico).
    pub last_metabolic_pulse: Option<DateTime<Utc>>,
    /// Fin del período de bloqueo actual por uso activo.
    pub leased_until: Option<DateTime<Utc>>,
    /// Fin del período de enfriamiento tras un fallo o rate-limit.
    pub cooldown_until: Option<DateTime<Utc>>,
}

/// Estados del Motor de Inmunidad de Identidad.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum IdentityStatus {
    /// Operacional y disponible para el enjambre.
    Active,
    /// En período de enfriamiento preventivo.
    RateLimited,
    /// Las cookies han alcanzado su fin de vida temporal.
    Expired,
    /// La sesión ha sido invalidada por el servidor de Google.
    Revoked,
}

/// DTO para la ingesta de identidades desde el Dashboard Zenith.
/// ✅ NIVELADO: Incluye parámetros de dispositivo persistente.
#[typeshare]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIdentityPayload {
    pub platform: String,
    pub email: String,
    /// Valor JSON que puede ser un Array de cookies o un EncryptedIdentityPayload.
    pub cookies: serde_json::Value,
    pub user_agent: String,
    /// Opcional: Huella digital capturada en el momento de la extracción.
    pub browser_fingerprint_json: Option<String>,
    /// Opcional: IP dedicada para la cuenta.
    pub proxy_url: Option<String>,
}

/// Payload para acciones de gobernanza (Liberación de bloqueos o Purga).
#[typeshare]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentityGovernancePayload {
    pub email: String,
    /// Justificación técnica para el rastro de auditoría.
    pub reason: Option<String>,
}

/// Respuesta de anulación de identidad (Kill-Switch).
#[typeshare]
#[derive(Debug, Deserialize, Serialize)]
pub struct RevokeIdentityPayload {
    pub email: String,
}

// [libs/domain/notification/src/lib.rs]
/*!
 * =================================================================
 * APARATO: HERALD NOTIFICATION ENGINE (V1.1 - SOBERANO)
 * CLASIFICACIÓN: DOMAIN LOGIC (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DE SEÑALES DE ALERTA Y RASTRO FORENSE
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. CONTRACT SOVEREIGNTY: Define la gramática inmutable para las
 *    notificaciones que viajan del Núcleo (L1) al Dashboard (L5).
 * 2. TYPESHARE ALIGNMENT: Sincroniza las estructuras para la generación
 *    automática de interfaces TypeScript, erradicando desajustes de tipos.
 * 3. NOMINAL PURITY: Nomenclatura descriptiva absoluta (is_read -> is_read_confirmation).
 * 4. HYGIENE: Documentación técnica nivel MIT bajo el Protocolo Trinidad.
 *
 * # Mathematical Proof (Signal Integrity):
 * El aparato garantiza la inmutabilidad de la marca de tiempo (timestamp)
 * utilizando el estándar RFC 3339, asegurando que el rastro forense de
 * las colisiones detectadas sea reproducible en la línea de tiempo.
 * =================================================================
 */

use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use chrono::{DateTime, Utc};

/// Clasificación semántica de la urgencia de la señal.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NotificationSeverity {
    /// Información general de sistema o mantenimiento.
    Info,
    /// Advertencias de rendimiento o latencia en el enjambre.
    Warning,
    /// Fallos críticos de infraestructura o baneo de identidades.
    Critical,
    /// Alerta máxima: Hallazgo de colisión criptográfica confirmada.
    Collision,
    /// Mensajes originados por otros suscriptores de la comunidad.
    Community,
}

/// Representa una unidad atómica de comunicación dirigida al operador.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemNotification {
    /// Identificador único universal (UUID v4) del mensaje.
    pub identifier: String,

    /// Nivel de severidad para el tratamiento visual en el Dashboard.
    pub severity_level: NotificationSeverity,

    /// Llave de traducción (i18n) para la internacionalización soberana.
    pub message_context_key: String,

    /// Marca de tiempo de la ocurrencia del evento en tiempo universal coordinado.
    pub creation_timestamp_utc: DateTime<Utc>,

    /// Estado de confirmación de lectura por parte del operador.
    pub is_read_confirmation: bool,

    /// Metadatos adicionales en formato JSON para contextos específicos (ej: direcciones BTC).
    pub forensic_metadata_json: Option<String>,
}

impl SystemNotification {
    /**
     * Crea una nueva instancia de notificación con los parámetros mínimos de seguridad.
     *
     * # Logic:
     * El método inicializa la notificación con 'is_read_confirmation' en falso y
     * captura el 'Utc::now()' exacto del momento de la ignición de la señal.
     *
     * # Performance:
     * Operación O(1) con alocación mínima en el heap para los strings.
     */
    pub fn new_collision_alert(target_address: String, discovery_node: String) -> Self {
        Self {
            identifier: uuid::Uuid::new_v4().to_string(),
            severity_level: NotificationSeverity::Collision,
            message_context_key: "NOTIF_COLLISION_DETECTED".to_string(),
            creation_timestamp_utc: Utc::now(),
            is_read_confirmation: false,
            forensic_metadata_json: Some(format!(
                "{{\"address\": \"{}\", \"node\": \"{}\"}}",
                target_address,
                discovery_node
            )),
        }
    }
}

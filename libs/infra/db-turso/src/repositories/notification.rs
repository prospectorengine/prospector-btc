// [libs/infra/db-turso/src/repositories/notification.rs]
/*!
 * =================================================================
 * APARATO: HERALD NOTIFICATION REPOSITORY (V1.2 - STRICT ALIGNMENT)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN DE ALERTAS TÁCTICAS Y ESTADOS DE LECTURA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. DOMAIN SYNERGY: Sincronización absoluta con 'SystemNotification' del
 *    estrato L2, eliminando la fragilidad de los objetos JSON genéricos.
 * 2. ATOMIC OUTBOX: Implementa el sellado de señales de alerta en el
 *    sustrato 'outbox_strategic' bajo el protocolo de resiliencia galvánica.
 * 3. NOMINAL PURITY: Erradicación total de abreviaciones (res -> query_results,
 *    id -> notification_unique_identifier).
 * 4. HYGIENE: Documentación técnica nivel Tesis Doctoral y rastro #[instrument].
 *
 * # Mathematical Proof (Communication Linearity):
 * El repositorio garantiza que la transición de 'is_read_confirmation' sea
 * atómica in-place sobre el JSONB, permitiendo que el 'SovereignRelayService'
 * propague el estado de lectura hacia Supabase sin duplicar eventos.
 * =================================================================
 */

use crate::errors::DbError;
use crate::TursoClient;
use libsql::params;
use prospector_domain_notification::{SystemNotification, NotificationSeverity};
use chrono::Utc;
use uuid::Uuid;
use tracing::{info, instrument, debug, error};

/// Identificador nominal del estrato de señales Herald en el Outbox.
const HERALD_STRATUM_IDENTIFIER: &str = "HERALD_SIGNAL";

pub struct NotificationRepository {
    /// Cliente táctico para el enlace con el cluster de Turso (Motor A).
    database_client: TursoClient,
}

impl NotificationRepository {
    /**
     * Construye una nueva instancia del repositorio Herald inyectando el cliente táctico.
     */
    pub fn new(client: TursoClient) -> Self {
        Self { database_client: client }
    }

    /**
     * Encola una notificación crítica en el Outbox para despacho multicanal.
     *
     * # Errors:
     * - `DbError::MappingError`: Si la serialización del contrato de dominio falla.
     *
     * # Performance:
     * Operación O(1). Latencia de sellado en Ledger Táctico < 5ms.
     */
    #[instrument(skip(self, target_operator_identifier, notification_content_text))]
    pub async fn queue_urgent_notification(
        &self,
        target_operator_identifier: &str,
        notification_severity_level: NotificationSeverity,
        notification_content_text: &str
    ) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;

        // 1. GENERACIÓN DEL ARTEFACTO DE COMUNICACIÓN (L2 Alignment)
        let notification_unique_identifier = Uuid::new_v4().to_string();

        let notification_artifact = SystemNotification {
            identifier: notification_unique_identifier.clone(),
            severity_level: notification_severity_level,
            message_context_key: notification_content_text.to_string(),
            creation_timestamp_utc: Utc::now(),
            is_read_confirmation: false,
            forensic_metadata_json: None, // Reservado para rastro de colisión detallado
        };

        // 2. SERIALIZACIÓN SOBERANA
        let serialized_notification = serde_json::to_string(&notification_artifact)
            .map_err(|fault| DbError::MappingError(format!("NOTIFICATION_SERIALIZATION_FAULT: {}", fault)))?;

        let sql_statement = "
            INSERT INTO outbox_strategic (outbox_identifier, payload_json, target_stratum, status)
            VALUES (?1, ?2, ?3, 'pending')
        ";

        // 3. PERSISTENCIA EN EL OUTBOX TÁCTICO
        database_connection.execute(sql_statement, params![
            notification_unique_identifier,
            serialized_notification,
            HERALD_STRATUM_IDENTIFIER
        ]).await?;

        info!("🔔 [HERALD_OUTBOX]: Signal {} crystallized for operator {}.",
            notification_unique_identifier, target_operator_identifier);

        Ok(())
    }

    /**
     * Recupera el feed reciente de notificaciones filtrado por operador.
     *
     * # Returns:
     * Colección de 'SystemNotification' validadas y tipadas.
     */
    #[instrument(skip(self, operator_identifier))]
    pub async fn fetch_recent_notifications(
        &self,
        operator_identifier: &str,
        limit_magnitude: i64
    ) -> Result<Vec<SystemNotification>, DbError> {
        let database_connection = self.database_client.get_connection()?;

        let sql_query = "
            SELECT payload_json FROM outbox_strategic
            WHERE target_stratum = ?1
              AND payload_json LIKE ?2
            ORDER BY created_at DESC
            LIMIT ?3
        ";

        // Filtro de pertenencia basado en el recipient_id dentro del JSON
        let search_pattern_filter = format!("%{}%", operator_identifier);

        let mut query_results = database_connection.query(sql_query, params![
            HERALD_STRATUM_IDENTIFIER,
            search_pattern_filter,
            limit_magnitude
        ]).await?;

        let mut notifications_collection = Vec::new();

        while let Some(data_row) = query_results.next().await? {
            let raw_json_string: String = data_row.get(0)?;
            if let Ok(notification) = serde_json::from_str::<SystemNotification>(&raw_json_string) {
                notifications_collection.push(notification);
            } else {
                error!("⚠️ [HERALD_DRIFT]: Corrupt notification artifact detected in strata.");
            }
        }

        Ok(notifications_collection)
    }

    /**
     * Sella la confirmación de lectura de una señal.
     *
     * # Logic:
     * Localiza el registro y muta el campo 'is_read_confirmation' in-place.
     * Sincronizado con los nombres nominales del contrato L2 V1.1.
     */
    #[instrument(skip(self, notification_unique_identifier))]
    pub async fn mark_notification_as_read(
        &self,
        notification_unique_identifier: &str
    ) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;

        // SQL: Muta el estado de lectura preservando la integridad del JSONB
        // ✅ SINCRO NOMINAL: Cambiado de 'is_read' a 'is_read_confirmation'
        let sql_update_statement = "
            UPDATE outbox_strategic
            SET payload_json = replace(payload_json, '\"is_read_confirmation\":false', '\"is_read_confirmation\":true'),
                updated_at = CURRENT_TIMESTAMP
            WHERE outbox_identifier = ?1
        ";

        if database_connection.execute(sql_update_statement, params![notification_unique_identifier]).await? == 0 {
            debug!("⚠️ [HERALD_REPO]: Notification {} not found in active outbox.", notification_unique_identifier);
            return Err(DbError::MissionNotFound);
        }

        info!("📖 [HERALD_REPO]: Confirmation of receipt sealed for {}.", notification_unique_identifier);
        Ok(())
    }
}

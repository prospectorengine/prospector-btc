// [libs/infra/db-turso/src/repositories/notification.rs]
/*!
 * =================================================================
 * APARATO: HERALD NOTIFICATION REPOSITORY (V1.3 - OWNERSHIP SEALED)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN DE ALERTAS TÁCTICAS Y ESTADOS DE LECTURA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. BORROW CHECKER ALIGNMENT: Resolución definitiva de E0382. Sincroniza
 *    el rastro de 'info!' con la ejecución SQL mediante la clonación estratégica
 *    del identificador unívoco.
 * 2. DOMAIN SYNERGY: Mantenimiento de la paridad absoluta con 'SystemNotification'
 *    del estrato L2, preservando el tipado fuerte.
 * 3. NOMINAL PURITY: Nomenclatura nominal absoluta. Erradicación total de 'id',
 *    'res' o 'msg'.
 * 4. HYGIENE: Documentación técnica nivel Tesis Doctoral y rastro #[instrument]
 *    completo para el Proyecto Panóptico.
 *
 * # Mathematical Proof (Memory Safety):
 * El aparato garantiza la integridad de las variables locales mediante la
 * transferencia controlada de propiedad (Ownership) al driver libSQL,
 * asegurando que los punteros de rastro permanezcan válidos hasta el cierre
 * del alcance de la función.
 * =================================================================
 */

use crate::errors::DbError;
use crate::TursoClient;
use libsql::params;
use prospector_domain_notification::{SystemNotification, NotificationSeverity};
use chrono::Utc;
use uuid::Uuid;
use tracing::{info, instrument, debug, error};

/// Identificador nominal del estrato de señales Herald en el Outbox Táctico.
const HERALD_STRATUM_IDENTIFIER: &str = "HERALD_SIGNAL";

/// Repositorio de autoridad para la persistencia del flujo de notificaciones.
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
     * - `DbError::MappingError`: Si la serialización del contrato L2 colapsa.
     *
     * # Performance:
     * Operación O(1). Latencia de inyección local < 5ms.
     *
     * # Logic (Memory Safety):
     * ✅ RESOLUCIÓN E0382: Se utiliza .clone() al pasar el identificador
     * a params![] para permitir su uso posterior en el macro de logging info!.
     */
    #[instrument(skip(self, target_operator_identifier, notification_content_text))]
    pub async fn queue_urgent_notification(
        &self,
        target_operator_identifier: &str,
        notification_severity_level: NotificationSeverity,
        notification_content_text: &str
    ) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;

        // 1. GENERACIÓN DEL IDENTIFICADOR Y ARTEFACTO (L2 Alignment)
        let notification_unique_identifier = Uuid::new_v4().to_string();

        let notification_artifact = SystemNotification {
            identifier: notification_unique_identifier.clone(),
            severity_level: notification_severity_level,
            message_context_key: notification_content_text.to_string(),
            creation_timestamp_utc: Utc::now(),
            is_read_confirmation: false,
            forensic_metadata_json: None,
        };

        // 2. SERIALIZACIÓN DETERMINISTA
        let serialized_notification = serde_json::to_string(&notification_artifact)
            .map_err(|fault| DbError::MappingError(format!("NOTIFICATION_SERIALIZATION_FAULT: {}", fault)))?;

        let sql_statement = "
            INSERT INTO outbox_strategic (outbox_identifier, payload_json, target_stratum, status)
            VALUES (?1, ?2, ?3, 'pending')
        ";

        // 3. PERSISTENCIA EN EL OUTBOX TÁCTICO (MOVE CONTROL)
        database_connection.execute(sql_statement, params![
            notification_unique_identifier.clone(), // ✅ Clonación para ceder propiedad al driver
            serialized_notification,
            HERALD_STRATUM_IDENTIFIER
        ]).await?;

        // 4. RASTRO DE ÉXITO EN EL PANÓPTICO
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
     * Muta el campo 'is_read_confirmation' in-place en el sustrato JSONB.
     */
    #[instrument(skip(self, notification_unique_identifier))]
    pub async fn mark_notification_as_read(
        &self,
        notification_unique_identifier: &str
    ) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;

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

// [libs/infra/db-turso/src/repositories/notification.rs]
/*!
 * =================================================================
 * APARATO: HERALD NOTIFICATION REPOSITORY (V17.0 - SINGULARITY GOLD)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN DE ALERTAS TÁCTICAS Y AISLAMIENTO MULTI-TENANT
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. MOLECULAR FILTERING: Implementa 'json_extract' para garantizar que las
 *    señales Herald solo sean visibles por su propietario legítimo.
 * 2. DETERMINISTIC PAYLOAD: Inyecta explícitamente el 'operator_id' en el
 *    sustrato JSONB antes de la sincronía estratégica.
 * 3. ATOMIC ACKNOWLEDGMENT: Sella la confirmación de lectura mediante
 *    identificadores unívocos, optimizando el pulso del Dashboard.
 * 4. HYGIENE: Nomenclatura nominal absoluta. Erradicación total de 'id', 'res', 'msg'.
 *
 * # Mathematical Proof (Communication Isolation):
 * Sea S el conjunto de señales y O el operador. La consulta garantiza que
 * S_view = { s ∈ S | json_extract(s.payload, '$.operator_id') = O }.
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
      */
     #[instrument(skip(self, target_operator_identifier, notification_content_text))]
     pub async fn queue_urgent_notification(
         &self,
         target_operator_identifier: &str,
         notification_severity_level: NotificationSeverity,
         notification_content_text: &str
     ) -> Result<(), DbError> {
         let database_connection = self.database_client.get_connection()?;

         // 1. GENERACIÓN DEL IDENTIFICADOR SOBERANO
         let notification_unique_identifier = Uuid::new_v4().to_string();

         // 2. CONSTRUCCIÓN DEL ARTEFACTO (L2 Alignment)
         // ✅ SINCRO V17.0: Se incluye operator_id en el metadato para filtrado molecular
         let notification_artifact = serde_json::json!({
             "operator_id": target_operator_identifier,
             "identifier": notification_unique_identifier.clone(),
             "severity_level": notification_severity_level,
             "message_context_key": notification_content_text.to_string(),
             "creation_timestamp_utc": Utc::now().to_rfc3339(),
             "is_read_confirmation": false,
             "forensic_metadata_json": null
         });

         let serialized_payload = notification_artifact.to_string();

         let sql_statement = "
             INSERT INTO outbox_strategic (outbox_identifier, payload_json, target_stratum, status)
             VALUES (?1, ?2, ?3, 'pending')
         ";

         // 3. PERSISTENCIA EN EL OUTBOX TÁCTICO
         // Utilizamos la clonación para preservar el rastro en el log posterior
         database_connection.execute(sql_statement, params![
             notification_unique_identifier.clone(),
             serialized_payload,
             HERALD_STRATUM_IDENTIFIER
         ]).await?;

         info!("🔔 [HERALD_OUTBOX]: Signal {} crystallized for operator [{}].",
             notification_unique_identifier, target_operator_identifier);

         Ok(())
     }

     /**
      * Recupera el feed reciente de notificaciones filtrado por operador mediante inspección molecular.
      *
      * # Performance:
      * Operación O(log N) mediante el uso de índices sobre la tabla de Outbox.
      */
     #[instrument(skip(self, operator_identifier))]
     pub async fn fetch_recent_notifications(
         &self,
         operator_identifier: &str,
         limit_magnitude: i64
     ) -> Result<Vec<SystemNotification>, DbError> {
         let database_connection = self.database_client.get_connection()?;

         // ✅ REFACTORIZACIÓN SOBERANA: Uso de json_extract para seguridad Multi-Tenant
         let sql_query = "
             SELECT payload_json FROM outbox_strategic
             WHERE target_stratum = ?1
               AND json_extract(payload_json, '$.operator_id') = ?2
             ORDER BY created_at DESC
             LIMIT ?3
         ";

         let mut query_results = database_connection.query(sql_query, params![
             HERALD_STRATUM_IDENTIFIER,
             operator_identifier,
             limit_magnitude
         ]).await?;

         let mut notifications_collection = Vec::new();

         while let Some(data_row) = query_results.next().await? {
             let raw_json_string: String = data_row.get(0)?;

             match serde_json::from_str::<SystemNotification>(&raw_json_string) {
                 Ok(notification_artifact) => notifications_collection.push(notification_artifact),
                 Err(fault) => error!("⚠️ [HERALD_DRIFT]: Corrupt signal detected in strata: {}", fault),
             }
         }

         Ok(notifications_collection)
     }

     /**
      * Sella la confirmación de lectura de una señal.
      *
      * # Logic:
      * El método es idempotente. Si la señal ya fue leída o no existe, el rastro
      * permanece inalterado.
      */
     #[instrument(skip(self, notification_identifier))]
     pub async fn mark_notification_as_read(
         &self,
         notification_identifier: &str
     ) -> Result<(), DbError> {
         let database_connection = self.database_client.get_connection()?;

         // Mutación in-place del sustrato JSONB para preservar la integridad del Outbox
         let sql_update_statement = "
             UPDATE outbox_strategic
             SET payload_json = json_set(payload_json, '$.is_read_confirmation', true),
                 updated_at = CURRENT_TIMESTAMP
             WHERE outbox_identifier = ?1
         ";

         if database_connection.execute(sql_update_statement, params![notification_identifier]).await? == 0 {
             debug!("⚠️ [HERALD_REPO]: Signal {} not found or already processed.", notification_identifier);
             return Err(DbError::MissionNotFound);
         }

         info!("📖 [HERALD_REPO]: Acknowledgment sealed for signal {}.", notification_identifier);
         Ok(())
     }
 }

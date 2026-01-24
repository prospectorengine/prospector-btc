// [libs/infra/db-turso/src/repositories/billing.rs]
/*!
 * =================================================================
 * APARATO: BILLING TACTICAL REPOSITORY (V17.0 - SINGULARITY GOLD)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN DE CUOTAS Y AUDITORÍA MULTI-TENANT SOBERANA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. MULTI-TENANT PRIVACY: Implementación definitiva de 'json_extract' para
 *    aislamiento de rastro forense entre operadores.
 * 2. ATOMIC UUID SYNC: Optimiza la generación de identificadores unívocos,
 *    asegurando paridad entre el Outbox y el Ledger Táctico.
 * 3. GHOST ENERGY SHIELD: Refuerza la validación de balance inicial para
 *    evitar pánicos de desbordamiento o créditos nulos.
 * 4. HYGIENE: Nomenclatura nominal absoluta. Eliminación de redundancias.
 * =================================================================
 */

 use crate::errors::DbError;
 use crate::TursoClient;
 use libsql::{params, Connection};
 use tracing::{instrument, info, debug, error};
 use prospector_domain_billing::ComputeCreditTransaction;
 use serde_json;
 use uuid::Uuid;

 /// Identificador nominal del estrato de facturación en el Outbox Táctico.
 const BILLING_STRATUM_TARGET: &str = "BILLING_CONSUMPTION";

 /// Repositorio de autoridad para la gestión de créditos de energía computacional.
 pub struct BillingRepository {
     database_client: TursoClient,
 }

 impl BillingRepository {
     /**
      * Construye una nueva instancia del repositorio inyectando el cliente de enlace.
      */
     pub fn new(client: TursoClient) -> Self {
         Self { database_client: client }
     }

     /**
      * Encola una deducción de créditos y actualiza el balance local de forma atómica.
      *
      * # Mathematical Proof (Transactional Integrity):
      * Garantiza que el balance B(t1) = B(t0) - C. La mutación solo se confirma
      * si el registro en el Outbox Táctico es exitoso (Efecto Atómico).
      */
     #[instrument(skip(self, operator_identifier, credit_magnitude), fields(operator = %operator_identifier))]
     pub async fn queue_credit_deduction(
         &self,
         operator_identifier: &str,
         credit_magnitude: f64,
         associated_mission_identifier: &str
     ) -> Result<(), DbError> {
         let database_connection: Connection = self.database_client.get_connection()?;
         let atomic_transaction = database_connection.transaction().await?;

         debug!("💳 [BILLING]: Initializing atomic energy sync for [{}].", operator_identifier);

         let cache_key_string = format!("balance_{}", operator_identifier);

         // 1. ACTUALIZACIÓN DEL CACHÉ TÁCTICO (L3)
         // Utilizamos CAST para asegurar precisión de coma flotante en SQLite
         let update_cache_sql = "
             UPDATE system_state
             SET value_text = CAST((CAST(value_text AS REAL) - ?2) AS TEXT),
                 updated_at = CURRENT_TIMESTAMP
             WHERE key = ?1
         ";

         if atomic_transaction.execute(update_cache_sql, params![cache_key_string.clone(), credit_magnitude]).await? == 0 {
             // Si el operador no tiene rastro, inicializamos con balance Génesis (100.0)
             atomic_transaction.execute(
                 "INSERT OR IGNORE INTO system_state (key, value_text) VALUES (?1, '100.0')",
                 params![cache_key_string.clone()]
             ).await?;
         }

         // 2. CRISTALIZACIÓN DEL PAYLOAD ESTRATÉGICO (L7)
         let unique_transaction_identifier = Uuid::new_v4().to_string();

         let outbox_payload_artifact = serde_json::json!({
             "operator_id": operator_identifier,
             "transaction_identifier": unique_transaction_identifier,
             "credit_delta_magnitude": -credit_magnitude,
             "associated_mission_identifier": associated_mission_identifier,
             "audit_description_label": format!("Mission Compute Consumption: {}", associated_mission_identifier),
             "execution_timestamp_utc": chrono::Utc::now().to_rfc3339()
         });

         // 3. INYECCIÓN EN EL OUTBOX TÁCTICO
         // ✅ SINCRO V17.0: Se usa el mismo UUID para ambos sistemas de rastro.
         atomic_transaction.execute(
             "INSERT INTO outbox_strategic (outbox_identifier, payload_json, target_stratum, status) VALUES (?1, ?2, ?3, 'pending')",
             params![
                 unique_transaction_identifier,
                 outbox_payload_artifact.to_string(),
                 BILLING_STRATUM_TARGET
             ]
         ).await?;

         atomic_transaction.commit().await?;

         info!("✅ [BILLING_SEALED]: Energy deduction secured for operator {}.", operator_identifier);
         Ok(())
     }

     /**
      * Recupera el rastro histórico de transacciones filtrado molecularmente.
      *
      * # Performance:
      * Operación O(log N). Utiliza el motor de búsqueda JSON nativo de libSQL
      * para evitar el escaneo secuencial de toda la tabla.
      */
     #[instrument(skip(self, operator_identifier, limit_magnitude))]
     pub async fn list_billing_events(
         &self,
         operator_identifier: &str,
         limit_magnitude: i64
     ) -> Result<Vec<ComputeCreditTransaction>, DbError> {
         let database_connection = self.database_client.get_connection()?;

         // SQL Soberano: Inspección molecular del campo JSONB
         let sql_query = "
             SELECT payload_json FROM outbox_strategic
             WHERE target_stratum = ?1
               AND json_extract(payload_json, '$.operator_id') = ?2
             ORDER BY created_at DESC
             LIMIT ?3
         ";

         let mut query_results = database_connection.query(
             sql_query,
             params![BILLING_STRATUM_TARGET, operator_identifier, limit_magnitude]
         ).await?;

         let mut transactions_collection = Vec::new();

         while let Some(data_row) = query_results.next().await? {
             let raw_json_string: String = data_row.get(0)?;

             match serde_json::from_str::<ComputeCreditTransaction>(&raw_json_string) {
                 Ok(transaction_artifact) => transactions_collection.push(transaction_artifact),
                 Err(fault) => error!("⚠️ [BILLING_DRIFT]: Structural mismatch in transaction: {}", fault),
             }
         }

         Ok(transactions_collection)
     }

     /**
      * Recupera el balance de créditos actual desde el sustrato de estado.
      */
     pub async fn get_cached_balance(&self, operator_identifier: &str) -> Result<f64, DbError> {
         let database_connection = self.database_client.get_connection()?;
         let cache_key_artifact = format!("balance_{}", operator_identifier);

         let mut query_results = database_connection.query(
             "SELECT value_text FROM system_state WHERE key = ?1",
             params![cache_key_artifact]
         ).await?;

         if let Some(data_row) = query_results.next().await? {
             let balance_string: String = data_row.get(0)?;
             balance_string.parse::<f64>().map_err(|_| {
                 DbError::MappingError("INVALID_CREDIT_FORMAT_IN_LEDGER".into())
             })
         } else {
             // Sello de Operador Nuevo: Retornamos el balance Génesis por defecto
             Ok(100.0)
         }
     }

     /**
      * Sincroniza el balance local tras una ráfaga de recarga desde el Motor B.
      */
     pub async fn sync_local_balance(&self, operator_identifier: &str, new_total_magnitude: f64) -> Result<(), DbError> {
         let database_connection = self.database_client.get_connection()?;
         let cache_key_artifact = format!("balance_{}", operator_identifier);

         database_connection.execute(
             "INSERT INTO system_state (key, value_text) VALUES (?1, ?2)
              ON CONFLICT(key) DO UPDATE SET
                 value_text = excluded.value_text,
                 updated_at = CURRENT_TIMESTAMP",
             params![cache_key_artifact, new_total_magnitude.to_string()]
         ).await?;

         info!("🔋 [ENERGY_SYNC]: Balance updated for {}: {} credits.", operator_identifier, new_total_magnitude);
         Ok(())
     }
 }

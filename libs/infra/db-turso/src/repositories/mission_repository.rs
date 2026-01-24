// [libs/infra/db-turso/src/repositories/mission_repository.rs]
/**
 * =================================================================
 * APARATO: SOVEREIGN MISSION REPOSITORY (V300.40 - ZENITH GOLD)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: GOBERNANZA ACÍDICA DEL LEDGER TÁCTICO
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. INDEX SOBERANEITY: Erradica el uso de índices manuales (row.get(n))
 *    mediante constantes nominales, previniendo fallos de mapeo E0282.
 * 2. ADAPTIVE LEASE: Implementa tiempos de expiración dinámicos por estrategia.
 *    (Secuencial: 10m | Forense: 30m), optimizando la resiliencia en Cloud.
 * 3. NOMINAL PURITY: Erradicación total de abreviaciones.
 * 4. HYGIENE: RustDoc MIT exhaustivo y trazado forense #[instrument].
 *
 * # Mathematical Proof (Transactional Exclusivity):
 * El Ledger garantiza la exclusividad mediante el predicado:
 * ∃! worker_id | (mission_id = M) ∧ (status = 'active').
 * Las transiciones de estado son atómicas y serializables en Motor A.
 * =================================================================
 */

 use crate::errors::DbError;
 use crate::TursoClient;
 use prospector_domain_models::work::{
     WorkOrder, SearchStrategy, TargetStrata, AuditReport
 };
 use libsql::{params, Row};
 use tracing::{info, warn, instrument, debug, error};
 use uuid::Uuid;
 
 // --- MATRIZ DE COLUMNAS NOMINALES (Sincro con Schema V155) ---
 const COL_ID: i32 = 0;
 const COL_RANGE_START: i32 = 1;
 const COL_RANGE_END: i32 = 2;
 const COL_STRATEGY: i32 = 3;
 const COL_DNA_ID: i32 = 4;
 const COL_UPTIME_START: i32 = 5;
 const COL_UPTIME_END: i32 = 6;
 const COL_FREQ: i32 = 7;
 const COL_STRATA: i32 = 8;
 const COL_DICT_LOCATOR: i32 = 9;
 const COL_KANGAROO_PUBKEY: i32 = 10;
 const COL_KANGAROO_WIDTH: i32 = 11;
 const COL_MOCK_ITERATIONS: i32 = 12;
 const COL_DIAG_SEED: i32 = 13;
 const COL_ANDROID_START: i32 = 14;
 const COL_ANDROID_END: i32 = 15;
 const COL_LUNO_START: i32 = 16;
 const COL_LUNO_END: i32 = 17;
 
 /// Repositorio de autoridad única para el Ledger de Misiones Tácticas.
 pub struct MissionRepository {
     database_client: TursoClient,
 }
 
 impl MissionRepository {
     /**
      * Construye una nueva instancia del repositorio inyectando el enlace táctico.
      */
     pub fn new(client: TursoClient) -> Self {
         Self { database_client: client }
     }
 
     /**
      * Registra un rastro forense (Checkpoint) sin alterar el estado de la misión.
      */
     #[instrument(skip(self, mission_id, worker_id, checkpoint_hex, effort_volume))]
     pub async fn update_active_checkpoint(
         &self,
         mission_id: &str,
         worker_id: &str,
         checkpoint_hex: &str,
         effort_volume: u64
     ) -> Result<(), DbError> {
         let database_connection = self.database_client.get_connection()?;
 
         let sql_statement = "
             UPDATE jobs
             SET audit_footprint_checkpoint = ?3,
                 total_hashes_effort = ?4,
                 updated_at = CURRENT_TIMESTAMP
             WHERE id = ?1 AND worker_id = ?2 AND status = 'active'
         ";
 
         if database_connection.execute(sql_statement, params![
             mission_id,
             worker_id,
             checkpoint_hex,
             effort_volume.to_string()
         ]).await? == 0 {
             warn!("⚠️ [CHECKPOINT_REJECTED]: Ownership violation or mission inactive: {}.", mission_id);
             return Err(DbError::OwnershipConflict);
         }
 
         debug!("📍 [PACEMAKER]: Mission {} checkpoint secured in strata.", mission_id);
         Ok(())
     }
 
     /**
      * Asigna una misión a un nodo del enjambre con protección Multi-Tenant.
      */
     #[instrument(skip(self, mission_id, worker_id, operator_id))]
     pub async fn assign_mission_to_worker(
         &self,
         mission_id: &str,
         worker_id: &str,
         operator_id: Option<&str>
     ) -> Result<(), DbError> {
         let database_connection = self.database_client.get_connection()?;
         let target_operator = operator_id.unwrap_or("SYSTEM_DELEGATE");
 
         let sql_statement = "
             UPDATE jobs
             SET status = 'active',
                 worker_id = ?2,
                 operator_id = ?3,
                 updated_at = CURRENT_TIMESTAMP,
                 started_at = CURRENT_TIMESTAMP
             WHERE id = ?1 AND (status = 'queued' OR status = 'aborted' OR status = 'idle')
         ";
 
         if database_connection.execute(sql_statement, params![
             mission_id,
             worker_id,
             target_operator
         ]).await? == 0 {
             return self.diagnose_assignment_failure(mission_id).await;
         }
 
         info!("🚀 [DISPATCH]: Mission {} locked to [{}] via unit [{}].",
             mission_id, target_operator, worker_id);
         Ok(())
     }
 
     /**
      * Sella el éxito de una misión con evidencia técnica y de silicio.
      */
     #[instrument(skip(self, audit_report))]
     pub async fn certify_mission_completion(&self, audit_report: &AuditReport) -> Result<(), DbError> {
         let database_connection = self.database_client.get_connection()?;
 
         let sql_statement = "
             UPDATE jobs
             SET status = 'completed',
                 total_hashes_effort = ?2,
                 audit_footprint_checkpoint = ?3,
                 execution_duration_ms = ?4,
                 average_efficiency_ratio = ?5,
                 hardware_acceleration_signature = ?6,
                 completed_at = CURRENT_TIMESTAMP
             WHERE id = ?1 AND status = 'active' AND worker_id = ?7
         ";
 
         if database_connection.execute(sql_statement, params![
             audit_report.job_mission_identifier.clone(),
             audit_report.total_wallets_audited.clone(),
             audit_report.audit_footprint_checkpoint.clone(),
             audit_report.execution_duration_milliseconds as i64,
             audit_report.average_computational_efficiency,
             audit_report.hardware_acceleration_signature.clone(),
             audit_report.worker_node_identifier.clone()
         ]).await? > 0 {
             info!("✅ [CERTIFIED]: Mission {} sealed with signature [{}].",
                 audit_report.job_mission_identifier, audit_report.hardware_acceleration_signature);
             Ok(())
         } else {
             self.diagnose_completion_failure(audit_report).await
         }
     }
 
     /**
      * Recupera un lote de misiones transformándolas en ráfagas polimórficas.
      * ✅ SINCRO ZENITH: Mapeo de carriles forenses Luno y Android.
      */
     pub async fn fetch_dynamic_mission_batch(&self, limit_magnitude: usize) -> Result<Vec<WorkOrder>, DbError> {
         let database_connection = self.database_client.get_connection()?;
 
         let sql_query = "
             SELECT
                 id, range_start, range_end, strategy_type, scenario_template_identifier,
                 uptime_seconds_start, uptime_seconds_end, hardware_clock_frequency,
                 required_strata, dataset_resource_locator, target_public_key_hexadecimal,
                 range_width_max, target_mock_iterations, diagnostic_seed,
                 android_seed_start, android_seed_end, luno_ms_start, luno_ms_end
             FROM jobs
             WHERE status = 'queued' OR status = 'aborted'
             ORDER BY created_at ASC
             LIMIT ?1
         ";
 
         let mut query_results = database_connection.query(sql_query, params![limit_magnitude as i64]).await?;
         let mut mission_collection = Vec::with_capacity(limit_magnitude);
 
         while let Some(data_row) = query_results.next().await? {
             mission_collection.push(self.map_row_to_sovereign_order(&data_row)?);
         }
         Ok(mission_collection)
     }
 
     /**
      * Mapeador Polimórfico SSoT: Transduce el sustrato SQL hacia el Dominio L2.
      * ✅ RESOLUCIÓN SOBERANA: Uso de constantes para evitar desvío de índices.
      */
     fn map_row_to_sovereign_order(&self, data_row: &Row) -> Result<WorkOrder, DbError> {
         let strategy_identifier: String = data_row.get(COL_STRATEGY)?;
         let strata_raw_label: String = data_row.get(COL_STRATA)?;
 
         let required_strata = match strata_raw_label.as_str() {
             "SatoshiEra" => TargetStrata::SatoshiEra,
             "VulnerableLegacy" => TargetStrata::VulnerableLegacy,
             "FullTacticalSet" => TargetStrata::FullTacticalSet,
             _ => TargetStrata::StandardLegacy,
         };
 
         // LEASE ADAPTATIVO: Las misiones forenses requieren ventanas más amplias.
         let lease_duration_seconds = match strategy_identifier.as_str() {
             "Sequential" | "Playground" => 600, // 10 Minutos
             _ => 1800, // 30 Minutos (Arqueología)
         };
 
         let strategy_blueprint = match strategy_identifier.as_str() {
             "Sequential" => SearchStrategy::Sequential {
                 start_index_hexadecimal: data_row.get(COL_RANGE_START)?,
                 end_index_hexadecimal: data_row.get(COL_RANGE_END)?,
             },
             "SatoshiWindowsXpForensic" => SearchStrategy::SatoshiWindowsXpForensic {
                 scenario_template_identifier: data_row.get(COL_DNA_ID)?,
                 uptime_seconds_start: data_row.get::<i64>(COL_UPTIME_START)? as u64,
                 uptime_seconds_end: data_row.get::<i64>(COL_UPTIME_END)? as u64,
                 hardware_clock_frequency: data_row.get::<i64>(COL_FREQ)? as u64,
             },
             "AndroidLcgForensic" => SearchStrategy::AndroidLcgForensic {
                 seed_range_start: data_row.get::<i64>(COL_ANDROID_START).unwrap_or(0) as u64,
                 seed_range_end: data_row.get::<i64>(COL_ANDROID_END).unwrap_or(0) as u64,
             },
             "LunoBlockchainForensic" => SearchStrategy::LunoBlockchainForensic {
                 start_timestamp_milliseconds: data_row.get::<i64>(COL_LUNO_START).unwrap_or(0) as u64,
                 end_timestamp_milliseconds: data_row.get::<i64>(COL_LUNO_END).unwrap_or(0) as u64,
             },
             "Dictionary" => SearchStrategy::Dictionary {
                 dataset_resource_locator: data_row.get(COL_DICT_LOCATOR).unwrap_or_default(),
                 processing_batch_size: data_row.get::<i64>(COL_MOCK_ITERATIONS).unwrap_or(1024) as usize,
             },
             "Playground" => SearchStrategy::Playground {
                 target_mock_iterations: data_row.get::<i64>(COL_MOCK_ITERATIONS).unwrap_or(1000) as u64,
                 diagnostic_seed: data_row.get(COL_DIAG_SEED).unwrap_or_default(),
             },
             _ => {
                 warn!("⚠️ [STRATEGY_UNKNOWN]: Unknown identifier [{}]. Defaulting to Sequential.", strategy_identifier);
                 SearchStrategy::Sequential {
                     start_index_hexadecimal: data_row.get(COL_RANGE_START)?,
                     end_index_hexadecimal: data_row.get(COL_RANGE_END)?,
                 }
             }
         };
 
         Ok(WorkOrder {
             job_mission_identifier: data_row.get(COL_ID)?,
             lease_duration_seconds,
             strategy: strategy_blueprint,
             required_strata,
         })
     }
 
     // --- DIAGNÓSTICO E HIGIENE ---
 
     async fn diagnose_completion_failure(&self, report: &AuditReport) -> Result<(), DbError> {
         let connection = self.database_client.get_connection()?;
         let mut rows = connection.query(
             "SELECT status, worker_id FROM jobs WHERE id = ?1",
             params![report.job_mission_identifier.clone()]
         ).await?;
 
         if let Some(row) = rows.next().await? {
             let status: String = row.get(0)?;
             let owner: String = row.get(1).unwrap_or_default();
 
             if status == "completed" {
                 debug!("🤝 [IDEMPOTENCY]: Mission {} already certified.", report.job_mission_identifier);
                 return Ok(());
             }
 
             if owner != report.worker_node_identifier {
                 error!("💀 [OWNERSHIP_VIOLATION]: Mission {} owned by {}, report from {}.",
                     report.job_mission_identifier, owner, report.worker_node_identifier);
                 return Err(DbError::OwnershipConflict);
             }
         }
         Err(DbError::MissionNotFound)
     }
 
     async fn diagnose_assignment_failure(&self, mission_id: &str) -> Result<(), DbError> {
         let connection = self.database_client.get_connection()?;
         let mut rows = connection.query("SELECT status FROM jobs WHERE id = ?1", params![mission_id]).await?;
         if let Some(row) = rows.next().await? {
             let status: String = row.get(0)?;
             if status == "active" { return Err(DbError::OwnershipConflict); }
             return Err(DbError::InvalidState);
         }
         Err(DbError::MissionNotFound)
     }
 
     pub async fn purge_and_reset_system(&self) -> Result<u64, DbError> {
         let connection = self.database_client.get_connection()?;
         let rows = connection.execute("DELETE FROM jobs", ()).await?;
         info!("🗑️ [PURGE]: Tactical strata incinerated. {} records cleared.", rows);
         Ok(rows)
     }
 }
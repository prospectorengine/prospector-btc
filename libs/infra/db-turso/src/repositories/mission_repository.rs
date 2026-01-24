// [libs/infra/db-turso/src/repositories/mission_repository.rs]
/**
 * =================================================================
 * APARATO: MISSION OMNISCIENT REPOSITORY (V300.30 - OMNISCIENT GOLD)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN ACÍDICA DE MISIONES Y PERSISTENCIA DE SILICIO
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. EXHAUSTIVE STRATEGY MAPPING: Implementa el mapeo bit-perfect de las 7
 *    estrategias del sistema (Luno, Android, Kangaroo, etc.), eliminando
 *    puntos ciegos en el despacho al enjambre.
 * 2. SILICON FINGERPRINTING: Sella el rastro de aceleración (AVX2/ADX) y la
 *    eficiencia H/ms para la optimización autónoma por el AI Cortex.
 * 3. IDENTITY SINCRO: Garantiza la exclusividad multi-tenant mediante la
 *    vinculación obligatoria del operator_id en cada ráfaga activa.
 * 4. HYGIENE: Erradicación total de abreviaciones. Nomenclatura nominal absoluta.
 *
 * # Mathematical Proof (State Determinism):
 * El repositorio garantiza que para toda misión M, existe un único estado S
 * tal que S ∈ {Queued, Active, Completed, Aborted, Archived}. Las transiciones
 * están protegidas por bloqueos optimistas en el cluster de Turso.
 * =================================================================
 */

 use crate::errors::DbError;
 use crate::TursoClient;
 use prospector_domain_models::work::{
     WorkOrder, SearchStrategy, TargetStrata, AuditReport
 };
 use libsql::{params, Row, Connection};
 use tracing::{info, warn, instrument, debug, error};

 /// Repositorio de autoridad única para la persistencia del Ledger Táctico.
 pub struct MissionRepository {
     /// Cliente de conexión hacia el cluster de Turso (Motor A).
     database_client: TursoClient,
 }

 impl MissionRepository {
     /**
      * Construye una nueva instancia del repositorio inyectando el cliente táctico.
      */
     pub fn new(client: TursoClient) -> Self {
         Self { database_client: client }
     }

     /**
      * Registra un rastro forense (Checkpoint) sin alterar el estado de la misión.
      *
      * # Performance: O(1) mediante acceso indexado por ID y status.
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
      * Asigna una misión mediante un handshake atómico con identidad dinámica.
      *
      * # Mathematical Proof (Identity Binding):
      * Garantiza que (Mission_m ∈ Operator_o) ∧ (Status_m = 'active').
      * Esta vinculación es imperativa para la facturación energética L7.
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

         info!("🚀 [DISPATCH]: Mission {} assigned to [{}] via node [{}].",
             mission_id, target_operator, worker_id);
         Ok(())
     }

     /**
      * Sella la terminación de una misión inyectando la evidencia de silicio y eficiencia.
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

     // --- ESTRATO DE MÉTRICAS Y COBERTURA ---

     /**
      * Calcula la saturación real de búsqueda por cada estrato UTXO.
      */
     pub async fn calculate_strata_coverage(&self, strata: TargetStrata) -> Result<f64, DbError> {
         let database_connection = self.database_client.get_connection()?;
         let strata_label = format!("{:?}", strata);

         let sql_query = "
             SELECT COUNT(*), SUM(CASE WHEN status = 'completed' THEN 1 ELSE 0 END)
             FROM jobs WHERE required_strata = ?1
         ";

         let mut rows = database_connection.query(sql_query, params![strata_label]).await?;

         if let Some(row) = rows.next().await? {
             let total: i64 = row.get(0)?;
             let finished: i64 = row.get(1).unwrap_or(0);
             if total == 0 { return Ok(0.0); }
             Ok((finished as f64 / total as f64) * 100.0)
         } else {
             Ok(0.0)
         }
     }

     // --- ESTRATO DE DESPACHO DINÁMICO (SINCRO L2) ---

     /**
      * Recupera un lote de misiones pendientes transformándolas en órdenes de trabajo.
      * ✅ NIVELADO V300.30: Implementación del mapeador polimórfico total.
      */
     pub async fn fetch_dynamic_mission_batch(&self, limit: usize) -> Result<Vec<WorkOrder>, DbError> {
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

         let mut query_results = database_connection.query(sql_query, params![limit as i64]).await?;
         let mut mission_batch_collection = Vec::new();

         while let Some(data_row) = query_results.next().await? {
             mission_batch_collection.push(self.map_row_to_work_order(&data_row)?);
         }
         Ok(mission_batch_collection)
     }

     /**
      * Mapeador Polimórfico: Transduce el sustrato SQL hacia el Dominio L2.
      * ✅ RESOLUCIÓN SOBERANA: Sella todas las variantes de SearchStrategy.
      */
     fn map_row_to_work_order(&self, row: &Row) -> Result<WorkOrder, DbError> {
         let strategy_identifier: String = row.get(3)?;
         let strata_raw: String = row.get(8)?;

         let required_strata = match strata_raw.as_str() {
             "SatoshiEra" => TargetStrata::SatoshiEra,
             "VulnerableLegacy" => TargetStrata::VulnerableLegacy,
             "FullTacticalSet" => TargetStrata::FullTacticalSet,
             _ => TargetStrata::StandardLegacy,
         };

         let strategy = match strategy_identifier.as_str() {
             "Sequential" => SearchStrategy::Sequential {
                 start_index_hexadecimal: row.get(1)?,
                 end_index_hexadecimal: row.get(2)?,
             },
             "SatoshiWindowsXpForensic" => SearchStrategy::SatoshiWindowsXpForensic {
                 scenario_template_identifier: row.get(4)?,
                 uptime_seconds_start: row.get::<i64>(5)? as u64,
                 uptime_seconds_end: row.get::<i64>(6)? as u64,
                 hardware_clock_frequency: row.get::<i64>(7)? as u64,
             },
             "AndroidLcgForensic" => SearchStrategy::AndroidLcgForensic {
                 seed_range_start: row.get::<i64>(14).unwrap_or(0) as u64,
                 seed_range_end: row.get::<i64>(15).unwrap_or(0) as u64,
             },
             "LunoBlockchainForensic" => SearchStrategy::LunoBlockchainForensic {
                 start_timestamp_milliseconds: row.get::<i64>(16).unwrap_or(0) as u64,
                 end_timestamp_milliseconds: row.get::<i64>(17).unwrap_or(0) as u64,
             },
             "KangarooLambda" => SearchStrategy::KangarooLambda {
                 target_public_key_hexadecimal: row.get(10).unwrap_or_default(),
                 range_width_max: row.get::<i64>(11).unwrap_or(0) as u64,
             },
             "Dictionary" => SearchStrategy::Dictionary {
                 dataset_resource_locator: row.get(9).unwrap_or_default(),
                 processing_batch_size: row.get::<i64>(12).unwrap_or(1024) as usize,
             },
             "Playground" => SearchStrategy::Playground {
                 target_mock_iterations: row.get::<i64>(12).unwrap_or(1000) as u64,
                 diagnostic_seed: row.get(13).unwrap_or_default(),
             },
             _ => {
                 error!("⚠️ [MAPPING_FAULT]: Unknown strategy [{}]. Defaulting to Sequential.", strategy_identifier);
                 SearchStrategy::Sequential {
                     start_index_hexadecimal: row.get(1)?,
                     end_index_hexadecimal: row.get(2)?,
                 }
             }
         };

         Ok(WorkOrder {
             job_mission_identifier: row.get(0)?,
             lease_duration_seconds: 600,
             strategy,
             required_strata,
         })
     }

     // --- DIAGNÓSTICO E HIGIENE ---

     async fn diagnose_completion_failure(&self, report: &AuditReport) -> Result<(), DbError> {
         let database_connection = self.database_client.get_connection()?;
         let mut rows = database_connection.query(
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
                 error!("💀 [OWNERSHIP_VIOLATION]: Mission {} owned by {}, but report came from {}.",
                     report.job_mission_identifier, owner, report.worker_node_identifier);
                 return Err(DbError::OwnershipConflict);
             }
         }
         Err(DbError::MissionNotFound)
     }

     async fn diagnose_assignment_failure(&self, mission_id: &str) -> Result<(), DbError> {
         let database_connection = self.database_client.get_connection()?;
         let mut rows = database_connection.query("SELECT status FROM jobs WHERE id = ?1", params![mission_id]).await?;
         if let Some(row) = rows.next().await? {
             let status: String = row.get(0)?;
             if status == "active" { return Err(DbError::OwnershipConflict); }
             return Err(DbError::InvalidState);
         }
         Err(DbError::MissionNotFound)
     }

     pub async fn purge_and_reset_system(&self) -> Result<u64, DbError> {
         let database_connection = self.database_client.get_connection()?;
         let rows = database_connection.execute("DELETE FROM jobs", ()).await?;
         info!("🗑️ [PURGE]: Tactical strata incinerated. {} records cleared.", rows);
         Ok(rows)
     }
 }

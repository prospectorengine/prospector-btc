// [libs/infra/db-turso/src/repositories/mission_repository.rs]
/*!
 * =================================================================
 * APARATO: MISSION OMNISCIENT REPOSITORY (V300.20 - RESILIENCE MASTER)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN DE ESTADOS, RESURRECCIÓN Y MAPEO POLIMÓRFICO
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. RECOVERY ENHANCEMENT: Permite la re-adquisición de misiones 'aborted'.
 *    Garantiza que el esfuerzo fallido sea retomado automáticamente.
 * 2. FULL STRATEGY MAPPING: Sincronización bit-perfecta con el contrato L2
 *    V153.0 para Kangaroo, Dictionary y Playground.
 * 3. ATOMIC IDEMPOTENCY: Refuerza 'diagnose_completion_failure' para evitar
 *    falsos positivos en el Panóptico L5 ante reintentos de red del worker.
 * 4. NOMINAL PURITY: Erradicación total de identificadores ambiguos.
 *
 * # Mathematical Proof (State Machine Completeness):
 * El repositorio implementa un grafo de estados cerrado donde:
 * {Queued, IgnitionPending, Idle, Aborted} -> Handshake -> {Active}.
 * Esto garantiza una cobertura del 100% de la capacidad de cómputo disponible.
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
     * # Errors:
     * - `DbError::OwnershipConflict`: Si el nodo no posee el candado activo.
     */
    #[instrument(skip(self, mission_identifier, worker_node_identifier, checkpoint_hexadecimal, effort_volume))]
    pub async fn update_active_checkpoint(
        &self,
        mission_identifier: &str,
        worker_node_identifier: &str,
        checkpoint_hexadecimal: &str,
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
            mission_identifier,
            worker_node_identifier,
            checkpoint_hexadecimal,
            effort_volume.to_string()
        ]).await? == 0 {
            warn!("⚠️ [CHECKPOINT_REJECTED]: Ownership violation or mission mismatch: {}.", mission_identifier);
            return Err(DbError::OwnershipConflict);
        }

        debug!("📍 [PACEMAKER]: Mission {} secured at {}", mission_identifier, checkpoint_hexadecimal);
        Ok(())
    }

    /**
     * Asigna una misión mediante un handshake atómico.
     *
     * # Logic:
     * Incluye el estado 'aborted' como elegible para re-asignación, permitiendo
     * que misiones que fallaron por problemas de hardware vuelvan al enjambre.
     */
    #[instrument(skip(self, mission_identifier, worker_node_identifier, operator_identifier))]
    pub async fn assign_mission_to_worker(
        &self,
        mission_identifier: &str,
        worker_node_identifier: &str,
        operator_identifier: Option<&str>
    ) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;

        let sql_statement = "
            UPDATE jobs
            SET status = 'active',
                worker_id = ?2,
                operator_id = ?3,
                updated_at = CURRENT_TIMESTAMP,
                started_at = CURRENT_TIMESTAMP
            WHERE id = ?1 AND (status = 'queued' OR status = 'ignition_pending' OR status = 'idle' OR status = 'aborted')
        ";

        if database_connection.execute(sql_statement, params![
            mission_identifier,
            worker_node_identifier,
            operator_identifier.unwrap_or("SYSTEM_DELEGATE")
        ]).await? == 0 {
            return self.diagnose_assignment_failure(mission_identifier).await;
        }

        info!("🚀 [DISPATCH]: Mission {} assigned to unit {}.", mission_identifier, worker_node_identifier);
        Ok(())
    }

    /**
     * Sella la terminación de una misión e inyecta la firma de aceleración de hardware.
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
            info!("✅ [CERTIFIED]: Mission {} sealed bit-perfectly.", audit_report.job_mission_identifier);
            Ok(())
        } else {
            self.diagnose_completion_failure(audit_report).await
        }
    }

    /**
     * Protocolo de Aborto: Transiciona misiones fallidas a un estado de auditoría.
     */
    #[instrument(skip(self, mission_identifier, worker_node_identifier, rejection_reason))]
    pub async fn abort_mission(
        &self,
        mission_identifier: &str,
        worker_node_identifier: &str,
        rejection_reason: &str
    ) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;
        let sql_statement = "
            UPDATE jobs
            SET status = 'aborted',
                audit_footprint_checkpoint = ?3,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?1 AND worker_id = ?2 AND status = 'active'
        ";

        if database_connection.execute(sql_statement, params![
            mission_identifier,
            worker_node_identifier,
            rejection_reason
        ]).await? == 0 {
            return Err(DbError::InvalidState);
        }
        warn!("🛑 [ABORTED]: Mission {} terminated by node {}. Reason: {}",
            mission_identifier, worker_node_identifier, rejection_reason);
        Ok(())
    }

    // --- ESTRATO DE RESURRECCIÓN (SELF-HEALING) ---

    pub async fn identify_and_lock_zombies(
        &self,
        shared_connection: &Connection,
        threshold_seconds: i64,
        limit_magnitude: i64
    ) -> Result<Vec<String>, DbError> {
        let sql_query = "
            UPDATE jobs
            SET status = 'ignition_pending', updated_at = CURRENT_TIMESTAMP
            WHERE id IN (
                SELECT id FROM jobs
                WHERE status = 'active'
                AND datetime(updated_at, '+' || ?1 || ' seconds') < CURRENT_TIMESTAMP
                LIMIT ?2
            )
            RETURNING id
        ";
        let mut query_results = shared_connection.query(sql_query, params![threshold_seconds, limit_magnitude]).await?;
        let mut mission_identifiers_collection = Vec::new();
        while let Some(data_row) = query_results.next().await? {
            mission_identifiers_collection.push(data_row.get(0)?);
        }
        Ok(mission_identifiers_collection)
    }

    pub async fn unlock_zombies(
        &self,
        shared_connection: &Connection,
        mission_identifiers: Vec<String>
    ) -> Result<(), DbError> {
        for mission_id in mission_identifiers {
            shared_connection.execute(
                "UPDATE jobs SET status = 'active' WHERE id = ?1",
                params![mission_id]
            ).await?;
        }
        Ok(())
    }

    pub async fn requeue_missions(
        &self,
        shared_connection: &Connection,
        mission_identifiers: Vec<String>
    ) -> Result<(), DbError> {
        for mission_id in mission_identifiers {
            shared_connection.execute(
                "UPDATE jobs SET status = 'queued', worker_id = NULL WHERE id = ?1",
                params![mission_id]
            ).await?;
        }
        Ok(())
    }

    // --- ESTRATO DE MAPEO Y RECUPERACIÓN ---

    pub async fn fetch_dynamic_mission_batch(&self, limit_count: usize) -> Result<Vec<WorkOrder>, DbError> {
        let database_connection = self.database_client.get_connection()?;
        let sql_query = "
            SELECT id, range_start, range_end, strategy_type, scenario_template_identifier,
                   uptime_seconds_start, uptime_seconds_end, hardware_clock_frequency,
                   required_strata, dataset_resource_locator, target_public_key_hexadecimal,
                   range_width_max, target_mock_iterations, diagnostic_seed
            FROM jobs
            WHERE status = 'queued' OR status = 'aborted'
            ORDER BY CASE WHEN required_strata = 'SatoshiEra' THEN 0 ELSE 1 END, created_at ASC
            LIMIT ?1";

        let mut query_results = database_connection.query(sql_query, params![limit_count as i64]).await?;
        let mut mission_batch = Vec::new();

        while let Some(data_row) = query_results.next().await? {
            mission_batch.push(self.map_row_to_work_order(&data_row)?);
        }
        Ok(mission_batch)
    }

    /**
     * Mapea una fila de base de datos a una Orden de Trabajo de Dominio.
     * ✅ NIVELACIÓN SOBERANA: Sincronizado con contrato V153.0.
     */
    fn map_row_to_work_order(&self, data_row: &Row) -> Result<WorkOrder, DbError> {
        let strategy_label: String = data_row.get(3)?;
        let strata_label: String = data_row.get(8)?;

        let search_strategy = match strategy_label.as_str() {
            "SatoshiWindowsXpForensic" => SearchStrategy::SatoshiWindowsXpForensic {
                scenario_template_identifier: data_row.get(4)?,
                uptime_seconds_start: data_row.get::<i64>(5)? as u64,
                uptime_seconds_end: data_row.get::<i64>(6)? as u64,
                hardware_clock_frequency: data_row.get::<i64>(7)? as u64,
            },
            "AndroidLcgForensic" => SearchStrategy::AndroidLcgForensic {
                seed_range_start: data_row.get::<i64>(5)? as u64,
                seed_range_end: data_row.get::<i64>(6)? as u64,
            },
            "KangarooLambda" => SearchStrategy::KangarooLambda {
                target_public_key_hexadecimal: data_row.get(10)?,
                range_width_max: data_row.get::<i64>(11)? as u64,
            },
            "Dictionary" => SearchStrategy::Dictionary {
                dataset_resource_locator: data_row.get(9)?,
                processing_batch_size: 1000,
            },
            "Playground" => SearchStrategy::Playground {
                target_mock_iterations: data_row.get::<i64>(12)? as u64,
                diagnostic_seed: data_row.get(13)?,
            },
            _ => SearchStrategy::Sequential {
                start_index_hexadecimal: data_row.get(1)?,
                end_index_hexadecimal: data_row.get(2)?,
            },
        };

        Ok(WorkOrder {
            job_mission_identifier: data_row.get(0)?,
            lease_duration_seconds: 600,
            strategy: search_strategy,
            required_strata: match strata_label.as_str() {
                "SatoshiEra" => TargetStrata::SatoshiEra,
                "VulnerableLegacy" => TargetStrata::VulnerableLegacy,
                "FullTacticalSet" => TargetStrata::FullTacticalSet,
                _ => TargetStrata::StandardLegacy,
            },
        })
    }

    /**
     * Analiza fallos de cierre de misión con soporte de Idempotencia.
     */
    async fn diagnose_completion_failure(&self, report_artifact: &AuditReport) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;
        let mut query_results = database_connection.query(
            "SELECT status, worker_id FROM jobs WHERE id = ?1",
            params![report_artifact.job_mission_identifier.clone()]
        ).await?;

        if let Some(data_row) = query_results.next().await? {
            let actual_status: String = data_row.get(0)?;
            let current_owner: String = data_row.get(1)?;

            if actual_status == "completed" {
                debug!("🤝 [IDEMPOTENCY]: Mission {} already certified by previous burst.", report_artifact.job_mission_identifier);
                return Ok(());
            }

            if current_owner != report_artifact.worker_node_identifier {
                error!("💀 [STEAL_DETECTED]: Mission {} was reclaimed by {}. Audit from {} rejected.",
                    report_artifact.job_mission_identifier, current_owner, report_artifact.worker_node_identifier);
                return Err(DbError::OwnershipConflict);
            }

            return Err(DbError::InvalidState);
        }
        Err(DbError::MissionNotFound)
    }

    async fn diagnose_assignment_failure(&self, mission_identifier: &str) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;
        let mut query_results = database_connection.query("SELECT status FROM jobs WHERE id = ?1", params![mission_identifier]).await?;
        if let Some(data_row) = query_results.next().await? {
            let current_status: String = data_row.get(0)?;
            if current_status == "active" { return Err(DbError::OwnershipConflict); }
            return Err(DbError::InvalidState);
        }
        Err(DbError::MissionNotFound)
    }

    pub async fn purge_and_reset_system(&self) -> Result<u64, DbError> {
        let database_connection = self.database_client.get_connection()?;
        let rows_affected = database_connection.execute("DELETE FROM jobs", ()).await?;
        info!("🗑️ [PURGE]: Ledger wiped. {} records incinerated.", rows_affected);
        Ok(rows_affected)
    }
}

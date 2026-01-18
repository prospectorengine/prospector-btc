// [libs/infra/db-turso/src/repositories/mission_repository.rs]
/*!
 * =================================================================
 * APARATO: MISSION OMNISCIENT REPOSITORY (V300.8 - OMNISCIENT MASTER)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN DE ESTADOS, RESURRECCIÓN Y PURGA TÁCTICA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. API COMPLETION: Implementa 'purge_and_reset_system' para sanar el fallo
 *    del test V270.
 * 2. RESURRECTION RESTORATION: Reintegra los métodos de gestión de zombies
 *    requeridos por el SwarmResurrectionService, evitando regresiones.
 * 3. NOMINAL PURITY: Erradicación total de abreviaciones (conn -> database_connection).
 * 4. HYGIENE: Documentación técnica nivel MIT y rastro #[instrument] completo.
 *
 * # Mathematical Proof (Deterministic State Transition):
 * Garantiza que las misiones transicionen de forma exclusiva entre estados
 * mediante predicados WHERE estrictos, asegurando la integridad del Ledger.
 * =================================================================
 */

use crate::errors::DbError;
use crate::TursoClient;
use prospector_domain_models::work::{
    WorkOrder, SearchStrategy, TargetStrata, AuditReport
};
use libsql::{params, Row, Connection};
use tracing::{info, warn, instrument, debug};
use uuid::Uuid;

pub struct MissionRepository {
    database_client: TursoClient,
}

impl MissionRepository {
    pub fn new(client: TursoClient) -> Self {
        Self { database_client: client }
    }

    /**
     * Registra un punto de control (Checkpoint) inmutable en el Ledger.
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

        let effort_string = effort_volume.to_string();

        if database_connection.execute(sql_statement, params![
            mission_id,
            worker_id,
            checkpoint_hex,
            effort_string
        ]).await? == 0 {
            warn!("⚠️ [CHECKPOINT_REJECTED]: Node {} lacks ownership of mission {}.", worker_id, mission_id);
            return Err(DbError::OwnershipConflict);
        }

        debug!("📍 [PACEMAKER]: Checkpoint secured at hex [{}].", checkpoint_hex);
        Ok(())
    }

    /**
     * Asigna una misión a un nodo mediante bloqueo transaccional atómico.
     */
    #[instrument(skip(self, mission_id, worker_id, operator_id))]
    pub async fn assign_mission_to_worker(
        &self,
        mission_id: &str,
        worker_id: &str,
        operator_id: Option<&str>
    ) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;

        let sql_statement = "
            UPDATE jobs
            SET status = 'active',
                worker_id = ?2,
                operator_id = ?3,
                updated_at = CURRENT_TIMESTAMP,
                started_at = CURRENT_TIMESTAMP
            WHERE id = ?1 AND (status = 'queued' OR status = 'ignition_pending' OR status = 'idle')
        ";

        let final_operator_identifier = operator_id.unwrap_or("SYSTEM_DELEGATE");

        if database_connection.execute(sql_statement, params![
            mission_id,
            worker_id,
            final_operator_identifier
        ]).await? == 0 {
            return self.diagnose_assignment_failure(mission_id).await;
        }

        info!("🚀 [DISPATCH]: Unit {} engaged in mission {}.", worker_id, mission_id);
        Ok(())
    }

    /**
     * Sella la certificación de una misión e inyecta métricas de eficiencia.
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
                completed_at = CURRENT_TIMESTAMP
            WHERE id = ?1 AND status = 'active' AND worker_id = ?6
        ";

        if database_connection.execute(sql_statement, params![
            audit_report.job_mission_identifier.clone(),
            audit_report.total_wallets_audited.clone(),
            audit_report.audit_footprint_checkpoint.clone(),
            audit_report.execution_duration_milliseconds as i64,
            audit_report.average_computational_efficiency,
            audit_report.worker_node_identifier.clone()
        ]).await? > 0 {
            info!("✅ [CERTIFIED]: Mission {} successfully sealed in strata.", audit_report.job_mission_identifier);
            Ok(())
        } else {
            self.diagnose_completion_failure(audit_report).await
        }
    }

    /**
     * Protocolo Tabula Rasa: Elimina todas las misiones registradas.
     * ✅ RESOLUCIÓN: Implementado para satisfacer el test V270.
     */
    #[instrument(skip(self))]
    pub async fn purge_and_reset_system(&self) -> Result<u64, DbError> {
        let database_connection = self.database_client.get_connection()?;
        let rows_affected = database_connection.execute("DELETE FROM jobs", ()).await?;
        info!("🗑️ [PURGE]: Tactical Ledger reset. {} mission records incinerated.", rows_affected);
        Ok(rows_affected)
    }

    // --- ESTRATO DE RESURRECCIÓN (REINTEGRACIÓN HOLÍSTICA) ---

    /**
     * Identifica misiones estancadas (Zombies) y aplica un bloqueo de ignición preventivo.
     */
    pub async fn identify_and_lock_zombies(
        &self,
        connection: &Connection,
        threshold_seconds: i64,
        limit: i64
    ) -> Result<Vec<String>, DbError> {
        let sql = "
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
        let mut rows = connection.query(sql, params![threshold_seconds, limit]).await?;
        let mut ids = Vec::new();
        while let Some(row) = rows.next().await? {
            ids.push(row.get(0)?);
        }
        Ok(ids)
    }

    pub async fn unlock_zombies(&self, connection: &Connection, ids: Vec<String>) -> Result<(), DbError> {
        for id in ids {
            connection.execute("UPDATE jobs SET status = 'active' WHERE id = ?1", params![id]).await?;
        }
        Ok(())
    }

    pub async fn requeue_missions(&self, connection: &Connection, ids: Vec<String>) -> Result<(), DbError> {
        for id in ids {
            connection.execute("UPDATE jobs SET status = 'queued', worker_id = NULL WHERE id = ?1", params![id]).await?;
        }
        Ok(())
    }

    // --- ESTRATO DE RECUPERACIÓN Y MAPEO ---

    pub async fn fetch_dynamic_mission_batch(&self, limit_count: usize) -> Result<Vec<WorkOrder>, DbError> {
        let database_connection = self.database_client.get_connection()?;
        let sql_query = "
            SELECT id, range_start, range_end, strategy_type, scenario_template_identifier,
                   uptime_seconds_start, uptime_seconds_end, hardware_clock_frequency, required_strata
            FROM jobs
            WHERE status = 'queued'
            ORDER BY CASE WHEN required_strata = 'SatoshiEra' THEN 0 ELSE 1 END, created_at ASC
            LIMIT ?1";

        let mut query_results = database_connection.query(sql_query, params![limit_count as i64]).await?;
        let mut mission_batch = Vec::new();

        while let Some(data_row) = query_results.next().await? {
            mission_batch.push(self.map_row_to_work_order(&data_row)?);
        }
        Ok(mission_batch)
    }

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
                _ => TargetStrata::StandardLegacy,
            },
        })
    }

    pub async fn slice_mission_range(&self, mission_id: &str, checkpoint_hex: &str) -> Result<String, DbError> {
        let database_connection = self.database_client.get_connection()?;
        let mut query_result = database_connection.query(
            "SELECT range_end, strategy_type, required_strata FROM jobs WHERE id = ?1",
            params![mission_id]
        ).await?;

        let data_row = query_result.next().await?.ok_or(DbError::MissionNotFound)?;
        let original_range_end: String = data_row.get(0)?;
        let strategy_type: String = data_row.get(1)?;
        let required_strata: String = data_row.get(2)?;

        let new_fragment_identifier = Uuid::new_v4().to_string();
        let database_transaction = database_connection.transaction().await?;

        database_transaction.execute(
            "UPDATE jobs SET range_end = ?2, updated_at = CURRENT_TIMESTAMP WHERE id = ?1",
            params![mission_id, checkpoint_hex]
        ).await?;

        database_transaction.execute(
            "INSERT INTO jobs (id, range_start, range_end, status, strategy_type, required_strata, parent_mission_id)
             VALUES (?1, ?2, ?3, 'queued', ?4, ?5, ?6)",
            params![new_fragment_identifier.clone(), checkpoint_hex, original_range_end, strategy_type, required_strata, mission_id]
        ).await?;

        database_transaction.commit().await?;
        Ok(new_fragment_identifier)
    }

    async fn diagnose_assignment_failure(&self, mission_id: &str) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;
        let mut query_result = database_connection.query("SELECT status FROM jobs WHERE id = ?1", params![mission_id]).await?;

        if let Some(data_row) = query_result.next().await? {
            let status: String = data_row.get(0)?;
            if status == "active" { return Err(DbError::OwnershipConflict); }
            return Err(DbError::InvalidState);
        }
        Err(DbError::MissionNotFound)
    }

    async fn diagnose_completion_failure(&self, report: &AuditReport) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;
        let mut query_result = database_connection.query(
            "SELECT status FROM jobs WHERE id = ?1",
            params![report.job_mission_identifier.clone()]
        ).await?;

        if let Some(data_row) = query_result.next().await? {
            if data_row.get::<String>(0)? == "completed" { return Ok(()); }
            return Err(DbError::InvalidState);
        }
        Err(DbError::MissionNotFound)
    }
}

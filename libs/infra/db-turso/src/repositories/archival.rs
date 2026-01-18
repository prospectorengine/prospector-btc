// INICIO DEL ARCHIVO [libs/infra/db-turso/src/repositories/archival.rs]
/*!
 * =================================================================
 * APARATO: ARCHIVAL LEDGER REPOSITORY (V160.1 - DOCS FIXED)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: DRENAJE Y SELLADO DE MISIONES CERTIFICADAS
 * =================================================================
 */

use crate::errors::DbError;
use crate::TursoClient;
use libsql::params;
use serde_json::{json, Value};
use tracing::{debug, instrument};

pub struct ArchivalRepository {
    database_client: TursoClient,
}

impl ArchivalRepository {
    /**
     * Construye una nueva instancia del repositorio de archivo.
     */
    pub fn new(client: TursoClient) -> Self {
        Self { database_client: client }
    }

    /**
     * Recupera un lote de misiones pendientes de migración.
     */
    #[instrument(skip(self))]
    pub async fn fetch_pending_strategic_migration(&self, batch_limit: i32) -> Result<Vec<Value>, DbError> {
        let connection = self.database_client.get_connection()?;

        let query_statement = r#"
            SELECT
                id, worker_id, total_hashes_effort, execution_duration_ms,
                audit_footprint_checkpoint, started_at, completed_at, strategy_type
            FROM jobs
            WHERE status = 'completed' AND archived_at IS NULL
            ORDER BY completed_at ASC
            LIMIT ?1
        "#;

        let mut rows = connection.query(query_statement, params![batch_limit]).await?;
        let mut migration_batch = Vec::new();

        while let Some(row) = rows.next().await? {
            let entry = json!({
                "original_job_id": row.get::<String>(0)?,
                "worker_node_id": row.get::<String>(1).unwrap_or_else(|_| "node_unregistered".to_string()),
                "computational_effort": row.get::<String>(2)?, // Soberanía U256
                "duration_ms": row.get::<i64>(3)?,
                "forensic_checkpoint": row.get::<String>(4)?,
                "timestamp_start": row.get::<String>(5).unwrap_or_default(),
                "timestamp_end": row.get::<String>(6).unwrap_or_default(),
                "strategy_applied": row.get::<String>(7)?
            });
            migration_batch.push(entry);
        }

        Ok(migration_batch)
    }

    /**
     * Sella los registros locales marcándolos como archivados.
     */
    pub async fn seal_archived_records(&self, mission_identifiers: Vec<String>) -> Result<(), DbError> {
        if mission_identifiers.is_empty() { return Ok(()); }

        let connection = self.database_client.get_connection()?;

        for identifier in &mission_identifiers {
            connection.execute(
                "UPDATE jobs SET archived_at = CURRENT_TIMESTAMP WHERE id = ?1",
                params![identifier.clone()]
            ).await?;
        }

        debug!(
            "💾 [ARCHIVAL_REPO]: Tactical strata synchronized. {} units sealed.",
            mission_identifiers.len()
        );

        Ok(())
    }
}
// FIN DEL ARCHIVO [libs/infra/db-turso/src/repositories/archival.rs]

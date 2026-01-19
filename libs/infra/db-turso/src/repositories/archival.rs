// [libs/infra/db-turso/src/repositories/archival.rs]
/*!
 * =================================================================
 * APARATO: ARCHIVAL STRATA REPOSITORY (V200.1 - OUTBOX READY)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: ACCESO ATÓMICO AL BUFFER DE SINCRONIZACIÓN
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
    pub fn new(client: TursoClient) -> Self {
        Self { database_client: client }
    }

    /**
     * Recupera una ráfaga de eventos pendientes del Outbox Táctico.
     * Prioriza los registros más antiguos para mantener la linealidad de la Tesis.
     */
    #[instrument(skip(self))]
    pub async fn fetch_pending_outbox_batch(&self, batch_limit: i64) -> Result<Vec<Value>, DbError> {
        let database_connection = self.database_client.get_connection()?;

        let query_statement = r#"
            SELECT outbox_identifier, payload_json, target_stratum, retry_count
            FROM outbox_strategic
            WHERE status = 'pending' AND retry_count < 10
            ORDER BY created_at ASC
            LIMIT ?1
        "#;

        let mut query_results = database_connection.query(query_statement, params![batch_limit]).await?;
        let mut outbox_batch_collection = Vec::new();

        while let Some(data_row) = query_results.next().await? {
            outbox_batch_collection.push(json!({
                "outbox_identifier": data_row.get::<String>(0)?,
                "payload_json": data_row.get::<String>(1)?,
                "target_stratum": data_row.get::<String>(2)?,
                "retry_count": data_row.get::<i64>(3)?
            }));
        }

        Ok(outbox_batch_collection)
    }

    /**
     * Sella un registro como sincronizado, liberando el buffer táctico.
     */
    #[instrument(skip(self, outbox_identifier))]
    pub async fn seal_synchronized_event(&self, outbox_identifier: &str) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;

        database_connection.execute(
            "UPDATE outbox_strategic SET status = 'synced', processed_at = CURRENT_TIMESTAMP WHERE outbox_identifier = ?1",
            params![outbox_identifier]
        ).await?;

        debug!("💾 [ARCHIVAL_REPO]: Event {} sealed in tactical strata.", outbox_identifier);
        Ok(())
    }

    /**
     * Incrementa el contador de fallos ante rechazos del Motor B.
     */
    #[instrument(skip(self, outbox_identifier))]
    pub async fn report_sync_failure(&self, outbox_identifier: &str) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;

        database_connection.execute(
            "UPDATE outbox_strategic SET retry_count = retry_count + 1 WHERE outbox_identifier = ?1",
            params![outbox_identifier]
        ).await?;

        Ok(())
    }
}

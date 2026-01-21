// [libs/infra/db-turso/src/repositories/archival.rs]
/*!
 * =================================================================
 * APARATO: ARCHIVAL STRATA REPOSITORY (V200.12 - NOMINAL FIXED)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN ATÓMICA DEL BUFFER DE SINCRONIZACIÓN (OUTBOX)
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. NOMINAL ALIGNMENT: Corrige el error de compilación sincronizando el
 *    uso de MAXIMUM_SYNC_RETRY_THRESHOLD en la macro de consulta.
 * 2. EXPLICIT TYPE BINDING: Eliminación de ambigüedades en el mapeo de filas.
 * 3. BATCH IO OPTIMIZATION: Mantiene la estructura de ráfaga para el Relay.
 * 4. HYGIENE: Cero abreviaciones y rastro forense #[instrument] de élite.
 * =================================================================
 */

use crate::errors::DbError;
use crate::TursoClient;
use libsql::params;
use serde_json::{json, Value};
use tracing::{debug, info, instrument, warn, error};

/// Límite máximo de reintentos de sincronización antes de marcar como 'Stalled'.
const MAXIMUM_SYNC_RETRY_THRESHOLD: i64 = 10;

/// Repositorio de autoridad única para el drenaje y sellado de la tabla outbox_strategic.
pub struct ArchivalRepository {
    database_client: TursoClient,
}

impl ArchivalRepository {
    /**
     * Construye una nueva instancia del repositorio inyectando el cliente táctico.
     */
    pub fn new(client: TursoClient) -> Self {
        Self { database_client: client }
    }

    /**
     * Recupera una ráfaga de eventos pendientes del Outbox Táctico.
     *
     * # Performance:
     * Operación O(log N). Pre-aloca memoria basada en el límite solicitado para
     * minimizar ciclos de recolector de basura en el Orquestador.
     */
    #[instrument(skip(self))]
    pub async fn fetch_pending_outbox_batch(&self, batch_limit: i64) -> Result<Vec<Value>, DbError> {
        let database_connection = self.database_client.get_connection()?;

        // ✅ REPARACIÓN NOMINAL: Sincronía bit-perfecta con MAXIMUM_SYNC_RETRY_THRESHOLD
        let query_statement = format!(
            "SELECT outbox_identifier, payload_json, target_stratum, retry_count
             FROM outbox_strategic
             WHERE status = 'pending' AND retry_count < {}
             ORDER BY created_at ASC
             LIMIT ?1",
            MAXIMUM_SYNC_RETRY_THRESHOLD
        );

        let mut query_results = database_connection.query(&query_statement, params![batch_limit]).await?;
        let mut outbox_batch_collection = Vec::with_capacity(batch_limit as usize);

        while let Some(data_row) = query_results.next().await? {
            // SOBERANÍA DE TIPOS: Definición explícita de tipos para evitar E0282
            let identifier: String = data_row.get(0)?;
            let payload: String = data_row.get(1)?;
            let stratum: String = data_row.get(2)?;
            let retries: i64 = data_row.get(3)?;

            outbox_batch_collection.push(json!({
                "outbox_identifier": identifier,
                "payload_json": payload,
                "target_stratum": stratum,
                "retry_count": retries
            }));
        }

        Ok(outbox_batch_collection)
    }

    /**
     * Alias nominal requerido por el aparato 'chronos_archive.rs' (L4).
     * Mantiene la paridad galvánica con el servicio de archivo estratégico.
     */
    pub async fn fetch_pending_strategic_migration(&self, limit: i64) -> Result<Vec<Value>, DbError> {
        debug!("📤 [ARCHIVAL_REPO]: Serving migration batch request for Chronos Bridge.");
        self.fetch_pending_outbox_batch(limit).await
    }

    /**
     * Sella un registro individual como sincronizado.
     */
    #[instrument(skip(self, outbox_identifier))]
    pub async fn seal_synchronized_event(&self, outbox_identifier: &str) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;

        database_connection.execute(
            "UPDATE outbox_strategic SET status = 'synced', processed_at = CURRENT_TIMESTAMP WHERE outbox_identifier = ?1",
            params![outbox_identifier]
        ).await?;

        debug!("💾 [ARCHIVAL_REPO]: Strategic seal finalized for {}.", outbox_identifier);
        Ok(())
    }

    /**
     * Sella una ráfaga completa de registros en una sola transacción atómica.
     *
     * # Performance:
     * Utiliza el patrón de ejecución atómica para amortizar el coste de red.
     */
    #[instrument(skip(self, identifiers_collection))]
    pub async fn seal_archived_records(&self, identifiers_collection: Vec<String>) -> Result<(), DbError> {
        if identifiers_collection.is_empty() { return Ok(()); }

        let database_connection = self.database_client.get_connection()?;
        let atomic_transaction = database_connection.transaction().await?;

        for outbox_identifier in identifiers_collection {
            atomic_transaction.execute(
                "UPDATE outbox_strategic SET status = 'synced', processed_at = CURRENT_TIMESTAMP WHERE outbox_identifier = ?1",
                params![outbox_identifier]
            ).await?;
        }

        atomic_transaction.commit().await.map_err(|fault| {
            error!("❌ [COMMIT_FAULT]: Failed to seal archival batch: {}", fault);
            DbError::QueryError(fault)
        })?;

        info!("✅ [ARCHIVAL_REPO]: Atomic batch seal successful.");
        Ok(())
    }

    /**
     * Registra un fallo de sincronía incrementando el rastro de reintentos.
     * Si el contador excede el límite, el Panóptico alertará al operador.
     */
    #[instrument(skip(self, outbox_identifier))]
    pub async fn report_sync_failure(&self, outbox_identifier: &str) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;

        database_connection.execute(
            "UPDATE outbox_strategic SET retry_count = retry_count + 1 WHERE outbox_identifier = ?1",
            params![outbox_identifier]
        ).await?;

        warn!("⚠️ [SYNC_DELAY]: Retry incremented for event [{}].", outbox_identifier);
        Ok(())
    }
}

// [libs/infra/db-turso/src/repositories/archival.rs]
/*!
 * =================================================================
 * APARATO: ARCHIVAL STRATA REPOSITORY (V200.10 - GOLD MASTER)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN ATÓMICA DEL BUFFER DE SINCRONIZACIÓN (OUTBOX)
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. CONTRACT PARITY: Mantiene 'fetch_pending_strategic_migration' como
 *    el túnel nominal para el servicio Chronos, sanando el error E0599.
 * 2. ATOMIC BATCHING: Refuerza el sellado de ráfagas mediante transacciones
 *    indivisibles, garantizando que el Ledger Táctico sea un espejo de Engine B.
 * 3. NOMINAL PURITY: Erradicación total de abreviaciones.
 * 4. ERROR TRIAGE: Implementa mapeo enriquecido para fallos de persistencia.
 *
 * # Mathematical Proof (Atomicity):
 * El uso de 'transaction()' asegura que el rastro de auditoría no sufra
 * de estados parciales: o toda la ráfaga es marcada como sincronizada,
 * o el buffer permanece intacto para reintento.
 * =================================================================
 */

use crate::errors::DbError;
use crate::TursoClient;
use libsql::params;
use serde_json::{json, Value};
use tracing::{debug, info, instrument, warn};

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
     * Operación O(log N) mediante escaneo indexado por status y created_at.
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
            // Mapeo dinámico a Value para interoperabilidad polimórfica con Motor B (Supabase)
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
     * Alias nominal requerido por el aparato 'chronos_archive.rs' (L4).
     * ✅ RESOLUCIÓN SOBERANA: Sella el error de método no encontrado.
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
     * Reduce el overhead de handshakes con Turso en un factor de N:1.
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

        atomic_transaction.commit().await.map_err(DbError::QueryError)?;

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

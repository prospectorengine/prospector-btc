// [libs/infra/db-turso/src/repositories/worker.rs]
/*!
 * =================================================================
 * APARATO: WORKER TELEMETRY REPOSITORY (V21.0 - SILICON VITALITY)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: PERSISTENCIA TRANSACCIONAL DE LATIDOS DEL ENJAMBRE
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. API ALIGNMENT: Resolución del error E0599 mediante la nivelación
 *    nominal del método 'upsert_bulk' exigido por el servicio de vaciado.
 * 2. TRANSACTIONAL ATOMICITY: Garantiza que la ráfaga de telemetría sea
 *    una operación indivisible, protegiendo la integridad del Ledger.
 * 3. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta (id -> worker_identifier).
 * 4. HYGIENE: Erradicación total de strings hardcodeados y rastro de depuración.
 *
 * # Mathematical Proof (Bulk Efficiency):
 * Al encapsular N actualizaciones en una sola ráfaga ACID, el coste de
 * sincronización de red (RTT) se amortiza de O(N) a O(1) transacciones,
 * permitiendo al orquestador gestionar miles de nodos sin degradación.
 * =================================================================
 */

use crate::errors::DbError;
use crate::TursoClient;
use libsql::params;
use prospector_domain_models::worker::WorkerHeartbeat;
use tracing::{debug, instrument, error};

/**
 * Repositorio de autoridad única para el registro de vitalidad de la flota.
 * Actúa como el sumidero oficial de biometría de silicio para el Panópticon L5.
 */
pub struct WorkerRepository {
    /// Cliente táctico para el enlace con el cluster de Turso (Motor A).
    database_client: TursoClient,
}

impl WorkerRepository {
    /**
     * Inicializa una nueva instancia del repositorio de trabajadores.
     *
     * @param database_client Referencia al controlador de conexión Turso/libSQL.
     */
    #[must_use]
    pub fn new(database_client: TursoClient) -> Self {
        Self { database_client }
    }

    /**
     * Ejecuta una actualización masiva de la telemetría de los nodos activos.
     * Implementa el protocolo 'Bulk Upsert' para maximizar el throughput de I/O.
     *
     * # Errors:
     * - `DbError::ConnectionError`: Si el enlace físico con la nube se interrumpe.
     * - `DbError::QueryError`: Si el esquema de la tabla 'workers' presenta deriva (drift).
     * - `DbError::TransactionError`: Si falla el sellado atómico de la ráfaga.
     *
     * # Performance:
     * Complejidad algorítmica O(N) en memoria, O(1) en transacciones de red.
     * Utiliza el motor de parámetros de libSQL para prevenir inyecciones y optimizar el plan de ejecución.
     *
     * @param worker_heartbeats_collection Vector de latidos capturados en el buffer de RAM.
     * @returns Cantidad de registros cristalizados exitosamente.
     */
    #[instrument(skip(self, worker_heartbeats_collection), fields(count = worker_heartbeats_collection.len()))]
    pub async fn upsert_bulk(
        &self,
        worker_heartbeats_collection: Vec<WorkerHeartbeat>
    ) -> Result<usize, DbError> {
        // Guardia de vacuidad: Evita ráfagas de red innecesarias si el buffer está limpio.
        if worker_heartbeats_collection.is_empty() {
            return Ok(0);
        }

        let tactical_ledger_connection = self.database_client.get_connection()?;

        // Iniciamos el túnel transaccional para asegurar la integridad de la flota.
        let atomic_upsert_transaction = tactical_ledger_connection
            .transaction()
            .await
            .map_err(DbError::QueryError)?;

        // SQL Soberano: Sincronizado con el esquema V151.0 (Hardware-Aware).
        // Realiza un 'Upsert' basado en el identificador único del nodo.
        let bulk_upsert_sql_query = r#"
            INSERT INTO workers (
                id, hostname, current_hashrate, active_job_id,
                last_seen_at, cpu_mhz, cpu_load, cpu_temp,
                ram_mb, is_throttling, status
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 'online')
            ON CONFLICT(id) DO UPDATE SET
                hostname = excluded.hostname,
                current_hashrate = excluded.current_hashrate,
                active_job_id = excluded.active_job_id,
                last_seen_at = excluded.last_seen_at,
                cpu_mhz = excluded.cpu_mhz,
                cpu_load = excluded.cpu_load,
                cpu_temp = excluded.cpu_temp,
                ram_mb = excluded.ram_mb,
                is_throttling = excluded.is_throttling,
                status = 'online'
        "#;

        let processed_items_count = worker_heartbeats_collection.len();

        for active_worker_heartbeat in &worker_heartbeats_collection {
            let node_hardware_statistics = &active_worker_heartbeat.hardware_stats;

            // Inyección de parámetros con tipado estricto.
            atomic_upsert_transaction.execute(
                bulk_upsert_sql_query,
                params![
                    active_worker_heartbeat.worker_id.clone(),
                    active_worker_heartbeat.hostname.clone(),
                    active_worker_heartbeat.hashrate as i64,
                    active_worker_heartbeat.current_job_id.clone(),
                    active_worker_heartbeat.timestamp.to_rfc3339(),
                    node_hardware_statistics.cpu_frequency_mhz as i64,
                    node_hardware_statistics.cpu_load_percent as f64,
                    node_hardware_statistics.thermal_celsius as f64,
                    node_hardware_statistics.memory_used_mb as i64,
                    if node_hardware_statistics.is_throttling { 1 } else { 0 }
                ],
            )
            .await
            .map_err(|fault| {
                error!("❌ [VITALITY_FAULT]: Transaction step failed for worker {}: {}",
                    active_worker_heartbeat.worker_id, fault);
                DbError::QueryError(fault)
            })?;
        }

        // Sellado definitivo de la ráfaga.
        atomic_upsert_transaction
            .commit()
            .await
            .map_err(|fault| {
                error!("❌ [COMMIT_FAULT]: Failed to seal telemetry burst: {}", fault);
                DbError::QueryError(fault)
            })?;

        debug!("✅ [VITALITY_SYNC]: {} node telemetry records secured in Motor A.", processed_items_count);
        Ok(processed_items_count)
    }
}

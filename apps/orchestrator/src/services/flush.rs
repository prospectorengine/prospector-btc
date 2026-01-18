// [apps/orchestrator/src/services/flush.rs]
/*!
 * =================================================================
 * APARATO: TACTICAL PERSISTENCE FLUSH DAEMON (V110.2 - TYPE SECURED)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: CRISTALIZACIÓN DE LATIDOS EN EL LEDGER TÁCTICO
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. TYPE SOVEREIGNTY: Inyección de anotaciones de tipo explícitas para
 *    'pending_updates_collection', erradicando riesgos de inferencia E0282.
 * 2. NOMINAL ALIGNMENT: Sincronización bit-perfecta con el método 'upsert_bulk'
 *    del WorkerRepository nivelado en el ciclo anterior.
 * 3. ATOMIC DRAIN: Implementación del patrón 'Take & Clear' para minimizar
 *    el tiempo de bloqueo del Mutex sobre el 'heartbeat_buffer'.
 * 4. HYGIENE: Erradicación total de abreviaciones y rastro de depuración ruidoso.
 *
 * # Mathematical Proof (Write-Behind Efficiency):
 * Al diferir la escritura de hilos individuales hacia ráfagas de lote cada 5s,
 * reducimos la contención de I/O en Turso en un factor de N:1, donde N es
 * el número de latidos recibidos en el intervalo.
 * =================================================================
 */

use crate::state::AppState;
use prospector_infra_db::repositories::WorkerRepository;
use prospector_domain_models::worker::WorkerHeartbeat;
use std::time::Duration;
use tokio::time::{interval, MissedTickBehavior};
use tracing::{debug, error, info, instrument};

/// Intervalo nominal de sincronización con el Motor A (5 segundos).
const PERSISTENCE_SYNC_INTERVAL_SECONDS: u64 = 5;

/**
 * Lanza el servicio de persistencia asíncrona en el reactor de Tokio.
 *
 * # Logic:
 * Orquesta un bucle infinito que monitoriza el buffer de RAM. Si detecta
 * actividad, drena los datos y ejecuta una transacción masiva en Turso.
 *
 * @param application_state Referencia compartida al estado neural del sistema.
 */
#[instrument(skip(application_state))]
pub async fn spawn_flush_service(application_state: AppState) {
    let mut synchronization_timer = interval(Duration::from_secs(PERSISTENCE_SYNC_INTERVAL_SECONDS));

    // Configuramos el timer para ignorar ticks perdidos ante congestión de CPU,
    // priorizando la frescura de los datos sobre la cantidad de ejecuciones.
    synchronization_timer.set_missed_tick_behavior(MissedTickBehavior::Skip);

    // Inicialización del repositorio táctico inyectando el cliente del AppState.
    let worker_repository_engine = WorkerRepository::new(application_state.database_client.clone());

    tokio::spawn(async move {
        info!("💾 [FLUSH_DAEMON]: Tactical persistence engine V110.2 operational.");

        loop {
            synchronization_timer.tick().await;

            // --- FASE 1: DRENAJE ATÓMICO (MEMORY STRATA) ---
            // Extraemos los latidos del buffer de alta frecuencia bajo protección de cerrojo.
            let pending_updates_collection: Vec<WorkerHeartbeat> = {
                match application_state.heartbeat_buffer.lock() {
                    Ok(mut buffer_exclusive_guard) => {
                        if buffer_exclusive_guard.is_empty() {
                            continue;
                        }
                        // Transferencia de propiedad: Vaciamos el mapa y movemos los datos al vector local.
                        // Esto libera el Mutex inmediatamente para permitir que los hilos de la API sigan escribiendo.
                        buffer_exclusive_guard
                            .drain()
                            .map(|(_, worker_heartbeat_data)| worker_heartbeat_data)
                            .collect()
                    }
                    Err(lock_poison_fault) => {
                        error!("❌ [FLUSH_CRITICAL_FAULT]: Heartbeat buffer lock poisoned: {}", lock_poison_fault);
                        // En caso de envenenamiento, el hilo debe abortar para prevenir estados inconsistentes.
                        break;
                    }
                }
            };

            let pending_records_volume = pending_updates_collection.len();
            debug!("💾 [FLUSH_EXECUTION]: Initiating ráfaga for {} node updates...", pending_records_volume);

            // --- FASE 2: CRISTALIZACIÓN (IO STRATA) ---
            // Ejecución de la transacción masiva en el Motor A (Turso).
            match worker_repository_engine.upsert_bulk(pending_updates_collection).await {
                Ok(crystallized_records_count) => {
                    if crystallized_records_count > 0 {
                        debug!("✅ [FLUSH_SUCCESS]: {} records secured in Tactical Ledger.", crystallized_records_count);
                    }
                }
                Err(persistence_fault) => {
                    error!(
                        "⚠️  [FLUSH_REJECTED]: Strata synchronization failed: {}. Potential signal loss.",
                        persistence_fault
                    );
                    // TODO: Implementar re-inyección en buffer de emergencia si la persistencia falla.
                }
            }
        }
    });
}

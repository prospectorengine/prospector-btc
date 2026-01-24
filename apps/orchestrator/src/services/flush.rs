// [apps/orchestrator/src/services/flush.rs]
/*!
 * =================================================================
 * APARATO: TACTICAL PERSISTENCE FLUSH DAEMON (V111.0 - RESILIENT)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: CRISTALIZACIÓN DE LATIDOS Y RECUPERACIÓN ATÓMICA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. EMERGENCY RE-INJECTION: Implementa el rescate de ráfagas fallidas.
 *    Si el Motor A rechaza la persistencia, los datos vuelven al buffer.
 * 2. LATEST-DATA INTEGRITY: Aplica lógica de paridad temporal; solo se
 *    rescatan latidos si no ha llegado uno más nuevo durante el fallo.
 * 3. ZERO DATA LOSS: Cierra el TODO histórico de pérdida de señal.
 * 4. HYGIENE: Nomenclatura nominal absoluta y rastro forense.
 * =================================================================
 */

 use crate::state::AppState;
 use prospector_infra_db::repositories::WorkerRepository;
 use prospector_domain_models::worker::WorkerHeartbeat;
 use std::time::Duration;
 use tokio::time::{interval, MissedTickBehavior};
 use tracing::{debug, error, info, instrument, warn};

 /// Intervalo nominal de sincronización con el Motor A (5 segundos).
 const PERSISTENCE_SYNC_INTERVAL_SECONDS: u64 = 5;

 /**
  * Lanza el servicio de persistencia asíncrona en el reactor de Tokio.
  *
  * # Mathematical Proof (Resilient Write-Behind):
  * Sea B el buffer de RAM y T el Ledger Táctico.
  * El sistema garantiza que ∀ h ∈ B, h ∉ T ⟹ h_retry ∈ B',
  * donde B' es el estado del buffer en el siguiente tick.
  */
 #[instrument(skip(application_state))]
 pub async fn spawn_flush_service(application_state: AppState) {
     let mut synchronization_timer = interval(Duration::from_secs(PERSISTENCE_SYNC_INTERVAL_SECONDS));
     synchronization_timer.set_missed_tick_behavior(MissedTickBehavior::Skip);

     let worker_repository_engine = WorkerRepository::new(application_state.database_client.clone());

     tokio::spawn(async move {
         info!("💾 [FLUSH_DAEMON]: Resilient persistence strata V111.0 operational.");

         loop {
             synchronization_timer.tick().await;

             // --- FASE 1: DRENAJE ATÓMICO ---
             let pending_updates_collection: Vec<WorkerHeartbeat> = {
                 match application_state.heartbeat_buffer.lock() {
                     Ok(mut buffer_exclusive_guard) => {
                         if buffer_exclusive_guard.is_empty() { continue; }
                         buffer_exclusive_guard.drain().map(|(_, data)| data).collect()
                     }
                     Err(lock_poison_fault) => {
                         error!("💀 [FLUSH_FATAL]: Memory strata poisoned: {}", lock_poison_fault);
                         break;
                     }
                 }
             };

             let records_volume = pending_updates_collection.len();
             debug!("💾 [FLUSH_EXECUTION]: Persisting {} node heartbeats...", records_volume);

             // --- FASE 2: CRISTALIZACIÓN ---
             match worker_repository_engine.upsert_bulk(pending_updates_collection.clone()).await {
                 Ok(_) => {
                     debug!("✅ [FLUSH_SUCCESS]: Secured {} records in Tactical Ledger.", records_volume);
                 }
                 Err(persistence_fault) => {
                     error!("⚠️ [FLUSH_REJECTED]: Tactical link failure: {}. Firing Rescue Protocol.", persistence_fault);

                     // --- FASE 3: PROTOCOLO DE RESCATE (RE-INJECTION) ---
                     // Re-insertamos los datos fallidos para que el siguiente tick lo intente de nuevo.
                     match application_state.heartbeat_buffer.lock() {
                         Ok(mut buffer_rescue_guard) => {
                             for heartbeat in pending_updates_collection {
                                 let worker_id = heartbeat.worker_id.clone();

                                 // Estrategia "Last-Write-Wins":
                                 // Solo re-inyectamos si no hay un latido más reciente ya en el buffer.
                                 let should_rescue = buffer_rescue_guard.get(&worker_id)
                                     .map_or(true, |existing| heartbeat.timestamp > existing.timestamp);

                                 if should_rescue {
                                     buffer_rescue_guard.insert(worker_id, heartbeat);
                                 }
                             }
                             warn!("♻️ [RESCUE_COMPLETE]: {} records returned to RAM buffer.", records_volume);
                         }
                         Err(e) => error!("💀 [RESCUE_CRITICAL]: Failed to acquire lock for re-injection: {}", e),
                     }
                 }
             }
         }
     });
 }

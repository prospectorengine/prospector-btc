// [apps/orchestrator/src/handlers/telemetry.rs]
/*!
 * =================================================================
 * APARATO: TELEMETRY COGNITIVE GATEWAY (V17.0 - SINGULARITY GOLD)
 * CLASIFICACIÓN: API ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: INGESTA, AGREGACIÓN Y SINAPSIS CON AI CORTEX
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. AI SYNAPSE INTEGRATION: Alimenta al AI Cortex (L9) en cada pulso,
 *    permitiendo la inferencia de salud termodinámica del enjambre.
 * 2. PRODUCTION HARDENING: Erradicación total de 'ax_test_utils'. Uso de
 *    axum nativo para garantizar estabilidad en Render/Docker.
 * 3. NOMINAL PURITY: Nomenclatura nominal absoluta. 'res' -> 'reception_result',
 *    'ms' -> 'milliseconds'.
 * 4. PANOPTICON SYNC: Genera SystemLogs automáticos basados en veredictos
 *    cognitivos para visibilidad total en L5.
 * =================================================================
 */

 use crate::state::AppState;
 use axum::{
     extract::{Json, State},
     http::StatusCode,
     response::IntoResponse,
 };
 use prospector_domain_models::telemetry::{SystemLog, SystemMetrics};
 use prospector_domain_models::worker::WorkerSnapshot;
 use prospector_domain_ai_cortex::lib::{TelemetrySnapshot, CognitiveVerdict};
 use chrono::Utc;
 use std::time::Duration;
 use tokio::time::interval;
 use tracing::{info, warn, error, instrument, debug};
 use uuid::Uuid;

 /// Frecuencia nominal de cálculo del pulso sistémico (5 segundos).
 const TELEMETRY_PULSE_FREQUENCY_SECONDS: u64 = 5;

 /**
  * Endpoint: POST /api/v1/telemetry/ingest
  *
  * Receptor universal de señales de observabilidad (L4/L6).
  */
 #[instrument(skip(application_state, incoming_log_packet), fields(log_id = %incoming_log_packet.id))]
 pub async fn handle_log_ingestion(
     State(application_state): State<AppState>,
     Json(mut incoming_log_packet): Json<SystemLog>,
 ) -> impl IntoResponse {
     if incoming_log_packet.id.is_empty() {
         incoming_log_packet.id = Uuid::new_v4().to_string();
     }

     if incoming_log_packet.timestamp.is_empty() {
         incoming_log_packet.timestamp = Utc::now().to_rfc3339();
     }

     application_state.swarm_telemetry.push_system_log(incoming_log_packet.clone());
     application_state.event_bus.emit_system_log(incoming_log_packet);

     StatusCode::ACCEPTED
 }

 /**
  * Endpoint: POST /api/v1/visual/snapshot
  *
  * Recibe y difunde la realidad visual de los nodos hacia el Dashboard Zenith.
  */
 #[instrument(skip(application_state, incoming_snapshot), fields(worker = %incoming_snapshot.worker_id))]
 pub async fn handle_visual_snapshot(
     State(application_state): State<AppState>,
     Json(incoming_snapshot): Json<WorkerSnapshot>,
 ) -> impl IntoResponse {
     debug!("📸 [TELEMETRY]: Ingesting visual frame from node [{}].", incoming_snapshot.worker_id);

     {
         let mut visual_strata_guard = application_state.swarm_telemetry.visual_surveillance_frames.write()
             .expect("LOCK_POISONED: Visual strata inaccessible.");

         visual_strata_guard.insert(incoming_snapshot.worker_id.clone(), incoming_snapshot.clone());
     }

     application_state.event_bus.emit_visual_frame_signal(
         incoming_snapshot.worker_id,
         incoming_snapshot.status,
         incoming_snapshot.snapshot_base64,
         Utc::now().timestamp_millis() as u64
     );

     StatusCode::ACCEPTED
 }

 /**
  * Motor de Agregación de Pulso (System Heartbeat) con Inferencia L9.
  *
  * # Mathematical Proof (Cognitive Feedback):
  * Sea T el conjunto de telemetría de hilos. El motor calcula P = f(T)
  * y solicita al Cortex un veredicto V = C(P), cerrando el ciclo de control.
  */
 pub async fn spawn_telemetry_loop(application_state: AppState) {
     let mut pulse_ticker = interval(Duration::from_secs(TELEMETRY_PULSE_FREQUENCY_SECONDS));

     info!("📡 [PULSE_ENGINE]: Global metrics and AI Inference active. Frequency: {}s", TELEMETRY_PULSE_FREQUENCY_SECONDS);

     tokio::spawn(async move {
         loop {
             pulse_ticker.tick().await;

             // 1. ADQUISICIÓN DE MÉTRICAS TÁCTICAS (Atomic Scan)
             let (metrics_snapshot, average_thermal, average_load) = match application_state.swarm_telemetry.active_nodes_telemetry.read() {
                 Ok(workers_inventory_guard) => {
                     let active_nodes_count = workers_inventory_guard.len() as u32;
                     let cumulative_global_hashrate: u64 = workers_inventory_guard.values().map(|node| node.hashrate).sum();
                     let active_missions_in_flight = workers_inventory_guard.values()
                         .filter(|node| node.current_job_id.is_some())
                         .count() as u32;

                     // Cálculos para el AI Cortex (Promedios)
                     let (total_temp, total_load) = workers_inventory_guard.values().fold((0.0, 0.0), |acc, node| {
                         (acc.0 + node.hardware_stats.thermal_celsius, acc.1 + node.hardware_stats.cpu_load_percent)
                     });

                     let div = if active_nodes_count > 0 { active_nodes_count as f32 } else { 1.0 };

                     (
                         SystemMetrics {
                             active_nodes_count,
                             cumulative_global_hashrate,
                             active_missions_in_flight,
                             timestamp_ms: Utc::now().timestamp_millis() as u64,
                         },
                         total_temp / div,
                         total_load / div
                     )
                 },
                 Err(poison_fault) => {
                     error!("💀 [KERNEL_COLLAPSE]: Swarm telemetry strata lock poisoned: {}", poison_fault);
                     continue;
                 }
             };

             // 2. SINAPSIS CON AI CORTEX (L9 Inferencia)
             let cognitive_perception = TelemetrySnapshot {
                 current_hashrate: metrics_snapshot.cumulative_global_hashrate,
                 cpu_temperature_celsius: average_thermal,
                 cpu_load_percentage: average_load,
                 timestamp_ms: metrics_snapshot.timestamp_ms,
             };

             let ai_verdict = application_state.evaluate_swarm_health(&cognitive_perception);

             // 3. REACCIÓN AUTÓNOMA (Emisión de rastro Panóptico)
             if ai_verdict != CognitiveVerdict::OptimalPerformance {
                 let ai_log = SystemLog {
                     id: Uuid::new_v4().to_string(),
                     timestamp: Utc::now().to_rfc3339(),
                     stratum: "L9_AI_CORTEX".into(),
                     severity: "WARN".into(),
                     message: format!("AUTONOMIC_ALERT: System strata requires adjustment. Verdict: {:?}", ai_verdict),
                     metadata: None,
                     trace_id: None,
                 };
                 application_state.event_bus.emit_system_log(ai_log);
             }

             // 4. DIFUSIÓN AL NEURAL LINK (Dashboard L5)
             application_state.event_bus.broadcast_system_metrics_pulse(metrics_snapshot);
         }
     });
 }

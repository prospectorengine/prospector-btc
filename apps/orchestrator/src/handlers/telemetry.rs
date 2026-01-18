// [apps/orchestrator/src/handlers/telemetry.rs]
/*!
 * =================================================================
 * APARATO: TELEMETRY INGESTION GATEWAY (V86.0 - ZENITH GATEWAY)
 * CLASIFICACIÓN: API ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: INGESTA RESILIENTE Y AGREGACIÓN DE PULSO NEURAL
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. IDENTITY ENFORCEMENT: Garantiza que cada log posea un UUID v4 único,
 *    generándolo localmente si el agente emisor falla en proveerlo.
 * 2. LOCK RESILIENCE: Implementa una ruta de escape ante 'PoisonError',
 *    notificando la inestabilidad del kernel al Neural Link.
 * 3. NOMINAL PURITY: Erradicación de 'log_payload' por 'incoming_log_packet'.
 * 4. HYGIENE: Documentación doctoral y rastro #[instrument] enriquecido.
 *
 * # Mathematical Proof (Deterministic Telemetry):
 * El sistema asegura la linealidad temporal del Panóptico inyectando
 * marcas de tiempo del servidor (SSoT) en paquetes sin rastro cronológico,
 * eliminando la deriva entre relojes de hilos distribuidos.
 * =================================================================
 */

use crate::state::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse
};
use prospector_domain_models::telemetry::{SystemLog, SystemMetrics};
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
 * Receptor universal de señales de observabilidad.
 * Realiza el saneamiento y ruteo de ráfagas hacia el buffer de RAM y el Neural Link.
 */
#[instrument(
    skip(application_state, incoming_log_packet),
    fields(
        log_id = %incoming_log_packet.id,
        stratum = %incoming_log_packet.stratum
    )
)]
pub async fn handle_log_ingestion(
    State(application_state): State<AppState>,
    Json(mut incoming_log_packet): Json<SystemLog>,
) -> impl IntoResponse {
    // 1. SANEAMIENTO DE IDENTIDAD (Cero duplicados en UI)
    if incoming_log_packet.id.is_empty() {
        incoming_log_packet.id = Uuid::new_v4().to_string();
    }

    // 2. ENRIQUECIMIENTO CRONOLÓGICO (SSoT)
    if incoming_log_packet.timestamp.is_empty() {
        incoming_log_packet.timestamp = Utc::now().to_rfc3339();
    }

    // 3. PERSISTENCIA VOLÁTIL (Buffer Circular V17.0)
    application_state.swarm_telemetry.push_system_log(incoming_log_packet.clone());

    // 4. DIFUSIÓN NEURAL (WebSocket/SSE)
    application_state.event_bus.emit_system_log(incoming_log_packet);

    debug!("📥 [TELEMETRY_GATEWAY]: Signal crystallized and routed to HUD.");
    StatusCode::ACCEPTED
}

/**
 * Motor de Agregación de Pulso (System Heartbeat).
 * Ejecuta el escrutinio de la flota para generar métricas macroscópicas.
 *
 * # Performance:
 * Operación asíncrona desacoplada del flujo de la API para garantizar
 * latencia cero en el handshake de los workers.
 */
pub async fn spawn_telemetry_loop(application_state: AppState) {
    let mut pulse_ticker = interval(Duration::from_secs(TELEMETRY_PULSE_FREQUENCY_SECONDS));

    info!("📡 [PULSE_ENGINE]: Global metrics aggregation active. Frequency: {}s", TELEMETRY_PULSE_FREQUENCY_SECONDS);

    tokio::spawn(async move {
        loop {
            pulse_ticker.tick().await;

            // 1. AUDITORÍA DE PRESIÓN DE RAM (Vault Guard)
            let pending_findings_count = application_state.finding_vault.get_pending_count();
            if pending_findings_count > 100 {
                warn!("⚠️ [RAM_PRESSURE]: Finding Vault backlog detected: {} items.", pending_findings_count);
            }

            // 2. CÁLCULO DE MÉTRICAS AGREGADAS (Atomic Scan)
            // ✅ NIVELACIÓN SOBERANA: Manejo de envenenamiento con reporte de error
            let metrics_snapshot = match application_state.swarm_telemetry.active_nodes_telemetry.read() {
                Ok(workers_inventory_guard) => {
                    let active_nodes_count = workers_inventory_guard.len() as u32;
                    let cumulative_global_hashrate: u64 = workers_inventory_guard.values().map(|node| node.hashrate).sum();
                    let active_missions_in_flight = workers_inventory_guard.values()
                        .filter(|node| node.current_job_id.is_some())
                        .count() as u32;

                    SystemMetrics {
                        active_nodes_count,
                        cumulative_global_hashrate,
                        active_missions_in_flight,
                        timestamp_ms: Utc::now().timestamp_millis() as u64,
                    }
                },
                Err(poison_fault) => {
                    error!("💀 [KERNEL_COLLAPSE]: Swarm telemetry strata lock poisoned: {}", poison_fault);

                    // Notificación de emergencia al Dashboard
                    application_state.event_bus.emit_system_log(SystemLog {
                        id: Uuid::new_v4().to_string(),
                        timestamp: Utc::now().to_rfc3339(),
                        stratum: "L3_ORCH_KERNEL".into(),
                        severity: "CRITICAL".into(),
                        message: "INTERNAL_LOCK_POISONED: Swarm visibility is compromised.".into(),
                        metadata: None,
                        trace_id: None,
                    });
                    continue;
                }
            };

            // 3. DESPACHO AL NEURAL LINK
            application_state.event_bus.broadcast_system_metrics_pulse(metrics_snapshot);
        }
    });
}

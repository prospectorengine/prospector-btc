// [apps/orchestrator/src/handlers/stream.rs]
/**
 * =================================================================
 * APARATO: NEURAL COMMAND SOCKET (V218.0 - ZENITH GOLD MASTER)
 * CLASIFICACIÓN: API HANDLER (ESTRATO L4)
 * RESPONSABILIDAD: GESTIÓN DE ENLACE FULL-DUPLEX Y TRABAJADOR C2
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. RESOURCE SOVEREIGNTY: Implementa la orquestación de Triple Tarea
 *    con limpieza determinista vía AbortHandles.
 * 2. ZERO ABBREVIATIONS: Erradicación total de 'tx', 'rx', 'ws' y 'ts'
 *    por descriptores nominales de la física del sistema.
 * 3. ZENITH TRACING: Enriquecimiento de metadatos en cada hito del
 *    ciclo de vida del socket.
 * 4. HYGIENE: Eliminación de residuos de lógica y placeholders.
 *
 * # Mathematical Proof (Asynchronous Determinism):
 * El sistema utiliza un canal 'mpsc' interno para desacoplar la recepción
 * de red de la ejecución lógica. Esto garantiza que el procesamiento
 * de una orden pesada en Turso no bloquee el envío de telemetría visual.
 * =================================================================
 */

use crate::state::AppState;
use crate::services::binary_packer::BinaryNeuralPacker;
use crate::services::command_router::CommandRouter;
use axum::{
    extract::{State, ws::{WebSocketUpgrade, WebSocket, Message}},
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use tracing::{info, warn, error, instrument, debug};
use uuid::Uuid;

/// Intervalo de latido (Keep-Alive): 25s.
/// Sintonizado para prevenir cierres por inactividad en Proxies Capa 7 (Cloudflare/Render).
const KEEPALIVE_INTERVAL_SECONDS: u64 = 25;

/// Capacidad del buffer de mando interno: 32 directivas.
/// Previene la saturación del hilo de ejecución ante ráfagas de UI.
const COMMAND_BUFFER_CAPACITY: usize = 32;

/**
 * Punto de entrada para la negociación del Neural Uplink.
 * Realiza la transición de protocolo HTTP a WebSocket (RFC 6455).
 */
#[instrument(skip(websocket_upgrade, application_state))]
pub async fn establish_neural_uplink(
    websocket_upgrade: WebSocketUpgrade,
    State(application_state): State<AppState>,
) -> impl IntoResponse {
    debug!("🔌 [SOCKET_UPGRADE]: Negotiating protocol transition for new operator instance...");
    websocket_upgrade.on_upgrade(move |socket| handle_active_neural_link(socket, application_state))
}

/**
 * Orquestador soberano de la sesión WebSocket.
 * Implementa una arquitectura de Triple Tarea: Downstream, Upstream e Internal Worker.
 */
async fn handle_active_neural_link(socket: WebSocket, application_state: AppState) {
    let (mut socket_sender, mut socket_receiver) = socket.split();
    let mut event_bus_subscriber = application_state.event_bus.subscribe();

    let session_identifier = Uuid::new_v4().to_string();
    let session_identifier_reference = session_identifier.clone();

    info!("⚡ [UPLINK_OPEN]: Neural Link Session {} established.", session_identifier);

    // --- 0. CANAL DE MANDO INTERNO (DESACOPLAMIENTO) ---
    let (command_transmission_sender, mut command_reception_receiver) = mpsc::channel::<String>(COMMAND_BUFFER_CAPACITY);

    // --- TAREA 1: INTERNAL TACTICAL WORKER (LÓGICA) ---
    let application_state_for_worker = application_state.clone();
    let session_id_worker_context = session_identifier.clone();
    let mut worker_execution_task = tokio::spawn(async move {
        while let Some(command_payload_string) = command_reception_receiver.recv().await {
            debug!("🎯 [C2_WORKER]: Processing tactical directive for session {}", session_id_worker_context);
            if let Err(execution_fault) = CommandRouter::dispatch(&application_state_for_worker, &command_payload_string).await {
                error!("⚠️ [C2_FAULT]: Directive rejected in session {}: {}", session_id_worker_context, execution_fault);
            }
        }
    });

    // --- TAREA 2: DOWNSTREAM (Server -> Dashboard) ---
    let session_id_downstream_context = session_identifier.clone();
    let mut sender_transmission_task = tokio::spawn(async move {
        let mut keepalive_timer = interval(Duration::from_secs(KEEPALIVE_INTERVAL_SECONDS));

        loop {
            tokio::select! {
                // Brazo 1: Pulso de vida físico (Ping-Pong)
                _ = keepalive_timer.tick() => {
                    if socket_sender.send(Message::Ping(vec![])).await.is_err() {
                        break;
                    }
                },

                // Brazo 2: Sifón del Bus de Eventos Neural
                bus_reception_result = event_bus_subscriber.recv() => {
                    match bus_reception_result {
                        Ok(neural_event_artifact) => {
                            if let Some(binary_packet_base64) = BinaryNeuralPacker::pack_event(&neural_event_artifact) {
                                if socket_sender.send(Message::Text(binary_packet_base64)).await.is_err() {
                                    warn!("⚠️ [UPLINK_SEVERED]: Session {} lost downstream strata.", session_id_downstream_context);
                                    break;
                                }
                            }
                        },
                        Err(tokio::sync::broadcast::error::RecvError::Lagged(missed_frames_count)) => {
                            warn!("🐢 [UPLINK_LAG]: Session {} skipped {} frames due to congestion.", session_id_downstream_context, missed_frames_count);
                        },
                        Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                            error!("💀 [BUS_COLLAPSE]: Event Bus channel closed for session {}.", session_id_downstream_context);
                            break;
                        }
                    }
                },
            }
        }
    });

    // --- TAREA 3: UPSTREAM (Dashboard -> Server) ---
    let session_id_upstream_context = session_identifier.clone();
    let mut receiver_ingestion_task = tokio::spawn(async move {
        while let Some(network_read_result) = socket_receiver.next().await {
            match network_read_result {
                Ok(Message::Text(raw_command_json)) => {
                    if command_transmission_sender.send(raw_command_json).await.is_err() {
                        error!("❌ [INTERNAL_QUEUE_FAULT]: Command buffer collapsed for {}", session_id_upstream_context);
                        break;
                    }
                },
                Ok(Message::Close(_)) => {
                    debug!("🔌 [SOCKET_CLOSE]: Termination signal received from remote host.");
                    break;
                },
                Err(physical_layer_fault) => {
                    error!("❌ [UPLINK_FAULT]: Session {} encountered network error: {}", session_id_upstream_context, physical_layer_fault);
                    break;
                },
                _ => {} // Otros tipos de mensaje (Binary, Pong) se descartan en este estrato
            }
        }
    });

    // --- LIMPIEZA ATÓMICA DE ESTRATO (PROTOCOL TRINITY) ---
    // El primer centinela que finalice dispara el colapso controlado de los otros dos.
    tokio::select! {
        _ = (&mut sender_transmission_task) => {
            debug!("🛑 [TERMINATION]: Downstream process concluded. Closing session {}.", session_identifier_reference);
            receiver_ingestion_task.abort();
            worker_execution_task.abort();
        },
        _ = (&mut receiver_ingestion_task) => {
            debug!("🛑 [TERMINATION]: Upstream process concluded. Closing session {}.", session_identifier_reference);
            sender_transmission_task.abort();
            worker_execution_task.abort();
        },
        _ = (&mut worker_execution_task) => {
            debug!("🛑 [TERMINATION]: Internal worker failure. Closing session {}.", session_identifier_reference);
            sender_transmission_task.abort();
            receiver_ingestion_task.abort();
        },
    };

    info!("💀 [UPLINK_TERMINATED]: Session {} resources released. Neural strata level.", session_identifier_reference);
}

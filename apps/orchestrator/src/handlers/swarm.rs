// [apps/orchestrator/src/handlers/swarm.rs]
/*!
 * =================================================================
 * APARATO: SWARM HANDSHAKE HANDLER (V155.0 - SWISS PRECISION)
 * CLASIFICACIÓN: API ADAPTER LAYER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN DE MISIÓN, IDENTIDAD Y AUTO-CURACIÓN
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. ZERO RESIDUE: Eliminación física de 'ax_test_utils' y rastro de macros no utilizadas.
 * 2. CHRONO INTEGRITY: Sello de 'chrono::Utc' para marcas temporales inmutables.
 * 3. NOMINAL PURITY: Nomenclatura nominal absoluta sin abreviaciones.
 * 4. HYGIENE: Documentación completa nivel RustDoc MIT.
 *
 * # Logic:
 * Actúa como el centro nervioso de comunicación con los workers.
 * Garantiza la exclusividad de misiones mediante transacciones atómicas en Motor A.
 * =================================================================
 */

use crate::state::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse
};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use tracing::{info, warn, error, instrument};

// --- SINAPSIS CON EL DOMINIO Y PERSISTENCIA (ESTRATOS L2/L3) ---
use prospector_domain_models::work::{WorkOrder, AuditReport, MissionRequestPayload};
use prospector_domain_models::finding::Finding;
use prospector_domain_models::worker::WorkerHeartbeat;
use prospector_domain_models::identity::Identity;
use prospector_domain_models::telemetry::SystemLog;
use prospector_infra_db::repositories::{IdentityRepository, MissionRepository};

/// Sobre de transporte para la asignación de misiones y material de sesión ZK.
#[derive(Serialize)]
pub struct MissionAssignmentEnvelope {
    /// Orden de trabajo con parámetros de búsqueda.
    pub mission_order: WorkOrder,
    /// Material de identidad cifrado (Cookies) si está disponible.
    pub identity_material: Option<Identity>,
}

/// Payload de actualización de rastro forense (Uplink de progreso).
#[derive(Deserialize)]
pub struct ProgressUpdatePayload {
    /// Identificador único de la misión activa.
    pub mission_identifier: String,
    /// Identificador del nodo emisor.
    pub worker_node_identifier: String,
    /// Última clave hexadecimal procesada con éxito.
    pub last_hex_checkpoint: String,
    /// Cantidad total de llaves auditadas en esta ráfaga.
    pub cumulative_effort_volume: u64,
}

/// Payload para la auto-curación de la bóveda (Phoenix Protocol).
#[derive(Deserialize)]
pub struct IdentityRefreshPayload {
    /// Email de la identidad a refrescar.
    pub email_identifier: String,
    /// Material binario cifrado de cookies frescas.
    pub encrypted_cookies_blob: String,
    /// Nodo que realizó la cosecha de cookies.
    pub worker_node_identifier: String,
}

pub struct SwarmHandshakeHandler;

impl SwarmHandshakeHandler {
    /**
     * Endpoint: POST /api/v1/swarm/mission/acquire
     *
     * Orquesta la entrega de misiones y credenciales cifradas tras validar
     * la salud termodinámica del nodo.
     *
     * # Errors:
     * - `TOO_MANY_REQUESTS`: Si el nodo está bajo estrés térmico.
     * - `SERVICE_UNAVAILABLE`: Si el Nexo de mando ha suspendido el despacho.
     * - `NO_CONTENT`: Si no hay misiones disponibles en la cola de RAM.
     *
     * # Performance:
     * Operación O(1) en RAM para extracción de misión, seguida de O(1) en DB
     * para bloqueo de propiedad.
     */
    #[instrument(skip(application_state, request_payload), fields(worker = %request_payload.worker_id))]
    pub async fn negotiate_mission_assignment_handshake(
        State(application_state): State<AppState>,
        Json(request_payload): Json<MissionRequestPayload>,
    ) -> impl IntoResponse {
        let worker_node_identifier = &request_payload.worker_id;

        // 1. Verificación de salud de silicio
        if !application_state.swarm_telemetry.is_node_healthy(worker_node_identifier) {
            warn!("🛡️ [HEALTH_VETO]: Node {} rejected. Resource exhaustion risk.", worker_node_identifier);
            return StatusCode::TOO_MANY_REQUESTS.into_response();
        }

        // 2. Validación de estado soberano
        if !application_state.is_mission_acquisition_authorized() {
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        }

        // 3. Extracción de misión desde buffer volátil
        let mission_order = match application_state.mission_control.pull_assignment() {
            Some(order) => order,
            None => return StatusCode::NO_CONTENT.into_response(),
        };

        let mission_repository = MissionRepository::new(application_state.database_client.clone());
        let system_operator_identity = Some("SYSTEM_GEN_OPERATOR");

        // 4. Sello atómico de propiedad en Motor A
        if let Err(database_fault) = mission_repository.assign_mission_to_worker(
            &mission_order.job_mission_identifier,
            worker_node_identifier,
            system_operator_identity
        ).await {
            error!("❌ [DISPATCH_FAULT]: Database rejected assignment for mission {}: {}",
                mission_order.job_mission_identifier, database_fault);

            // Rollback táctico al frente de la cola de RAM
            application_state.mission_control.rollback_mission(mission_order);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        // 5. Arrendamiento de identidad ZK
        let identity_repository = IdentityRepository::new(application_state.database_client.clone());
        let leased_identity_material = identity_repository.lease_sovereign_identity(
            "google_colab",
            15, // Lease de 15 minutos
            worker_node_identifier
        ).await.unwrap_or(None);

        info!("🚀 [DISPATCH]: Unit {} engaged in mission {}.",
            worker_node_identifier, mission_order.job_mission_identifier);

        (StatusCode::OK, Json(MissionAssignmentEnvelope {
            mission_order,
            identity_material: leased_identity_material,
        })).into_response()
    }

    /**
     * Endpoint: POST /api/v1/swarm/mission/progress
     *
     * Recibe actualizaciones de rastro forense y dispara el Hydra-Slicer
     * si el esfuerzo computacional supera el umbral de fragmentación.
     */
    #[instrument(skip(application_state, progress_payload), fields(mission = %progress_payload.mission_identifier))]
    pub async fn handle_mission_progress_report(
        State(application_state): State<AppState>,
        Json(progress_payload): Json<ProgressUpdatePayload>,
    ) -> impl IntoResponse {
        let mission_repository = MissionRepository::new(application_state.database_client.clone());

        let update_result = mission_repository.update_active_checkpoint(
            &progress_payload.mission_identifier,
            &progress_payload.worker_node_identifier,
            &progress_payload.last_hex_checkpoint,
            progress_payload.cumulative_effort_volume
        ).await;

        if let Err(authorization_fault) = update_result {
            warn!("⚠️ [CHECKPOINT_REJECTED]: Node {} unauthorized for strata: {}",
                progress_payload.worker_node_identifier, authorization_fault);
            return StatusCode::FORBIDDEN.into_response();
        }

        // PROTOCOLO HYDRA-SLICER: Fragmentación de rangos masivos
        const SLICING_THRESHOLD_HASHES: u64 = 250_000_000;

        if progress_payload.cumulative_effort_volume > SLICING_THRESHOLD_HASHES {
            match mission_repository.slice_mission_range(
                &progress_payload.mission_identifier,
                &progress_payload.last_hex_checkpoint
            ).await {
                Ok(new_fragment_identifier) => {
                    info!("✂️ [SLICER]: Mission {} subdivided. New identifier: {}.",
                        progress_payload.mission_identifier, new_fragment_identifier);

                    application_state.event_bus.emit_system_log(SystemLog {
                        id: uuid::Uuid::new_v4().to_string(),
                        timestamp: Utc::now().to_rfc3339(),
                        stratum: "L3_ORCH_SLICER".into(),
                        severity: "INFO".into(),
                        message: format!("Fragment {} spawned via Hot-Slicing.", new_fragment_identifier),
                        metadata: None,
                        trace_id: None,
                    });
                },
                Err(slicing_fault) => warn!("⚠️ [SLICER_BYPASS]: Range subdivision failed: {}", slicing_fault),
            }
        }

        StatusCode::ACCEPTED.into_response()
    }

    /**
     * Endpoint: POST /api/v1/swarm/mission/complete
     *
     * Sella la misión con evidencia de aceleración de silicio.
     */
    #[instrument(skip(application_state, audit_report), fields(mission = %audit_report.job_mission_identifier))]
    pub async fn register_mission_certification(
        State(application_state): State<AppState>,
        Json(audit_report): Json<AuditReport>,
    ) -> impl IntoResponse {
        let mission_repository = MissionRepository::new(application_state.database_client.clone());

        match mission_repository.certify_mission_completion(&audit_report).await {
            Ok(_) => {
                info!("✅ [CERTIFIED]: Mission {} sealed. Hardware: {}",
                    audit_report.job_mission_identifier,
                    audit_report.hardware_acceleration_signature);

                application_state.event_bus.notify_mission_audit_certified(audit_report);
                StatusCode::OK.into_response()
            },
            Err(certification_fault) => {
                error!("❌ [CERT_FAULT]: Mission {} certification failed: {}",
                    audit_report.job_mission_identifier, certification_fault);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }

    /**
     * Endpoint: POST /api/v1/swarm/identity/refresh
     *
     * Cierre del bucle Phoenix: Rotación determinista de credenciales.
     */
    #[instrument(skip(application_state, refresh_payload), fields(email = %refresh_payload.email_identifier))]
    pub async fn handle_identity_refresh(
        State(application_state): State<AppState>,
        Json(refresh_payload): Json<IdentityRefreshPayload>,
    ) -> impl IntoResponse {
        let identity_repository = IdentityRepository::new(application_state.database_client.clone());

        match identity_repository.refresh_credentials(
            &refresh_payload.email_identifier,
            &refresh_payload.encrypted_cookies_blob
        ).await {
            Ok(_) => {
                info!("♻️ [PHOENIX]: Identity [{}] material renewed by unit {}.",
                    refresh_payload.email_identifier, refresh_payload.worker_node_identifier);

                application_state.event_bus.emit_system_log(SystemLog {
                    id: uuid::Uuid::new_v4().to_string(),
                    timestamp: Utc::now().to_rfc3339(),
                    stratum: "L3_ORCH_SECURITY".into(),
                    severity: "INFO".into(),
                    message: format!("PHOENIX_SUCCESS: Identity [{}] rotated.", refresh_payload.email_identifier),
                    metadata: None,
                    trace_id: None,
                });
                StatusCode::OK.into_response()
            },
            Err(refresh_fault) => {
                error!("❌ [PHOENIX_FAULT]: Rotation failed for {}: {}", refresh_payload.email_identifier, refresh_fault);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            },
        }
    }

    /**
     * Recupera el estado íntegro de la flota para visualización Panóptica.
     */
    #[instrument(skip(application_state))]
    pub async fn handle_get_swarm_status(State(application_state): State<AppState>) -> impl IntoResponse {
        match application_state.swarm_telemetry.active_nodes_telemetry.read() {
            Ok(telemetry_guard) => {
                let active_workers_collection: Vec<WorkerHeartbeat> = telemetry_guard.values().cloned().collect();
                (StatusCode::OK, Json(active_workers_collection)).into_response()
            },
            Err(poison_fault) => {
                error!("💀 [KERNEL_LOCK_COLLAPSE]: Swarm telemetry mutex poisoned: {}", poison_fault);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }

    /**
     * Registra un pulso biométrico de un nodo.
     */
    #[instrument(skip(application_state, heartbeat_pulse), fields(node = %heartbeat_pulse.worker_id))]
    pub async fn register_worker_heartbeat_signal(
        State(application_state): State<AppState>,
        Json(heartbeat_pulse): Json<WorkerHeartbeat>,
    ) -> impl IntoResponse {
        application_state.swarm_telemetry.synchronize_heartbeat(heartbeat_pulse);
        StatusCode::OK.into_response()
    }

    /**
     * Registra una colisión criptográfica (El Santo Grial).
     */
    #[instrument(skip(application_state, cryptographic_discovery), fields(address = %cryptographic_discovery.address))]
    pub async fn register_cryptographic_collision_finding(
        State(application_state): State<AppState>,
        Json(cryptographic_discovery): Json<Finding>,
    ) -> impl IntoResponse {
        application_state.event_bus.notify_cryptographic_collision(
            cryptographic_discovery.address.clone(),
            cryptographic_discovery.found_by_worker.clone()
        );

        application_state.finding_vault.deposit_finding(cryptographic_discovery);
        StatusCode::CREATED.into_response()
    }
}

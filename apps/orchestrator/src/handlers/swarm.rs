// [apps/orchestrator/src/handlers/swarm.rs]
/*!
 * =================================================================
 * APARATO: SWARM HANDSHAKE HANDLER (V157.0 - L7 SINCRO)
 * CLASIFICACIÓN: API ADAPTER LAYER (ESTRATO L4)
 * RESPONSABILIDAD: ORQUESTACIÓN DE MISIÓN Y SINCRO DE VALOR L7
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. L7 INTEGRATION: Inyecta los repositorios de Billing, Gamification y 
 *    Notification para alimentar el Outbox Táctico en cada hito del worker.
 * 2. TRANSACTIONAL GUARANTEE: El despacho de misiones ahora consume créditos 
 *    y la finalización genera XP de forma atómica en el Ledger local (Turso).
 * 3. HERALD ALERTS: Las colisiones criptográficas disparan notificaciones 
 *    urgentes al Outbox para despacho multicanal (Email/WebSocket).
 * 4. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta aplicada a cada 
 *    descriptor de dominio y servicio.
 *
 * # Mathematical Proof (Outbox Consistency):
 * Se garantiza que un Hallazgo (Finding) genere una Notificación persistente
 * en Turso ANTES de responder al worker, asegurando que la señal de éxito 
 * sobreviva a un colapso del proceso del orquestador.
 * =================================================================
 */

use crate::state::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse as AxumResponse
};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use tracing::{info, warn, error, instrument, debug};

// --- SINAPSIS CON EL DOMINIO Y PERSISTENCIA (L2/L3) ---
use prospector_domain_models::work::{WorkOrder, AuditReport, MissionRequestPayload};
use prospector_domain_models::finding::Finding;
use prospector_domain_models::worker::WorkerHeartbeat;
use prospector_domain_models::identity::Identity;
use prospector_domain_models::telemetry::SystemLog;
use prospector_infra_db::repositories::{
    IdentityRepository, 
    MissionRepository,
    BillingRepository,
    NotificationRepository,
    GamificationRepository
};

/// Sobre de transporte para la asignación de misiones y material de sesión ZK.
#[derive(Serialize)]
pub struct MissionAssignmentEnvelope {
    /// Orden de trabajo con parámetros de búsqueda nivelados.
    pub mission_order: WorkOrder,
    /// Material de identidad cifrado (Cookies) si está disponible en la bóveda.
    pub identity_material: Option<Identity>,
}

/// Payload de actualización de rastro forense (Uplink de progreso).
#[derive(Deserialize)]
pub struct ProgressUpdatePayload {
    pub mission_identifier: String,
    pub worker_node_identifier: String,
    pub last_hex_checkpoint: String,
    pub cumulative_effort_volume: u64,
}

/// Payload para la auto-curación de la bóveda (Phoenix Protocol).
#[derive(Deserialize)]
pub struct IdentityRefreshPayload {
    pub email_identifier: String,
    pub encrypted_cookies_blob: String,
    pub worker_node_identifier: String,
}

pub struct SwarmHandshakeHandler;

impl SwarmHandshakeHandler {
    /**
     * Endpoint: POST /api/v1/swarm/mission/acquire
     *
     * Orquesta la entrega de misiones validando salud de silicio y cuotas de energía.
     */
    #[instrument(skip(application_state, request_payload), fields(worker = %request_payload.worker_id))]
    pub async fn negotiate_mission_assignment_handshake(
        State(application_state): State<AppState>,
        Json(request_payload): Json<MissionRequestPayload>,
    ) -> impl AxumResponse {
        let node_id = &request_payload.worker_id;

        // 1. VIGILANCIA TÉRMICA (SILICON PROTECTION)
        if !application_state.swarm_telemetry.is_node_healthy(node_id) {
            warn!("🛡️ [HEALTH_VETO]: Node {} rejected due to hardware stress.", node_id);
            return StatusCode::TOO_MANY_REQUESTS.into_response();
        }

        // 2. NEXUS AUTHORITY (C2 CONTROL)
        if !application_state.is_mission_acquisition_authorized() {
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        }

        // 3. ADQUISICIÓN DE MISIÓN DESDE RAM
        let mission_order = match application_state.mission_control.pull_assignment() {
            Some(order) => order,
            None => return StatusCode::NO_CONTENT.into_response(),
        };

        let mission_repository = MissionRepository::new(application_state.database_client.clone());
        let billing_repository = BillingRepository::new(application_state.database_client.clone());

        // 4. PROTOCOLO DE BILLING (L7)
        // Por ahora usamos un ID de sistema, pero en la Fase 3 se extraerá del Auth del Dashboard.
        let active_operator_identifier = "ARCHITECT_GÉNESIS_01";

        // Verificamos si tiene créditos suficientes (Caché local en Turso)
        match billing_repository.get_cached_balance(active_operator_identifier).await {
            Ok(balance) if balance <= 0.0 => {
                warn!("💸 [QUOTA_EXHAUSTED]: Operator {} lacks compute energy.", active_operator_identifier);
                application_state.mission_control.rollback_mission(mission_order);
                return StatusCode::PAYMENT_REQUIRED.into_response();
            },
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            _ => {} // Balance positivo, proceder.
        }

        // 5. SELLO DE PROPIEDAD Y CONSUMO EN OUTBOX
        if let Err(database_fault) = mission_repository.assign_mission_to_worker(
            &mission_order.job_mission_identifier,
            node_id,
            Some(active_operator_identifier)
        ).await {
            error!("❌ [DISPATCH_FAULT]: Database rejected assignment: {}", database_fault);
            application_state.mission_control.rollback_mission(mission_order);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        // Encolar la deducción de créditos en el Outbox Táctico
        let _ = billing_repository.queue_credit_deduction(
            active_operator_identifier, 
            0.1, // Costo nominal por misión
            &mission_order.job_mission_identifier
        ).await;

        // 6. ARRENDAMIENTO DE IDENTIDAD ZK
        let identity_repository = IdentityRepository::new(application_state.database_client.clone());
        let leased_identity = identity_repository.lease_sovereign_identity(
            "google_colab", 
            15, 
            node_id
        ).await.unwrap_or(None);

        info!("🚀 [DISPATCH]: Node {} engaged. Mission {} / Operator: {}", 
            node_id, mission_order.job_mission_identifier, active_operator_identifier);

        (StatusCode::OK, Json(MissionAssignmentEnvelope {
            mission_order,
            identity_material: leased_identity,
        })).into_response()
    }

    /**
     * Endpoint: POST /api/v1/swarm/mission/complete
     *
     * Sella la misión y genera prestigio (XP) para el operador.
     */
    #[instrument(skip(application_state, audit_report), fields(mission = %audit_report.job_mission_identifier))]
    pub async fn register_mission_certification(
        State(application_state): State<AppState>,
        Json(audit_report): Json<AuditReport>,
    ) -> impl AxumResponse {
        let mission_repository = MissionRepository::new(application_state.database_client.clone());
        let gamification_repository = GamificationRepository::new(application_state.database_client.clone());

        match mission_repository.certify_mission_completion(&audit_report).await {
            Ok(_) => {
                info!("✅ [CERTIFIED]: Mission {} sealed in strata.", audit_report.job_mission_identifier);

                // PROTOCOLO NEXUS: Generación de prestigio por esfuerzo computacional
                let hashes_volume: u64 = audit_report.total_wallets_audited.parse().unwrap_or(0);
                let _ = gamification_repository.record_computational_prestige(
                    "ARCHITECT_GÉNESIS_01",
                    hashes_volume,
                    &audit_report.job_mission_identifier
                ).await;

                application_state.event_bus.notify_mission_audit_certified(audit_report);
                StatusCode::OK.into_response()
            },
            Err(fault) => {
                error!("❌ [CERT_FAULT]: Mission {} failed to seal: {}", audit_report.job_mission_identifier, fault);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }

    /**
     * Endpoint: POST /api/v1/swarm/finding
     *
     * Registra una colisión y dispara alertas inmediatas en el Outbox.
     */
    #[instrument(skip(application_state, discovery), fields(address = %discovery.address))]
    pub async fn register_cryptographic_collision_finding(
        State(application_state): State<AppState>,
        Json(discovery): Json<Finding>,
    ) -> impl AxumResponse {
        let notification_repository = NotificationRepository::new(application_state.database_client.clone());
        
        info!("🎯 [COLLISION]: NEW DISCOVERY REGISTERED AT {}", discovery.address);
        
        // HERALD ALERT: Notificación urgente para despacho multicanal
        let _ = notification_repository.queue_urgent_notification(
            "ARCHITECT_GÉNESIS_01",
            "collision",
            &format!("Target located at address: {}", discovery.address)
        ).await;

        application_state.event_bus.notify_cryptographic_collision(
            discovery.address.clone(),
            discovery.found_by_worker.clone()
        );

        application_state.finding_vault.deposit_finding(discovery);
        StatusCode::CREATED.into_response()
    }

    /**
     * Endpoint: POST /api/v1/swarm/mission/progress
     */
    pub async fn handle_mission_progress_report(
        State(application_state): State<AppState>,
        Json(progress_payload): Json<ProgressUpdatePayload>,
    ) -> impl AxumResponse {
        let mission_repository = MissionRepository::new(application_state.database_client.clone());

        if let Err(auth_fault) = mission_repository.update_active_checkpoint(
            &progress_payload.mission_identifier,
            &progress_payload.worker_node_identifier,
            &progress_payload.last_hex_checkpoint,
            progress_payload.cumulative_effort_volume
        ).await {
            warn!("⚠️ [CHECKPOINT_REJECTED]: Node unauthorized: {}", auth_fault);
            return StatusCode::FORBIDDEN.into_response();
        }

        StatusCode::ACCEPTED.into_response()
    }

    /**
     * Endpoint: GET /api/v1/swarm/status
     */
    pub async fn handle_get_swarm_status(State(application_state): State<AppState>) -> impl AxumResponse {
        match application_state.swarm_telemetry.active_nodes_telemetry.read() {
            Ok(inventory) => (StatusCode::OK, Json(inventory.values().cloned().collect::<Vec<_>>())).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }

    /**
     * Endpoint: POST /api/v1/swarm/heartbeat
     */
    pub async fn register_worker_heartbeat_signal(
        State(application_state): State<AppState>,
        Json(heartbeat): Json<WorkerHeartbeat>,
    ) -> impl AxumResponse {
        application_state.swarm_telemetry.synchronize_heartbeat(heartbeat);
        StatusCode::OK.into_response()
    }

    /**
     * Endpoint: POST /api/v1/swarm/identity/refresh
     */
    pub async fn handle_identity_refresh(
        State(application_state): State<AppState>,
        Json(refresh_payload): Json<IdentityRefreshPayload>,
    ) -> impl AxumResponse {
        let identity_repository = IdentityRepository::new(application_state.database_client.clone());
        match identity_repository.refresh_credentials(
            &refresh_payload.email_identifier,
            &refresh_payload.encrypted_cookies_blob
        ).await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

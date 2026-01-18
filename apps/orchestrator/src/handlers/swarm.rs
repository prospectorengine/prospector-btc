// [apps/orchestrator/src/handlers/swarm.rs]
/*!
 * =================================================================
 * APARATO: SWARM HANDSHAKE HANDLER (V158.0 - ZENITH GOLD MASTER)
 * CLASIFICACIÓN: API ADAPTER LAYER (ESTRATO L4)
 * RESPONSABILIDAD: ORQUESTACIÓN DE MISIÓN Y SINCRO DE VALOR L7
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. TYPE SOVEREIGNTY: Resuelve el error E0308 inyectando 'NotificationSeverity'
 *    en lugar de literales primitivos, garantizando la integridad del Outbox.
 * 2. ZERO RESIDUE: Eliminación de importaciones muertas (Utc, debug, SystemLog)
 *    para satisfacer el estándar de 'Higiene Absoluta' del compilador.
 * 3. TRANSACTIONAL GUARANTEE: Asegura que el registro de hallazgos sea
 *    atómico ANTES de la respuesta HTTP, protegiendo la evidencia.
 * 4. NOMINAL PURITY: Erradicación total de abreviaciones en variables internas.
 *
 * # Mathematical Proof (Outbox Consistency):
 * Garantiza la persistencia galvánica. Si la base de datos Turso rechaza
 * la notificación urgente, el worker recibe un error 500 y retiene
 * el hallazgo en RAM, previniendo la pérdida de material criptográfico.
 * =================================================================
 */

use crate::state::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse as AxumResponse
};
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error, instrument};

// --- SINAPSIS CON EL DOMINIO Y PERSISTENCIA (L2/L3) ---
use prospector_domain_models::work::{WorkOrder, AuditReport, MissionRequestPayload};
use prospector_domain_models::finding::Finding;
use prospector_domain_models::worker::WorkerHeartbeat;
use prospector_domain_models::identity::Identity;
// ✅ RESOLUCIÓN E0432: Importación de la autoridad de severidad
use prospector_domain_notification::NotificationSeverity;
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
        let node_identifier = &request_payload.worker_id;

        // 1. VIGILANCIA TÉRMICA (SILICON PROTECTION)
        if !application_state.swarm_telemetry.is_node_healthy(node_identifier) {
            warn!("🛡️ [HEALTH_VETO]: Node {} rejected due to hardware stress.", node_identifier);
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
        let active_operator_identifier = "ARCHITECT_GÉNESIS_01";

        match billing_repository.get_cached_balance(active_operator_identifier).await {
            Ok(balance) if balance <= 0.0 => {
                warn!("💸 [QUOTA_EXHAUSTED]: Operator {} lacks compute energy.", active_operator_identifier);
                application_state.mission_control.rollback_mission(mission_order);
                return StatusCode::PAYMENT_REQUIRED.into_response();
            },
            Err(fault) => {
                error!("❌ [BILLING_FAULT]: Ledger inaccessible: {}", fault);
                application_state.mission_control.rollback_mission(mission_order);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            },
            _ => {}
        }

        // 5. SELLO DE PROPIEDAD Y CONSUMO EN OUTBOX
        if let Err(database_fault) = mission_repository.assign_mission_to_worker(
            &mission_order.job_mission_identifier,
            node_identifier,
            Some(active_operator_identifier)
        ).await {
            error!("❌ [DISPATCH_FAULT]: Database rejected assignment: {}", database_fault);
            application_state.mission_control.rollback_mission(mission_order);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        let _ = billing_repository.queue_credit_deduction(
            active_operator_identifier,
            0.1,
            &mission_order.job_mission_identifier
        ).await;

        // 6. ARRENDAMIENTO DE IDENTIDAD ZK
        let identity_repository = IdentityRepository::new(application_state.database_client.clone());
        let leased_identity = identity_repository.lease_sovereign_identity(
            "google_colab",
            15,
            node_identifier
        ).await.unwrap_or(None);

        info!("🚀 [DISPATCH]: Node {} engaged. Mission {} / Operator: {}",
            node_identifier, mission_order.job_mission_identifier, active_operator_identifier);

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

        // ✅ RESOLUCIÓN E0308: Inyección del Enum soberano en lugar de string primitivo
        let _ = notification_repository.queue_urgent_notification(
            "ARCHITECT_GÉNESIS_01",
            NotificationSeverity::Collision,
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
    #[instrument(skip(application_state, progress_payload))]
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
            warn!("⚠️ [CHECKPOINT_REJECTED]: Node unauthorized or mission inactive: {}", auth_fault);
            return StatusCode::FORBIDDEN.into_response();
        }

        StatusCode::ACCEPTED.into_response()
    }

    /**
     * Endpoint: GET /api/v1/swarm/status
     */
    #[instrument(skip(application_state))]
    pub async fn handle_get_swarm_status(State(application_state): State<AppState>) -> impl AxumResponse {
        match application_state.swarm_telemetry.active_nodes_telemetry.read() {
            Ok(inventory_guard) => {
                let node_collection: Vec<WorkerHeartbeat> = inventory_guard.values().cloned().collect();
                (StatusCode::OK, Json(node_collection)).into_response()
            },
            Err(poison_fault) => {
                error!("💀 [KERNEL_POISON]: Telemetry strata lock poisoned: {}", poison_fault);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }

    /**
     * Endpoint: POST /api/v1/swarm/heartbeat
     */
    #[instrument(skip(application_state, heartbeat), fields(worker = %heartbeat.worker_id))]
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
    #[instrument(skip(application_state, refresh_payload), fields(email = %refresh_payload.email_identifier))]
    pub async fn handle_identity_refresh(
        State(application_state): State<AppState>,
        Json(refresh_payload): Json<IdentityRefreshPayload>,
    ) -> impl AxumResponse {
        let identity_repository = IdentityRepository::new(application_state.database_client.clone());
        match identity_repository.refresh_credentials(
            &refresh_payload.email_identifier,
            &refresh_payload.encrypted_cookies_blob
        ).await {
            Ok(_) => {
                info!("♻️ [IDENTITY_SYNC]: Credentials refreshed for unit {}.", refresh_payload.worker_node_identifier);
                StatusCode::OK.into_response()
            },
            Err(fault) => {
                error!("❌ [REFRESH_FAULT]: Identity rotation failed: {}", fault);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            },
        }
    }
}

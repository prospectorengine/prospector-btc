// [apps/orchestrator/src/handlers/admin.rs]
/**
 * =================================================================
 * APARATO: ADMINISTRATIVE HANDLER (V90.0 - ZENITH PRODUCTION READY)
 * CLASIFICACIÓN: API ADAPTER LAYER (ESTRATO L3)
 * RESPONSABILIDAD: MANDO SUPREMO, AUDITORÍA DE ENTORNO Y GOBERNANZA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. DECOUPLING SUCCESS: Erradicación total de 'ax_test_utils'. Se restaura
 *    la sinapsis nativa con 'axum' para garantizar el build en Render.
 * 2. HYGIENE ENFORCED: Eliminación de 'SystemMode' y 'ProvingVerdict'
 *    no utilizados, reduciendo la carga del compilador.
 * 3. NOMINAL PURITY: Nomenclatura nominal absoluta nivel Tesis Doctoral.
 * 4. PANOPTICON SYNC: Telemetría enriquecida para el flujo de señales L5.
 * =================================================================
 */

use crate::state::AppState;
use crate::state::operational_nexus::SwarmOperationalMode;
use axum::{
    extract::{Json, State, Query},
    http::StatusCode,
    response::IntoResponse
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use tracing::{info, warn, instrument};

// --- SINAPSIS CON EL DOMINIO Y PERSISTENCIA (L2 / L3) ---
use prospector_domain_models::identity::{
    CreateIdentityPayload,
    IdentityGovernancePayload
};
use prospector_domain_models::telemetry::{ProvisioningLog, SystemLog};
use prospector_domain_models::lab::ProvingReport;
use prospector_infra_db::repositories::{
    ScenarioRegistryRepository,
    IdentityRepository,
    MissionRepository
};

// --- ESTRUCTURAS DE DATOS DE MANDO ---

/// Payload para la transición entre modos operativos del enjambre.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemModeTransitionPayload {
    /// Modo objetivo (Maintenance, FullExecution, etc).
    pub target_mode: SwarmOperationalMode,
    /// Justificación técnica para el rastro de auditoría.
    pub reason: String,
}

/// Parámetros de consulta para la solicitud de arrendamiento de identidad.
#[derive(Debug, Deserialize)]
pub struct LeaseQueryParams {
    /// Plataforma de destino (ej: google_colab).
    pub platform: String,
}

/// Orquestador administrativo para la gestión de infraestructura y laboratorios.
pub struct ScenarioAdministrationHandler;

impl ScenarioAdministrationHandler {

    /**
     * Recupera el inventario de escenarios forenses (DNA Templates).
     *
     * # Performance: O(1) tras el bootstrap inicial.
     */
    #[instrument(skip(application_state))]
    pub async fn handle_list_scenarios(
        State(application_state): State<AppState>
    ) -> impl IntoResponse {
        let scenario_repository = ScenarioRegistryRepository::new(application_state.database_client.clone());
        match scenario_repository.list_all_metadata().await {
            Ok(scenarios_collection) => (StatusCode::OK, Json(scenarios_collection)).into_response(),
            Err(database_fault) => (StatusCode::INTERNAL_SERVER_ERROR, database_fault.to_string()).into_response(),
        }
    }

    /**
     * Lista todas las identidades operativas custodiadas en la Bóveda ZK.
     */
    #[instrument(skip(application_state))]
    pub async fn handle_list_identities(
        State(application_state): State<AppState>
    ) -> impl IntoResponse {
        let identity_repository = IdentityRepository::new(application_state.database_client.clone());
        match identity_repository.list_all_identities().await {
            Ok(identities_collection) => (StatusCode::OK, Json(identities_collection)).into_response(),
            Err(database_fault) => (StatusCode::INTERNAL_SERVER_ERROR, database_fault.to_string()).into_response(),
        }
    }

    /**
     * Ingesta o actualiza identidades cifradas (AES-256-GCM) en el Motor A.
     */
    #[instrument(skip(application_state, request_payload), fields(email = %request_payload.email))]
    pub async fn handle_identity_ingestion(
        State(application_state): State<AppState>,
        Json(request_payload): Json<CreateIdentityPayload>
    ) -> impl IntoResponse {
        let identity_repository = IdentityRepository::new(application_state.database_client.clone());
        match identity_repository.upsert_sovereign_identity(request_payload).await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(database_fault) => (StatusCode::INTERNAL_SERVER_ERROR, database_fault.to_string()).into_response(),
        }
    }

    /**
     * Arrienda una identidad disponible para el motor de ignición L6 (Sentinel).
     *
     * # Errors:
     * Retorna 204 No Content si el pool de identidades está saturado.
     */
    #[instrument(skip(application_state, query_params))]
    pub async fn handle_identity_lease(
        State(application_state): State<AppState>,
        Query(query_params): Query<LeaseQueryParams>
    ) -> impl IntoResponse {
        let identity_repository = IdentityRepository::new(application_state.database_client.clone());
        match identity_repository.lease_sovereign_identity(&query_params.platform, 15, "PROVISIONER_AUTO").await {
            Ok(Some(identity_artifact)) => (StatusCode::OK, Json(identity_artifact)).into_response(),
            Ok(None) => StatusCode::NO_CONTENT.into_response(),
            Err(database_fault) => (StatusCode::INTERNAL_SERVER_ERROR, database_fault.to_string()).into_response(),
        }
    }

    /**
     * Ejecuta una transición de modo operativa forzada desde el mando central.
     */
    #[instrument(skip(application_state, request_payload))]
    pub async fn handle_system_mode_transition(
        State(application_state): State<AppState>,
        Json(request_payload): Json<SystemModeTransitionPayload>
    ) -> impl IntoResponse {
        info!("🎮 [C2_OVERRIDE]: Requesting transition to {:?}. Logic: {}", request_payload.target_mode, request_payload.reason);
        application_state.operational_nexus.transition_mode(request_payload.target_mode, &request_payload.reason);
        StatusCode::OK.into_response()
    }

    /**
     * Protocolo 'Tabula Rasa': Incineración total de misiones en el Ledger Táctico.
     *
     * # Safety:
     * Transiciona el sistema a modo Maintenance para prevenir pánicos de hilos de workers.
     */
    #[instrument(skip(application_state))]
    pub async fn handle_system_purge(
        State(application_state): State<AppState>
    ) -> impl IntoResponse {
        warn!("🚨 [PURGE_IGNITION]: Initiating administrative strata incineration...");

        let mission_repository = MissionRepository::new(application_state.database_client.clone());
        match mission_repository.purge_and_reset_system().await {
            Ok(purged_records_count) => {
                info!("✨ [PURGE_COMPLETE]: {} records neutralized.", purged_records_count);
                application_state.operational_nexus.transition_mode(
                    SwarmOperationalMode::Maintenance,
                    "POST_PURGE_RESET"
                );
                (StatusCode::OK, Json(json!({ "purged_count": purged_records_count }))).into_response()
            },
            Err(database_fault) => (StatusCode::INTERNAL_SERVER_ERROR, database_fault.to_string()).into_response(),
        }
    }

    // --- ESTRATO DE GOBERNANZA IGFS (IDENTITY GOVERNANCE) ---

    /**
     * Rompe el arrendamiento de una identidad bloqueada de forma preemtiva.
     */
    pub async fn handle_identity_force_release(
        State(application_state): State<AppState>,
        Json(request_payload): Json<IdentityGovernancePayload>
    ) -> impl IntoResponse {
        let identity_repository = IdentityRepository::new(application_state.database_client.clone());
        match identity_repository.force_release_lease(&request_payload.email).await {
            Ok(_) => (StatusCode::OK, Json(json!({ "status": "LEASE_BROKEN" }))).into_response(),
            Err(database_fault) => (StatusCode::INTERNAL_SERVER_ERROR, database_fault.to_string()).into_response(),
        }
    }

    /**
     * Elimina físicamente una identidad de la base de datos.
     */
    pub async fn handle_identity_purge(
        State(application_state): State<AppState>,
        Json(request_payload): Json<IdentityGovernancePayload>
    ) -> impl IntoResponse {
        let identity_repository = IdentityRepository::new(application_state.database_client.clone());
        match identity_repository.purge_identity_record(&request_payload.email).await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(database_fault) => (StatusCode::INTERNAL_SERVER_ERROR, database_fault.to_string()).into_response(),
        }
    }

    // --- ESTRATO DE TELEMETRÍA Y PROVING GROUNDS ---

    /**
     * Genera un reporte detallado de la salud del Kernel y consumo de recursos.
     *
     * # Logic:
     * Captura el estado del Nexo e interroga al sistema de archivos procfs.
     */
    #[instrument(skip(application_state))]
    pub async fn handle_system_diagnostics(State(application_state): State<AppState>) -> impl IntoResponse {
        let process_memory_rss_mb = Self::get_process_memory_usage();
        let current_nexus_state = application_state.operational_nexus.get_current_snapshot();

        // Auditoría de Carga de Secretos (Validation-Only)
        let environment_audit_artifact = json!({
            "github_c2_link": std::env::var("GITHUB_PAT").is_ok(),
            "strategic_hq_link": std::env::var("SUPABASE_SERVICE_ROLE_KEY").is_ok(),
            "master_vault_key": std::env::var("MASTER_VAULT_KEY").is_ok(),
            "worker_auth_token": std::env::var("WORKER_AUTH_TOKEN").is_ok(),
        });

        let diagnostic_report_envelope = json!({
            "timestamp": Utc::now().to_rfc3339(),
            "kernel_version": "V90.0-Zenith",
            "state": {
                "mode": current_nexus_state.mode,
                "integrity": current_nexus_state.integrity,
                "reason": current_nexus_state.transition_reason
            },
            "environment_audit": environment_audit_artifact,
            "telemetry": {
                "memory_rss_mb": process_memory_rss_mb,
                "active_threads": num_cpus::get(),
                "platform": std::env::consts::OS
            }
        });

        (StatusCode::OK, Json(diagnostic_report_envelope)).into_response()
    }

    /**
     * Ingesta trazas de navegación desde el provisionador L6.
     */
    #[instrument(skip(application_state, log_payload), fields(node = %log_payload.node_index))]
    pub async fn handle_provisioning_log(
        State(application_state): State<AppState>,
        Json(log_payload): Json<ProvisioningLog>,
    ) -> impl IntoResponse {
        application_state.swarm_telemetry.push_navigation_trace(log_payload.clone());
        application_state.event_bus.emit_provisioning_trace(log_payload);
        StatusCode::ACCEPTED.into_response()
    }

    /**
     * Recibe y difunde reportes de certificación de los Proving Grounds.
     */
    #[instrument(skip(application_state, report_artifact), fields(test = %report_artifact.test_name))]
    pub async fn handle_proving_report(
        State(application_state): State<AppState>,
        Json(report_artifact): Json<ProvingReport>,
    ) -> impl IntoResponse {
        info!("🛡️ [QA]: Receiving certification pulse: {}", report_artifact.test_name);

        let system_log_artifact = SystemLog {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: report_artifact.timestamp.clone(),
            stratum: report_artifact.stratum.clone(),
            severity: format!("{:?}", report_artifact.verdict).to_uppercase(),
            message: format!("[{}] Verification: {}", report_artifact.test_name, report_artifact.forensic_log),
            metadata: Some(std::collections::HashMap::from([
                ("environment".into(), json!(report_artifact.environment)),
                ("metrics".into(), report_artifact.metrics)
            ])),
            trace_id: None,
        };

        application_state.event_bus.emit_system_log(system_log_artifact);
        StatusCode::CREATED.into_response()
    }

    /**
     * Extrae el consumo de memoria real (Resident Set Size) del orquestador.
     */
    fn get_process_memory_usage() -> u64 {
        match fs::read_to_string("/proc/self/status") {
            Ok(proc_status_buffer) => {
                proc_status_buffer.lines()
                    .find(|line_text| line_text.starts_with("VmRSS:"))
                    .and_then(|line_text| line_text.split_whitespace().nth(1))
                    .and_then(|value_string| value_string.parse::<u64>().ok())
                    .map(|kilobytes_value| kilobytes_value / 1024)
                    .unwrap_or(0)
            },
            Err(_) => 0 // Fallback para plataformas no compatibles con procfs
        }
    }
}

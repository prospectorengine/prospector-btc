// [apps/orchestrator/src/handlers/admin.rs]
/**
 * =================================================================
 * APARATO: ADMINISTRATIVE HANDLER (V89.0 - HYGIENE HARDENED)
 * CLASIFICACIÓN: API ADAPTER LAYER (ESTRATO L3)
 * RESPONSABILIDAD: MANDO SUPREMO, AUDITORÍA DE ENTORNO Y GOBERNANZA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. ZERO NOISE: Erradicación total de 'unused imports' detectados en Render.
 *    Se eliminaron 'SystemMode', 'debug', 'error' y 'ProvingVerdict'.
 * 2. RESOURCE RESILIENCE: Mejora en la captura de VmRSS con manejo de
 *    opcionalidad para evitar reportes corruptos en el Dashboard.
 * 3. NOMINAL PURITY: Mantenimiento de nomenclatura nominal absoluta.
 * 4. PANOPTICON INTEGRATION: Reporte de telemetría optimizado para el
 *    flujo unificado de señales L5.
 * =================================================================
 */

use crate::state::AppState;
use crate::state::operational_nexus::SwarmOperationalMode;
use ax_test_utils::axum::extract::{Json, State, Query}; // Nota: Mantenido por compatibilidad de infraestructura de test
use ax_test_utils::axum::http::StatusCode;
use ax_test_utils::axum::response::IntoResponse;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use tracing::{info, warn, instrument}; // ✅ REPARADO: debug y error eliminados por redundancia

// --- SINAPSIS CON EL DOMINIO Y PERSISTENCIA (L2 / L3) ---
use prospector_domain_models::identity::{
    CreateIdentityPayload,
    IdentityGovernancePayload
};
use prospector_domain_models::telemetry::{ProvisioningLog, SystemLog};
use prospector_domain_models::lab::ProvingReport; // ✅ REPARADO: ProvingVerdict eliminado
use prospector_infra_db::repositories::{
    ScenarioRegistryRepository,
    IdentityRepository,
    MissionRepository
};

// --- ESTRUCTURAS DE DATOS DE MANDO ---

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemModeTransitionPayload {
    pub target_mode: SwarmOperationalMode,
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct LeaseQueryParams {
    pub platform: String,
}

pub struct ScenarioAdministrationHandler;

impl ScenarioAdministrationHandler {

    /**
     * Recupera el inventario de escenarios forenses (DNA Templates).
     */
    #[instrument(skip(application_state))]
    pub async fn handle_list_scenarios(
        State(application_state): State<AppState>
    ) -> impl IntoResponse {
        let scenario_repository = ScenarioRegistryRepository::new(application_state.database_client.clone());
        match scenario_repository.list_all_metadata().await {
            Ok(scenarios) => (StatusCode::OK, Json(scenarios)).into_response(),
            Err(database_fault) => (StatusCode::INTERNAL_SERVER_ERROR, database_fault.to_string()).into_response(),
        }
    }

    /**
     * Lista todas las identidades operativas en la Bóveda ZK.
     */
    #[instrument(skip(application_state))]
    pub async fn handle_list_identities(
        State(application_state): State<AppState>
    ) -> impl IntoResponse {
        let identity_repository = IdentityRepository::new(application_state.database_client.clone());
        match identity_repository.list_all_identities().await {
            Ok(identities) => (StatusCode::OK, Json(identities)).into_response(),
            Err(database_fault) => (StatusCode::INTERNAL_SERVER_ERROR, database_fault.to_string()).into_response(),
        }
    }

    /**
     * Ingesta o actualiza identidades cifradas (AES-256-GCM).
     */
    #[instrument(skip(application_state, payload), fields(email = %payload.email))]
    pub async fn handle_identity_ingestion(
        State(application_state): State<AppState>,
        Json(payload): Json<CreateIdentityPayload>
    ) -> impl IntoResponse {
        let identity_repository = IdentityRepository::new(application_state.database_client.clone());
        match identity_repository.upsert_sovereign_identity(payload).await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(database_fault) => (StatusCode::INTERNAL_SERVER_ERROR, database_fault.to_string()).into_response(),
        }
    }

    /**
     * Arrienda (Lease) una identidad para el Provisioner L6.
     */
    #[instrument(skip(application_state, query_params))]
    pub async fn handle_identity_lease(
        State(application_state): State<AppState>,
        Query(query_params): Query<LeaseQueryParams>
    ) -> impl IntoResponse {
        let identity_repository = IdentityRepository::new(application_state.database_client.clone());
        match identity_repository.lease_sovereign_identity(&query_params.platform, 15, "PROVISIONER_AUTO").await {
            Ok(Some(identity)) => (StatusCode::OK, Json(identity)).into_response(),
            Ok(None) => StatusCode::NO_CONTENT.into_response(),
            Err(database_fault) => (StatusCode::INTERNAL_SERVER_ERROR, database_fault.to_string()).into_response(),
        }
    }

    /**
     * Ejecuta una transición de modo operativa auditada.
     */
    #[instrument(skip(application_state, payload))]
    pub async fn handle_system_mode_transition(
        State(application_state): State<AppState>,
        Json(payload): Json<SystemModeTransitionPayload>
    ) -> impl IntoResponse {
        info!("🎮 [C2_OVERRIDE]: Requesting transition to {:?}. Logic: {}", payload.target_mode, payload.reason);
        application_state.operational_nexus.transition_mode(payload.target_mode, &payload.reason);
        StatusCode::OK.into_response()
    }

    /**
     * Protocolo 'Tabula Rasa': Incineración total de misiones.
     */
    #[instrument(skip(application_state))]
    pub async fn handle_system_purge(
        State(application_state): State<AppState>
    ) -> impl IntoResponse {
        warn!("🚨 [PURGE_IGNITION]: Executing administrative strata incineration...");

        let mission_repository = MissionRepository::new(application_state.database_client.clone());
        match mission_repository.purge_and_reset_system().await {
            Ok(count) => {
                info!("✨ [PURGE_COMPLETE]: {} records neutralized.", count);
                application_state.operational_nexus.transition_mode(
                    SwarmOperationalMode::Maintenance,
                    "POST_PURGE_RESET"
                );
                (StatusCode::OK, Json(json!({ "purged_count": count }))).into_response()
            },
            Err(fault) => (StatusCode::INTERNAL_SERVER_ERROR, fault.to_string()).into_response(),
        }
    }

    // --- ESTRATO DE GOBERNANZA IGFS ---

    pub async fn handle_identity_force_release(
        State(application_state): State<AppState>,
        Json(payload): Json<IdentityGovernancePayload>
    ) -> impl IntoResponse {
        let identity_repository = IdentityRepository::new(application_state.database_client.clone());
        match identity_repository.force_release_lease(&payload.email).await {
            Ok(_) => (StatusCode::OK, Json(json!({ "status": "LEASE_BROKEN" }))).into_response(),
            Err(database_fault) => (StatusCode::INTERNAL_SERVER_ERROR, database_fault.to_string()).into_response(),
        }
    }

    pub async fn handle_identity_purge(
        State(application_state): State<AppState>,
        Json(payload): Json<IdentityGovernancePayload>
    ) -> impl IntoResponse {
        let identity_repository = IdentityRepository::new(application_state.database_client.clone());
        match identity_repository.purge_identity_record(&payload.email).await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(database_fault) => (StatusCode::INTERNAL_SERVER_ERROR, database_fault.to_string()).into_response(),
        }
    }

    // --- ESTRATO DE TELEMETRÍA Y PROVING GROUNDS ---

    /**
     * Genera un reporte de salud 360° del Orquestador.
     */
    #[instrument(skip(application_state))]
    pub async fn handle_system_diagnostics(State(application_state): State<AppState>) -> impl IntoResponse {
        let current_memory_mb = Self::get_process_memory_usage();
        let current_nexus_state = application_state.operational_nexus.get_current_snapshot();

        let env_audit = json!({
            "github_c2_link": std::env::var("GITHUB_PAT").is_ok(),
            "strategic_hq_link": std::env::var("SUPABASE_SERVICE_ROLE_KEY").is_ok(),
            "master_vault_key": std::env::var("MASTER_VAULT_KEY").is_ok(),
            "worker_auth_token": std::env::var("WORKER_AUTH_TOKEN").is_ok(),
        });

        let diagnostic_report = json!({
            "timestamp": Utc::now().to_rfc3339(),
            "kernel_version": "V89.0-Sovereign",
            "state": {
                "mode": current_nexus_state.mode,
                "integrity": current_nexus_state.integrity,
                "reason": current_nexus_state.transition_reason
            },
            "environment_audit": env_audit,
            "telemetry": {
                "memory_rss_mb": current_memory_mb,
                "active_threads": num_cpus::get(),
                "platform": std::env::consts::OS
            }
        });

        (StatusCode::OK, Json(diagnostic_report)).into_response()
    }

    #[instrument(skip(application_state, log_payload), fields(node = %log_payload.node_index))]
    pub async fn handle_provisioning_log(
        State(application_state): State<AppState>,
        Json(log_payload): Json<ProvisioningLog>,
    ) -> impl IntoResponse {
        application_state.swarm_telemetry.push_navigation_trace(log_payload.clone());
        application_state.event_bus.emit_provisioning_trace(log_payload);
        StatusCode::ACCEPTED.into_response()
    }

    #[instrument(skip(application_state, report), fields(test = %report.test_name))]
    pub async fn handle_proving_report(
        State(application_state): State<AppState>,
        Json(report): Json<ProvingReport>,
    ) -> impl IntoResponse {
        info!("🛡️ [QA]: Receiving certification pulse: {}", report.test_name);

        let log_entry = SystemLog {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: report.timestamp.clone(),
            stratum: report.stratum.clone(),
            severity: format!("{:?}", report.verdict).to_uppercase(),
            message: format!("[{}] Verification: {}", report.test_name, report.forensic_log),
            metadata: Some(std::collections::HashMap::from([
                ("environment".into(), json!(report.environment)),
                ("metrics".into(), report.metrics)
            ])),
            trace_id: None,
        };

        application_state.event_bus.emit_system_log(log_entry);
        StatusCode::CREATED.into_response()
    }

    /**
     * Extrae el consumo de memoria RSS de forma segura.
     * # Logic:
     * Si el sistema no es Linux, retorna 0 para evitar pánicos.
     */
    fn get_process_memory_usage() -> u64 {
        match fs::read_to_string("/proc/self/status") {
            Ok(status_content) => {
                status_content.lines()
                    .find(|line| line.starts_with("VmRSS:"))
                    .and_then(|line| line.split_whitespace().nth(1))
                    .and_then(|value| value.parse::<u64>().ok())
                    .map(|kb| kb / 1024)
                    .unwrap_or(0)
            },
            Err(_) => 0 // Fallback para entornos no-procfs
        }
    }
}

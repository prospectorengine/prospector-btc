// [apps/orchestrator/src/handlers/admin.rs]
/**
 * =================================================================
 * APARATO: ADMINISTRATIVE HANDLER (V86.2 - ZENITH GOLD)
 * CLASIFICACIÓN: API ADAPTER LAYER (ESTRATO L3)
 * RESPONSABILIDAD: MANDO SUPREMO, GOBERNANZA E INTEGRIDAD TÉCNICA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. NEXUS ALIGNMENT: Resolución definitiva de E0599. Sincroniza con
 *    get_current_snapshot() y transition_reason del OperationalNexus V200.1.
 * 2. ZERO RESIDUE: Eliminación de la importación huérfana de MissionRepository
 *    (Warning E0433) y dependencias inexistentes de test.
 * 3. CONTRACT INTEGRITY: Garantiza que el JSON de diagnóstico reporte
 *    la integridad criptográfica real del sistema.
 * 4. PERFORMANCE: Uso de selectores nominales para el mapeo de identidades.
 *
 * # Mathematical Proof (Administrative Sovereignty):
 * El handler actúa como la única interfaz autorizada para mutar el
 * estado de despacho y la visibilidad de la Bóveda ZK, garantizando
 * que ninguna acción administrativa viole la seguridad del enjambre.
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
use tracing::{info, warn, error, instrument};

// --- SINAPSIS CON EL DOMINIO Y PERSISTENCIA (L2 / L3) ---
use prospector_domain_models::identity::{
    CreateIdentityPayload,
    IdentityGovernancePayload
};
use prospector_domain_models::telemetry::{ProvisioningLog, SystemLog};
use prospector_domain_models::lab::{ProvingReport, ProvingVerdict};
use prospector_infra_db::repositories::{
    ScenarioRegistryRepository,
    IdentityRepository
    // ✅ RESOLUCIÓN WARNING: MissionRepository eliminado por ser código muerto aquí
};

// --- ESTRUCTURAS DE DATOS DE MANDO ---

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemModeTransitionPayload {
    /// Nuevo modo operativo solicitado por el Centro de Mando L5.
    pub target_mode: SwarmOperationalMode,
}

#[derive(Debug, Deserialize)]
pub struct LeaseQueryParams {
    /// Filtro de plataforma (ej: google_colab).
    pub platform: String,
}

pub struct ScenarioAdministrationHandler;

impl ScenarioAdministrationHandler {

    /**
     * Recupera el inventario de escenarios forenses registrados en el Ledger Táctico.
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
     * Lista todas las identidades (Google/Kaggle) custodiadas en la Bóveda ZK.
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
     * Ingesta una nueva identidad cifrada AES-256-GCM desde el Dashboard Zenith.
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
     * Arrienda (Lease) una identidad disponible para un nuevo worker.
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
     * Ejecuta una transición de modo operativa en el OperationalNexus (C2 Signal).
     */
    #[instrument(skip(application_state, payload))]
    pub async fn handle_system_mode_transition(
        State(application_state): State<AppState>,
        Json(payload): Json<SystemModeTransitionPayload>
    ) -> impl IntoResponse {
        application_state.operational_nexus.transition_mode(payload.target_mode, "REMOTE_C2_SIGNAL");
        StatusCode::OK.into_response()
    }

    /**
     * Protocolo de Limpieza Total (Tabula Rasa). Reservado para reset de campaña.
     */
    #[instrument(skip(_state))]
    pub async fn handle_system_purge(
        State(_state): State<AppState>
    ) -> impl IntoResponse {
        warn!("🚨 [PURGE]: Manual system reset requested. Protocol in verification.");
        StatusCode::NOT_IMPLEMENTED.into_response()
    }

    // --- ESTRATO DE GOBERNANZA IGFS (IDENTITY GOVERNANCE) ---

    /**
     * Rompe el candado de una identidad (Identity Governance Protocol).
     */
    #[instrument(skip(application_state, payload))]
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

    /**
     * Incinera físicamente el rastro de una identidad en el Ledger Táctico.
     */
    #[instrument(skip(application_state, payload))]
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
     * Genera un reporte de salud del Kernel Rust analizando el consumo RSS de RAM y el Nexo.
     * ✅ RESOLUCIÓN E0599: Nivelación con get_current_snapshot() y transition_reason.
     */
    #[instrument(skip(application_state))]
    pub async fn handle_system_diagnostics(State(application_state): State<AppState>) -> impl IntoResponse {
        let current_utc_time = Utc::now();
        let current_memory_rss_megabytes = Self::get_process_memory_usage();

        // Sincronía con el nuevo API del Nexo V200.1
        let current_nexus_state = application_state.operational_nexus.get_current_snapshot();

        let diagnostic_report = json!({
            "timestamp": current_utc_time.to_rfc3339(),
            "status": {
                "operational_mode": format!("{:?}", current_nexus_state.mode),
                "integrity_level": format!("{:?}", current_nexus_state.integrity),
                "transition_logic": current_nexus_state.transition_reason,
            },
            "resources": {
                "memory_usage_mb": current_memory_rss_megabytes,
                "cpu_cores_logical": num_cpus::get(),
                "runtime_platform": std::env::consts::OS
            }
        });
        (StatusCode::OK, Json(diagnostic_report)).into_response()
    }

    /**
     * Receptor de trazas de navegación desde el automatizador L6 (Sentinel).
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
     * Punto de convergencia para reportes de certificación (Proving Grounds).
     */
    #[instrument(skip(application_state, report), fields(test = %report.test_name))]
    pub async fn handle_proving_report(
        State(application_state): State<AppState>,
        Json(report): Json<ProvingReport>,
    ) -> impl IntoResponse {
        info!("🛡️ [PROVING_GROUNDS]: Receiving {} certification pulse.", report.test_name);

        let log_severity_label = match report.verdict {
            ProvingVerdict::GoldMaster | ProvingVerdict::Stable => "INFO",
            ProvingVerdict::Degraded => "WARN",
            ProvingVerdict::Failed => "CRITICAL",
        };

        let log_entry_artifact = SystemLog {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: report.timestamp.clone(),
            stratum: report.stratum.clone(),
            severity: log_severity_label.to_string(),
            message: format!("[{}] Veredicto: {:?} -> {}", report.test_name, report.verdict, report.forensic_log),
            metadata: Some(std::collections::HashMap::from([
                ("environment_origin".to_string(), json!(report.environment)),
                ("performance_metrics".to_string(), report.metrics)
            ])),
            trace_id: None,
        };

        // Emisión inmediata al HUD de mando Zenith
        application_state.event_bus.emit_system_log(log_entry_artifact);

        if log_severity_label == "CRITICAL" {
            error!("💀 [QA_ALERT]: Critical strata failure in {}. Analysis: {}", report.test_name, report.forensic_log);
        }

        StatusCode::CREATED.into_response()
    }

    /**
     * Extrae el consumo de memoria real (Resident Set Size) del kernel actual.
     */
    fn get_process_memory_usage() -> u64 {
        fs::read_to_string("/proc/self/status")
            .unwrap_or_default()
            .lines()
            .find(|line| line.starts_with("VmRSS:"))
            .and_then(|line| line.split_whitespace().nth(1))
            .and_then(|value| value.parse::<u64>().ok())
            .map(|kilobytes| kilobytes / 1024)
            .unwrap_or(0)
    }
}

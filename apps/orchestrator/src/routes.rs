// INICIO DEL ARCHIVO [apps/orchestrator/src/routes.rs]
/*!
 * =================================================================
 * APARATO: SOVEREIGN ROUTING MATRIX (V201.0 - GATEWAY UNLOCKED)
 * CLASIFICACIÓN: API ADAPTER LAYER (ESTRATO L3)
 * RESPONSABILIDAD: ORQUESTACIÓN DE ESTRATOS VISUALES, TÁCTICOS Y NEURALES
 *
 * VISION HIPER-HOLÍSTICA:
 * Integra el 'Neural Data Gateway' (GraphQL) junto al 'Neural Socket'.
 * La topología ahora soporta:
 * 1. REST Táctico (High Frequency).
 * 2. WebSockets (Real-Time Control).
 * 3. GraphQL (Deep Relation Querying).
 * =================================================================
 */

// ✅ NUEVO: Importación del módulo 'graphql'
use crate::handlers::{admin, lab, stream, swarm, assets, visual, telemetry, graphql};
use crate::middleware::{auth_guard, health_guard};
use crate::state::AppState;
use axum::{
    middleware,
    routing::{get, post},
    Router,
    http::{header, Method}
};
use tower_http::cors::{Any, CorsLayer};
use std::time::Duration;

pub fn create_sovereign_router(application_shared_state: AppState) -> Router {
    // Escudo de Red: Permite CORS para el Dashboard (Vercel) y herramientas de Ops
    let network_security_shield = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .max_age(Duration::from_secs(3600));

    // ESTRATO TÁCTICO: Operaciones del Enjambre (Workers)
    let swarm_operations_stratum = Router::new()
        // Estado del enjambre
        .route("/status", get(swarm::SwarmHandshakeHandler::handle_get_swarm_status))
        // Negociación de Misión e Identidad
        .route("/mission/acquire", post(swarm::SwarmHandshakeHandler::negotiate_mission_assignment_handshake))
        // Reporte de Progreso (Checkpointing)
        .route("/mission/progress", post(swarm::SwarmHandshakeHandler::handle_mission_progress_report))
        // Certificación de Misión (Sellado)
        .route("/mission/complete", post(swarm::SwarmHandshakeHandler::register_mission_certification))
        // Protocolo Phoenix (Auto-Regeneración de Cookies)
        .route("/identity/refresh", post(swarm::SwarmHandshakeHandler::handle_identity_refresh))
        // Latidos de Vida (Biometría de Silicio)
        .route("/heartbeat", post(swarm::SwarmHandshakeHandler::register_worker_heartbeat_signal))
        // Reporte de Colisión (El Santo Grial)
        .route("/finding", post(swarm::SwarmHandshakeHandler::register_cryptographic_collision_finding));

    // ESTRATO DE LABORATORIO Y ADMINISTRACIÓN (Command Center)
    let lab_and_admin_stratum = Router::new()
        .nest("/lab", Router::new()
            .route("/certification/ignite", post(lab::CertificationHandler::handle_certification_ignition))
            .route("/verify", post(lab::CertificationHandler::handle_manual_verification))
            .route("/audit/brainwallet-dataset", get(lab::CertificationHandler::handle_brainwallet_dataset_audit))
            .route("/scenarios", get(admin::ScenarioAdministrationHandler::handle_list_scenarios)))

        .nest("/admin", Router::new()
            // Gestión de Identidad (ZK-Vault)
            .route("/identities", get(admin::ScenarioAdministrationHandler::handle_list_identities).post(admin::ScenarioAdministrationHandler::handle_identity_ingestion))
            .route("/identities/lease", get(admin::ScenarioAdministrationHandler::handle_identity_lease))
            .route("/identities/revoke", post(admin::ScenarioAdministrationHandler::handle_system_mode_transition))

            // Gobernanza de Identidad (IGFS - Release & Purge)
            .route("/identities/release", post(admin::ScenarioAdministrationHandler::handle_identity_force_release))
            .route("/identities/purge", post(admin::ScenarioAdministrationHandler::handle_identity_purge))

            // Observabilidad y C2 (Logs de Aprovisionamiento)
            .route("/provisioning/log", post(admin::ScenarioAdministrationHandler::handle_provisioning_log))
            .route("/diagnostics", get(admin::ScenarioAdministrationHandler::handle_system_diagnostics))

            // Protocolo de Recuperación de Desastres (System Reset)
            .route("/maintenance/purge", post(admin::ScenarioAdministrationHandler::handle_system_purge)));

    // ESTRATO NEURAL DE DATOS (GraphQL Oracle)
    let graphql_stratum = Router::new()
        // Endpoint Transaccional (JSON POST)
        .route("/", post(graphql::handle_graphql_query))
        // Endpoint Visual (Playground UI)
        .route("/playground", get(graphql::handle_playground));

    // COMPOSICIÓN GLOBAL (Root Topology)
    Router::new()
        .route("/", get(visual::handle_visual_landing))
        .route("/health", get(|| async { "STATUS_OK" }))
        .nest("/api/v1", Router::new()
            .nest("/swarm", swarm_operations_stratum)
            .merge(lab_and_admin_stratum)
            // ✅ NUEVO: Fusión del Estrato GraphQL
            .nest("/graphql", graphql_stratum)
            .route("/telemetry/ingest", post(telemetry::handle_log_ingestion))
            // Singularidad Activa: Endpoint WebSocket (Full Duplex)
            .route("/stream/metrics", get(stream::establish_neural_uplink))
            // Middleware de Salud y Autenticación (Escudo Global)
            .layer(middleware::from_fn_with_state(application_shared_state.clone(), health_guard))
            .layer(middleware::from_fn(auth_guard))
        )
        // Gateway de Archivos Binarios (Shards)
        .route("/api/v1/assets/dna/:strata/:filename", get(assets::AssetGatewayHandler::download_shard))
        .layer(network_security_shield)
        .with_state(application_shared_state)
}
// FIN DEL ARCHIVO [apps/orchestrator/src/routes.rs]

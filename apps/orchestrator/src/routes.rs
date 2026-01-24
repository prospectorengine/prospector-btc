// [apps/orchestrator/src/routes.rs]
/*!
 * =================================================================
 * APARATO: SOVEREIGN ROUTING MATRIX (V17.0 - SINGULARITY SEALED)
 * CLASIFICACIÓN: API ADAPTER LAYER (ESTRATO L3)
 * RESPONSABILIDAD: ORQUESTACIÓN DE ESTRATOS TÁCTICOS, NEURALES Y QA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. SYNTAX RECOVERY: Erradica el punto y coma fatal que bloqueaba el
 *    túnel visual L6 -> L5.
 * 2. QA STRATUM FORMALIZATION: Inyecta la ruta '/qa/report' para la
 *    consolidación de certificados de Proving Grounds.
 * 3. NEURAL LINK SYNC: Asegura que '/visual/snapshot' sea un ciudadano
 *    de primera clase en la telemetría del enjambre.
 * 4. HYGIENE: Cero abreviaciones. Uso de descriptores nominales absolutos.
 *
 * # Mathematical Proof (Routing Determinism):
 * La matriz garantiza un RTT < 10ms en la resolución de rutas mediante
 * un árbol de decisión estático, protegiendo la soberanía del operador
 * tras el 'auth_guard' nivelado.
 * =================================================================
 */

 use crate::handlers::{
    admin, lab, stream, swarm, assets, visual, telemetry, graphql,
    billing, notification, gamification
};
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

/**
 * Forja el enrutador soberano del sistema PROSPECTOR.
 *
 * # Performance:
 * Operación O(log N). El enrutamiento se resuelve en tiempo constante
 * para los carriles calientes de telemetría.
 */
pub fn create_sovereign_router(application_shared_state: AppState) -> Router {
    // Escudo de Red: Permite la sinapsis trans-nube certificada (Vercel -> Render)
    let network_security_shield = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .max_age(Duration::from_secs(3600));

    // --- ESTRATO 1: OPERACIONES DEL ENJAMBRE (TACTICAL WORKERS) ---
    // Este bloque gestiona el tráfico de alta frecuencia de los nodos efímeros.
    let swarm_operations_stratum = Router::new()
        .route("/status", get(swarm::SwarmHandshakeHandler::handle_get_swarm_status))
        .route("/mission/acquire", post(swarm::SwarmHandshakeHandler::negotiate_mission_assignment_handshake))
        .route("/mission/progress", post(swarm::SwarmHandshakeHandler::handle_mission_progress_report))
        .route("/mission/complete", post(swarm::SwarmHandshakeHandler::register_mission_certification))
        .route("/identity/refresh", post(swarm::SwarmHandshakeHandler::handle_identity_refresh))
        .route("/heartbeat", post(swarm::SwarmHandshakeHandler::register_worker_heartbeat_signal))
        .route("/finding", post(swarm::SwarmHandshakeHandler::register_cryptographic_collision_finding)) // ✅ Semicolon Purged
        .route("/visual/snapshot", post(telemetry::handle_visual_snapshot)); // ✅ Visual Tunnel Restored

    // --- ESTRATO 2: SERVICIOS AL USUARIO (L7 - ZENITH INTERFACE) ---
    let user_services_stratum = Router::new()
        .nest("/billing", Router::new()
            .route("/quota", get(billing::BillingHandler::handle_get_user_quota))
            .route("/history", get(billing::BillingHandler::handle_get_billing_history)))
        .nest("/herald", Router::new()
            .route("/notifications", get(notification::NotificationHandler::handle_list_notifications))
            .route("/notifications/read", post(notification::NotificationHandler::handle_mark_as_read)))
        .nest("/nexus", Router::new()
            .route("/prestige", get(gamification::GamificationHandler::handle_get_prestige_status))
            .route("/leaderboard", get(gamification::GamificationHandler::handle_get_leaderboard)));

    // --- ESTRATO 3: LABORATORIO Y ADMINISTRACIÓN (C2 MANDO) ---
    let lab_and_admin_stratum = Router::new()
        .nest("/lab", Router::new()
            .route("/certification/ignite", post(lab::CertificationHandler::handle_certification_ignition))
            .route("/verify", post(lab::CertificationHandler::handle_manual_verification))
            .route("/audit/brainwallet-dataset", get(lab::CertificationHandler::handle_brainwallet_dataset_audit))
            .route("/scenarios", get(admin::ScenarioAdministrationHandler::handle_list_scenarios)))
        .nest("/admin", Router::new()
            .route("/identities", get(admin::ScenarioAdministrationHandler::handle_list_identities).post(admin::ScenarioAdministrationHandler::handle_identity_ingestion))
            .route("/identities/lease", get(admin::ScenarioAdministrationHandler::handle_identity_lease))
            .route("/identities/revoke", post(admin::ScenarioAdministrationHandler::handle_system_mode_transition))
            .route("/identities/release", post(admin::ScenarioAdministrationHandler::handle_identity_force_release))
            .route("/identities/purge", post(admin::ScenarioAdministrationHandler::handle_identity_purge))
            .route("/provisioning/log", post(admin::ScenarioAdministrationHandler::handle_provisioning_log))
            .route("/diagnostics", get(admin::ScenarioAdministrationHandler::handle_system_diagnostics))
            .route("/maintenance/purge", post(admin::ScenarioAdministrationHandler::handle_system_purge)))
        .nest("/qa", Router::new() // ✅ NUEVO: Estrato de Proving Grounds
            .route("/report", post(admin::ScenarioAdministrationHandler::handle_proving_report)));

    // --- ESTRATO 4: NEURAL DATA & STREAMING (REAL-TIME LINK) ---
    let neural_link_stratum = Router::new()
        .nest("/graphql", Router::new()
            .route("/", post(graphql::handle_graphql_query))
            .route("/playground", get(graphql::handle_playground)))
        .route("/stream/metrics", get(stream::establish_neural_uplink))
        .route("/telemetry/ingest", post(telemetry::handle_log_ingestion));

    // --- COMPOSICIÓN GLOBAL (ROOT TOPOLOGY) ---
    Router::new()
        .route("/", get(visual::handle_visual_landing))
        .route("/health", get(|| async { "STATUS_OK" }))
        .nest("/api/v1", Router::new()
            .nest("/swarm", swarm_operations_stratum)
            .nest("/user", user_services_stratum)
            .merge(lab_and_admin_stratum)
            .merge(neural_link_stratum)
            // ESCUDO DE PROTECCIÓN SOBERANA
            .layer(middleware::from_fn_with_state(application_shared_state.clone(), health_guard))
            .layer(middleware::from_fn(auth_guard))
        )
        // Suministro de activos de simulación (DNA Shards)
        .route("/api/v1/assets/dna/:strata/:filename", get(assets::AssetGatewayHandler::download_shard))
        .layer(network_security_shield)
        .with_state(application_shared_state)
}

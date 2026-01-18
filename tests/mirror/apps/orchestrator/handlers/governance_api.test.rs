// INICIO DEL ARCHIVO [tests/mirror/apps/orchestrator/handlers/governance_api.test.rs]
/**
 * =================================================================
 * APARATO: GOVERNANCE API INTEGRITY TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar enrutamiento y deserialización de acciones IGFS.
 * =================================================================
 */

use prospector_orchestrator::state::AppState;
use prospector_orchestrator::handlers::admin::ScenarioAdministrationHandler;
use prospector_domain_models::identity::IdentityGovernancePayload;
use prospector_infra_db::TursoClient;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::post,
    Router,
};
use tower::ServiceExt;

#[tokio::test]
async fn certify_governance_endpoints_connectivity() {
    // 1. SETUP: Ambiente volátil
    let client = TursoClient::connect("file::memory:", None).await.unwrap();
    let state = AppState::new(client);

    let app = Router::new()
        .route("/admin/identities/release", post(ScenarioAdministrationHandler::handle_identity_force_release))
        .route("/admin/identities/purge", post(ScenarioAdministrationHandler::handle_identity_purge))
        .with_state(state);

    let payload = IdentityGovernancePayload {
        email: "zombie_test@prospector.io".to_string(),
        reason: Some("INTEGRATION_TEST".to_string()),
    };
    let json_body = serde_json::to_string(&payload).unwrap();

    // 2. TEST: Force Release
    let req_release = Request::builder()
        .method("POST")
        .uri("/admin/identities/release")
        .header("Content-Type", "application/json")
        .body(Body::from(json_body.clone()))
        .unwrap();

    let res_release = app.clone().oneshot(req_release).await.unwrap();

    // Debería ser 200 OK (incluso si no encuentra el email, el handler loguea warning y retorna OK por diseño idempotente o manejo de errores interno)
    assert_eq!(res_release.status(), StatusCode::OK, "Endpoint RELEASE falló");

    // 3. TEST: Purge
    let req_purge = Request::builder()
        .method("POST")
        .uri("/admin/identities/purge")
        .header("Content-Type", "application/json")
        .body(Body::from(json_body))
        .unwrap();

    let res_purge = app.oneshot(req_purge).await.unwrap();
    assert_eq!(res_purge.status(), StatusCode::OK, "Endpoint PURGE falló");

    println!("✅ GOVERNANCE_API: Endpoints routed and payloads accepted.");
}
// FIN DEL ARCHIVO [tests/mirror/apps/orchestrator/handlers/governance_api.test.rs]

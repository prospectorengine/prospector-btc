// INICIO DEL ARCHIVO [tests/mirror/apps/orchestrator/routes/ws_wiring.test.rs]
/**
 * =================================================================
 * APARATO: ROUTER WIRING TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE (INTEGRATION)
 * OBJETIVO: Verificar que /api/v1/stream/metrics acepta Upgrade.
 * =================================================================
 */

use prospector_infra_db::TursoClient;
use prospector_orchestrator::state::AppState;
use prospector_orchestrator::routes::create_sovereign_router;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn certify_global_router_websocket_upgrade() {
    // 1. SETUP
    let client = TursoClient::connect("file::memory:", None).await.unwrap();
    let state = AppState::new(client);
    let app = create_sovereign_router(state);

    // 2. EXECUTION: Request de Upgrade simulado con Token de Auth
    // Nota: El auth_guard requiere el token, incluso para WS en esta fase.
    let req = Request::builder()
        .method("GET")
        .uri("/api/v1/stream/metrics")
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header("Sec-WebSocket-Key", "dGhlIHNhbXBsZSBub25jZQ==")
        // Simulamos un token válido (o el default del middleware si no hay env)
        .header("Authorization", "Bearer ")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    // 3. VALIDATION
    // Si el router conecta el handler correcto, Axum debe responder 101
    assert_eq!(response.status(), StatusCode::SWITCHING_PROTOCOLS, "El router no negoció el Upgrade a WebSockets.");
    println!("✅ ROUTER: WebSocket endpoint /stream/metrics is wired correctly.");
}
// FIN DEL ARCHIVO [tests/mirror/apps/orchestrator/routes/ws_wiring.test.rs]

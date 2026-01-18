// INICIO DEL ARCHIVO [tests/mirror/apps/orchestrator/handlers/websocket_stream.test.rs]
/**
 * =================================================================
 * APARATO: WEBSOCKET HANDSHAKE TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Validar la negociación de upgrade del Neural Uplink.
 * =================================================================
 */

use prospector_orchestrator::state::AppState;
use prospector_orchestrator::handlers::stream::establish_neural_uplink;
use prospector_infra_db::TursoClient;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::get,
    Router,
};
use tower::ServiceExt;

#[tokio::test]
async fn certify_websocket_upgrade_negotiation() {
    // 1. SETUP: Estado Volátil
    let client = TursoClient::connect("file::memory:", None).await.unwrap();
    let state = AppState::new(client);

    let app = Router::new()
        .route("/stream/metrics", get(establish_neural_uplink))
        .with_state(state);

    // 2. EJECUCIÓN: Petición de Upgrade WS estándar
    let req = Request::builder()
        .method("GET")
        .uri("/stream/metrics")
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header("Sec-WebSocket-Key", "dGhlIHNhbXBsZSBub25jZQ==")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    // 3. VALIDACIÓN: Código 101 Switching Protocols
    assert_eq!(response.status(), StatusCode::SWITCHING_PROTOCOLS);
    println!("✅ WEBSOCKET: Upgrade handshake certified (HTTP 101).");
}
// FIN DEL ARCHIVO [tests/mirror/apps/orchestrator/handlers/websocket_stream.test.rs]

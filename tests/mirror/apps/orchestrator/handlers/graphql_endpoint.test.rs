// INICIO DEL ARCHIVO [tests/mirror/apps/orchestrator/handlers/graphql_endpoint.test.rs]
/**
 * =================================================================
 * APARATO: GRAPHQL HTTP ENDPOINT TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar la tubería HTTP -> GraphQL -> Resolver.
 * =================================================================
 */

use prospector_orchestrator::state::AppState;
use prospector_orchestrator::handlers::graphql;
use prospector_infra_db::TursoClient;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::post,
    Router,
};
use tower::ServiceExt;
use serde_json::json;

#[tokio::test]
async fn certify_http_graphql_resolution() {
    // 1. SETUP: Estado Volátil
    let client = TursoClient::connect("file::memory:", None).await.unwrap();
    let state = AppState::new(client);

    // 2. ROUTER: Router aislado solo con el endpoint GQL
    let app = Router::new()
        .route("/graphql", post(graphql::handle_graphql_query))
        .with_state(state);

    // 3. PAYLOAD: Query estándar
    let query_payload = json!({
        "query": "{ neuralGatewayStatus }"
    });

    // 4. EJECUCIÓN
    let req = Request::builder()
        .method("POST")
        .uri("/graphql")
        .header("Content-Type", "application/json")
        .body(Body::from(query_payload.to_string()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    // 5. VALIDACIÓN
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();

    // Verificamos la estructura { "data": { "neuralGatewayStatus": "..." } }
    let status = body_json["data"]["neuralGatewayStatus"].as_str().unwrap();

    assert!(status.contains("ONLINE"), "La respuesta HTTP no contiene el dato del resolver");
    println!("✅ GQL_ENDPOINT: HTTP pipe certified.");
}
// FIN DEL ARCHIVO [tests/mirror/apps/orchestrator/handlers/graphql_endpoint.test.rs]

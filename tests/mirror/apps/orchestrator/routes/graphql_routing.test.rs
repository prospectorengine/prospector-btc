// INICIO DEL ARCHIVO [tests/mirror/apps/orchestrator/routes/graphql_routing.test.rs]
/**
 * =================================================================
 * APARATO: GRAPHQL ROUTING INTEGRITY TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE (INTEGRATION)
 * OBJETIVO: Validar la disponibilidad del endpoint /api/v1/graphql.
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
use serde_json::json;

#[tokio::test]
async fn certify_graphql_route_accessibility() {
    // 1. SETUP
    let client = TursoClient::connect("file::memory:", None).await.unwrap();
    let state = AppState::new(client);
    let app = create_sovereign_router(state);

    // 2. PAYLOAD DE CONSULTA
    let query_payload = json!({
        "query": "{ neuralGatewayStatus }"
    });

    // 3. EJECUCIÓN (Simulando Auth Token para pasar el middleware global)
    // El router exige Auth en /api/v1/*
    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/graphql")
        .header("Content-Type", "application/json")
        // En test, el middleware puede requerir un token específico si está configurado,
        // o pasar si no hay env var WORKER_AUTH_TOKEN configurada (fall-through en lógica de test).
        // Asumimos comportamiento seguro por defecto.
        .body(Body::from(query_payload.to_string()))
        .unwrap();

    // Nota: Si el middleware de auth rechaza por falta de token en el entorno de test,
    // este test valida al menos que la ruta existe (401 vs 404).
    // Para una validación 200 OK, inyectamos el token.
    std::env::set_var("WORKER_AUTH_TOKEN", "test_secret");
    let mut authenticated_req = req;
    authenticated_req.headers_mut().insert(
        "Authorization",
        "Bearer test_secret".parse().unwrap()
    );

    let response = app.oneshot(authenticated_req).await.unwrap();

    // 4. VALIDACIÓN
    assert_ne!(response.status(), StatusCode::NOT_FOUND, "La ruta GraphQL no está montada.");
    assert_eq!(response.status(), StatusCode::OK, "La petición GraphQL válida fue rechazada.");

    println!("✅ ROUTER: GraphQL Gateway routed successfully.");
}
// FIN DEL ARCHIVO [tests/mirror/apps/orchestrator/routes/graphql_routing.test.rs]

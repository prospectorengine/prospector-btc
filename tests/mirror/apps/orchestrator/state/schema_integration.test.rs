// INICIO DEL ARCHIVO [tests/mirror/apps/orchestrator/state/schema_integration.test.rs]
/**
 * =================================================================
 * APARATO: NEURAL SCHEMA INTEGRATION TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Validar que AppState expone el QueryRoot funcional.
 * =================================================================
 */

use prospector_orchestrator::state::AppState;
use prospector_infra_db::TursoClient;
use async_graphql::Request;

#[tokio::test]
async fn certify_app_state_resolves_neural_queries() {
    // 1. SETUP
    let client = TursoClient::connect("file::memory:", None).await.unwrap();
    let state = AppState::new(client);

    // 2. EJECUCIÓN: Query real contra el AppState
    let query = "{ neuralGatewayStatus }";
    let response = state.graphql_schema.execute(Request::new(query)).await;

    // 3. VALIDACIÓN
    assert!(response.is_ok());
    let json = serde_json::to_string(&response.data).unwrap();

    // Debe contener la respuesta definida en graphql/mod.rs
    assert!(json.contains("NEURAL_GATEWAY_ONLINE_V1"), "Fallo en la resolución de QueryRoot desde AppState");

    println!("✅ SCHEMA_INTEGRATION: AppState is powering the Neural Gateway.");
}
// FIN DEL ARCHIVO [tests/mirror/apps/orchestrator/state/schema_integration.test.rs]

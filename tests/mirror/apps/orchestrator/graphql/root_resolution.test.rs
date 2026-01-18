// INICIO DEL ARCHIVO [tests/mirror/apps/orchestrator/graphql/root_resolution.test.rs]
/**
 * =================================================================
 * APARATO: GRAPHQL ROOT RESOLVER TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Validar ejecución de QueryRoot y acceso a Contexto.
 * =================================================================
 */

use prospector_orchestrator::graphql::QueryRoot;
use prospector_infra_db::TursoClient;
use async_graphql::{Schema, EmptyMutation, EmptySubscription, Request};

#[tokio::test]
async fn certify_graphql_root_context_access() {
    // 1. SETUP: Infraestructura Simulada
    let db_client = TursoClient::connect("file::memory:", None).await.unwrap();

    // 2. CONSTRUCCIÓN: Esquema con QueryRoot real
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(db_client) // Inyección crítica
        .finish();

    // 3. EJECUCIÓN: Query de prueba
    let query = "{ neuralGatewayStatus }";
    let response = schema.execute(Request::new(query)).await;

    // 4. VALIDACIÓN
    assert!(response.is_ok(), "El resolver falló internamente.");

    let json_res = serde_json::to_string(&response.data).unwrap();
    assert!(json_res.contains("NEURAL_GATEWAY_ONLINE"), "Respuesta de estado incorrecta.");

    println!("✅ GRAPHQL_ROOT: Resolver executed successfully with DB Context.");
}
// FIN DEL ARCHIVO [tests/mirror/apps/orchestrator/graphql/root_resolution.test.rs]

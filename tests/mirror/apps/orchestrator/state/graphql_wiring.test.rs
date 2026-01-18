// INICIO DEL ARCHIVO [tests/mirror/apps/orchestrator/state/graphql_wiring.test.rs]
/**
 * =================================================================
 * APARATO: GRAPHQL WIRING TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Validar la ignición del Neural Schema y Contexto.
 * =================================================================
 */

use prospector_orchestrator::state::AppState;
use prospector_infra_db::TursoClient;
use async_graphql::{Request, Variables};

#[tokio::test]
async fn certify_neural_schema_initialization() {
    // 1. SETUP: Estado con DB en memoria
    let client = TursoClient::connect("file::memory:", None).await.unwrap();
    let state = AppState::new(client);

    // 2. INSPECCIÓN: Verificar que el esquema existe en el estado
    let schema = &state.graphql_schema;

    // 3. EJECUCIÓN: Query de Introspección (Debe funcionar incluso con EmptyQuery)
    let query = "{ __schema { types { name } } }";
    let request = Request::new(query);
    let response = schema.execute(request).await;

    // 4. VALIDACIÓN
    assert!(response.is_ok(), "El esquema GraphQL falló al ejecutar introspección.");
    let json = serde_json::to_string(&response.data).unwrap();

    // Debe contener tipos básicos del sistema GraphQL (Boolean, String, etc.)
    assert!(json.contains("Boolean"), "El esquema no está correctamente formado.");

    println!("✅ GRAPHQL_WIRING: Neural Schema is active and introspectable.");
}
// FIN DEL ARCHIVO [tests/mirror/apps/orchestrator/state/graphql_wiring.test.rs]

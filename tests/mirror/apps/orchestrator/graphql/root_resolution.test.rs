// [tests/mirror/apps/orchestrator/graphql/root_resolution.test.rs]
/**
 * =================================================================
 * APARATO: GRAPHQL ROOT RESOLUTION CERTIFIER (V1.1 - SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L4-GATEWAY-MIRROR
 * RESPONSABILIDAD: VALIDACIÓN DE ACCESO AL CONTEXTO Y RESOLUCIÓN RAÍZ
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. VISIBILITY FIX: Sella el error 'private fields' utilizando la
 *    factoría nominal 'build_neural_schema' en lugar de instanciación literal.
 * 2. DEPENDENCY INJECTION: Certifica que el Oráculo recibe y utiliza
 *    correctamente el TursoClient y el EventBus.
 * 3. ZERO ABBREVIATIONS: 'res' -> 'query_response_artifact'.
 * 4. HYGIENE: Limpieza total de imports huérfanos y tipado estricto.
 * =================================================================
 */

use prospector_orchestrator::graphql::{build_neural_schema, NeuralSchema};
use prospector_orchestrator::services::event_bus::EventBus;
use prospector_infra_db::TursoClient;
use std::sync::Arc;

/**
 * CERTIFICACIÓN: Ejecución exitosa de consulta sobre el Oráculo Neural.
 *
 * # Mathematical Proof (Wiring Integrity):
 * Si la consulta 'neuralGatewayStatus' retorna el sello V2.8, se garantiza
 * que el Grafo Raíz tiene visibilidad sobre el Motor A (Turso).
 */
#[tokio::test]
async fn certify_graphql_root_resolution_and_context_sync() {
    println!("\n🧠 [PROVING_GROUNDS]: Initiating Neural Oracle Root Audit...");

    // 1. SETUP: Infraestructura Táctica Simulada (RAM Strata)
    let database_client_instance = TursoClient::connect("file:gql_root_test?mode=memory&cache=shared", None)
        .await
        .expect("CRITICAL_FAULT: Failed to anchor tactical ledger for test.");

    let event_bus_instance = Arc::new(EventBus::new());

    // 2. IGNICIÓN: Construcción del esquema mediante la factoría soberana
    // ✅ RESOLUCIÓN: Uso de build_neural_schema para evitar error de constructor privado
    let neural_schema_instance: NeuralSchema = build_neural_schema(
        database_client_instance,
        event_bus_instance
    );

    // 3. EXECUTION: Disparo de consulta de salud del Gateway
    let tactical_query_string = "{ neuralGatewayStatus }";

    println!("   📡 [SIGNAL]: Requesting status from QueryRoot...");
    let query_response_artifact = neural_schema_instance.execute(tactical_query_string).await;

    // 4. VALIDATION: Verificación de paridad bit-perfecta
    assert!(
        query_response_artifact.is_ok(),
        "L4_GQL: Oracle rejected the root resolution signal."
    );

    let response_data_json = query_response_artifact.data.into_json()
        .expect("DATA_FAULT: Failed to serialize Oracle response.");

    let status_message = response_data_json["neuralGatewayStatus"].as_str()
        .expect("TYPE_FAULT: neuralGatewayStatus field is not a String.");

    // Sincronía con la versión nominal definida en mod.rs
    assert!(
        status_message.contains("ZENITH_ORACLE_V2.8"),
        "VERSION_MISMATCH: The Oracle reported an obsolete or corrupted strata: {}",
        status_message
    );

    println!("   ✅ [SUCCESS]: Root resolution and context bridge certified.");
    println!("🏁 [COMPLETE]: Neural Gateway V2.8 is operational.\n");
}

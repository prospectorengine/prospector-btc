// [tests/mirror/apps/orchestrator/graphql/gateway_v2_1.test.rs]
/**
 * =================================================================
 * APARATO: NEURAL GATEWAY CERTIFIER (V2.3 - ZENITH ALIGNED)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L4-MIRROR
 * RESPONSABILIDAD: VALIDACIÓN DE ESQUEMA Y CONTEXTO DEL ORÁCULO
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. SIGNATURE PARITY: Resolución definitiva de E0061 mediante la inyección
 *    del EventBus en el constructor del esquema, garantizando paridad L3-L4.
 * 2. HYGIENE: Eliminación de imports redundantes de async_graphql (EmptyMutation,
 *    EmptySubscription, Schema) que ahora residen tras la abstracción del Orquestador.
 * 3. CONTEXT INTEGRITY: Certifica que el Oráculo tiene acceso soberano tanto
 *    al Ledger Táctico (Turso) como al sistema de señales.
 * 4. NOMINAL VERACITY: Validación bit-perfecta del campo 'neuralGatewayStatus'.
 * =================================================================
 */

use prospector_orchestrator::graphql::{build_neural_schema, NeuralSchema};
use prospector_orchestrator::services::event_bus::EventBus;
use prospector_infra_db::TursoClient;
use std::sync::Arc;

/**
 * CERTIFICACIÓN: Handshake de Salud del Oráculo.
 *
 * Valida que el esquema se construya correctamente y que el resolver de
 * infraestructura pueda interrogar al contexto inyectado.
 */
#[tokio::test]
async fn certify_neural_oracle_at_layer_4() {
    println!("\n🔮 [PROVING_GROUNDS]: Neural GraphQL Oracle Audit...");

    // 1. SETUP: Mock de Infraestructura (Estratigrafía L3)
    let mock_client = TursoClient::connect("file:gql_test?mode=memory&cache=shared", None)
        .await
        .expect("FALLO_CRÍTICO: No se pudo anclar el Ledger en RAM para el test.");

    // Inyección del sistema nervioso (Requerido por build_neural_schema V2.7+)
    let mock_event_bus = Arc::new(EventBus::new());

    // 2. FORJA DEL ESQUEMA: Uso de la factoría soberana con 2 argumentos
    // ✅ RESOLUCIÓN E0061: Firma nivelada con (TursoClient, Arc<EventBus>)
    let schema: NeuralSchema = build_neural_schema(mock_client, mock_event_bus);

    // 3. QUERY DE INSPECCIÓN: Verificación de Salud del Oráculo
    // Solicitamos el estado del gateway, que certifica el enlace físico con Turso.
    let inspection_query = r#"
        query {
            neuralGatewayStatus
        }
    "#;

    println!("   📡 [QUERY]: Dispatching health pulse to the Oracle...");
    let response = schema.execute(inspection_query).await;

    // 4. VERIFICACIÓN DE SOBERANÍA
    assert!(response.is_ok(), "El Oráculo rechazó la consulta de integridad inicial.");

    let response_data = response.data.into_json().unwrap();
    let health_status = response_data["neuralGatewayStatus"].as_str()
        .expect("FALLO_TIPO: neuralGatewayStatus no devolvió un String.");

    // El valor debe certificar la versión operativa Zenith Gold Master
    assert!(health_status.contains("ZENITH_ORACLE_V2.7"), "Versión del Oráculo desactualizada en el reporte.");

    println!("✅ GRAPHQL_V2.3: Signature and Cross-Stratum Injection certified.");
}

/**
 * CERTIFICACIÓN: Integridad de Fusión de Esquemas (MergedObject).
 *
 * Verifica que el oráculo haya unificado correctamente los dominios
 * de Infraestructura y Academia en la raíz del grafo.
 */
#[tokio::test]
async fn audit_schema_merging_integrity() {
    println!("   📚 [AUDIT]: Verifying Domain Merging (System + Academy)...");

    let mock_client = TursoClient::connect("file:merge_test?mode=memory&cache=shared", None).await.unwrap();
    let mock_event_bus = Arc::new(EventBus::new());
    let schema = build_neural_schema(mock_client, mock_event_bus);

    // Consulta de introspección para validar que el oráculo unificó los campos
    let introspection_query = r#"
        query {
            __schema {
                queryType {
                    fields {
                        name
                    }
                }
            }
        }
    "#;

    let response = schema.execute(introspection_query).await;
    let fields = response.data.into_json().unwrap();

    let field_names: Vec<&str> = fields["__schema"]["queryType"]["fields"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["name"].as_str().unwrap())
        .collect();

    // Verificamos la coexistencia de ambos dominios en la raíz
    assert!(field_names.contains(&"neuralGatewayStatus"), "Falta el campo de infraestructura en la raíz.");
    assert!(field_names.contains(&"academyStatus"), "Falta el campo académico en la raíz o no está enlazado.");

    println!("✅ GRAPHQL_V2.3: MergedObject structural parity confirmed.");
}

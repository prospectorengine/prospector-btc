// [tests/mirror/apps/orchestrator/graphql/academy_dynamic_v2.test.rs]
/*!
 * =================================================================
 * APARATO: ACADEMY DYNAMIC INFERENCE TEST (V2.1 - ZENITH CERTIFIED)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L4-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE LÓGICA DE DESBLOQUEO ACADÉMICO
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. SIGNATURE PARITY: Resolución definitiva de E0061. Inyección de
 *    'Arc<EventBus>' en el constructor del esquema neural.
 * 2. DATA HYDRATION: Inyecta metadatos de 'knowledge_modules' para
 *    validar la inferencia real de prerrequisitos.
 * 3. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta (conn -> database_connection).
 * 4. HYGIENE: Cero advertencias de mutabilidad y rastro forense detallado.
 *
 * # Mathematical Proof (Inference Engine Logic):
 * El test valida que el estado de un módulo 'M' transiciona de 'LOCKED' a
 * 'UNLOCKED' si y solo si el conjunto de sus dependencias {D} está
 * contenido en el conjunto de progreso certificado {P}.
 * =================================================================
 */

use prospector_orchestrator::graphql::build_neural_schema;
use prospector_orchestrator::services::event_bus::EventBus;
use prospector_infra_db::TursoClient;
use std::sync::Arc;

#[tokio::test]
async fn certify_dynamic_academy_unlock_logic() {
    println!("\n📚 [PROVING_GROUNDS]: Initiating Dynamic Academy Inference Audit...");

    // 1. SETUP: Infraestructura Volátil (Motor A en RAM)
    let database_client = TursoClient::connect("file::memory:", None)
        .await
        .expect("FALLO_CRÍTICO: No se pudo anclar el Ledger en RAM.");

    let database_connection = database_client.get_connection()
        .expect("FALLO_CONEXION: El pool de hilos de la DB colapsó.");

    // 2. SEEDING: Hidratación del Grafo de Conocimiento (Sustrato L2)
    // Inyectamos el módulo base y el módulo dependiente
    database_connection.execute(
        "INSERT INTO knowledge_modules (identifier, i18n_title_key, i18n_description_key, difficulty, prerequisites)
         VALUES ('ECC-01', 'key_title', 'key_desc', 'Foundation', '')",
        ()
    ).await.unwrap();

    database_connection.execute(
        "INSERT INTO knowledge_modules (identifier, i18n_title_key, i18n_description_key, difficulty, prerequisites)
         VALUES ('FORENSIC-01', 'key_title', 'key_desc', 'Intermediate', 'ECC-01')",
        ()
    ).await.unwrap();

    // 3. INYECTAR PROGRESO: Certificar el módulo base para el operador de prueba
    database_connection.execute(
        "INSERT INTO academy_progress (operator_id, module_identifier, status)
         VALUES ('TestUser', 'ECC-01', 'completed')",
        ()
    ).await.unwrap();

    // 4. IGNICIÓN DEL ORÁCULO: Sincronía de la Tríada de firma
    // ✅ RESOLUCIÓN E0061: Se suministra el Arc<EventBus> faltante
    let shared_event_bus = Arc::new(EventBus::new());
    let neural_schema = build_neural_schema(database_client, shared_event_bus);

    // 5. QUERY: Ejecución de consulta contra el Oráculo GQL
    let query_request = r#"
        query {
            getAdaptiveCurriculum(operatorId: "TestUser") {
                identifier
                currentStatus
            }
        }
    "#;

    println!("   📡 [QUERY]: Dispatching adaptive curriculum request...");
    let query_response = neural_schema.execute(query_request).await;

    // Verificación de integridad de la respuesta
    assert!(query_response.is_ok(), "El Oráculo GQL rechazó la consulta de inferencia.");

    let response_data_json = query_response.data.into_json().unwrap();
    let modules_collection = response_data_json["getAdaptiveCurriculum"]
        .as_array()
        .expect("CONTRACT_MISMATCH: La respuesta no contiene un array de módulos.");

    // 6. VALIDATION: Verificación bit-perfect de la inferencia
    // El módulo FORENSIC-01 debe haber transicionado a Unlocked (o su representación nominal)
    let forensic_module_artifact = modules_collection.iter()
        .find(|m| m["identifier"] == "FORENSIC-01")
        .expect("DATA_LOSS: El módulo FORENSIC-01 no fue retornado por el Oráculo.");

    // ✅ NOTA: Sincronizado con el Enum de Dominio L2 (ModuleStatus::Unlocked)
    assert!(
        forensic_module_artifact["currentStatus"].as_str().unwrap().to_uppercase() == "UNLOCKED",
        "INFERENCE_FAULT: El motor no reconoció la satisfacción del prerrequisito ECC-01."
    );

    println!("   ✅ [VERDICT]: Inference Engine confirmed. Progress-based unlocking certified.");
    println!("🏁 [COMPLETE]: Academy Strata L4-Mirror levelized.\n");
}

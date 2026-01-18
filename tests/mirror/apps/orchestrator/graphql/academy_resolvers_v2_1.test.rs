/**
 * =================================================================
 * APARATO: ACADEMY RESOLVER TEST (V2.1 - DYNAMIC)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L4-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE INFERENCIA DE CONOCIMIENTO
 * =================================================================
 */

use prospector_orchestrator::graphql::build_neural_schema;
use prospector_infra_db::TursoClient;
use libsql::params;

#[tokio::test]
async fn certify_dynamic_module_unlocking_logic() {
    println!("\n📚 [PROVING_GROUNDS]: Academy Inference Engine Audit...");

    // 1. SETUP: Base de datos en memoria con esquema de conocimiento
    let client = TursoClient::connect("file::memory:", None).await.unwrap();
    let conn = client.get_connection().unwrap();

    // Inyectar Definiciones
    conn.execute(
        "INSERT INTO knowledge_modules (identifier, i18n_title_key, prerequisites)
         VALUES ('MOD-A', 'Genesis', ''), ('MOD-B', 'Advanced', 'MOD-A')",
        ()
    ).await.unwrap();

    // Inyectar Progreso (MOD-A completado)
    conn.execute(
        "INSERT INTO academy_progress (operator_id, module_identifier, status)
         VALUES ('TEST-OP', 'MOD-A', 'completed')",
        ()
    ).await.unwrap();

    let schema = build_neural_schema(client);

    // 2. QUERY: Pedir currículum
    let query = r#"
        query {
            getAdaptiveCurriculum(operatorId: "TEST-OP") {
                identifier
                currentStatus
            }
        }
    "#;

    let response = schema.execute(query).await;
    let data = response.data.into_json().unwrap();
    let modules = data["getAdaptiveCurriculum"].as_array().unwrap();

    // 3. VALIDACIÓN: MOD-B debe estar UNLOCKED porque MOD-A está completo
    let mod_b = modules.iter().find(|m| m["identifier"] == "MOD-B").unwrap();
    assert_eq!(mod_b["currentStatus"], "Unlocked");

    println!("   ✅ [SUCCESS]: Dynamic unlocking logic certified bit-perfect.");
}

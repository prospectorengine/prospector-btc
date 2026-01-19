/**
 * =================================================================
 * APARATO: SCHEMA V151 INTEGRITY TEST
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L3-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE LA TABLA DE CONOCIMIENTO
 * =================================================================
 */

use prospector_infra_db::TursoClient;


#[tokio::test]
async fn certify_knowledge_strata_materialization() {
    println!("\n🏗️ [PROVING_GROUNDS]: Database Schema V151 Audit...");

    // 1. SETUP: In-Memory DB (El constructor llama a apply_full_sovereign_schema internamente)
    let client = TursoClient::connect("file::memory:", None).await.unwrap();
    let conn = client.get_connection().unwrap();

    // 2. VALIDATION: Verificar presencia física de la tabla knowledge_modules
    let check_table = conn.query(
        "SELECT identifier FROM knowledge_modules LIMIT 0",
        ()
    ).await;

    assert!(check_table.is_ok(), "La tabla 'knowledge_modules' no fue materializada en el sustrato.");

    // 3. VALIDATION: Verificar integridad de columnas requeridas por el Oracle V2.1
    let check_columns = conn.query(
        "SELECT i18n_title_key, prerequisites, difficulty FROM knowledge_modules LIMIT 0",
        ()
    ).await;

    assert!(check_columns.is_ok(), "Faltan columnas críticas en 'knowledge_modules'.");

    println!("   ✅ [SUCCESS]: Database strata V151 certified for dynamic Academy.");
}

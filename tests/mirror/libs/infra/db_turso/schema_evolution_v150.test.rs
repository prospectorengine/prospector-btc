use prospector_infra_db::TursoClient;
use prospector_infra_db::schema::apply_full_sovereign_schema;

#[tokio::test]
async fn certify_schema_v150_idempotency_and_completeness() {
    println!("\n🏗️ [PROVING_GROUNDS]: Database Strata Evolution Audit...");

    // 1. SETUP: In-Memory DB
    let client = TursoClient::connect("file::memory:", None).await.unwrap();
    let conn = client.get_connection().unwrap();

    // 2. EXECUTION: La primera pasada crea todo (apply_full_sovereign_schema se llama en connect)
    // Forzamos una segunda pasada para validar la IDEMPOTENCIA
    let result = apply_full_sovereign_schema(&conn).await;
    assert!(result.is_ok(), "El motor de esquema no es idempotente");

    // 3. VALIDATION: Verificar presencia de nuevas columnas críticas
    let check_cols = conn.query("SELECT operator_id, parent_mission_id FROM jobs LIMIT 0", ()).await;
    assert!(check_cols.is_ok(), "Faltan columnas de Slicing/Afiliados en la tabla JOBS");

    let check_academy = conn.query("SELECT * FROM academy_progress LIMIT 0", ()).await;
    assert!(check_academy.is_ok(), "Tabla academy_progress no materializada");

    println!("✅ SCHEMA_V150: Evolution and Idempotency certified.");
}

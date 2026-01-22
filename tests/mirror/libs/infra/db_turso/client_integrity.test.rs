// [tests/mirror/libs/infra/db_turso/client_integrity.test.rs]
/**
 * =================================================================
 * APARATO: DB CLIENT INTEGRITY TEST (V1.2 - SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar el rechazo de variables vacías y ruteo mmap.
 * =================================================================
 */

use prospector_infra_db::TursoClient;

#[tokio::test]
async fn certify_client_rejects_empty_environment_variables() {
    println!("\n🕵️ [PROVING_GROUNDS]: Auditing Database Client Environment Shield...");

    // Escenario: DATABASE_URL extraída de un secreto de GitHub no configurado (String vacío)
    let empty_url = "";
    let result = TursoClient::connect(empty_url, None).await;

    assert!(result.is_err(), "L3_CLIENT_FAULT: The client accepted an empty URL strata.");

    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("ENV_VAR_EMPTY"), "Error message should diagnose environment issues.");

    println!("   ✅ [SUCCESS]: Empty environment variable detected and rejected.");
}

#[tokio::test]
async fn certify_memory_anchor_persistence() {
    println!("   🧪 Auditing Memory Strata Anchor...");
    // Si este test no falla por 'Table not found', el ancla del V180.6 está funcionando.
    let client = TursoClient::connect("file::memory:?cache=shared", None).await.unwrap();
    let conn = client.get_connection().unwrap();

    let res = conn.execute("SELECT 1 FROM jobs LIMIT 0", ()).await;
    assert!(res.is_ok(), "L3_CLIENT_FAULT: Schema not visible after connection.");

    println!("   ✅ [SUCCESS]: Memory strata anchor verified.");
}

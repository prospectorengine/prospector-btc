// [tests/mirror/libs/infra/db_turso/archival_integrity.test.rs]

use prospector_infra_db::repositories::ArchivalRepository;
use prospector_infra_db::TursoClient;

#[tokio::test]
async fn certify_archival_buffer_and_retry_limits() {
    println!("\n⚖️  [PROVING_GROUNDS]: Auditing Archival Strata Persistence...");

    // 1. SETUP: In-memory con esquema nivelado
    let client = TursoClient::connect("file::memory:", None).await.unwrap();
    let repo = ArchivalRepository::new(client.clone());
    let conn = client.get_connection().unwrap();

    // 2. INJECT: Crear evento pendiente
    conn.execute(
        "INSERT INTO outbox_strategic (outbox_identifier, payload_json, target_stratum, status)
         VALUES ('EVT-001', '{}', 'TEST_STRATA', 'pending')",
        ()
    ).await.unwrap();

    // 3. EXECUTION: Validar drenaje
    let batch = repo.fetch_pending_outbox_batch(10).await.unwrap();
    assert_eq!(batch.len(), 1);
    assert_eq!(batch[0]["outbox_identifier"], "EVT-001");

    // 4. EXECUTION: Validar escalado de fallos
    repo.report_sync_failure("EVT-001").await.unwrap();
    let batch_after_failure = repo.fetch_pending_outbox_batch(10).await.unwrap();
    assert_eq!(batch_after_failure[0]["retry_count"], 1);

    println!("   ✅ Archival parity and retry strata certified.");
}

// [tests/mirror/libs/infra/db_turso/archival_nominal.test.rs]

use prospector_infra_db::repositories::ArchivalRepository;
use prospector_infra_db::TursoClient;

#[tokio::test]
async fn certify_archival_nominal_synchronization() {
    println!("\n⚖️  [PROVING_GROUNDS]: Auditing Archival Nominal Strata...");

    // SETUP: Ledger en memoria
    let client = TursoClient::connect("file::memory:", None).await.unwrap();
    let repo = ArchivalRepository::new(client.clone());
    let conn = client.get_connection().unwrap();

    // INJECT: Evento simulado
    conn.execute(
        "INSERT INTO outbox_strategic (outbox_identifier, payload_json, target_stratum, status, retry_count)
         VALUES ('TEST-FIX-001', '{}', 'TEST', 'pending', 0)",
        ()
    ).await.unwrap();

    // EXECUTION: La recuperación no debe fallar (Prueba de que la constante es correcta)
    let batch_result = repo.fetch_pending_outbox_batch(1).await;

    assert!(batch_result.is_ok(), "L3_DB_FAULT: The repository failed due to nominal mismatch.");
    let batch = batch_result.unwrap();
    assert_eq!(batch.len(), 1);
    assert_eq!(batch[0]["outbox_identifier"], "TEST-FIX-001");

    println!("   ✅ Nominal synchronization certified. Error E0432 eliminated.");
}

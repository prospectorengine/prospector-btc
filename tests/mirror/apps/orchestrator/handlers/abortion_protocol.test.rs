// INICIO DEL ARCHIVO [tests/mirror/apps/orchestrator/handlers/abortion_protocol.test.rs]
/**
 * =================================================================
 * APARATO: ABORTION PROTOCOL TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar la transición a estado 'aborted'.
 * =================================================================
 */

use prospector_infra_db::repositories::MissionRepository;
use prospector_infra_db::TursoClient;
use prospector_domain_models::work::AuditReport;

#[tokio::test]
async fn certify_mission_abortion() {
    let client = TursoClient::connect("file::memory:", None).await.unwrap();
    let repo = MissionRepository::new(client.clone());
    let conn = client.get_connection().unwrap();

    // 1. SETUP: Misión activa
    conn.execute(
        "INSERT INTO jobs (id, range_start, range_end, status, worker_id) VALUES ('M_ABORT', '0', '100', 'active', 'W_KILL')",
        ()
    ).await.unwrap();

    // 2. EXECUTION: Abortar
    let result = repo.abort_mission("M_ABORT", "W_KILL", "OOM_CRASH").await;
    assert!(result.is_ok(), "Abort operation failed");

    // 3. VALIDATION: Estado DB
    let mut rows = conn.query("SELECT status, audit_footprint_checkpoint FROM jobs WHERE id = 'M_ABORT'", ()).await.unwrap();
    let row = rows.next().await.unwrap().unwrap();

    let status: String = row.get(0).unwrap();
    let reason: String = row.get(1).unwrap();

    assert_eq!(status, "aborted");
    assert!(reason.contains("OOM_CRASH"));

    // 4. VALIDATION: Certificación posterior debe fallar gracefully
    let report = AuditReport {
        job_mission_identifier: "M_ABORT".into(),
        worker_node_identifier: "W_KILL".into(),
        total_wallets_audited: "0".into(),
        execution_duration_milliseconds: 0,
        final_mission_status: "completed".into(),
        audit_footprint_checkpoint: "".into(),
        completed_at_timestamp: "".into(),
        average_computational_efficiency: 0.0,
    };

    let cert_result = repo.certify_mission_completion(&report).await;
    assert!(cert_result.is_err(), "Should rely on error for control flow");

    // Verificamos que el error sea el específico mapeado en el handler
    let err_msg = cert_result.err().unwrap().to_string();
    assert!(err_msg.contains("MISSION_ABORTED"));

    println!("✅ ABORT_PROTOCOL: State transition and rejection certified.");
}
// FIN DEL ARCHIVO [tests/mirror/apps/orchestrator/handlers/abortion_protocol.test.rs]

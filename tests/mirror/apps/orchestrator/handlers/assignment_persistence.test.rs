// INICIO DEL ARCHIVO [tests/mirror/apps/orchestrator/handlers/assignment_persistence.test.rs]
/**
 * =================================================================
 * APARATO: ASSIGNMENT PERSISTENCE TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Validar atomicidad RAM-DB en el handshake.
 * =================================================================
 */

use prospector_orchestrator::state::AppState;
use prospector_orchestrator::handlers::swarm::SwarmHandshakeHandler;
use prospector_domain_models::work::{MissionRequestPayload, NodeHardwareCapacity, WorkOrder, SearchStrategy, TargetStrata};
use prospector_infra_db::TursoClient;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::post,
    Router,
};
use tower::ServiceExt;

#[tokio::test]
async fn certify_handshake_persists_worker_id() {
    // 1. SETUP
    let client = TursoClient::connect("file::memory:", None).await.unwrap();
    let state = AppState::new(client.clone());
    let conn = client.get_connection().unwrap();

    // Inyectar misión en cola
    state.mission_control.hydrate_queue(vec![WorkOrder {
        job_mission_identifier: "mission-001".into(),
        lease_duration_seconds: 600,
        strategy: SearchStrategy::Sequential { start_index_hexadecimal: "0".into(), end_index_hexadecimal: "100".into() },
        required_strata: TargetStrata::SatoshiEra,
    }]);

    // Insertar misión en DB (estado 'queued')
    conn.execute("INSERT INTO jobs (id, range_start, range_end, status) VALUES ('mission-001', '0', '100', 'queued')", ()).await.unwrap();

    // 2. EXECUTION: Disparar handshake
    let app = Router::new()
        .route("/acquire", post(SwarmHandshakeHandler::negotiate_mission_assignment_handshake))
        .with_state(state);

    let payload = MissionRequestPayload {
        worker_id: "worker-test-unit".into(),
        hardware_capacity: NodeHardwareCapacity { ram_available_mb: 8000, cpu_cores: 4, supports_avx2: true }
    };

    let req = Request::builder()
        .method("POST")
        .uri("/acquire")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&payload).unwrap()))
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // 3. VERIFICATION: Consultar DB para asegurar que worker_id se guardó
    let mut rows = conn.query("SELECT status, worker_id FROM jobs WHERE id = 'mission-001'", ()).await.unwrap();
    let row = rows.next().await.unwrap().unwrap();

    let status: String = row.get(0).unwrap();
    let worker_id: String = row.get(1).unwrap();

    assert_eq!(status, "active", "El estado en DB debe ser 'active'");
    assert_eq!(worker_id, "worker-test-unit", "El worker_id debe estar persistido");

    println!("✅ HANDSHAKE: Atomic persistence certified.");
}
// FIN DEL ARCHIVO [tests/mirror/apps/orchestrator/handlers/assignment_persistence.test.rs]

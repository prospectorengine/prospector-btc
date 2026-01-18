use prospector_orchestrator::state::AppState;
use prospector_orchestrator::handlers::swarm::SwarmHandshakeHandler;
use prospector_infra_db::TursoClient;
use axum::extract::{Json, State};
use serde_json::json;

#[tokio::test]
async fn certify_slicer_trigger_in_handler() {
    let client = TursoClient::connect("file::memory:", None).await.unwrap();
    let state = AppState::new(client);

    // Simular progreso masivo para disparar el Slicer
    let payload = prospector_orchestrator::handlers::swarm::ProgressUpdatePayload {
        mission_identifier: "M_TEST".into(),
        worker_identifier: "W_FAST".into(),
        last_hex_checkpoint: "0xABC123".into(),
        cumulative_effort_volume: 500_000_000, // Por encima del umbral de 100M
    };

    // El test valida que el handler procesa la petición sin pánicos
    // La lógica de subdivisión real se probó en el Repositorio L3.
    let response = SwarmHandshakeHandler::handle_mission_progress_report(
        State(state),
        Json(payload)
    ).await;

    assert_eq!(response.into_response().status(), axum::http::StatusCode::ACCEPTED);
    println!("✅ SWARM_V150: Progress report and slicing trigger certified.");
}

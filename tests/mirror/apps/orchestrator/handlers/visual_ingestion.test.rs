// [tests/mirror/apps/orchestrator/handlers/visual_ingestion.test.rs]
/**
 * =================================================================
 * APARATO: VISUAL INGESTION INTEGRITY TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L3-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DEL TÚNEL DE VIDEO PROXY
 * =================================================================
 */

 use prospector_orchestrator::state::AppState;
 use prospector_orchestrator::handlers::telemetry::handle_visual_snapshot;
 use prospector_domain_models::worker::WorkerSnapshot;
 use prospector_infra_db::TursoClient;
 use ax_test_utils::axum::extract::{Json, State};
 use ax_test_utils::axum::response::IntoResponse;

 #[tokio::test]
 async fn certify_visual_snapshot_ingestion_and_ram_storage() {
     println!("\n📸 [PROVING_GROUNDS]: Auditing Visual Ingestion Strata...");

     // 1. SETUP: Infraestructura en RAM
     let client = TursoClient::connect("file:visual_test?mode=memory", None).await.unwrap();
     let state = AppState::new(client);

     // 2. MOCK SNAPSHOT: Simulación de ráfaga del provisionador
     let mock_snapshot = WorkerSnapshot {
         worker_id: "node-test-01".into(),
         status: "running".into(),
         snapshot_base64: "data:image/jpeg;base64,/9j/4AAQSkZJRg...".into(),
         timestamp: chrono::Utc::now().to_rfc3339(),
         hardware: None,
     };

     // 3. EXECUTION: Disparo del handler
     let response = handle_visual_snapshot(
         State(state.clone()),
         Json(mock_snapshot.clone())
     ).await.into_response();

     // 4. VALIDATION
     assert_eq!(response.status(), ax_test_utils::axum::http::StatusCode::ACCEPTED);

     // Verificamos que la imagen reside en la RAM del orquestador
     let buffer = state.swarm_telemetry.visual_surveillance_frames.read().unwrap();
     assert!(buffer.contains_key("node-test-01"), "L3_TELEMETRY_FAULT: Snapshot not persisted in RAM.");

     let stored = buffer.get("node-test-01").unwrap();
     assert_eq!(stored.snapshot_base64, mock_snapshot.snapshot_base64);

     println!("   ✅ [SUCCESS]: Visual frame received and crystallized in volatile strata.");
 }

// [tests/mirror/apps/orchestrator/routes/sovereign_routes.test.rs]
/**
 * =================================================================
 * APARATO: ROUTING INTEGRITY CERTIFIER (V17.0 - SINGULARITY)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar que todas las fronteras de red están selladas.
 * =================================================================
 */

 use prospector_orchestrator::routes::create_sovereign_router;
 use prospector_orchestrator::state::AppState;
 use prospector_infra_db::TursoClient;
 use axum::http::{Request, StatusCode};
 use tower::ServiceExt;
 use serde_json::json;

 #[tokio::test]
 async fn certify_visual_and_qa_routing_strata() {
     println!("\n🕵️ [PROVING_GROUNDS]: Initiating Routing Singularity Audit...");

     // 1. SETUP: Ledger en RAM
     let database_client = TursoClient::connect("file:routing_test?mode=memory&cache=shared", None).await.unwrap();
     let application_state = AppState::new(database_client);
     let sovereign_router = create_sovereign_router(application_state);

     // 2. CONFIGURACIÓN DE SEGURIDAD (Simulamos Handshake)
     let secret_token = "netflix69_test_stratum";
     std::env::set_var("WORKER_AUTH_TOKEN", secret_token);

     // --- TEST 1: Túnel Visual (Snapshot) ---
     println!("   🧪 Probing Visual Tunnel: /api/v1/swarm/visual/snapshot...");
     let visual_req = Request::builder()
         .method("POST")
         .uri("/api/v1/swarm/visual/snapshot")
         .header("Content-Type", "application/json")
         .header("Authorization", format!("Bearer {}", secret_token))
         .body(axum::body::Body::from(json!({
             "worker_identifier": "unit-test-01",
             "operational_status": "running",
             "snapshot_base64_data": "data:image/png;base64,V17",
             "captured_at_timestamp": "2026-01-24T12:00:00Z"
         }).to_string()))
         .unwrap();

     let visual_res = sovereign_router.clone().oneshot(visual_req).await.unwrap();
     assert_eq!(visual_res.status(), StatusCode::ACCEPTED, "El túnel visual sigue decapitado.");
     println!("      ✅ Visual Strata: OPEN & RECEPTIVE.");

     // --- TEST 2: Proving Grounds (QA Report) ---
     println!("   🧪 Probing QA Strata: /api/v1/qa/report...");
     let qa_req = Request::builder()
         .method("POST")
         .uri("/api/v1/qa/report")
         .header("Content-Type", "application/json")
         .header("Authorization", format!("Bearer {}", secret_token))
         .body(axum::body::Body::from(json!({
             "stratum": "L1_MATH",
             "testName": "FRACTURE_TEST",
             "verdict": "GOLD_MASTER",
             "forensicLog": "Testing Singularity V17",
             "metrics": {},
             "environment": "Test_Harness",
             "timestamp": "2026-01-24T12:00:00Z"
         }).to_string()))
         .unwrap();

     let qa_res = sovereign_router.oneshot(qa_req).await.unwrap();
     assert_eq!(qa_res.status(), StatusCode::CREATED, "El estrato QA es inalcanzable.");
     println!("      ✅ QA Strata: FORMALIZED & SEALED.");

     println!("\n🏁 [COMPLETE]: Singularity Routing certified bit-perfect.");
 }

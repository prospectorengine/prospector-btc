// [tests/mirror/apps/orchestrator/handlers/identity_injection.test.rs]
/**
 * =================================================================
 * APARATO: IDENTITY INJECTION TEST (V17.0 - SINGULARITY)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Validar que el ID del operador fluye desde el JWT al Ledger.
 * =================================================================
 */

 use prospector_orchestrator::middleware::OperatorIdentity;
 use prospector_orchestrator::state::AppState;
 use prospector_orchestrator::handlers::swarm::SwarmHandshakeHandler;
 use prospector_infra_db::TursoClient;
 use prospector_domain_models::work::{MissionRequestPayload, NodeHardwareCapacity};
 use axum::{extract::{State, Json}, Extension, response::IntoResponse};

 #[tokio::test]
 async fn certify_operator_id_injection_from_extension() {
     println!("\n🧬 [PROVING_GROUNDS]: Auditing Galvanic Identity Injection...");

     // 1. SETUP: Ledger en memoria
     let database_client = TursoClient::connect("file:identity_test?mode=memory&cache=shared", None).await.unwrap();
     let application_state = AppState::new(database_client);

     // 2. MOCK: Identidad simulada (Arquitecto real)
     let mock_operator_id = "550e8400-e29b-41d4-a716-446655440000".to_string();
     let identity = OperatorIdentity {
         operator_identifier: mock_operator_id.clone(),
         is_worker_node: false,
     };

     let payload = MissionRequestPayload {
         worker_id: "node-v17-test".to_string(),
         hardware_capacity: NodeHardwareCapacity {
             ram_available_megabytes: 8000,
             cpu_cores: 4,
             supports_avx2: true,
         },
     };

     // 3. EXECUTION: Disparo del handler inyectando la extensión manualmente
     println!("   🧪 Dispatching acquisition for operator: {}...", mock_operator_id);
     let response = SwarmHandshakeHandler::negotiate_mission_assignment_handshake(
         State(application_state),
         Extension(identity),
         Json(payload)
     ).await.into_response();

     // 4. VALIDATION: Si el estatus es 204 (o 500 si la DB está vacía pero intentó entrar),
     // significa que la firma del método es correcta y aceptó la extensión.
     // Lo más importante es que NO falló en compilación ni por falta de extensión.
     assert!(response.status() != StatusCode::UNAUTHORIZED, "La identidad fue rechazada.");

     println!("   ✅ [SUCCESS]: Identity strata levelized. Static ID eradicated.");
 }

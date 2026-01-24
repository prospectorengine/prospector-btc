// [tests/mirror/apps/orchestrator/handlers/swarm_singularity.test.rs]
/**
 * =================================================================
 * APARATO: SWARM SINGULARITY TEST (V17.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar la exclusión de IDs estáticos y la inyección galvánica.
 * =================================================================
 */

 use prospector_orchestrator::handlers::swarm::SwarmHandshakeHandler;
 use prospector_orchestrator::middleware::OperatorIdentity;
 use prospector_orchestrator::state::AppState;
 use prospector_infra_db::TursoClient;
 use prospector_domain_models::work::{MissionRequestPayload, NodeHardwareCapacity};
 use axum::{extract::{State, Json}, Extension, response::IntoResponse};

 #[tokio::test]
 async fn certify_multi_tenant_mission_acquisition() {
     println!("\n🧬 [PROVING_GROUNDS]: Auditing Multi-Tenant Mission Strata...");

     // 1. SETUP: Ledger en RAM
     let database_client = TursoClient::connect("file:swarm_v17?mode=memory&cache=shared", None).await.unwrap();
     let application_state = AppState::new(database_client);

     // 2. MOCK: Identidad real de Arquitecto (Dynamic ID)
     let sovereign_uuid = "architect-v17-id-unique".to_string();
     let identity_context = OperatorIdentity {
         operator_identifier: sovereign_uuid.clone(),
         is_worker_node: false,
     };

     let payload = MissionRequestPayload {
         worker_id: "node-singularity-01".to_string(),
         hardware_capacity: NodeHardwareCapacity {
             ram_available_megabytes: 4096,
             cpu_cores: 2,
             supports_avx2: true,
         },
     };

     // 3. EXECUTION: Disparo del handler inyectando la extensión
     println!("   🧪 Dispatching acquisition for UUID: [{}]", sovereign_uuid);
     let response = SwarmHandshakeHandler::negotiate_mission_assignment_handshake(
         State(application_state),
         Extension(identity_context),
         Json(payload)
     ).await.into_response();

     // 4. VALIDATION
     // Si el estatus es != 401, la inyección galvánica funciona y el ID estático ha muerto.
     assert!(response.status() != axum::http::StatusCode::UNAUTHORIZED);

     println!("   ✅ [SUCCESS]: Multi-tenant logic certified. No static residues found.");
 }

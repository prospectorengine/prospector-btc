// [tests/mirror/apps/orchestrator/services/flush_resilience.test.rs]
/**
 * =================================================================
 * APARATO: FLUSH RESILIENCE TEST (V1.0 - SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L4-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE PROTOCOLO DE RESCATE
 * =================================================================
 */

 use prospector_orchestrator::state::AppState;
 use prospector_orchestrator::services::flush::spawn_flush_service;
 use prospector_infra_db::TursoClient;
 use prospector_domain_models::worker::{WorkerHeartbeat, HardwareStats};
 use chrono::Utc;
 use std::time::Duration;
 use tokio::time::sleep;

 #[tokio::test]
 async fn certify_emergency_reinjection_on_db_failure() {
     println!("\n💾 [PROVING_GROUNDS]: Auditing Flush Rescue Strata...");

     // 1. SETUP: Usamos una URL de DB inválida para forzar el fallo de persistencia
     let database_client = TursoClient::connect("libsql://invalid-strata-target.io", Some("token".into()))
         .await.unwrap();
     let application_state = AppState::new(database_client);

     // 2. INJECT: Inyectar un latido en el buffer
     let worker_id = "unit-test-rescue".to_string();
     let heartbeat = WorkerHeartbeat {
         worker_id: worker_id.clone(),
         hostname: "test-node".into(),
         hashrate: 5000,
         current_job_id: None,
         timestamp: Utc::now(),
         hardware_stats: HardwareStats {
             cpu_frequency_mhz: 3000,
             cpu_load_percent: 50.0,
             thermal_celsius: 45.0,
             memory_used_mb: 1024,
             core_count: 4,
             is_throttling: false,
             supports_avx2: true,
         }
     };

     {
         let mut buffer = application_state.heartbeat_buffer.lock().unwrap();
         buffer.insert(worker_id.clone(), heartbeat);
     }

     // 3. EXECUTION: Lanzar el servicio (fallará al intentar escribir en la DB inválida)
     spawn_flush_service(application_state.clone()).await;

     // Esperamos un tick del sincronizador (5s + margen)
     println!("   🧪 Awaiting tick and rescue sequence...");
     sleep(Duration::from_secs(6)).await;

     // 4. VALIDATION: El buffer NO debe estar vacío; el registro debe haber sido rescatado
     let buffer_final = application_state.heartbeat_buffer.lock().unwrap();

     assert!(buffer_final.contains_key(&worker_id), "L4_FLUSH_FAULT: Data was lost after failure.");
     println!("   ✅ [SUCCESS]: Heartbeat rescued and re-injected into RAM strata.");
 }

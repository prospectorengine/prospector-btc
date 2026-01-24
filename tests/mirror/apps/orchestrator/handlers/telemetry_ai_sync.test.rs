// [tests/mirror/apps/orchestrator/handlers/telemetry_ai_sync.test.rs]
/**
 * =================================================================
 * APARATO: TELEMETRY AI SYNC TEST (V17.0 - SINGULARITY)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar que el pulso de telemetría activa al AI Cortex.
 * =================================================================
 */

 use prospector_orchestrator::state::AppState;
 use prospector_orchestrator::handlers::telemetry::spawn_telemetry_loop;
 use prospector_infra_db::TursoClient;
 use prospector_domain_models::worker::{WorkerHeartbeat, HardwareStats};
 use std::time::Duration;
 use tokio::time::sleep;

 #[tokio::test]
 async fn certify_telemetry_loop_triggers_ai_logic() {
     println!("\n🧠 [PROVING_GROUNDS]: Auditing Telemetry-to-AI Synapse...");

     // 1. SETUP: Estado con DB en RAM
     let database_client = TursoClient::connect("file:tele_ai_test?mode=memory&cache=shared", None).await.unwrap();
     let application_state = AppState::new(database_client);
     let mut neural_subscriber = application_state.event_bus.subscribe();

     // 2. INJECT: Simulación de un nodo sobrecalentado
     let hot_node = WorkerHeartbeat {
         worker_id: "overheated-node".into(),
         hostname: "colab-instance".into(),
         hashrate: 10_000,
         current_job_id: None,
         timestamp: chrono::Utc::now(),
         hardware_stats: HardwareStats {
             cpu_frequency_mhz: 2500,
             cpu_load_percent: 99.0,
             thermal_celsius: 95.0, // Punto crítico para la IA
             memory_used_mb: 4096,
             core_count: 2,
             is_throttling: true,
         }
     };
     application_state.swarm_telemetry.synchronize_heartbeat(hot_node);

     // 3. EXECUTION: Arrancar el loop (5s)
     spawn_telemetry_loop(application_state.clone()).await;

     // 4. VALIDATION: Esperar la alerta del AI Cortex en el Bus
     println!("   🧪 Awaiting autonomic verdict from L9...");
     let mut alert_found = false;

     // Timeout de 7s para capturar el tick de 5s
     let timeout = sleep(Duration::from_secs(7));
     tokio::pin!(timeout);

     loop {
         tokio::select! {
             event = neural_subscriber.recv() => {
                 if let Ok(prospector_domain_models::telemetry::RealTimeEvent::SystemLog(log)) = event {
                     if log.stratum == "L9_AI_CORTEX" {
                         alert_found = true;
                         println!("      ✅ AI_ACK: {}", log.message);
                         break;
                     }
                 }
             }
             _ = &mut timeout => break,
         }
     }

     assert!(alert_found, "L9_SYNAPSE_FAULT: El AI Cortex no emitió alerta ante el sobrecalentamiento.");
     println!("   ✅ [SUCCESS]: Telemetry-to-AI loop certified bit-perfect.");
 }

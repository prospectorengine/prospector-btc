// [tests/mirror/apps/orchestrator/state/cortex_wiring.test.rs]
/**
 * =================================================================
 * APARATO: AI CORTEX WIRING TEST (V17.0 - SINGULARITY)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar que la IA puede interrogar al estado maestro.
 * =================================================================
 */

 use prospector_orchestrator::state::AppState;
 use prospector_infra_db::TursoClient;
 use prospector_domain_ai_cortex::lib::{TelemetrySnapshot, CognitiveVerdict};

 #[tokio::test]
 async fn certify_ai_cortex_wiring_in_app_state() {
     println!("\n🧠 [PROVING_GROUNDS]: Auditing AI Cortex Neural Wiring...");

     // 1. SETUP: Inicialización de AppState con Ledger simulado
     let database_client = TursoClient::connect("file:wiring_test?mode=memory", None).await.unwrap();
     let application_state = AppState::new(database_client);

     // 2. ESCENARIO: Simulación de ráfaga de telemetría degradada
     let degraded_snapshot = TelemetrySnapshot {
         current_hashrate: 5_000_000,
         cpu_temperature_celsius: 88.5, // Sobre umbral L9
         cpu_load_percentage: 99.0,
         timestamp_ms: 1737700000000,
     };

     // 3. EXECUTION: Interrogación a través del método nivelado en AppState
     println!("   🧪 Probing AI decision through State Hub...");
     let veredicto = application_state.evaluate_swarm_health(&degraded_snapshot);

     // 4. VALIDATION: El veredicto debe ser procesado por el miembro 'ai_cortex' inyectado
     assert_eq!(veredicto, CognitiveVerdict::OptimizationRequired, "L1_APP: AI Hub failed to process the strata pulse.");

     println!("   ✅ [SUCCESS]: Neural wiring certified. AI Cortex is active in the Hub.");
 }

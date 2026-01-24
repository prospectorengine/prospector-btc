use prospector_domain_ai_cortex::decision_engine::DecisionEngine;
use prospector_domain_ai_cortex::telemetry_observer::TelemetryObserver;
use prospector_domain_ai_cortex::optimization_controller::OptimizationController;
use prospector_domain_ai_cortex::lib::CognitiveVerdict;
use prospector_domain_models::telemetry::SystemMetrics;

#[test]
fn certify_full_cognitive_loop() {
    println!("\n🧠 [PROVING_GROUNDS]: Auditing AI Cortex Integration...");

    // 1. Aferente: Ingesta
    let metrics = SystemMetrics {
        cumulative_global_hashrate: 1000,
        timestamp_ms: 12345,
        active_nodes_count: 1,
        active_missions_in_flight: 1
    };
    let snapshot = TelemetryObserver::crystallize_snapshot(&metrics, 90.0, 99.0);

    // 2. Proceso: Inferencia
    let verdict = DecisionEngine::evaluate_node_efficiency(&snapshot);
    assert_eq!(verdict, CognitiveVerdict::OptimizationRequired);

    // 3. Eferente: Acción
    let directive = OptimizationController::generate_directive("unit-01", &verdict);
    assert!(directive.is_some());
    assert_eq!(directive.unwrap().recommended_batch_size, 512);

    println!("   ✅ [SUCCESS]: Cognitive loop from perception to action certified.");
}

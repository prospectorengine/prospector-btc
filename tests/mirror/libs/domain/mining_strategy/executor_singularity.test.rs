// [tests/mirror/libs/domain/mining_strategy/executor_luno_dispatch.test.rs]
/**
 * =================================================================
 * APARATO: EXECUTOR LUNO DISPATCH CERTIFIER (V17.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar el flujo completo de la misión Luno 2014.
 * =================================================================
 */

 use std::sync::Arc;
 use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
 use prospector_core_probabilistic::sharded::ShardedFilter;
 use prospector_core_math::prelude::*;
 use prospector_domain_models::work::{WorkOrder, SearchStrategy, TargetStrata};
 use prospector_domain_strategy::{StrategyExecutor, FindingHandler};

 struct MockSiphon;
 impl FindingHandler for MockSiphon {
     fn on_finding(&self, _: String, _: SafePrivateKey, _: String) {}
 }

 #[test]
 fn certify_luno_polymorphic_dispatch_and_report() {
     println!("\n🧬 [PROVING_GROUNDS]: Auditing Executor Luno Dispatch Strata...");

     // 1. SETUP: Orden de misión Luno real
     let mission_order = WorkOrder {
         job_mission_identifier: "LUNO-CERT-2026".into(),
         lease_duration_seconds: 60,
         strategy: SearchStrategy::LunoBlockchainForensic {
             start_timestamp_milliseconds: 1417392000000,
             end_timestamp_milliseconds: 1417392000099, // 100ms ráfaga
         },
         required_strata: TargetStrata::VulnerableLegacy,
     };

     let filter = ShardedFilter::new(1, 100, 0.01);
     let effort = Arc::new(AtomicU64::new(0));
     let stop = Arc::new(AtomicBool::new(false));
     let handler = MockSiphon;

     // 2. EXECUTION: Disparo del despachador maestro
     let report = StrategyExecutor::execute_mission_sequence(
         &mission_order,
         &filter,
         stop,
         effort.clone(),
         "unit-v17-luno".into(),
         &handler,
         None
     );

     // 3. VALIDATION
     assert_eq!(report.job_mission_identifier, "LUNO-CERT-2026");
     assert_eq!(effort.load(Ordering::SeqCst), 100, "El volumen de esfuerzo Luno es incorrecto.");
     assert!(report.audit_footprint_checkpoint.contains("Temporal_Audit_Complete"));

     println!("   ✅ [SUCCESS]: Luno mission dispatched and certified via StrategyExecutor.");
 }

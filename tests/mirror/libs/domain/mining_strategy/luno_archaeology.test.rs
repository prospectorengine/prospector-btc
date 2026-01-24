// [tests/mirror/libs/domain/mining_strategy/luno_archaeology.test.rs]
/**
 * =================================================================
 * APARATO: LUNO ARCHAEOLOGY INTEGRATION TEST (V17.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar que el despacho L2 activa el motor temporal real.
 * =================================================================
 */

 use prospector_domain_strategy::{ForensicArchaeologyEngine, FindingHandler};
 use prospector_core_probabilistic::sharded::ShardedFilter;
 use prospector_core_math::prelude::*;
 use std::sync::Arc;
 use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};

 struct MockCollisionSiphon {
     pub count: Arc<AtomicU64>,
 }
 impl FindingHandler for MockCollisionSiphon {
     fn on_finding(&self, _addr: String, _pk: SafePrivateKey, _meta: String) {
         self.count.fetch_add(1, Ordering::SeqCst);
     }
 }

 #[test]
 fn certify_luno_reconstruction_dispatch() {
     println!("\n🧬 [PROVING_GROUNDS]: Auditing Luno 2014 Strata Dispatch...");

     let filter = ShardedFilter::new(1, 100, 0.01);
     let stop = AtomicBool::new(false);
     let effort = Arc::new(AtomicU64::new(0));
     let siphon = MockCollisionSiphon { count: Arc::new(AtomicU64::new(0)) };

     // EXECUTION: Disparo con rango temporal real de 100ms
     let result = ForensicArchaeologyEngine::execute_forensic_scan(
         "Luno_Blockchain_2014",
         &filter,
         &stop,
         effort.clone(),
         &siphon,
         Some((1417392000000, 1417392000099)) // Ventana de prueba
     );

     // VALIDATION
     assert!(result.contains("Temporal_Audit_Complete"));
     assert_eq!(effort.load(Ordering::SeqCst), 100, "El motor no procesó los 100ms solicitados.");

     println!("   ✅ [SUCCESS]: Luno 2014 engine is no longer a placeholder. Logic certified.");
 }

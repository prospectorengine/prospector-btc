// [tests/mirror/libs/domain/mining_strategy/engines/kangaroo_lambda.test.rs]
/**
 * APARATO: KANGAROO RUNNER STRATEGY TEST (V25.1)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar la orquestación L2 -> L1 de los Canguros.
 */

use prospector_domain_strategy::{KangarooRunner, FindingHandler};
use prospector_core_math::prelude::*;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

struct MockCollisionSiphon {
    pub found: Arc<AtomicBool>,
}

impl FindingHandler for MockCollisionSiphon {
    fn on_finding(&self, addr: String, _: SafePrivateKey, _: String) {
        println!("      🎯 [TEST_MATCH]: Address recovered: {}", addr);
        self.found.store(true, Ordering::SeqCst);
    }
}

#[test]
fn certify_kangaroo_runner_orchestration() {
    println!("\n🦘 [PROVING_GROUNDS]: Auditing Kangaroo Strategy Runner...");

    // 1. SETUP: Vector conocido (k=500)
    let target_pub = "026ad9b14a7453b7488daa0c6acbc258b1506f52c441c7c465474c1a564394ff";
    let stop = Arc::new(AtomicBool::new(false));
    let effort = Arc::new(AtomicU64::new(0));
    let siphon = MockCollisionSiphon { found: Arc::new(AtomicBool::new(false)) };

    // 2. EXECUTION: Disparar Runner
    KangarooRunner::run(
        target_pub,
        "0000000000000000000000000000000000000000000000000000000000000000",
        1000, // Ancho de búsqueda
        stop,
        effort,
        &siphon
    );

    // 3. VALIDATION
    assert!(siphon.found.load(Ordering::SeqCst), "L2_KANGAROO_FAULT: Runner failed to orchestrate resolution.");
    println!("   ✅ [SUCCESS]: Kangaroo Strategy Runner levelized with L1 Math.");
}

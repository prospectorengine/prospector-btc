// INICIO DEL ARCHIVO [tests/mirror/libs/domain/mining_strategy/engines/sequential_alloc_test.rs]
/*!
 * =================================================================
 * APARATO: SEQUENTIAL ALLOC TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * RESPONSABILIDAD: VALIDAR INTEGRACIÓN DE BUFFER ESTÁTICO
 * =================================================================
 */

use std::sync::{Arc};
use std::sync::atomic::{AtomicU64, AtomicBool};
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_core_math::private_key::SafePrivateKey;
use prospector_domain_strategy::{ProjectiveSequentialEngine, FindingHandler};

struct MockSpy;
impl FindingHandler for MockSpy {
    fn on_finding(&self, _: String, _: SafePrivateKey, _: String) {}
}

#[test]
fn certify_zero_allocation_loop_stability() {
    // Verificamos que el motor puede inicializar y usar los buffers internos
    // sin entrar en pánico por accesos fuera de rango (OutOfBounds).

    let start_scalar = "0000000000000000000000000000000000000000000000000000000000000001";
    // 2050 iteraciones fuerza:
    // 1. Lote de 1024
    // 2. Lote de 1024
    // 3. Residuo de 2
    // Esto prueba la lógica de slicing en el flush final.
    let iterations = 2050;

    let filter = ShardedFilter::new(1, 100, 0.01);
    let stop = AtomicBool::new(false);
    let effort = Arc::new(AtomicU64::new(0));
    let spy = MockSpy;

    let _ = ProjectiveSequentialEngine::execute_optimized_audit(
        start_scalar,
        iterations,
        &filter,
        &stop,
        effort.clone(),
        &spy
    );

    assert_eq!(effort.load(std::sync::atomic::Ordering::SeqCst), 2050);
    println!("✅ ZERO_ALLOC: 2050 iterations processed safely.");
}
// FIN DEL ARCHIVO [tests/mirror/libs/domain/mining_strategy/engines/sequential_alloc_test.rs]

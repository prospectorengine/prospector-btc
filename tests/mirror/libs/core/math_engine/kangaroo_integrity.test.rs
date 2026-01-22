// [tests/mirror/libs/core/math_engine/kangaroo_integrity.test.rs]
/**
 * APARATO: KANGAROO SOLVER INTEGRITY TEST (V20.1 - DYNAMIC MASK)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar la resolución ECDLP bajo ráfagas dinámicas.
 */

use prospector_core_math::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU64};

#[test]
fn certify_kangaroo_resolution_with_dynamic_bitmask() {
    println!("\n🦘 [PROVING_GROUNDS]: Auditing Kangaroo Solver V20.1 (Dynamic Mask)...");

    // 1. SETUP: Problema conocido (Escalar = 7777)
    let target_private_hex = "0000000000000000000000000000000000000000000000000000000000001E61";
    let target_private_key = SafePrivateKey::from_bytes(&hex::decode(target_private_hex).unwrap()).unwrap();
    let target_public_key = SafePublicKey::from_private(&target_private_key);

    // Configuración con máscara de 4 bits (1/16 trampas)
    let tactical_config = KangarooConfig {
        start_scalar_bytes: [0u8; 32],
        search_width_magnitude: 20000,
        distinguished_point_bitmask: 0x0F, // Detección acelerada para el test
        maximum_traps_capacity_limit: 5000,
    };

    let stop_signal = AtomicBool::new(false);
    let effort_counter = AtomicU64::new(0);

    // 2. EXECUTION: Resolución bit-perfecta
    println!("   🧪 Searching for scalar 0x1E61 in range 20,000...");
    let result = KangarooSolver::solve_discrete_logarithm(
        &target_public_key,
        &tactical_config,
        &stop_signal,
        &effort_counter
    ).expect("SOLVER_COLLAPSE");

    // 3. VALIDATION
    assert!(result.is_some(), "L1_KANGAROO_FAULT: Target scalar was not recovered.");
    let recovered_hex = hex::encode(result.unwrap());
    assert_eq!(recovered_hex, target_private_hex, "DATA_CORRUPTION: Recovered scalar mismatch.");

    println!("   ✅ [SUCCESS]: Scalar recovered in {} leaps.", effort_counter.into_inner());
}

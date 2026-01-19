// [tests/mirror/libs/core/math_engine/kangaroo_integrity.test.rs]
/**
 * =================================================================
 * APARATO: KANGAROO SOLVER TEST (V1.1 - SYNCED)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * RESPONSABILIDAD: CERTIFICACIÓN DE RESOLUCIÓN ECDLP EN RANGO CORTO
 * =================================================================
 */

use prospector_core_math::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU64};

#[test]
fn certify_kangaroo_short_range_resolution() {
    println!("\n🦘 [PROVING_GROUNDS]: Auditing Kangaroo Solver (Nominal Sync)...");

    // 1. SETUP: Crear un problema conocido (Clave privada = 5000)
    let known_private_scalar_hex = "0000000000000000000000000000000000000000000000000000000000001388";
    let private_key = SafePrivateKey::from_bytes(&hex::decode(known_private_scalar_hex).unwrap()).unwrap();
    let public_key = SafePublicKey::from_private(&private_key);

    let config = KangarooConfig {
        start_scalar: [0u8; 32],
        search_width: 10000,
        distinguished_point_mask: 0x03, // Frecuencia alta para test rápido
        maximum_traps_capacity: 1000,
    };

    let stop_signal = AtomicBool::new(false);
    let effort = AtomicU64::new(0);

    // 2. EXECUTION: Resolver
    println!("   🧪 Attempting to resolve discrete logarithm for scalar 5000...");
    let result = KangarooSolver::solve_discrete_logarithm(
        &public_key,
        &config,
        &stop_signal,
        &effort
    ).unwrap();

    // 3. VALIDATION
    assert!(result.is_some(), "L1_KANGAROO_FAULT: Failed to resolve known point.");
    let recovered_hex = hex::encode(result.unwrap());
    assert_eq!(recovered_hex, known_private_scalar_hex, "DATA_CORRUPTION: Recovered scalar mismatch.");

    println!("   ✅ [SUCCESS]: Kangaroo resolved point in {} leaps.", effort.into_inner());
}

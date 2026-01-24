// [tests/mirror/libs/core/math_engine/kangaroo_integrity.test.rs]
/**
 * =================================================================
 * APARATO: KANGAROO SOLVER INTEGRITY TEST (V22.0 - GOLD MASTER)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L1-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE RESOLUCIÓN ECDLP BIT-PERFECT
 * =================================================================
 */

 use prospector_core_math::prelude::*;
 use std::sync::atomic::{AtomicBool, AtomicU64};

 #[test]
 fn certify_kangaroo_resolution_logic_v22() {
     println!("\n🦘 [PROVING_GROUNDS]: Auditing Kangaroo Solver V22.0...");

     // 1. SETUP: Definimos un problema de logaritmo discreto conocido
     // Escalar objetivo: 0xDEAD (57005)
     let target_hex = "000000000000000000000000000000000000000000000000000000000000DEAD";
     let target_priv = SafePrivateKey::from_bytes(&hex::decode(target_hex).unwrap()).unwrap();
     let target_pub = SafePublicKey::from_private(&target_priv);

     // Configuración táctica: rango de búsqueda de 100,000 llaves
     let config = KangarooConfig {
         start_scalar_bytes: [0u8; 32],
         search_width_magnitude: 100000,
         distinguished_point_bitmask: 0x07, // 1/8 densidad de trampas
         maximum_traps_capacity_limit: 1000,
     };

     let stop_signal = AtomicBool::new(false);
     let effort = AtomicU64::new(0);

     // 2. EXECUTION: Intento de resolución paralela
     println!("   🧪 Attempting to resolve scalar 0xDEAD in range [0, 100,000]...");
     let result = KangarooSolver::solve_discrete_logarithm(
         &target_pub,
         &config,
         &stop_signal,
         &effort
     ).expect("SOLVER_PANIC");

     // 3. VALIDATION: Verificación bit-perfecta
     assert!(result.is_some(), "L1_KANGAROO_FAULT: Resolution failed to converge.");
     let resolved_hex = hex::encode(result.unwrap());
     assert_eq!(resolved_hex, target_hex, "DATA_CORRUPTION: Resolved scalar mismatch.");

     println!("   ✅ [SUCCESS]: Scalar 0xDEAD recovered successfully.");
     println!("   📊 Effort: {} leaps recorded.", effort.into_inner());
 }

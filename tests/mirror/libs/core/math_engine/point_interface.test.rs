// [tests/mirror/libs/core/math_engine/point_interface.test.rs]
/**
 * =================================================================
 * APARATO: POINT INTERFACE CERTIFIER (V1.0 - SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar la existencia de from_private y paridad k*G.
 * =================================================================
 */

use prospector_core_math::prelude::*;

#[test]
fn certify_from_private_interface_and_window_parity() {
    println!("\n📐 [PROVING_GROUNDS]: Auditing JacobianPoint Public Interface...");

    // 1. SETUP: Escalar k=1 (Punto G)
    let mut scalar_one_bytes = [0u8; 32];
    scalar_one_bytes[31] = 1;
    let private_key = SafePrivateKey::from_bytes(&scalar_one_bytes).unwrap();

    // 2. EXECUTION: Derivación vía from_private (Nuevo método)
    // El test falla si el método no existe (Error E0599 resuelto)
    let point_via_private = JacobianPoint::from_private(&private_key);
    let point_via_window = JacobianPoint::from_private_scalar_windowed(&scalar_one_bytes);

    // 3. VALIDATION: Paridad interna
    assert_eq!(point_via_private.x, point_via_window.x, "L1_INTERFACE_FAULT: Method drift.");
    assert!(!point_via_private.is_infinity);

    println!("   ✅ [SUCCESS]: Method 'from_private' is active and synced with window engine.");
}

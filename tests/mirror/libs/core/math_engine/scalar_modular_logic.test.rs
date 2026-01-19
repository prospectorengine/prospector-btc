// [tests/mirror/libs/core/math_engine/scalar_modular_logic.test.rs]
/**
 * =================================================================
 * APARATO: SCALAR MODULAR LOGIC TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * RESPONSABILIDAD: CERTIFICAR REDUCCIÓN MOD N Y NOMENCLATURA
 * =================================================================
 */

use prospector_core_math::prelude::*;

#[test]
fn certify_scalar_reduction_at_order_boundary() {
    println!("\n⚖️  [PROVING_GROUNDS]: Auditing Scalar Modular Strata...");

    // 1. SETUP: Valor igual al orden N
    let n_limbs = [
        0xBFD25E8CD0364141, 0xBAAEDCE6AF48A03B,
        0xFFFFFFFFFFFFFFFE, 0xFFFFFFFFFFFFFFFF
    ];
    let n_bytes = convert_limbs_u64_to_u256_big_endian(&n_limbs);

    // 2. EXECUTION: N mod N debe ser 0, pero Scalar::from_u256_big_endian
    // debe rechazarlo por seguridad (is_zero check).
    let result_at_n = Scalar::from_u256_big_endian(n_bytes);
    assert!(result_at_n.is_err(), "L1_SCALAR_FAULT: Order N must collapse to error (zero point).");

    // 3. EXECUTION: N + 1 mod N debe ser 1
    let mut n_plus_one_bytes = n_bytes;
    n_plus_one_bytes[31] += 1;

    let result_plus_one = Scalar::from_u256_big_endian(n_plus_one_bytes).unwrap();
    assert_eq!(result_plus_one.to_u256_big_endian()[31], 1, "L1_SCALAR_FAULT: Modular reduction drift.");

    println!("   ✅ [SUCCESS]: Scalar naming and reduction certified.");
}

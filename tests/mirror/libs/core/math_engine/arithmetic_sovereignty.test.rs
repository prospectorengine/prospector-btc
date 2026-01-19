// [tests/mirror/libs/core/math_engine/arithmetic_sovereignty.test.rs]
/**
 * =================================================================
 * APARATO: ARITHMETIC SOVEREIGNTY TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * RESPONSABILIDAD: VALIDAR LA REPARACIÓN NOMINAL BIG_ENDIAN
 * =================================================================
 */

use prospector_core_math::prelude::*;

#[test]
fn certify_big_endian_conversion_parity() {
    println!("\n🔢 [PROVING_GROUNDS]: Auditing Big-Endian Strata parities...");

    // Test: u128 -> u256
    let input: u128 = 0xDEADC0DEBAADF00D1337BEEFCAFEBABE;
    let buffer = convert_u128_to_u256_big_endian(input);

    assert_eq!(buffer[0], 0, "L1_MATH_FAULT: Padding corruption.");
    assert_eq!(u128::from_be_bytes(buffer[16..32].try_into().unwrap()), input);

    // Test: Limbs <-> Bytes
    let limbs = [1, 2, 3, 4];
    let bytes = convert_limbs_u64_to_u256_big_endian(&limbs);
    let recovered_limbs = convert_u256_big_endian_to_limbs_u64(&bytes);

    assert_eq!(limbs, recovered_limbs, "L1_MATH_FAULT: Limb/Byte isomorphism failed.");

    println!("   ✅ [SUCCESS]: Nominal naming and data mapping certified.");
}

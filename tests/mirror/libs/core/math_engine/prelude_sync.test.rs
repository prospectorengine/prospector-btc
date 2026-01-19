// [tests/mirror/libs/core/math_engine/prelude_sync.test.rs]
/**
 * =================================================================
 * APARATO: PRELUDE SYNCHRONIZATION TEST (V1.0 - SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * RESPONSABILIDAD: CERTIFICAR LA EXPOSICIÓN NOMINAL DE ARITMÉTICA
 * =================================================================
 */

use prospector_core_math::prelude::*;

#[test]
fn certify_prelude_nominal_alignment() {
    println!("\n🔍 [PROVING_GROUNDS]: Auditing Prelude Nominal Strata...");

    // 1. TEST: Existencia de funciones 'big_endian'
    let mut buffer = [0u8; 32];
    let result = add_u64_to_u256_big_endian(&mut buffer, 1);

    assert!(result.is_ok(), "L1_PRELUDE_FAULT: add_u64_to_u256_big_endian not exposed.");
    assert_eq!(buffer[31], 1, "DATA_MAPPING_ERROR: Inconsistent state.");

    // 2. TEST: Isomorfismo de conversión
    let input: u128 = 0xFFFF;
    let be_buffer = convert_u128_to_u256_big_endian(input);
    assert_eq!(be_buffer[31], 0xFF, "L1_PRELUDE_FAULT: convert_u128_to_u256_big_endian not exposed.");

    println!("   ✅ [SUCCESS]: Nominal arithmetic strata verified in prelude.");
}

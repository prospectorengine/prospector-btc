// [tests/mirror/libs/core/math_engine/field_simd_isomorphism.test.rs]
/**
 * =================================================================
 * APARATO: FIELD SIMD ISOMORPHISM TEST (V1.0 - SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar paridad entre backends AVX2 y Emulación.
 * =================================================================
 */

use prospector_core_math::prelude::*;

#[test]
fn certify_field_simd_isomorphism_across_lanes() {
    println!("\n🧬 [PROVING_GROUNDS]: Auditing Field SIMD Isomorphism...");

    // 1. SETUP: 4 elementos de campo aleatorios
    let e0 = FieldElement::from_u64(0xDEADBEEF);
    let e1 = FieldElement::from_u64(0xCAFEBABE);
    let e2 = FieldElement::from_u64(0x13371337);
    let e3 = FieldElement::from_u64(0x42424242);

    let vector = FieldElementVector4::from_elements(&e0, &e1, &e2, &e3);

    // 2. EXECUTION: Adición consigo mismo (2 * E)
    // En AVX2 esto usa _mm256_add_epi64, en SW usa un bucle.
    let result_vector = unsafe { vector.add_modular_vectorized(&vector) };

    // 3. VALIDATION: Comparación contra oráculo escalar
    for lane in 0..4 {
        let extracted = result_vector.extract_and_reduce_lane(lane);
        let expected = match lane {
            0 => e0.add_modular(&e0),
            1 => e1.add_modular(&e1),
            2 => e2.add_modular(&e2),
            3 => e3.add_modular(&e3),
            _ => unreachable!(),
        };

        assert_eq!(extracted, expected, "L1_SIMD_FAULT: Lane {} drift detected.", lane);
    }

    println!("   ✅ [SUCCESS]: Field SIMD isomorphism certified bit-perfect.");
}

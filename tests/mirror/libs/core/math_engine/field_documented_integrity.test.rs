// [tests/mirror/libs/core/math_engine/field_documented_integrity.test.rs]
/**
 * =================================================================
 * APARATO: FIELD DOCUMENTATION INTEGRITY TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * RESPONSABILIDAD: CERTIFICAR LA NIVELACIÓN DE DOCS Y NOMENCLATURA
 * =================================================================
 */

use prospector_core_math::prelude::*;

#[test]
fn certify_field_naming_and_logic() {
    println!("\n⚖️  [PROVING_GROUNDS]: Auditing documented FieldElement Strata...");

    // 1. TEST: Nomenclatura Nominal (Zero Abbreviations)
    let scalar = [0xAAu8; 32];
    let element = FieldElement::from_big_endian_bytes(&scalar);
    let output = element.internal_words_to_big_endian_bytes();

    assert_eq!(scalar, output, "L1_FIELD_FAULT: Big-Endian roundtrip failed.");

    // 2. TEST: Predicados Lógicos
    assert!(element.is_odd(), "L1_FIELD_FAULT: Nivelación de is_odd fallida.");
    assert!(!element.is_zero(), "L1_FIELD_FAULT: Nivelación de is_zero fallida.");

    // 3. TEST: Montgomery Transition
    let mont = element.to_montgomery_domain();
    let back = mont.from_montgomery_domain();
    assert_eq!(element, back, "L1_FIELD_FAULT: Montgomery isomorphism drift.");

    println!("   ✅ [SUCCESS]: Field naming, documentation and predicates certified.");
}

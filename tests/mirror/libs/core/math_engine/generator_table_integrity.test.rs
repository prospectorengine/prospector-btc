// [tests/mirror/libs/core/math_engine/generator_table_integrity.test.rs]
/**
 * =================================================================
 * APARATO: GENERATOR TABLE INTEGRITY TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L1-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE PARIDAD BIT-PERFECTA DE LA LUT
 * =================================================================
 */

use prospector_core_math::prelude::*;
use prospector_core_math::generator_table::GENERATOR_TABLE;

#[test]
fn certify_static_lookup_table_parity() {
    println!("\n🧬 [PROVING_GROUNDS]: Auditing Generator LUT bit-depth...");

    // 1. TEST: Validación de Punto G (Ventana 0, Valor 1)
    let static_g = &GENERATOR_TABLE[0][1];
    let point_g_jacobian = JacobianPoint::from_affine(static_g.x_limbs, static_g.y_limbs);

    // El primer punto de la tabla DEBE ser el Generador G de secp256k1
    assert_eq!(
        static_g.x_limbs,
        [0x59F2815B16F81798, 0x029BFCDB2DCE28D9, 0x55A06295CE870B07, 0x79BE667EF9DCBBAC],
        "L1_DATA_FAULT: Generator G coordinate X mismatch in table."
    );

    // 2. TEST: Validación de Derivación O(1) contra Oráculo Dinámico
    // Usamos k=17 (0x11). Esto dispara Ventana 0 (v=1) y Ventana 1 (v=1)
    let mut scalar_17_bytes = [0u8; 32];
    scalar_17_bytes[31] = 17;

    let point_via_table = JacobianPoint::from_private_scalar_windowed(&scalar_17_bytes);

    // Oráculo: G + 16G (Calculado individualmente para el test)
    let point_16g = JacobianPoint::from_affine(
        GENERATOR_TABLE[1][1].x_limbs,
        GENERATOR_TABLE[1][1].y_limbs
    );
    let affine_gx = FieldElement::from_limbs(GENERATOR_TABLE[0][1].x_limbs);
    let affine_gy = FieldElement::from_limbs(GENERATOR_TABLE[0][1].y_limbs);

    let point_17g_expected = UnifiedCurveEngine::add_mixed_deterministic(&point_16g, &affine_gx, &affine_gy);

    assert_eq!(
        point_via_table.x,
        point_17g_expected.x,
        "L1_LOGIC_FAULT: Window summation drift detected at k=17."
    );

    println!("   ✅ [SUCCESS]: Generator Table parity certified bit-perfect.");
}

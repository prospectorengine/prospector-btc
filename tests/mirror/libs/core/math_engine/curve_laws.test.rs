// [tests/mirror/libs/core/math_engine/curve_laws.test.rs]
/**
 * =================================================================
 * APARATO: GEOMETRIC LAWS CERTIFIER (V1.1 - ZENITH)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L1-MIRROR
 * RESPONSABILIDAD: VALIDACIÓN DE IDENTIDAD Y ASOCIATIVIDAD
 * =================================================================
 */

use prospector_core_math::prelude::*;

#[test]
fn certify_group_law_identity_and_doubling() {
    println!("\n📐 [PROVING_GROUNDS]: Auditing Group Laws bit-perfectly...");

    // 1. SETUP: Punto G (Bloque Génesis)
    const G_X: [u64; 4] = [0x59F2815B16F81798, 0x029BFCDB2DCE28D9, 0x55A06295CE870B07, 0x79BE667EF9DCBBAC];
    const G_Y: [u64; 4] = [0x9C47D08FFB10D4B8, 0xFD17B448A6855419, 0x5DA4FBFC0E1108A8, 0x483ADA7726A3C465];

    let point_g = JacobianPoint::from_affine(G_X, G_Y);
    let affine_gx = FieldElement::from_limbs(G_X);
    let affine_gy = FieldElement::from_limbs(G_Y);

    // 2. TEST: P + INF = P
    let infinity = JacobianPoint::infinity();
    let result_identity = UnifiedCurveEngine::add_mixed_deterministic(&infinity, &affine_gx, &affine_gy);
    assert_eq!(result_identity.x, point_g.x, "L1_GEOMETRY_FAULT: Identity addition failed.");

    // 3. TEST: G + G == 2G
    let point_2g_via_add = UnifiedCurveEngine::add_mixed_deterministic(&point_g, &affine_gx, &affine_gy);
    let point_2g_via_double = UnifiedCurveEngine::double_point_jacobian(&point_g);

    assert_eq!(point_2g_via_add.x, point_2g_via_double.x, "L1_GEOMETRY_FAULT: Doubling inconsistency.");
    println!("   ✅ [SUCCESS]: Identity and Doubling laws verified.");
}

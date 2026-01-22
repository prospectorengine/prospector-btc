// [tests/mirror/libs/core/math_engine/secp256k1_unification.test.rs]
/**
 * APARATO: GEOMETRIC NEXUS UNIFICATION TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar paridad bit-perfecta entre derivación LUT y Satoshi.
 */

use prospector_core_math::prelude::*;

#[test]
fn certify_static_lut_windowed_derivation_parity() {
    println!("\n📐 [PROVING_GROUNDS]: Auditing Unified Geometric Nexus V140.0...");

    // 1. SETUP: Escalar k=1 (Punto G)
    let mut scalar_one_big_endian = [0u8; 32];
    scalar_one_big_endian[31] = 1;

    // 2. EXECUTION: Derivación a través del Hub Geométrico
    let point_via_hub = JacobianPoint::from_private_scalar_windowed(&scalar_one_big_endian);
    let (affine_x_bytes, _) = point_via_hub.to_affine_bytes().expect("PROJECTION_FAULT");

    // 3. VALIDATION: Paridad con el Vector Génesis de Satoshi
    let satoshi_x_hex = "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
    let actual_x_hex = hex::encode(affine_x_bytes);

    assert_eq!(actual_x_hex, satoshi_x_hex, "L1_GEOMETRY_FAULT: Window table mismatch at k=1.");

    // 4. SETUP: Escalar k=2 (Punto 2G)
    let mut scalar_two_big_endian = [0u8; 32];
    scalar_two_big_endian[31] = 2;

    let point_via_hub_2g = JacobianPoint::from_private_scalar_windowed(&scalar_two_big_endian);
    let (affine_x_2g, _) = point_via_hub_2g.to_affine_bytes().expect("PROJECTION_FAULT");

    let satoshi_2g_x_hex = "c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5";
    assert_eq!(hex::encode(affine_x_2g), satoshi_2g_x_hex, "L1_GEOMETRY_FAULT: Window table mismatch at k=2.");

    println!("   ✅ [SUCCESS]: Unified LUT derivation is bit-perfect.");
}

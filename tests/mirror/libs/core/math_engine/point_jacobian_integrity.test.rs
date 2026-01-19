// [tests/mirror/libs/core/math_engine/point_jacobian_nominal.test.rs]
/**
 * =================================================================
 * APARATO: POINT NOMINAL INTEGRITY TEST (V2.2 - SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L1-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE ENLACE NOMINAL BIG_ENDIAN
 * =================================================================
 */

use prospector_core_math::prelude::*;

#[test]
fn certify_point_to_field_nominal_handshake() {
    println!("\n📐 [PROVING_GROUNDS]: Auditing Point-to-Field Nominal Link...");

    // 1. SETUP: Coordenadas de prueba (Punto Arbitrario)
    let scalar_bytes = [0x01u8; 32];
    let point = JacobianPoint::from_private_scalar_windowed(&scalar_bytes);

    // 2. EXECUTION: Proyección
    // La prueba de éxito es que esto compile y ejecute sin pánico nominal.
    let projection_result = point.to_affine_bytes();

    // 3. VALIDATION
    assert!(projection_result.is_ok(), "L1_POINT_FAULT: Handshake with field nominals failed.");

    let (ax, ay) = projection_result.unwrap();
    println!("   ✅ [SUCCESS]: Nominal Strata Levelized.");
    println!("      X_HEAD: 0x{:02x}{:02x}...", ax[0], ax[1]);
    println!("      Y_HEAD: 0x{:02x}{:02x}...", ay[0], ay[1]);
}

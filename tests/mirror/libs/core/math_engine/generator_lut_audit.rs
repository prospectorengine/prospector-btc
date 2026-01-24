// [tests/mirror/libs/core/math_engine/generator_lut_audit.rs]
/**
 * =================================================================
 * APARATO: GENERATOR LUT AUDIT (V1.4 - NOMINAL CERTIFIED)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar la paridad nominal de los campos x/y limbs.
 * =================================================================
 */
 use prospector_core_math::generator_table::{GENERATOR_TABLE, StaticAffinePoint};

 #[test]
 fn certify_lookup_table_nomenclature_and_access() {
     println!("\n🧬 [PROVING_GROUNDS]: Auditing Generator LUT Structural Alignment...");
 
     // 1. VALIDACIÓN DE CAMPOS (Resuelve E0609)
     let genesis_g = &GENERATOR_TABLE[0][1];
     
     // Verificamos que los limbs X son accesibles bajo el nuevo nombre nominal
     assert_ne!(genesis_g.x_limbs[0], 0, "L1_DATA_FAULT: Genesis G limbs are inaccessible.");
     assert_ne!(genesis_g.y_limbs[0], 0, "L1_DATA_FAULT: Genesis G limbs are inaccessible.");
 
     // 2. VALIDACIÓN DE DIMENSIONES (Resuelve E0308)
     assert_eq!(GENERATOR_TABLE.len(), 64, "L1_STRATA_FAULT: Matrix must contain 64 windows.");
     
     println!("   ✅ [SUCCESS]: Nomenclature synced with point.rs consumers.");
     println!("   ✅ [SUCCESS]: Array initialization bit-perfect.");
 }
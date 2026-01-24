// [tests/mirror/libs/core/math_engine/generator_table_integrity.test.rs]
/**
 * =================================================================
 * APARATO: GENERATOR TABLE INTEGRITY TEST (V1.1 - GOLD MASTER)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L1-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE LA MATRIZ DE SILICIO
 * =================================================================
 */

 use prospector_core_math::prelude::*;
 use prospector_core_math::generator_table::GENERATOR_TABLE;

 #[test]
 fn certify_lookup_table_quantum_boundaries() {
     println!("\n🧬 [PROVING_GROUNDS]: Auditing Generator LUT structural integrity...");

     // 1. TEST: Validación de Dimensiones (64x16)
     assert_eq!(GENERATOR_TABLE.len(), 64, "L1_DATA_FAULT: Matrix must have 64 windows.");
     for (idx, window) in GENERATOR_TABLE.iter().enumerate() {
         assert_eq!(window.len(), 16, "L1_DATA_FAULT: Window {} must have 16 entries.", idx);
         // El primer elemento (v=0) siempre debe ser la identidad
         assert_eq!(window[0].x_limbs, [0,0,0,0], "L1_DATA_FAULT: Window {} v=0 is not Infinity.", idx);
     }

     // 2. TEST: Paridad Bit-Perfect con Satoshi Genesis
     let static_g = &GENERATOR_TABLE[0][1];
     assert_eq!(static_g.x_limbs[0], 0x59F2815B16F81798, "L1_GEOMETRY_FAULT: Generator G limb 0 mismatch.");
     assert_eq!(static_g.x_limbs[3], 0x79BE667EF9DCBBAC, "L1_GEOMETRY_FAULT: Generator G limb 3 mismatch.");

     // 3. TEST: Validación de Salto de Ventana (16G)
     let static_16g = &GENERATOR_TABLE[1][1];
     assert_ne!(static_16g.x_limbs[0], 0, "L1_DATA_FAULT: Window 1 (16G) is unpopulated.");

     println!("   ✅ [SUCCESS]: Generator Table structure is level and compliant.");
 }

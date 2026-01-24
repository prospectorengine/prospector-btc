// [tests/mirror/libs/core/math_engine/scalar_integrity.test.rs]
/**
 * =================================================================
 * APARATO: SCALAR MODULAR INTEGRITY TEST (V22.0 - GOLD MASTER)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L1-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE ARITMÉTICA MOD N
 * =================================================================
 */

 use prospector_core_math::prelude::*;

 #[test]
 fn certify_scalar_modular_arithmetic_v13_2() {
     println!("\n⚖️  [PROVING_GROUNDS]: Auditing Scalar Modular Engine V13.2...");

     // 1. TEST: Reducción N + 1 == 1
     let mut n_plus_one_bytes = [0xFFu8; 32];
     // Sincronizamos con el valor real de N (limbs[0] = 0xBFD25E8CD0364141)
     // N+1 = limbs[0] + 1
     let mut n_bytes = [0u8; 32];
     let n_limbs = [0xBFD25E8CD0364141, 0xBAAEDCE6AF48A03B, 0xFFFFFFFFFFFFFFFE, 0xFFFFFFFFFFFFFFFF];
     for i in 0..4 {
         let start = (3 - i) * 8;
         n_bytes[start..start+8].copy_from_slice(&n_limbs[i].to_be_bytes());
     }

     let mut n_plus_one = n_bytes;
     n_plus_one[31] = 0x42; // D0364141 -> D0364142

     let scalar = Scalar::from_u256_big_endian(n_plus_one).expect("FAILED_TO_CREATE_SCALAR");
     assert_eq!(scalar.private_scalar_limbs[0], 1, "L1_SCALAR_FAULT: Modular reduction drift.");
     println!("   ✅ N+1 Reduction: Certified.");

     // 2. TEST: Adición (N-1) + 1 == 0 (MathError)
     let s_one = Scalar::from_u256_big_endian(convert_u128_to_u256_big_endian(1)).unwrap();
     let mut n_minus_one_bytes = n_bytes;
     n_minus_one_bytes[31] = 0x40; // D0364141 -> D0364140
     let s_max = Scalar::from_u256_big_endian(n_minus_one_bytes).unwrap();

     let sum = s_max.add_n(&s_one);
     assert!(sum.is_zero(), "L1_SCALAR_FAULT: Modular addition overflow.");
     println!("   ✅ (N-1) + 1 = 0 mod N: Certified.");

     println!("   ✅ [SUCCESS]: Scalar modular strata is bit-perfect.");
 }

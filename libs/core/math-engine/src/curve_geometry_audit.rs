/**
 * =================================================================
 * APARATO: CURVE GEOMETRY AUDIT (L1-MATH-CERT)
 * RESPONSABILIDAD: Validación de leyes de grupo y derivación escalar.
 * =================================================================
 */
 use prospector_core_math::prelude::*;

 #[test]
 fn certify_secp256k1_geometric_integrity() {
     println!("\n📐 [AUDIT]: Initiating Geometric Integrity Sequence...");
 
     // 1. Validar G y 2G (Vectores Génesis)
     let g = JacobianPoint::point_generator_g();
     let two_g = UnifiedCurveEngine::double_point_jacobian(&g);
     let (tx, _) = two_g.to_affine_bytes().unwrap();
     
     let expected_2g_x = "c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5";
     assert_eq!(hex::encode(tx), expected_2g_x, "L1_GEOMETRY_FAULT: Doubling drift at 2G.");
 
     // 2. Validar Homomorfismo (P+G == increment)
     let sk = SafePrivateKey::from_bytes(&[0x01; 32]).unwrap();
     let pk = SafePublicKey::from_private(&sk);
     let pk_inc = pk.increment().unwrap();
     
     let mut sk_val = [0x01u8; 32];
     prospector_core_math::arithmetic::add_u64_to_u256_big_endian(&mut sk_val, 1).unwrap();
     let pk_expected = SafePublicKey::from_private(&SafePrivateKey::from_bytes(&sk_val).unwrap());
     
     assert_eq!(pk_inc, pk_expected, "L1_GEOMETRY_FAULT: Homomorphic increment failed.");
 
     // 3. Validar Identidad (P + INF = P)
     let inf = JacobianPoint::infinity();
     let res = UnifiedCurveEngine::add_mixed_deterministic(&inf, &g.x, &g.y);
     assert_eq!(res.x, g.x, "L1_GEOMETRY_FAULT: Identity law violation.");
 
     println!("   ✅ Curve Geometry: Secp256k1 Axioms Certified.");
 }
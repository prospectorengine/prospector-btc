// [tests/mirror/libs/core/math_engine/public_key_integrity.test.rs]
/**
 * =================================================================
 * APARATO: PUBLIC KEY INTEGRITY TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * RESPONSABILIDAD: CERTIFICAR LEY DE GRUPO Y DOCUMENTACIÓN
 * =================================================================
 */

use prospector_core_math::prelude::*;

#[test]
fn certify_public_key_homomorphic_increment() {
    println!("\n🗝️  [PROVING_GROUNDS]: Auditing Public Key Group Laws...");

    // 1. SETUP: Clave privada k=1 -> PubKey G
    let sk_1 = SafePrivateKey::from_bytes(&[
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1
    ]).unwrap();
    let pk_g = SafePublicKey::from_private(&sk_1);

    // 2. EXECUTION: PK(k=1) + 1*G = PK(k=2)
    let pk_2g_via_increment = pk_g.increment().expect("Increment failed");

    // 3. VALIDATION: Comparar con derivación directa de k=2
    let sk_2 = SafePrivateKey::from_bytes(&[
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2
    ]).unwrap();
    let pk_2g_direct = SafePublicKey::from_private(&sk_2);

    assert_eq!(pk_2g_via_increment, pk_2g_direct, "L1_PUBKEY_FAULT: Homomorphic increment drift.");

    println!("   ✅ [SUCCESS]: Public Key increment certified bit-perfect.");
}

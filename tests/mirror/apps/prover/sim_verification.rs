// INICIO DEL ARCHIVO [apps/prover/tests/sim_verification.rs]
/*!
 * =================================================================
 * APARATO: PROOF SIMULATOR (V2.0 - RAW HASH SYNC)
 * CLASIFICACIÓN: INTEGRATION TEST (L3)
 * RESPONSABILIDAD: VERIFICACIÓN E2E DE DETECCIÓN EN FILTRO
 *
 * FIX: Adaptado para inyectar Hash160 ([u8; 20]) en lugar de String.
 * =================================================================
 */

use prospector_core_gen::address_legacy::pubkey_to_address;
use prospector_core_math::public_key::SafePublicKey;
use prospector_core_math::hashing::hash160; // ✅ NUEVO IMPORT
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_domain_strategy::brainwallet::phrase_to_private_key;

#[test]
fn test_golden_ticket_detectability() {
    // 1. Setup del Escenario
    let phrase = "GOLD777TEST";

    // 2. Derivación
    let pk = phrase_to_private_key(phrase);
    let pubk = SafePublicKey::from_private(&pk);

    // Generamos la dirección para validación humana (opcional en lógica de filtro)
    let address_string = pubkey_to_address(&pubk, false);

    // ✅ FIX CRÍTICO: Generar Hash160 crudo para el filtro
    // Usamos 'false' (uncompressed) para simular direcciones legacy antiguas
    let pubkey_bytes = pubk.to_bytes(false);
    let raw_hash = hash160(&pubkey_bytes);

    // 3. Creación del Filtro
    let mut filter = ShardedFilter::new(4, 100, 0.00001);

    // 4. Inyección (Ahora acepta [u8; 20])
    filter.add(&raw_hash);

    // 5. Verificación (Assert)
    assert!(
        filter.contains(&raw_hash),
        "El filtro falló en detectar el Hash160 conocido (False Negative)"
    );

    println!("✅ [PROVER]: Address {} verified via Hash160.", address_string);

    // 6. Control Negativo
    let fake_phrase = "SILVER888TEST";
    let fake_pk = phrase_to_private_key(fake_phrase);
    let fake_pubk = SafePublicKey::from_private(&fake_pk);

    let fake_pubkey_bytes = fake_pubk.to_bytes(false);
    let fake_hash = hash160(&fake_pubkey_bytes);

    assert!(
        !filter.contains(&fake_hash),
        "El filtro detectó un falso positivo obvio"
    );
}
// FIN DEL ARCHIVO [apps/prover/tests/sim_verification.rs]

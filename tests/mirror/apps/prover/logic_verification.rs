// INICIO DEL ARCHIVO [apps/prover/tests/logic_verification.rs]
use prospector_core_math::public_key::SafePublicKey;
use prospector_core_gen::address_legacy::pubkey_to_address;
use prospector_domain_strategy::brainwallet::phrase_to_private_key;

#[test]
fn certify_satoshi_logic_instantly() {
    let phrase = "satoshi";

    // 1. Derivar Clave Privada (SHA256)
    let sk = phrase_to_private_key(phrase);
    let pk = SafePublicKey::from_private(&sk);

    // 2. Generar Dirección Uncompressed (Legacy)
    let address = pubkey_to_address(&pk, false);

    println!("Target Phrase: {}", phrase);
    println!("Generated Address: {}", address);

    // Esta es la dirección REAL de 'satoshi' uncompressed conocida históricamente
    assert_eq!(address, "1ADJqstUMBB5zFquWg19UqZ7Zc6ePCpzLE");
}
// FIN DEL ARCHIVO [apps/prover/tests/logic_verification.rs]

// [tests/mirror/libs/infra/db_turso/scenarios_integrity.test.rs]
/**
 * =================================================================
 * APARATO: SCENARIOS REPOSITORY TORTURE TEST
 * OBJETIVO: Certificar el mapeo de direcciones y frases (V2.0 Sync).
 * =================================================================
 */

use prospector_infra_db::repositories::ScenarioRepository;
use prospector_infra_db::TursoClient;

#[tokio::test]
async fn certify_scenario_mapping_integrity() {
    println!("\n🧪 [PROVING_GROUNDS]: Auditing Scenario V2.0 Mapping...");

    // 1. SETUP: In-Memory con esquema aplicado
    let client = TursoClient::connect("file::memory:", None).await.unwrap();
    let repository = ScenarioRepository::new(client.clone());

    // 2. EXECUTION: Crear un Golden Ticket
    let name = "SATOSHI_TEST";
    let phrase = "correct horse battery staple";
    let addr = "1PzYwVuTotg15ridCGNnAo8u3dr6bE2Yxy";
    let wif = "5HsjrA5VEhok91VzRTe4dhpGBtwmoF2MgZtLFCa1eZ1aVQ6FrNC";

    let created = repository.create_atomic(name, phrase, addr, wif).await.unwrap();

    // 3. VALIDATION: ¿Están los datos en sus columnas correctas?
    assert_eq!(created.target_bitcoin_address, addr, "L3_MAPPING_FAULT: Address fell into phrase column.");
    assert_eq!(created.entropy_seed_phrase, phrase, "L3_MAPPING_FAULT: Phrase fell into address column.");

    // 4. SEARCH: Probar Interceptor
    let found = repository.find_by_target_address(addr).await.unwrap().unwrap();
    assert_eq!(found.operation_name, name);

    println!("   ✅ [SUCCESS]: Scenario mapping is bit-perfect and synchronized.");
}

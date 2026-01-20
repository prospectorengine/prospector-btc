// [tests/mirror/libs/domain/mining_strategy/combinatoric_logic.test.rs]
/**
 * =================================================================
 * APARATO: COMBINATORIC LOGIC CERTIFIER (V1.0 - SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * RESPONSABILIDAD: VALIDACIÓN DE RANGO Y NOMENCLATURA BIG_ENDIAN
 * =================================================================
 */

use prospector_domain_strategy::combinatoric::CombinatoricIterator;

#[test]
fn certify_combinatoric_range_and_naming_sync() {
    println!("\n🧩 [PROVING_GROUNDS]: Auditing Combinatoric Strata...");

    // 1. SETUP: Rango corto (00...01 a 00...03)
    let start_hex = "0000000000000000000000000000000000000000000000000000000000000001";
    let end_hex = "0000000000000000000000000000000000000000000000000000000000000003";

    let mut iterator = CombinatoricIterator::new(
        start_hex,
        end_hex,
        "HYDRA_".into(),
        "_X".into()
    );

    // 2. EXECUTION: Escaneo de estados
    let first = iterator.next().unwrap();
    let third = iterator.next().unwrap();
    let fourth = iterator.next(); // Debe ser None

    // 3. VALIDATION
    assert_eq!(first.0, "HYDRA_0000000000000000000000000000000000000000000000000000000000000001_X");
    assert_eq!(third.0, "HYDRA_0000000000000000000000000000000000000000000000000000000000000003_X");
    assert!(fourth.is_none(), "El iterador no respetó el límite superior Big-Endian.");

    println!("   ✅ [SUCCESS]: Combinatoric range and naming levelized.");
}

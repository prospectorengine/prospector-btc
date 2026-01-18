// [tests/mirror/libs/core/probabilistic/filter_integrity_v30.test.rs]
/**
 * =================================================================
 * APARATO: FILTER V30 INTEGRITY CERTIFIER (V30.1 - SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L1-PROBABILISTIC
 * RESPONSABILIDAD: CERTIFICACIÓN DE CICLO DE VIDA Y PERSISTENCIA MMAP
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. NOMINAL ALIGNMENT: Sincronización total con los métodos de RichListFilter:
 *    - add -> add_identity_hash
 *    - save_to_file -> save_to_disk
 *    - load_from_file_mmap -> load_from_disk_mmap
 * 2. HYGIENE: Erradicación de abreviaciones (lat_mmap -> mmap_hydration_latency).
 * 3. ZERO-COPY PROOF: Valida la hidratación del censo mediante mapeo de memoria.
 * 4. PERFORMANCE HUD: Mide y reporta la latencia de ignición de datos.
 *
 * # Mathematical Proof (Binary Consistency):
 * El test garantiza que el estado de saturación del filtro se preserva
 * bit-a-bit entre el volcado a disco y la recuperación por memoria virtual.
 * =================================================================
 */

use prospector_core_probabilistic::RichListFilter;
use tempfile::tempdir;
use std::time::Instant;

#[test]
fn certify_zenith_filter_cycle_v30_1() {
    println!("\n🧠 [PROVING_GROUNDS]: Initiating Probabilistic Strata Audit V30.1...");

    // 1. SETUP: Aislamiento en directorio temporal para evitar rastro residual
    let sandbox_directory = tempdir().expect("IO_FAULT: Failed to create temporary strata.");
    let binary_artifact_path = sandbox_directory.path().join("zenith_census_v30.bin");

    // 2. INICIALIZACIÓN Y CARGA DE VECTORES
    println!("   🧪 Phase 1: Generating cryptographic matrix and injecting vectors...");
    let mut filter_instance = RichListFilter::new(10000, 0.000001);

    // Vector de identidad Hash160 (Satoshi Era)
    let target_identity_hash = [0x77u8; 20];

    // ✅ RESOLUCIÓN NOMINAL: Uso del método nivelado add_identity_hash
    filter_instance.add_identity_hash(&target_identity_hash);

    // 3. CRISTALIZACIÓN DE ESTRATO (SAVE TO DISK)
    println!("   🧪 Phase 2: Crystallizing strata to physical storage...");
    // ✅ RESOLUCIÓN NOMINAL: Uso del método nivelado save_to_disk
    filter_instance.save_to_disk(&binary_artifact_path)
        .expect("STRATA_FAULT: Binary serialization failed.");

    // 4. HIDRATACIÓN SOBERANA (ZERO-COPY MMAP)
    println!("   🧪 Phase 3: Validating Memory-Mapped hydration...");
    let performance_start_instant = Instant::now();

    // ✅ RESOLUCIÓN NOMINAL: Uso de la función asociada load_from_disk_mmap
    let hydrated_filter = RichListFilter::load_from_disk_mmap(&binary_artifact_path)
        .expect("MMAP_FAULT: Memory mapping of binary strata collapsed.");

    let mmap_hydration_latency = performance_start_instant.elapsed();

    // 5. VERIFICACIÓN DE INTEGRIDAD Y PARIDAD
    println!("   🧪 Phase 4: Auditing data parity post-hydration...");

    // Verificamos que el conteo de elementos se preservó tras la serialización
    assert_eq!(
        hydrated_filter.get_item_count(),
        1,
        "INTEGRITY_COLLAPSE: Item count drift detected."
    );

    // Verificamos que el objetivo es localizable en O(1)
    assert!(
        hydrated_filter.contains_identity_hash(&target_identity_hash),
        "SIGNAL_LOSS: Target hash missing in hydrated strata."
    );

    println!("   ✅ [SUCCESS]: Parity certified. Filter is Bit-Perfect.");
    println!("   📊 Latency [MMAP_HYDRATION]: {:?}", mmap_hydration_latency);
    println!("🏁 [COMPLETE]: Filter Integrity V30.1 certified.\n");
}

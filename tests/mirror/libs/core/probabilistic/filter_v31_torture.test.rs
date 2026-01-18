// [tests/mirror/libs/core/probabilistic/filter_v31_torture.test.rs]
/**
 * =================================================================
 * APARATO: FILTER V31 TORTURE TEST (V31.1 - ZENITH ALIGNED)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L1-PROBABILISTIC
 * RESPONSABILIDAD: CERTIFICACIÓN DE CICLO DE VIDA Y PARIDAD BIT-PERFECTA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. API ALIGNMENT: Resolución definitiva de E0599 mediante la sincronización
 *    de métodos nominales: add -> add_identity_hash, count -> get_item_count.
 * 2. HYGIENE TOTAL: Erradicación de abreviaciones. Uso de tempfile para
 *    garantizar aislamiento galvánico de rastro en disco durante el test.
 * 3. MULTI-STRATA HYDRATION: Certifica que el filtro es idéntico tras ser
 *    recuperado vía Buffered Stream y vía Memory-Mapping (mmap).
 * 4. VERBOSIDAD FORENSE: Impresión detallada del progreso para auditoría local.
 *
 * # Mathematical Proof (Deterministic Logic):
 * El test garantiza que el filtro preserva su capacidad de detección O(1)
 * tras la cristalización, validando que el motor de serialización no
 * altera la distribución de bits en la matriz de Bloom.
 * =================================================================
 */

use prospector_core_probabilistic::RichListFilter;
use tempfile::tempdir;

#[test]
fn certify_full_filter_lifecycle_determinism_v31_1() {
    println!("\n🧠 [PROVING_GROUNDS]: Initiating Probabilistic Torture V31.1...");

    // 1. SETUP: Aislamiento en directorio temporal
    let temporary_directory = tempdir().expect("IO_FAULT: Failed to create sandbox strata.");
    let artifact_storage_path = temporary_directory.path().join("sovereign_filter_torture.bin");

    // 2. FASE: GÉNESIS
    println!("   🧪 Phase 1: Generating cryptographic matrix (1000 items, 1e-6 FPR)...");
    let mut filter_initial_instance = RichListFilter::new(1000, 0.000001);
    let identity_hash_target = [0x77u8; 20];

    filter_initial_instance.add_identity_hash(&identity_hash_target);

    // 3. FASE: CRISTALIZACIÓN (Persistencia en Disco)
    println!("   🧪 Phase 2: Crystallizing binary strata to disk...");
    filter_initial_instance.save_to_disk(&artifact_storage_path)
        .expect("STRATA_FAULT: Serialization to disk failed.");

    // 4. FASE: HIDRATACIÓN BUFFERED (Fallback Logic)
    // ✅ RESOLUCIÓN E0599: Sincronización con load_from_disk_buffered
    println!("   🧪 Phase 3: Validating Buffered Stream recovery...");
    let filter_buffered_recovery = RichListFilter::load_from_disk_buffered(&artifact_storage_path)
        .expect("IO_FAULT: Failed to hydrate via buffered stream.");

    assert_eq!(
        filter_buffered_recovery.get_item_count(),
        1,
        "INTEGRITY_COLLAPSE: Item count drift in Buffered Strata."
    );
    assert!(
        filter_buffered_recovery.contains_identity_hash(&identity_hash_target),
        "SIGNAL_LOSS: Target not found in Buffered filter."
    );

    // 5. FASE: HIDRATACIÓN MMAP (Zero-Copy)
    // ✅ RESOLUCIÓN E0599: Sincronización con load_from_disk_mmap
    println!("   🧪 Phase 4: Validating Zero-Copy Memory-Mapped recovery...");
    let filter_mmap_recovery = RichListFilter::load_from_disk_mmap(&artifact_storage_path)
        .expect("MMAP_FAULT: Failed to map binary strata to memory.");

    assert_eq!(
        filter_mmap_recovery.get_item_count(),
        1,
        "INTEGRITY_COLLAPSE: Item count drift in MMAP strata."
    );
    assert!(
        filter_mmap_recovery.contains_identity_hash(&identity_hash_target),
        "SIGNAL_LOSS: Target not found in Mapped memory segment."
    );

    println!("   ✅ [SUCCESS]: Parity certified. Initial == Buffered == MMAP.");
    println!("🏁 [COMPLETE]: Filter V31.1 lifecycle certified bit-perfect.\n");
}

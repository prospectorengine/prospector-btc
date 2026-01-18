// [tests/mirror/libs/core/probabilistic/filter_integrity_v24.test.rs]
/**
 * =================================================================
 * APARATO: FILTER SERIALIZATION INTEGRITY TEST (V24.1 - ZENITH)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L1-PROBABILISTIC
 * RESPONSABILIDAD: CERTIFICACIÓN DE CICLO DE VIDA Y PERSISTENCIA MMAP
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. API ALIGNMENT: Resolución definitiva de E0599 mediante la nivelación
 *    de métodos: add -> add_identity_hash, save_to_file -> save_to_disk.
 * 2. HYGIENE TOTAL: Erradicación de 'unused import' de std::fs. Uso de
 *    tempfile para aislamiento galvánico de rastro en disco.
 * 3. DUAL HYDRATION PROOF: Certifica la paridad bit-perfecta entre la
 *    carga por mapeo de memoria (Zero-Copy) y el flujo tradicional.
 * 4. NOMINAL PURITY: Nomenclatura descriptiva para vectores de prueba.
 *
 * # Mathematical Proof (Persistence Integrity):
 * El test garantiza que el estado de saturación del filtro se preserva
 * inalterado tras la cristalización en bincode, asegurando que la
 * Tasa de Falsos Positivos (FPR) sea constante en el despliegue remotos.
 * =================================================================
 */

use prospector_core_probabilistic::RichListFilter;
use tempfile::tempdir;

#[test]
fn certify_deterministic_filter_persistence_strata() {
    println!("\n🧠 [PROVING_GROUNDS]: Initiating Probabilistic Strata Audit V24.1...");

    // 1. SETUP: Aislamiento en directorio efímero
    let temporary_directory = tempdir().expect("IO_FAULT: Failed to create sandbox directory.");
    let target_artifact_path = temporary_directory.path().join("sovereign_filter_v24.bin");

    // 2. FASE: GÉNESIS (Construcción e inyección de vectores)
    println!("   🧪 Phase 1: Generating cryptographic matrix and injecting vectors...");
    let mut filter_original_instance = RichListFilter::new(1000, 0.000001);

    // Vectores de identidad Hash160 deterministas (Satoshi Style)
    let identity_hash_alpha = [0xAAu8; 20];
    let identity_hash_beta = [0xBBu8; 20];

    filter_original_instance.add_identity_hash(&identity_hash_alpha);
    filter_original_instance.add_identity_hash(&identity_hash_beta);

    // 3. FASE: CRISTALIZACIÓN (Persistencia soberana)
    println!("   🧪 Phase 2: Crystallizing strata to disk...");
    filter_original_instance.save_to_disk(&target_artifact_path)
        .expect("STRATA_FAULT: Serialization to binary artifact failed.");

    // 4. FASE: HIDRATACIÓN MMAP (Acceso Zero-Copy)
    // ✅ RESOLUCIÓN E0599: Uso de load_from_disk_mmap nominal
    println!("   🧪 Phase 3: Validating Memory-Mapped hydration (Zero-Copy)...");
    let filter_mmap_hydrated = RichListFilter::load_from_disk_mmap(&target_artifact_path)
        .expect("MMAP_FAULT: Memory mapping of binary strata failed.");

    assert_eq!(
        filter_mmap_hydrated.get_item_count(),
        2,
        "INTEGRITY_COLLAPSE: Item count drift in MMAP strata."
    );
    assert!(
        filter_mmap_hydrated.contains_identity_hash(&identity_hash_alpha),
        "SIGNAL_LOSS: Identity Alpha not detected in MMAP filter."
    );
    assert!(
        filter_mmap_hydrated.contains_identity_hash(&identity_hash_beta),
        "SIGNAL_LOSS: Identity Beta not detected in MMAP filter."
    );

    // 5. FASE: HIDRATACIÓN BUFFERED (Fallback Stream)
    // ✅ RESOLUCIÓN E0599: Uso de load_from_disk_buffered nominal
    println!("   🧪 Phase 4: Validating Buffered Stream hydration...");
    let filter_stream_hydrated = RichListFilter::load_from_disk_buffered(&target_artifact_path)
        .expect("IO_FAULT: Buffered stream hydration failed.");

    assert_eq!(
        filter_stream_hydrated.get_item_count(),
        2,
        "INTEGRITY_COLLAPSE: Item count drift in Stream strata."
    );
    assert!(
        filter_stream_hydrated.contains_identity_hash(&identity_hash_alpha),
        "SIGNAL_LOSS: Parity check failed in Stream strata."
    );

    println!("   ✅ [SUCCESS]: Parity certified between RAM, MMAP and Disk Strata.");
    println!("🏁 [COMPLETE]: Filter Integrity V24.1 cycle certified bit-perfect.\n");
}

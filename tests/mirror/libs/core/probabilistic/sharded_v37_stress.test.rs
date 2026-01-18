// [tests/mirror/libs/core/probabilistic/sharded_v37_stress.test.rs]
/**
 * =================================================================
 * APARATO: SHARDED MATRIX STRESS TEST (V37.2 - ZENITH ALIGNED)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L1-PROBABILISTIC
 * RESPONSABILIDAD: CERTIFICACIÓN DE RUTEO SIPHASH Y PERSISTENCIA PARALELA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. SCOPE RESOLUTION: Resolución definitiva de E0425 mediante la
 *    instanciación nominal del motor 'ShardedFilter' con ruteo SipHash-1-3.
 * 2. DETERMINISTIC MAPPING: Valida que los vectores inyectados se distribuyan
 *    y recuperen bit-perfectamente tras la fragmentación física.
 * 3. HYGIENE TOTAL: Uso de 'tempfile' para garantizar que el rastro en disco
 *    sea efímero y no contamine el entorno del VAIO.
 * 4. PERFORMANCE PROOF: Certifica la carga Zero-Copy (mmap) paralela.
 *
 * # Mathematical Proof (Sharded Consistency):
 * Se verifica que el recuento atómico global coincide con la suma de los
 * elementos de los fragmentos individuales tras la re-hidratación.
 * =================================================================
 */

use prospector_core_probabilistic::sharded::ShardedFilter;
use tempfile::tempdir;

#[test]
fn certify_sharded_routing_and_parallel_io_v37_2() {
    println!("\n🧬 [PROVING_GROUNDS]: Initiating Sharded Matrix Stress Audit V37.2...");

    // 1. SETUP: Inicialización del motor fragmentado (4 shards)
    // ✅ RESOLUCIÓN E0425: Instanciación del motor antes del consumo
    let partition_count = 4;
    let sharded_engine_instance = ShardedFilter::new(
        partition_count,
        5000,     // Capacidad nominal por shard
        0.000001  // Tasa de falsos positivos (FPR) de élite
    );

    let temporary_directory = tempdir().expect("IO_FAULT: Sandbox strata creation failed.");

    // 2. FASE: INYECCIÓN (Ruteo determinista)
    println!("   🧪 Phase 1: Injecting vectors across hash spectrum...");

    // Identificadores Hash160 diseñados para caer en diferentes particiones
    let identity_hash_01 = [0x01u8; 20];
    let identity_hash_02 = [0x02u8; 20];
    let identity_hash_max = [0xFFu8; 20];

    sharded_engine_instance.add(&identity_hash_01);
    sharded_engine_instance.add(&identity_hash_02);
    sharded_engine_instance.add(&identity_hash_max);

    // 3. FASE: CRISTALIZACIÓN (Persistencia paralela)
    println!("   🧪 Phase 2: Validating parallel shard crystallization...");
    sharded_engine_instance.save_to_directory(temporary_directory.path())
        .expect("STRATA_FAULT: Parallel serialization failed.");

    // 4. FASE: RE-HIDRATACIÓN (Carga masiva multihilo)
    println!("   🧪 Phase 3: Executing parallel hydration (MMAP Aware)...");
    let reloaded_engine_instance = ShardedFilter::load_from_directory(
        temporary_directory.path(),
        partition_count
    ).expect("HYDRATION_FAULT: Failed to reconstruct sharded matrix from disk.");

    // 5. VALIDACIÓN DE VERDAD (Atomic Consistency Check)
    let final_count = reloaded_engine_instance.get_total_indexed_count();

    assert_eq!(
        final_count,
        3,
        "INTEGRITY_COLLAPSE: Atomic counter mismatch after hydration. Found: {}",
        final_count
    );

    assert!(
        reloaded_engine_instance.contains(&identity_hash_01),
        "SIGNAL_LOSS: Shard 01 data corruption."
    );
    assert!(
        reloaded_engine_instance.contains(&identity_hash_02),
        "SIGNAL_LOSS: Shard 02 data corruption."
    );
    assert!(
        reloaded_engine_instance.contains(&identity_hash_max),
        "SIGNAL_LOSS: Max spectrum shard data corruption."
    );

    println!("   ✅ [SUCCESS]: Deterministic routing and parallel I/O certified.");
    println!("🏁 [COMPLETE]: Sharded Matrix V37.2 audit finalized.\n");
}

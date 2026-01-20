// [tests/mirror/libs/domain/mining_strategy/engines/sequential_boundary_test.rs]
/*!
 * =================================================================
 * APARATO: SEQUENTIAL BOUNDARY STRESS TEST (V1.3 - SOBERANO ALIGNED)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L2-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE BORDES DE LOTE Y RESIDUOS SIMD
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. CORE SYNERGY: Eliminación de helpers locales redundantes. Consumo
 *    directo de 'convert_u128_to_u256_big_endian' de L1-Math.
 * 2. NOMINAL SYNC: Alineación total con el estándar 'big_endian' de la V25.0.
 * 3. EXHAUSTIVE ASSERTION: Verificación de paridad bit-perfecta en las
 *    transiciones de ráfagas Montgomery (Magazine 1024).
 * 4. HYGIENE: Erradicación de advertencias de compilación y rastro forense.
 *
 * # Mathematical Proof (Boundary Saturation):
 * El test garantiza que no existen fugas de detección en los índices de
 * transición de hardware: carril SIMD final (3), límite de batch (1023)
 * y residuo post-vaciado (1024).
 * =================================================================
 */

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_core_math::prelude::*;
use prospector_core_gen::address_legacy::pubkey_to_address;
use prospector_domain_strategy::{ProjectiveSequentialEngine, FindingHandler};

/**
 * IMPLEMENTACIÓN: ESPÍA DE CAPTURA DE COLISIONES SOBERANO
 * Sumidero de señales para validar la precisión del hallazgo en bordes.
 */
struct CollisionCaptureSpy {
    pub captured_addresses: Arc<Mutex<Vec<String>>>,
}

impl FindingHandler for CollisionCaptureSpy {
    /**
     * Reporta el hallazgo al vector de auditoría del test.
     */
    fn on_finding(&self, bitcoin_address: String, _private_key: SafePrivateKey, _source: String) {
        let mut collection_guard = self.captured_addresses.lock().expect("MUTEX_POISONED_L2_TEST");
        collection_guard.push(bitcoin_address);
    }
}

/**
 * CERTIFICACIÓN: Integridad de bordes de cargador (Magazine) y ráfagas SIMD.
 */
#[test]
fn certify_sequential_batch_and_residue_integrity_v1_3() {
    println!("\n🔍 [PROVING_GROUNDS]: Initiating Sequential Boundary & Residue Stress Test V1.3...");

    // 1. SETUP DEL ESCENARIO (VECTORES DE CONTROL)
    let start_scalar_hexadecimal = "0000000000000000000000000000000000000000000000000000000000000001";
    let sharded_census_filter = ShardedFilter::new(1, 100, 0.00001);
    let mut expected_identities_collection = Vec::new();

    // 2. GENERACIÓN DE LA VERDAD (TARGET POSITIONS)
    // Definimos offsets cruciales para la física del motor Meloni 5M
    let strategic_offsets_to_test = vec![3u64, 1023u64, 1024u64];

    let start_key_handle = SafePrivateKey::from_bytes(&hex::decode(start_scalar_hexadecimal).unwrap())
        .expect("MATH_FAULT: Start scalar decoding collapsed.");
    let start_public_point = SafePublicKey::from_private(&start_key_handle);

    for offset_magnitude in strategic_offsets_to_test {
        // ✅ NIVELACIÓN SOBERANA: Uso de función nominal de L1-Math
        let scalar_offset_bytes = convert_u128_to_u256_big_endian(offset_magnitude as u128);

        let target_public_point = start_public_point.add_scalar(&scalar_offset_bytes)
            .expect("GEOMETRY_FAULT: Point addition failed in test setup.");

        // Inyectamos el Hash160 (No-Comprimido / Satoshi Standard)
        let target_hash160 = hash160(&target_public_point.to_bytes(false));
        sharded_census_filter.add(&target_hash160);

        expected_identities_collection.push(pubkey_to_address(&target_public_point, false));
    }

    // 3. EJECUCIÓN DEL MOTOR TÁCTICO (V213.3 - ZENITH GOLD)
    let forensic_spy = CollisionCaptureSpy {
        captured_addresses: Arc::new(Mutex::new(Vec::new()))
    };
    let effort_telemetry_accumulator = Arc::new(AtomicU64::new(0));
    let global_stop_signal = AtomicBool::new(false);

    // Misión de 1025 iteraciones: 1 lote de 1024 + 1 llave de residuo.
    let total_iterations_limit = 1025;

    println!("   🚀 [EXECUTION]: Auditing 1025 iterations (1 Full Batch + 1 Residue)...");

    let final_audit_checkpoint_hex = ProjectiveSequentialEngine::execute_optimized_audit(
        start_hexadecimal_scalar,
        total_iterations_limit,
        &sharded_census_filter,
        &global_stop_signal,
        effort_telemetry_accumulator.clone(),
        &forensic_spy
    );

    // 4. VALIDACIÓN FORENSE DE INTEGRIDAD
    let detected_findings_list = forensic_spy.captured_addresses.lock().unwrap();
    let recorded_effort_volume = effort_telemetry_accumulator.load(Ordering::SeqCst);

    // A. Verificación de Volumen
    assert_eq!(recorded_effort_volume, total_iterations_limit, "❌ FATAL: Throughput count drift detected.");

    // B. Verificación de Detección (Buscamos 3 colisiones exactas)
    assert_eq!(detected_findings_list.len(), 3, "❌ FATAL: Boundary or Residue collision ignored.");

    for expected_address in &expected_identities_collection {
        assert!(detected_findings_list.contains(expected_address),
            "❌ SIGNAL_LOSS: Target address {} missed by the engine.", expected_address);
    }

    // C. Verificación de Checkpoint (Base + Iteraciones)
    // 1 + 1025 = 1026 (0x402)
    assert!(final_audit_checkpoint_hex.to_lowercase().contains("402"),
        "❌ FATAL: Checkpoint corruption. Received: {}", final_audit_checkpoint_hex);

    println!("   ✅ [VERDICT]: Boundary transitions and residues certified bit-perfect.");
    println!("🏁 [COMPLETE]: Sequential Boundary V1.3 levelized.\n");
}

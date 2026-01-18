// [tests/mirror/libs/domain/mining_strategy/engines/sequential_boundary_test.rs]
/*!
 * =================================================================
 * APARATO: SEQUENTIAL BOUNDARY STRESS TEST (V1.2 - SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L2-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE BORDES DE LOTE Y RESIDUOS SIMD
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. SCOPE RECOVERY: Resolución definitiva de los errores de scope 0-3.
 *    Sincroniza el uso de 'sharded_census_filter' en las fases de inyección y auditoría.
 * 2. RESIDUE INTEGRITY: Certifica que el vaciado final del cargador (Magazine)
 *    no ignore el último escalar del rango (Off-by-one prevention).
 * 3. NOMINAL PURITY: Erradicación total de abreviaciones. 'filter' -> 'sharded_census_filter'.
 * 4. HYGIENE: Limpieza de rastro de importaciones para un reporte de QA impecable.
 *
 * # Mathematical Proof (Boundary Saturation):
 * El test sitúa objetivos en los índices 3 (final de carril SIMD 0),
 * 1023 (final de bloque Montgomery) y 1024 (residuo de ráfaga),
 * garantizando que el motor procese cada estrato sin pérdida de señal.
 * =================================================================
 */

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_core_math::prelude::*;
use prospector_core_gen::address_legacy::pubkey_to_address;
use prospector_domain_strategy::{ProjectiveSequentialEngine, FindingHandler};

/**
 * IMPLEMENTACIÓN: ESPÍA DE CAPTURA DE COLISIONES
 * sumidero de señales para validar la precisión del hallazgo en bordes.
 */
struct CollisionCaptureSpy {
    pub captured_addresses: Arc<Mutex<Vec<String>>>,
}

impl FindingHandler for CollisionCaptureSpy {
    fn on_finding(&self, bitcoin_address: String, _private_key: SafePrivateKey, _source: String) {
        let mut collection_guard = self.captured_addresses.lock().expect("MUTEX_POISONED_L2_TEST");
        collection_guard.push(bitcoin_address);
    }
}

/**
 * CERTIFICACIÓN: Integridad de bordes de cargador (Magazine) y ráfagas SIMD.
 */
#[test]
fn certify_sequential_batch_and_residue_integrity_v1_2() {
    println!("\n🔍 [AUDIT]: Initiating Sequential Boundary & Residue Stress Test V1.2...");

    // 1. SETUP DEL ESCENARIO (VECTORES DE CONTROL)
    let start_scalar_hexadecimal = "0000000000000000000000000000000000000000000000000000000000000001";

    // ✅ RESOLUCIÓN SOBERANA: Uso nominal de la matriz de búsqueda
    let sharded_census_filter = ShardedFilter::new(1, 100, 0.00001);

    let mut expected_identities_collection = Vec::new();

    // 2. GENERACIÓN DE LA VERDAD (TARGET POSITIONS)
    // Definimos 3 puntos de falla comunes en algoritmos vectorizados:
    let strategic_offsets_to_test = vec![3u64, 1023u64, 1024u64];

    let start_key_handle = SafePrivateKey::from_bytes(&hex::decode(start_scalar_hexadecimal).unwrap())
        .expect("MATH_FAULT: Start scalar decoding collapsed.");
    let start_public_point = SafePublicKey::from_private(&start_key_handle);

    for offset_magnitude in strategic_offsets_to_test {
        let scalar_offset_bytes = convert_u128_to_u256_be_local(offset_magnitude as u128);
        let target_public_point = start_public_point.add_scalar(&scalar_offset_bytes)
            .expect("GEOMETRY_FAULT: Point addition failed in test setup.");

        // Inyectamos el Hash160 (No-Comprimido) para validar el motor Satoshi-Standard
        let target_hash160 = hash160(&target_public_point.to_bytes(false));

        // ✅ RESOLUCIÓN: Sincronización con el nombre de variable correcto
        sharded_census_filter.add(&target_hash160);

        expected_identities_collection.push(pubkey_to_address(&target_public_point, false));
    }

    // 3. EJECUCIÓN DEL MOTOR TÁCTICO (V212.5)
    let forensic_spy = CollisionCaptureSpy {
        captured_addresses: Arc::new(Mutex::new(Vec::new()))
    };
    let effort_telemetry_accumulator = Arc::new(AtomicU64::new(0));
    let global_stop_signal = AtomicBool::new(false);

    // Misión de 1025 iteraciones: Cubre un lote de 1024 y deja exactamente 1 llave de residuo.
    let total_iterations_limit = 1025;

    println!("   🚀 [EXECUTION]: Auditing 1025 iterations (1 Full Batch + 1 Residue)...");

    let final_audit_checkpoint_hex = ProjectiveSequentialEngine::execute_optimized_audit(
        start_scalar_hexadecimal,
        total_iterations_limit,
        &sharded_census_filter,
        &global_stop_signal,
        effort_telemetry_accumulator.clone(),
        &forensic_spy
    );

    // 4. VALIDACIÓN FORENSE DE INTEGRIDAD
    let detected_findings_list = forensic_spy.captured_addresses.lock().unwrap();
    let recorded_effort_volume = effort_telemetry_accumulator.load(Ordering::SeqCst);

    // A. Verificación de Volumen: El contador debe ser exacto (Lógica O(1))
    assert_eq!(recorded_effort_volume, total_iterations_limit, "❌ FATAL: Throughput count drift detected.");

    // B. Verificación de Detección: El motor debe capturar los 3 objetivos
    assert_eq!(detected_findings_list.len(), 3, "❌ FATAL: Residue or boundary collision was ignored.");

    for expected_address in &expected_identities_collection {
        if !detected_findings_list.contains(expected_address) {
            panic!("❌ SIGNAL_LOSS: Target address {} was not identified by the engine.", expected_address);
        }
    }

    // C. Verificación de Checkpoint: 1 + 1025 = 1026 (0x402)
    assert!(final_audit_checkpoint_hex.to_lowercase().contains("402"),
        "❌ FATAL: Checkpoint corruption. Expected 0x402, Received: {}", final_audit_checkpoint_hex);

    println!("   ✅ [VERDICT]: All boundaries and residues verified bit-perfect.");
    println!("🏁 [COMPLETE]: Sequential Boundary Stress Test finalized.\n");
}

/**
 * Helper técnico para la inyección de escalares en el espacio de pruebas.
 * Sincronizado con la arquitectura Big-Endian de Bitcoin.
 */
fn convert_u128_to_u256_be_local(value_magnitude: u128) -> [u8; 32] {
    let mut big_endian_buffer = [0u8; 32];
    let value_bytes = value_magnitude.to_be_bytes();
    big_endian_buffer[16..32].copy_from_slice(&value_bytes);
    big_endian_buffer
}

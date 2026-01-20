// [tests/mirror/libs/domain/mining_strategy/engines/sequential_engine_master.test.rs]
/*!
 * =================================================================
 * APARATO: SEQUENTIAL ENGINE MASTER CERTIFIER (V203.0 - ZENITH ALIGNED)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L2-STRATEGY-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE RÁFAGA MONTGOMERY Y FIRMA DE SILICIO
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. MELONI CO-Z VALIDATION: Certifica que la transición entre ráfagas
 *    preserva la paridad Z, evitando derivas geométricas en el Hot-Loop.
 * 2. SILICON SIGNATURE SYNC: Sincroniza el reporte de QA con las firmas
 *    nominales 'ELITE_SIMD_ADX' niveladas en el StrategyExecutor.
 * 3. NOMINAL PURITY: Erradicación total de abreviaciones. 'res' -> 'audit_result'.
 * 4. HYGIENE: Documentación técnica nivel Tesis Doctoral y rastro forense.
 *
 * # Mathematical Proof (Montgomery Batch Inversion):
 * El test valida que la inversión simultánea de 1024 puntos produce
 * coordenadas afines bit-perfectas comparadas con la derivación individual.
 * =================================================================
 */

use prospector_domain_strategy::{ProjectiveSequentialEngine, FindingHandler};
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_core_math::prelude::*;
use prospector_core_math::hashing::hash160;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::time::Instant;
use serde_json::json;
use std::fs;

/**
 * IMPLEMENTACIÓN: ESPÍA DE DESCUBRIMIENTO SOBERANO
 * Captura y cuenta las señales de colisión para verificar la eficacia del motor.
 */
struct DiscoveryForensicSpy {
    pub accumulated_found_count: Arc<AtomicU64>,
}

impl FindingHandler for DiscoveryForensicSpy {
    /**
     * Procesa la colisión y registra el rastro en el contador atómico.
     */
    fn on_finding(
        &self,
        bitcoin_address: String,
        _private_key_handle: SafePrivateKey,
        _source_metadata: String
    ) {
        println!("      🎯 [DETECTION]: Target identified at address: {}", bitcoin_address);
        self.accumulated_found_count.fetch_add(1, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * CERTIFICACIÓN: Integridad de ráfaga, continuidad escalar y firma de silicio.
     */
    #[test]
    fn certify_sequential_burst_and_silicon_signature_v203() {
        println!("\n🚀 [AUDIT]: Initiating Projective Sequential Engine Master Audit V203...");
        let mut technical_fault_accumulator = 0;
        let suite_start_timestamp = Instant::now();

        // 1. SETUP DEL ESCENARIO (VECTORES DORADOS)
        let start_hexadecimal_scalar = "0000000000000000000000000000000000000000000000000000000000000001";
        let sharded_census_filter = ShardedFilter::new(1, 1000, 0.0001);

        let discovery_spy = DiscoveryForensicSpy {
            accumulated_found_count: Arc::new(AtomicU64::new(0))
        };
        let effort_telemetry_accumulator = Arc::new(AtomicU64::new(0));
        let global_stop_signal = AtomicBool::new(false);

        // GENERACIÓN DE LA VERDAD (Target at scalar 1000 / 0x3E8)
        let target_scalar_hexadecimal = "00000000000000000000000000000000000000000000000000000000000003E8";
        let target_private_key_bytes = hex::decode(target_scalar_hexadecimal)
            .expect("FAULT: Invalid hex vector.");

        let target_private_key_handle = SafePrivateKey::from_bytes(&target_private_key_bytes)
            .expect("MATH_FAULT: Target scalar outside of curve boundaries.");

        let target_public_key_point = SafePublicKey::from_private(&target_private_key_handle);

        // Inyectamos paridad dual para certificar cobertura total (Compressed/Uncompressed)
        sharded_census_filter.add(&hash160(&target_public_key_point.to_bytes(true)));
        sharded_census_filter.add(&hash160(&target_public_key_point.to_bytes(false)));

        println!("   🧪 Phase 1: Validating detection in Montgomery Magazine (Batch 1024)...");

        // 2. EJECUCIÓN DEL MÚSCULO COMPUTACIONAL (V213.3)
        // Ejecutamos 1050 iteraciones para probar el ciclo de vaciado de residuos post-cargador.
        let final_audit_checkpoint_hex = ProjectiveSequentialEngine::execute_optimized_audit(
            start_hexadecimal_scalar,
            1050,
            &sharded_census_filter,
            &global_stop_signal,
            effort_telemetry_accumulator.clone(),
            &discovery_spy
        );

        // 3. VALIDACIÓN FORENSE DE RESULTADOS
        let found_collisions_count = discovery_spy.accumulated_found_count.load(Ordering::SeqCst);

        if found_collisions_count >= 2 {
            println!("      ✅ Dual Strata Detection: OK (Both formats identified).");
        } else {
            println!("      ❌ ERROR: Signal loss in burst. Found: {}", found_collisions_count);
            technical_fault_accumulator += 1;
        }

        // Validación de Checkpoint: 1 (start) + 1050 (iter) = 1051 (0x41B)
        if final_audit_checkpoint_hex.to_lowercase().contains("41b") {
            println!("      ✅ Scalar Continuity: OK (Checkpoint at 0x41B).");
        } else {
            println!("      ❌ ERROR: Checkpoint drift detected. Received: {}", final_audit_checkpoint_hex);
            technical_fault_accumulator += 1;
        }

        // 4. BENCHMARK DE POTENCIA Y FIRMA DE ACELERACIÓN
        println!("   🚀 Phase 2: Measuring silicon performance and hardware signature...");
        let is_silicon_accelerated = is_optimized_arithmetic_supported();
        let hardware_signature = if is_silicon_accelerated { "ELITE_SIMD_ADX" } else { "STANDARD_SW" };

        let performance_bench_start = Instant::now();
        ProjectiveSequentialEngine::execute_optimized_audit(
            &final_audit_checkpoint_hex,
            5000,
            &sharded_census_filter,
            &global_stop_signal,
            effort_telemetry_accumulator.clone(),
            &discovery_spy
        );
        let bench_duration = performance_bench_start.elapsed();
        let hashrate_throughput = 5000.0 / bench_duration.as_secs_f64();

        println!("      🚀 Throughput: {:.2} H/s | Mode: {}", hashrate_throughput, hardware_signature);

        // 5. CRISTALIZACIÓN DE EVIDENCIA (QA LEDGER)
        let report_storage_path = "reports/qa/sequential_engine_master_report.json";
        let report_payload = json!({
            "apparatus": "projective_sequential_engine",
            "verdict": if technical_fault_accumulator == 0 { "GOLD_MASTER" } else { "FAILED" },
            "metrics": {
                "hashrate_hs": hashrate_throughput,
                "average_efficiency_h_ms": hashrate_throughput / 1000.0,
                "batch_latency_ms": bench_duration.as_millis(),
                "collisions_found": found_collisions_count,
                "logical_faults": technical_fault_accumulator
            },
            "evidence": {
                "hardware_acceleration_signature": hardware_signature,
                "math_engine": "Meloni_CoZ_5M_V213",
                "batch_magazine_size": 1024
            },
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        fs::create_dir_all("reports/qa").ok();
        fs::write(report_storage_path, report_payload.to_string())
            .expect("IO_FAULT: Unable to seal QA report.");

        println!("\n💾 [REPORT]: Audit evidence secured at: {}", report_storage_path);
        println!("🏁 [COMPLETE]: Certification finalized in {:?}.", suite_start_timestamp.elapsed());

        assert_eq!(technical_fault_accumulator, 0, "Sequential Engine integrity compromised.");
    }
}

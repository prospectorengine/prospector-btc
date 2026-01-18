// [tests/mirror/libs/domain/mining_strategy/engines/sequential_engine_test.rs]
/*!
 * =================================================================
 * APARATO: SEQUENTIAL ENGINE MASTER TEST (V203.1 - SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L2-STRATEGY
 * RESPONSABILIDAD: CERTIFICACIÓN DE RÁFAGA MONTGOMERY Y RESIDUOS
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. ZERO RESIDUE: Resolución definitiva de errores de scope. Instanciación
 *    nominal del filtro particionado L1.
 * 2. NOMINAL PURITY: Erradicación de abreviaciones. 'pk' transiciona a
 *    'public_key' y 'spy' a 'forensic_collision_spy'.
 * 3. CO-Z VALIDATION: Certifica que el motor Meloni procesa ráfagas de
 *    1024 y residuos de 2 sin pérdida de precisión.
 * 4. PANOPTICON SYNC: Emisión de informe técnico al Dashboard Zenith.
 * =================================================================
 */

use prospector_domain_strategy::{ProjectiveSequentialEngine, FindingHandler};
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_core_math::prelude::*;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::time::{Instant, Duration};
use serde_json::json;
use reqwest::blocking::Client;

/**
 * IMPLEMENTACIÓN: ESPÍA FORENSE DE COLISIONES
 * Captura las señales de hallazgo para validar la efectividad del motor.
 */
struct ForensicCollisionSpy {
    pub captured_addresses: Arc<Mutex<Vec<String>>>,
    pub success_signal_received: Arc<AtomicBool>,
}

impl FindingHandler for ForensicCollisionSpy {
    /**
     * Callback de hallazgo inyectado por el StrategyExecutor.
     */
    fn on_finding(&self, bitcoin_address: String, _private_key: SafePrivateKey, _source: String) {
        println!("      🎯 [DETECTION]: Target identified at address: {}", bitcoin_address);
        let mut collection_guard = self.captured_addresses.lock().expect("MUTEX_POISONED");
        collection_guard.push(bitcoin_address);
        self.success_signal_received.store(true, Ordering::SeqCst);
    }
}

/**
 * Transmite el veredicto de la prueba al Orquestador Central para su visualización en L5.
 */
fn dispatch_technical_audit_report(
    verdict_label: &str,
    hashrate_magnitude: f64,
    technical_log: String,
    detected_faults_count: u32
) {
    let orchestrator_endpoint = std::env::var("ORCHESTRATOR_URL").unwrap_or_else(|_| "http://localhost:3000".into());
    let authority_token = std::env::var("WORKER_AUTH_TOKEN").unwrap_or_else(|_| "observer".into());

    let payload_artifact = json!({
        "testName": "SEQUENTIAL_ENGINE_V203_1",
        "stratum": "L2_STRATEGY",
        "verdict": verdict_label,
        "metrics": {
            "throughput": hashrate_magnitude,
            "latency_ms": 0,
            "error_rate": detected_faults_count as f64
        },
        "forensicLog": technical_log,
        "environment": "Local_VAIO_Arithmetic_Sanctum",
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    let network_client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("INFRA_FAULT: Reporting engine failed.");

    let _ = network_client.post(format!("{}/api/v1/admin/qa/report", orchestrator_endpoint))
        .header("Authorization", format!("Bearer {}", authority_token))
        .json(&payload_artifact)
        .send();
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * CERTIFICACIÓN: Integridad de ráfaga y continuidad de rastro forense.
     *
     * # Mathematical Proof:
     * Verifica que el cargador de Montgomery procesa lotes exactos y que
     * el Saneamiento de Residuo captura llaves fuera de la potencia de 2.
     */
    #[test]
    fn certify_sequential_engine_burst_integrity_v203() {
        println!("\n🚀 [AUDIT]: Initiating Projective Sequential Engine Audit V203.1...");
        let start_suite_timer = Instant::now();
        let mut forensic_bitacora = String::new();
        let mut accumulated_faults_count = 0;

        // 1. SETUP DEL ESCENARIO (VECTORES DORADOS)
        let start_scalar_hexadecimal = "0000000000000000000000000000000000000000000000000000000000000001";

        // ✅ RESOLUCIÓN SCOPE: Instanciación nominal del filtro soberano
        let sharded_census_filter = ShardedFilter::new(1, 1000, 0.0001);

        println!("   🧪 Phase 1: Configuring Sharded Filter with known targets...");

        // Generamos un objetivo en el escalar 1025 (Fuerza procesado de batch completo + 1 residuo)
        let target_scalar_hex = "0000000000000000000000000000000000000000000000000000000000000401";
        let target_private_key_handle = SafePrivateKey::from_bytes(&hex::decode(target_scalar_hex).unwrap()).unwrap();
        let target_public_key_point = SafePublicKey::from_private(&target_private_key_handle);

        // Inyectamos Hash160 (No-Comprimido) para validar arqueología Satoshi
        let target_hash160 = prospector_core_math::hashing::hash160(&target_public_key_point.to_bytes(false));
        sharded_census_filter.add(&target_hash160);

        forensic_bitacora.push_str("✅ SETUP: Target scalar 0x401 injected in local strata.\n");

        // 2. EJECUCIÓN DEL MÚSCULO COMPUTACIONAL (Burst Mode)
        println!("   🧪 Phase 2: Dispatching motor in burst (1026 iterations)...");
        let forensic_spy = ForensicCollisionSpy {
            captured_addresses: Arc::new(Mutex::new(vec![])),
            success_signal_received: Arc::new(AtomicBool::new(false)),
        };
        let effort_telemetry_accumulator = Arc::new(AtomicU64::new(0));
        let global_stop_signal = AtomicBool::new(false);

        let performance_bench_start = Instant::now();
        // 1026 iteraciones = 1 Batch de 1024 + 2 de residuo final
        let final_audit_checkpoint = ProjectiveSequentialEngine::execute_optimized_audit(
            start_scalar_hexadecimal,
            1026,
            &sharded_census_filter,
            &global_stop_signal,
            effort_telemetry_accumulator.clone(),
            &forensic_spy
        );
        let bench_duration = performance_bench_start.elapsed();

        // 3. AUDITORÍA FORENSE DE RESULTADOS
        println!("   🧪 Phase 3: Analyzing forensic trail and signals...");

        // A. Validación de Colisión
        if forensic_spy.success_signal_received.load(Ordering::SeqCst) {
            println!("      ✅ Collision: Target identified in residual burst strata.");
            forensic_bitacora.push_str("✅ LOGIC: Collision detected bit-perfect in residue flush.\n");
        } else {
            println!("      ❌ ERROR: Engine ignored the target in keyspace strata.");
            accumulated_faults_count += 1;
            forensic_bitacora.push_str("❌ LOGIC: Target detection failed in residue strata.\n");
        }

        // B. Validación de Continuidad (Checkpoint Accuracy)
        // 1 (start) + 1026 (iter) = 1027 (0x403)
        if final_audit_checkpoint.to_lowercase().contains("403") {
            println!("      ✅ Checkpoint: Scaler continuity verified at 0x403.");
            forensic_bitacora.push_str(&format!("✅ PERSISTENCE: Checkpoint secured at {}.\n", final_audit_checkpoint));
        } else {
            println!("      ❌ ERROR: Checkpoint drift detected. Received: {}", final_audit_checkpoint);
            accumulated_faults_count += 1;
            forensic_bitacora.push_str("❌ PERSISTENCE: Scaler continuity drift identified.\n");
        }

        // 4. BENCHMARK DE EFICIENCIA
        let hashrate_throughput = 1026.0 / bench_duration.as_secs_f64();
        println!("   🚀 Phase 4: Reporting Throughput: {:.2} H/s.", hashrate_throughput);
        forensic_bitacora.push_str(&format!("📊 PERFORMANCE: {:.2} H/s recorded on local silicon.\n", hashrate_throughput));

        // 5. SENTENCIA Y REPORTE AL DASHBOARD
        let final_verdict = if accumulated_faults_count == 0 { "GOLD_MASTER" } else { "FAILED" };
        forensic_bitacora.push_str(&format!("\nVEREDICTO_SISTEMA: {}\n", final_verdict));

        dispatch_technical_audit_report(
            final_verdict,
            hashrate_throughput,
            forensic_bitacora,
            accumulated_faults_count
        );

        println!("\n🏁 [INFORME]: Audit completed in {:?}. Verdict: {}", start_suite_timer.elapsed(), final_verdict);

        assert_eq!(accumulated_faults_count, 0, "Integrity of the Sequential Engine strata has been compromised.");
    }
}

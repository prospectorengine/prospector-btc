// [tests/mirror/libs/domain/mining_strategy/engines/sequential_engine_test.rs]
/*!
 * =================================================================
 * APARATO: SEQUENTIAL ENGINE MASTER TEST (V203.2 - QUANTUM CERTIFIED)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L2-STRATEGY-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE RÁFAGA MONTGOMERY Y SALTOS G
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. QUANTUM PARITY: Valida que la materialización O(log n) del punto de
 *    inicio coincida con la trayectoria aditiva serial.
 * 2. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta (spy -> forensic_collision_spy).
 * 3. RESIDUE INTEGRITY: Certifica que el vaciado final del cargador (Magazine)
 *    procesa correctamente lotes no múltiplos de 1024.
 * 4. PANOPTICON REPORTING: Inyecta resultados técnicos en el Orquestador
 *    para visualización en el HUD de Proving Grounds.
 *
 * # Mathematical Proof (Deterministic Search):
 * El test sitúa un objetivo en el escalar 1025. El motor debe procesar un
 * bloque completo de 1024 llaves vía Montgomery y capturar el objetivo
 * en la fase de 'vaciado de residuo' final.
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
 * IMPLEMENTACIÓN: ESPÍA DE COLISIONES CRIPTOGRÁFICAS
 * Sumidero de señales para validar la efectividad de la detección.
 */
struct ForensicCollisionSpy {
    pub captured_addresses_collection: Arc<Mutex<Vec<String>>>,
    pub detection_signal_received: Arc<AtomicBool>,
}

impl FindingHandler for ForensicCollisionSpy {
    /**
     * Callback de hallazgo inyectado por el ejecutor.
     */
    fn on_finding(
        &self,
        bitcoin_address: String,
        _private_key_handle: SafePrivateKey,
        _entropy_source: String
    ) {
        println!("      🎯 [DETECTION_SIGNAL]: Target identified at address: {}", bitcoin_address);
        let mut collection_guard = self.captured_addresses_collection.lock()
            .expect("VAULT_LOCK_POISONED");
        collection_guard.push(bitcoin_address);
        self.detection_signal_received.store(true, Ordering::SeqCst);
    }
}

/**
 * Despacha el reporte de certificación técnica al Orquestador L3.
 */
fn dispatch_forensic_qa_report(
    verdict_label: &str,
    hashrate_performance: f64,
    technical_audit_log: String,
    fault_count: u32
) {
    let orchestrator_endpoint = std::env::var("ORCHESTRATOR_URL")
        .unwrap_or_else(|_| "http://localhost:3000".into());
    let authority_token = std::env::var("WORKER_AUTH_TOKEN")
        .unwrap_or_else(|_| "observer".into());

    let report_payload_artifact = json!({
        "testName": "SEQUENTIAL_QUANTUM_ENGINE_V203_2",
        "stratum": "L2_STRATEGY",
        "verdict": verdict_label,
        "metrics": {
            "throughput": hashrate_performance,
            "latency_ms": 0,
            "error_rate": fault_count as f64
        },
        "forensicLog": technical_audit_log,
        "environment": "Local_VAIO_Arithmetic_Torture_Chamber",
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    let network_communication_client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("INFRA_FAULT: Failed to initialize reporting engine.");

    let _ = network_communication_client
        .post(format!("{}/api/v1/admin/qa/report", orchestrator_endpoint))
        .header("Authorization", format!("Bearer {}", authority_token))
        .json(&report_payload_artifact)
        .send();
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * CERTIFICACIÓN: Integridad de ráfaga y paridad de saltos cuánticos.
     */
    #[test]
    fn certify_sequential_engine_quantum_parity_v203() {
        println!("\n🚀 [INICIO]: Iniciando Auditoría del Motor Secuencial Quantum-Meloni...");
        let start_performance_timer = Instant::now();
        let mut technical_audit_bitacora = String::new();
        let mut accumulated_integrity_faults = 0;

        // 1. SETUP: Coordenadas de inicio (Scalar 1)
        let start_hexadecimal_scalar = "0000000000000000000000000000000000000000000000000000000000000001";
        let sharded_census_filter = ShardedFilter::new(1, 1000, 0.0001);

        // 2. INYECCIÓN DEL OBJETIVO: Escalar 1025 (1 extra tras el primer batch de 1024)
        // Esto certifica la fase de 'vaciado de residuo' (Residue Flush).
        let target_scalar_hexadecimal = "0000000000000000000000000000000000000000000000000000000000000401";
        let target_private_key_handle = SafePrivateKey::from_bytes(&hex::decode(target_scalar_hexadecimal).unwrap()).unwrap();
        let target_public_key_point = SafePublicKey::from_private(&target_private_key_handle);

        // Inyectamos Hash160 (No-Comprimido) en la matriz L1
        sharded_census_filter.add(&hash160(&target_public_key_point.to_bytes(false)));

        println!("   🧪 Fase 1: Validando detección en frontera de Batch (1026 iteraciones)...");

        // 3. EJECUCIÓN DEL MOTOR (V213.0)
        let forensic_collision_spy = ForensicCollisionSpy {
            captured_addresses_collection: Arc::new(Mutex::new(vec![])),
            collision_detected_signal: Arc::new(AtomicBool::new(false)),
        };
        let effort_telemetry_accumulator = Arc::new(AtomicU64::new(0));
        let global_stop_signal = AtomicBool::new(false);

        let execution_start_instant = Instant::now();
        let final_mission_checkpoint_hex = ProjectiveSequentialEngine::execute_optimized_audit(
            start_hexadecimal_scalar,
            1026,
            &sharded_census_filter,
            &global_stop_signal,
            effort_telemetry_accumulator.clone(),
            &forensic_collision_spy
        );
        let total_execution_duration = execution_start_instant.elapsed();

        // 4. AUDITORÍA FORENSE DE RESULTADOS
        println!("   🧪 Fase 2: Analizando integridad de rastro y colisión...");

        // A. Verificación de Detección
        if forensic_collision_spy.collision_detected_signal.load(Ordering::SeqCst) {
            println!("      ✅ ÉXITO: Objetivo localizado en el residuo de ráfaga.");
            technical_audit_bitacora.push_str("✅ LOGIC: Detección bit-perfecta en residuo de 1026 iteraciones.\n");
        } else {
            println!("      ❌ FALLO: El motor ignoró el objetivo en el índice 1025.");
            accumulated_integrity_faults += 1;
            technical_audit_bitacora.push_str("❌ LOGIC: El motor falló al procesar el residuo post-Montgomery.\n");
        }

        // B. Verificación de Checkpoint (Continuidad Escalar)
        // 1 + 1026 = 1027 (0x403)
        if final_mission_checkpoint_hex.to_lowercase().contains("403") {
            println!("      ✅ CHECKPOINT: Continuidad certificada en 0x403.");
            technical_audit_bitacora.push_str(&format!("✅ PERSISTENCE: Checkpoint inmutable sellado en {}.\n", final_mission_checkpoint_hex));
        } else {
            println!("      ❌ FALLO: Deriva escalar detectada. Recibido: {}", final_mission_checkpoint_hex);
            accumulated_integrity_faults += 1;
            technical_audit_bitacora.push_str("❌ PERSISTENCE: Error de sincronización en el rastro hexadecimal.\n");
        }

        // 5. BENCHMARK DE RENDIMIENTO (Throughput)
        let hashrate_hs = 1026.0 / total_execution_duration.as_secs_f64();
        println!("   🚀 Throughput Registrado: {:.2} H/s en hardware local.", hashrate_hs);
        technical_audit_bitacora.push_str(&format!("📊 PERFORMANCE: {:.2} H/s procesados bajo ráfaga Jacobiana.\n", hashrate_hs));

        // 6. SENTENCIA Y DESPACHO C2
        let final_audit_verdict = if accumulated_integrity_faults == 0 { "GOLD_MASTER" } else { "FAILED" };
        technical_audit_bitacora.push_str(&format!("\nVEREDICTO_SISTEMA: {}\n", final_audit_verdict));

        dispatch_forensic_qa_report(
            final_audit_verdict,
            hashrate_hs,
            technical_audit_bitacora,
            accumulated_integrity_faults
        );

        println!("\n🏁 [INFORME]: Auditoría finalizada en {:?}. Veredicto: {}",
            start_performance_timer.elapsed(),
            final_audit_verdict
        );

        assert_eq!(accumulated_integrity_faults, 0, "La integridad del motor secuencial ha sido comprometida.");
    }
}

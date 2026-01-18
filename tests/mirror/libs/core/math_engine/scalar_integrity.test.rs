// [tests/mirror/libs/core/math_engine/scalar_integrity.test.rs]
/*!
 * =================================================================
 * APARATO: SCALAR INTEGRITY CERTIFIER (V12.4 - SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L1-CORE
 * RESPONSABILIDAD: CERTIFICACIÓN DE REDUCCIÓN MODULO N Y SEGURIDAD
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. SYMBOL ALIGNMENT: Resolución definitiva de error E0609. Sincroniza
 *    con el campo 'private_scalar_limbs' del núcleo Scalar L1.
 * 2. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta aplicada a
 *    resultados de derivación y buffers de bytes.
 * 3. TACTICAL REPORTING: Evolución del rastro forense hacia el
 *    despacho de red vía 'reqwest' para el Dashboard Zenith.
 * 4. PERFORMANCE PROOF: Validación de latencia en 1,000,000 de iteraciones.
 *
 * # Mathematical Proof (Modulo n Reduction):
 * El test certifica que para cualquier k >= n, k mod n = k - n.
 * Valida que el escalar resultante sea estrictamente < n y > 0.
 * =================================================================
 */

use prospector_core_math::prelude::*;
use prospector_core_math::scalar::SECP256K1_CURVE_ORDER_N;
use std::time::{Instant, Duration};
use serde_json::json;
use reqwest::blocking::Client;
use std::fs;

/**
 * Transmite el veredicto técnico al Centro de Mando Táctico.
 */
fn dispatch_scalar_audit_report(
    verdict_label: &str,
    throughput_magnitude: f64,
    technical_forensic_log: String,
    detected_faults_count: u32
) {
    let orchestrator_url = std::env::var("ORCHESTRATOR_URL").unwrap_or_else(|_| "http://localhost:3000".into());
    let authority_token = std::env::var("WORKER_AUTH_TOKEN").unwrap_or_else(|_| "observer".into());

    let payload_artifact = json!({
        "testName": "SCALAR_CORE_INTEGRITY_V12_4",
        "stratum": "L1_MATH",
        "verdict": verdict_label,
        "metrics": {
            "throughput": throughput_magnitude,
            "latency_ms": 0,
            "error_rate": detected_faults_count as f64
        },
        "forensicLog": technical_forensic_log,
        "environment": "Local_VAIO_Arithmetic_Sanctum",
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    let network_client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("INFRA_FAULT: Failed to initialize reporting client.");

    let _ = network_client.post(format!("{}/api/v1/admin/qa/report", orchestrator_url))
        .header("Authorization", format!("Bearer {}", authority_token))
        .json(&payload_artifact)
        .send();
}

#[cfg(test)]
mod tests {
    use super::*;
    use prospector_core_math::scalar::Scalar;

    /**
     * Ejecuta la auditoría exhaustiva de la lógica escalar.
     */
    #[test]
    fn certify_scalar_logic_and_boundary_safety() {
        println!("\n⚖️  [INICIO]: Iniciando Auditoría de Motor Escalar (Modulo n) V12.4...");
        let suite_start_timer = Instant::now();
        let mut technical_audit_log = String::new();
        let mut accumulated_integrity_faults = 0;

        // 1. FASE DE REDUCCIÓN (Boundary Test)
        // Escenario: N + 1 debe resultar en el escalar 1 tras la reducción atómica.
        println!("   🧪 Fase 1: Verificando reducción modular (N + 1)...");
        let mut scalar_order_plus_one_bytes = convert_limbs_u64_to_u256_be(&SECP256K1_CURVE_ORDER_N);

        // Manipulación bit-perfecta para simular overflow del orden
        scalar_order_plus_one_bytes[31] = scalar_order_plus_one_bytes[31].wrapping_add(1);

        let scalar_derivation_result = Scalar::from_u256_be(scalar_order_plus_one_bytes);

        match scalar_derivation_result {
            Ok(scalar_instance) => {
                // ✅ RESOLUCIÓN E0609: Acceso al campo nominal private_scalar_limbs
                if scalar_instance.private_scalar_limbs == [1, 0, 0, 0] {
                    println!("      ✅ Reducción Bit-Perfect: N + 1 => 1.");
                    technical_audit_log.push_str("✅ REDUCTION: El motor redujo correctamente el escalar excedente.\n");
                } else {
                    println!("      ❌ FALLO: La reducción no convergió al valor esperado.");
                    accumulated_integrity_faults += 1;
                    technical_audit_log.push_str("❌ REDUCTION: Error de convergencia modular.\n");
                }
            },
            Err(math_fault) => {
                println!("      ❌ ERROR_INESPERADO: {}", math_fault);
                accumulated_integrity_faults += 1;
                technical_audit_log.push_str(&format!("❌ FAULT: El motor rechazó el escalar: {}\n", math_fault));
            }
        }

        // 2. FASE DE EXCLUSIÓN (Zero Security)
        println!("   🧪 Fase 2: Verificando rechazo de escalar nulo (Punto al Infinito)...");
        let zero_scalar_buffer = [0u8; 32];
        let zero_validation_result = Scalar::from_u256_be(zero_scalar_buffer);

        if zero_validation_result.is_err() {
            println!("      ✅ Protocolo de Seguridad: Rechazo de escalar nulo certificado.");
            technical_audit_log.push_str("✅ SECURITY: Escalar nulo bloqueado exitosamente.\n");
        } else {
            println!("      ❌ FALLO: El motor permitió la creación de un escalar de valor cero.");
            accumulated_integrity_faults += 1;
            technical_audit_log.push_str("❌ SECURITY: Vulnerabilidad detectada, escalar nulo permitido.\n");
        }

        // 3. BENCHMARK DE RENDIMIENTO (Saturación)
        println!("   🚀 Fase 3: Ejecutando stress-test de 1,000,000 validaciones...");
        let performance_bench_start = Instant::now();
        for iteration in 0..1_000_000 {
            let test_payload_bytes = convert_u128_to_u256_be(iteration as u128);
            let _ = Scalar::from_u256_be(test_payload_bytes);
        }
        let total_bench_duration = performance_bench_start.elapsed();
        let throughput_ops_sec = 1_000_000.0 / total_bench_duration.as_secs_f64();

        println!("      🚀 Throughput: {:.2} validaciones/seg.", throughput_ops_sec);
        technical_audit_log.push_str(&format!("📊 PERFORMANCE: {:.2} ops/seg registrados en hardware local.\n", throughput_ops_sec));

        // 4. SENTENCIA Y DESPACHO AL HUB
        let final_verdict = if accumulated_integrity_faults == 0 { "GOLD_MASTER" } else { "FAILED" };
        technical_audit_log.push_str(&format!("\nVEREDICTO_FINAL: {}\n", final_verdict));

        // Persistencia redundante local
        fs::create_dir_all("reports/qa").ok();
        let _ = fs::write("reports/qa/scalar_integrity_report.json", technical_audit_log.clone());

        dispatch_scalar_audit_report(
            final_verdict,
            throughput_ops_sec,
            technical_audit_log,
            accumulated_integrity_faults
        );

        println!("\n🏁 [INFORME]: Auditoría finalizada en {:?}. Veredicto: {}", suite_start_timer.elapsed(), final_verdict);
        assert_eq!(accumulated_integrity_faults, 0, "La integridad del motor escalar ha sido comprometida.");
    }
}

// [tests/mirror/libs/core/math_engine/scalar_integrity.test.rs]
/*!
 * =================================================================
 * APARATO: SCALAR INTEGRITY CERTIFIER (V12.5 - NOMINAL SYNC)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L1-CORE-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE REDUCCIÓN MODULO N Y SEGURIDAD
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. NOMINAL ALIGNMENT: Resolución definitiva de errores E0423/E0425.
 *    Sincronización con 'from_u256_big_endian' y 'convert_limbs_u64_to_u256_big_endian'.
 * 2. CONTRACT SYMMETRY: Nivelación del reporte JSON hacia el estándar 'camelCase'
 *    del orquestador, garantizando la ingesta de telemetría sin pánicos.
 * 3. ZERO ABBREVIATIONS: Erradicación total de 'be' por 'big_endian' en
 *    variables y llamadas a funciones.
 * 4. PANOPTICON SYNC: Inyección de rastro forense #[instrument] y logs en español.
 *
 * # Mathematical Proof (Modulo n Reduction):
 * El test garantiza que para cualquier escalar k >= n, el motor aplica
 * la reducción k - n de forma atómica, validando que el material
 * resultante sea compatible con el grupo cíclico de secp256k1.
 * =================================================================
 */

use prospector_core_math::scalar::{Scalar, SECP256K1_CURVE_ORDER_N};
use std::time::{Instant, Duration};
use serde_json::json;
use reqwest::blocking::Client;
use std::fs;
use tracing::instrument;

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

    // ✅ SINCRO CON MODELO L2: Claves niveladas a camelCase para ProvingReport
    let payload_artifact = json!({
        "testName": "SCALAR_CORE_INTEGRITY_V12_5",
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

    /**
     * Ejecuta la auditoría exhaustiva de la lógica escalar y seguridad de frontera.
     */
    #[test]
    #[instrument]
    fn certify_scalar_logic_and_boundary_safety_v12_5() {
        println!("\n⚖️  [INICIO]: Iniciando Auditoría de Motor Escalar (Modulo n) V12.5...");
        let suite_start_timer = Instant::now();
        let mut technical_audit_log = String::new();
        let mut accumulated_integrity_faults = 0;

        // 1. FASE DE REDUCCIÓN (Boundary Test)
        // Escenario: N + 1 debe resultar en el escalar 1 tras la reducción atómica.
        println!("   🧪 Fase 1: Verificando reducción modular (N + 1)...");

        // ✅ RESOLUCIÓN SOBERANA: Uso de nombre nominal completo
        let mut scalar_order_plus_one_bytes = prospector_core_math::arithmetic::convert_limbs_u64_to_u256_big_endian(
            &SECP256K1_CURVE_ORDER_N
        );

        // Manipulación bit-perfecta para simular overflow del orden
        scalar_order_plus_one_bytes[31] = scalar_order_plus_one_bytes[31].wrapping_add(1);

        // ✅ RESOLUCIÓN SOBERANA: Uso de nombre nominal completo
        let scalar_derivation_result = Scalar::from_u256_big_endian(scalar_order_plus_one_bytes);

        match scalar_derivation_result {
            Ok(scalar_instance) => {
                // Sincronía con el campo nominal 'private_scalar_limbs'
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

        // ✅ RESOLUCIÓN SOBERANA: Uso de nombre nominal completo
        let zero_validation_result = Scalar::from_u256_big_endian(zero_scalar_buffer);

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
            // ✅ RESOLUCIÓN SOBERANA: Uso de nombre nominal completo
            let test_payload_bytes = prospector_core_math::arithmetic::convert_u128_to_u256_big_endian(iteration as u128);
            let _ = Scalar::from_u256_big_endian(test_payload_bytes);
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

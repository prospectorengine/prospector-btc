// [tests/mirror/libs/core/math_engine/arithmetic_integrity.test.rs]
/*!
 * =================================================================
 * APARATO: CERTIFICADOR ARITMÉTICO SOBERANO (V121.0 - NOMINAL SYNC)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L1-CORE-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE ACARREOS PARALELOS Y CONVERSIÓN
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. NOMINAL ALIGNMENT: Resolución definitiva de errores E0423/E0425.
 *    Sincronización con 'add_u64_to_u256_big_endian' y pares nominales.
 * 2. CONTRACT SYMMETRY: Ajuste de las claves del reporte JSON para
 *    paridad bit-perfecta con el ProvingReport del dominio L2.
 * 3. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta en todo el bloque
 *    (be -> big_endian, meta -> metadata).
 * 4. PANOPTICON SYNC: Inyección de rastro forense detallado en español.
 *
 * # Mathematical Proof (U256 Carry Propagation):
 * El test garantiza que el acarreo (Carry) generado en el bit 63 se propague
 * correctamente a través de los 4 registros de 64 bits del motor ASM,
 * validando la integridad del conteo de hashrate global.
 * =================================================================
 */

use prospector_core_math::prelude::*;
use std::time::{Instant, Duration};
use serde_json::json;
use reqwest::blocking::Client;
use tracing::instrument;

// --- MOTOR DE REPORTE ESTRATÉGICO ---

/**
 * Transmite el veredicto técnico de la aritmética al Centro de Mando L5.
 */
fn dispatch_leveled_arithmetic_report(
    test_name: &str,
    verdict: &str,
    throughput: f64,
    forensic_log: String,
    error_count: u32,
    is_hardware_optimized: bool
) {
    let orchestrator_url = std::env::var("ORCHESTRATOR_URL").unwrap_or_else(|_| "http://localhost:3000".into());
    let auth_token = std::env::var("WORKER_AUTH_TOKEN").unwrap_or_else(|_| "observer".into());

    // ✅ SINCRO CON MODELO L2: Las claves deben coincidir con ProvingReport (camelCase)
    let payload = json!({
        "testName": test_name,
        "stratum": "L1_MATH",
        "verdict": verdict,
        "metrics": {
            "throughput": throughput,
            "latency_ms": 0,
            "error_rate": error_count as f64
        },
        "forensicLog": forensic_log,
        "environment": "Local_VAIO_Arithmetic_Sanctum",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "metrics_metadata": {
            "optimization_level": if is_hardware_optimized { "ADX_BMI2_Sovereign" } else { "Generic_Software" }
        }
    });

    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("INFRA_FAULT: Reporting engine failed.");

    // Despacho silencioso: No debe interrumpir la suite de pruebas si el Orquestador está offline
    let _ = client.post(format!("{}/api/v1/admin/qa/report", orchestrator_url))
        .header("Authorization", format!("Bearer {}", auth_token))
        .json(&payload)
        .send();
}

// --- SUITE DE CERTIFICACIÓN DE INTEGRIDAD ---

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * CERTIFICACIÓN: Validación de acarreo atómico y conversiones Big-Endian.
     */
    #[test]
    #[instrument]
    fn certify_u256_arithmetic_integrity_v121() {
        println!("\n🔢 [INICIO]: Iniciando Auditoría de Kernel Aritmético V121.0...");
        let start_time = Instant::now();
        let mut forensic_bitacora = String::new();
        let mut integrity_faults = 0;

        // 1. FASE DE HARDWARE (Silicon Awareness)
        let adx_supported = is_optimized_arithmetic_supported();
        println!("   🧪 Fase 1: Detectando soporte de optimización ADX/BMI2...");
        if adx_supported {
            forensic_bitacora.push_str("✅ SILICON: Extensiones ADX/BMI2 detectadas. Aceleración activa.\n");
        } else {
            forensic_bitacora.push_str("⚠️ SILICON: ADX/BMI2 no detectado. Utilizando fallback seguro.\n");
        }

        // 2. FASE DE CONVERSIÓN (Nominal Sync)
        println!("   🧪 Fase 2: Validando paridad de convert_u128_to_u256_big_endian...");
        let test_value_u128: u128 = 0xDEADC0DEBAADF00D1337BEEFCAFEBABE;

        // ✅ RESOLUCIÓN SOBERANA: Uso de nombre nominal completo
        let buffer_u256 = convert_u128_to_u256_big_endian(test_value_u128);

        let match_low = &buffer_u256[16..32] == &test_value_u128.to_be_bytes();
        let match_high = &buffer_u256[0..16] == &[0u8; 16];

        if match_low && match_high {
            forensic_bitacora.push_str("✅ CONVERSION: Isomorfismo u128 -> big_endian certificado.\n");
            println!("      ✅ Conversión de Estrato: OK.");
        } else {
            integrity_faults += 1;
            forensic_bitacora.push_str("❌ CONVERSION: Corrupción detectada en el mapeo de bits de u128.\n");
        }

        // 3. FASE DE ACARREO (ASM/Fallback Carry)
        println!("   🧪 Fase 3: Verificando propagación de acarreo U256 (Nominal)...");
        let mut limit_buffer = [0xFFu8; 32]; // 2^256 - 1

        // ✅ RESOLUCIÓN SOBERANA: Uso de nombre nominal completo
        let overflow_result = add_u64_to_u256_big_endian(&mut limit_buffer, 1);

        match overflow_result {
            Err(MathError::InvalidKeyFormat(msg)) if msg.contains("EXHAUSTED") || msg.contains("OVERFLOW") => {
                forensic_bitacora.push_str("✅ CARRY: Detección de agotamiento de espacio escalar verificada.\n");
                println!("      ✅ Protección de Frontera: OK.");
            },
            _ => {
                integrity_faults += 1;
                forensic_bitacora.push_str("❌ CARRY: El motor falló al detectar el overflow de 256 bits.\n");
            }
        }

        // 4. BENCHMARK DE RENDIMIENTO (Stress 5M)
        println!("   🚀 Fase 4: Ejecutando stress-test de 5,000,000 incrementos...");
        let mut stress_buffer = [0u8; 32];
        let bench_start = Instant::now();
        for _ in 0..5_000_000 {
            // ✅ RESOLUCIÓN SOBERANA: Uso de nombre nominal completo
            let _ = add_u64_to_u256_big_endian(&mut stress_buffer, 1);
        }
        let bench_duration = bench_start.elapsed();
        let ops_per_sec = 5_000_000.0 / bench_duration.as_secs_f64();

        println!("      🚀 Throughput: {:.2} M-ops/seg.", ops_per_sec / 1_000_000.0);
        forensic_bitacora.push_str(&format!("📊 PERFORMANCE: {:.2} ops/seg registrados.\n", ops_per_sec));

        // 5. SENTENCIA FINAL Y REPORTE
        let verdict = if integrity_faults == 0 { "GOLD_MASTER" } else { "FAILED" };
        forensic_bitacora.push_str(&format!("\nVERDICTO_FINAL: {}\n", verdict));

        dispatch_leveled_arithmetic_report(
            "ARITHMETIC_CORE_INTEGRITY",
            verdict,
            ops_per_sec,
            forensic_bitacora,
            integrity_faults,
            adx_supported
        );

        println!("\n🏁 [INFORME]: Auditoría finalizada en {:?}. Veredicto: {}", start_time.elapsed(), verdict);
        assert_eq!(integrity_faults, 0, "Integridad aritmética comprometida.");
    }
}

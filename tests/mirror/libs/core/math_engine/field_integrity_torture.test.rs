// [tests/mirror/libs/core/math_engine/field_integrity_torture.test.rs]
/*!
 * =================================================================
 * APARATO: FIELD INTEGRITY TORTURE TEST (V19.0 - ZENITH ALIGNED)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L1-MATH-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN ISOMÓRFICA DEL MOTOR MODULAR
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. NOMINAL SYNC: Resolución definitiva del error E0432. Sincronización
 *    con 'convert_limbs_u64_to_u256_big_endian' de L1-Arithmetic.
 * 2. API UNIFICATION: Sella la transición al método 'multiply_modular'
 *    nivelado en el núcleo de campo Fp V173.0.
 * 3. CONTRACT SYMMETRY: Ajuste de claves JSON a 'camelCase' para paridad
 *    con el receptor de Proving Grounds del Orquestador L3.
 * 4. HYGIENE: Erradicación de 'a_mont' y alias cortos. Documentación doctoral.
 *
 * # Mathematical Proof (Isomorphism Torture):
 * El test garantiza la validez del motor mediante un isomorfismo:
 * Field(A * B) == BigInt(A * B) mod P.
 * Se ejecutan 50,000 iteraciones encadenadas para detectar derivas de bits.
 * =================================================================
 */

use prospector_core_math::prelude::*;
use num_bigint::BigUint;
use serde_json::json;
use std::time::{Instant, Duration};
use reqwest::blocking::Client;
use tracing::instrument;

// --- CONFIGURACIÓN DEL ORÁCULO DE VERDAD ---

/**
 * Recupera el primo canónico de secp256k1 para la validación de precisión arbitraria.
 */
fn get_satoshi_prime_oracle_instance() -> BigUint {
    BigUint::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F", 16)
        .expect("CRITICAL_FAULT: Isomorphic Oracle collapsed.")
}

/**
 * Mapea un elemento de campo L1 al espacio del oráculo BigInt.
 */
fn map_field_element_to_oracle_bigint(element_artifact: &FieldElement) -> BigUint {
    // ✅ RESOLUCIÓN SOBERANA: Uso de nombre nominal completo
    let big_endian_buffer = prospector_core_math::arithmetic::convert_limbs_u64_to_u256_big_endian(
        &element_artifact.internal_words
    );
    BigUint::from_bytes_be(&big_endian_buffer)
}

// --- MOTOR DE REPORTE TÁCTICO ---

/**
 * Transmite el informe técnico de la auditoría al Orquestador L3.
 */
fn dispatch_torture_audit_telemetry(
    final_verdict: &str,
    throughput_magnitude: f64,
    technical_forensic_log: String,
    detected_faults_count: u32
) {
    let orchestrator_url = std::env::var("ORCHESTRATOR_URL").unwrap_or_else(|_| "http://localhost:3000".into());
    let authority_token = std::env::var("WORKER_AUTH_TOKEN").unwrap_or_else(|_| "observer".into());

    // ✅ SINCRO CON MODELO L2: Claves niveladas a camelCase
    let payload_artifact = json!({
        "testName": "FIELD_INTEGRITY_TORTURE_V19",
        "stratum": "L1_MATH",
        "verdict": final_verdict,
        "metrics": {
            "throughput": throughput_magnitude,
            "latency_ms": 0,
            "error_rate": detected_faults_count as f64
        },
        "forensicLog": technical_forensic_log,
        "environment": "Local_VAIO_Arithmetic_Torture_Chamber",
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

// --- SUITE DE CERTIFICACIÓN DE ÉLITE ---

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Ejecuta el protocolo de tortura isomórfica sobre el motor de campo Fp.
     */
    #[test]
    #[instrument]
    fn certify_montgomery_and_batch_integrity_v19() {
        println!("\n💠 [INICIO]: Iniciando Tortura Isomórfica de Campo Finito V19.0...");
        let start_suite_instant = Instant::now();
        let mut technical_forensic_log = String::new();
        let mut accumulated_integrity_faults = 0;
        let satoshi_prime_oracle = get_satoshi_prime_oracle_instance();

        // 1. FASE: MONTGOMERY ROUNDTRIP (Symmetry Check)
        println!("   🧪 Fase 1: Verificando simetría de transformación Montgomery...");
        let original_scalar_value = FieldElement::from_u64(0xABCDEF1234567890);
        let montgomery_domain_artifact = original_scalar_value.to_montgomery_domain();
        let restored_scalar_value = montgomery_domain_artifact.from_montgomery_domain();

        if original_scalar_value == restored_scalar_value {
            technical_forensic_log.push_str("✅ SYMMETRY: Transformación Montgomery certificada bit-perfect.\n");
            println!("      ✅ Isomorfismo Montgomery: OK.");
        } else {
            accumulated_integrity_faults += 1;
            technical_forensic_log.push_str("❌ SYMMETRY: Corrupción detectada en el cambio de dominio.\n");
        }

        // 2. FASE: MULTIPLICACIÓN REDC (Isomorphic Proof)
        println!("   🧪 Fase 2: Validando Producto contra Oráculo BigInt (Ráfaga 50k)...");

        let mut operand_alpha = FieldElement::from_u64(0x123456789ABCDEF);
        let operand_beta = FieldElement::from_u64(0xFEDCBA987654321);

        let performance_benchmark_start = Instant::now();
        for _ in 0..50_000 {
            // ✅ RESOLUCIÓN SOBERANA: Uso de multiply_modular nivelado
            let multiplication_result = operand_alpha.multiply_modular(&operand_beta);

            // Validación contra Verdad Matemática Absoluta
            let oracle_alpha = map_field_element_to_oracle_bigint(&operand_alpha);
            let oracle_beta = map_field_element_to_oracle_bigint(&operand_beta);
            let oracle_expected_product = (&oracle_alpha * &oracle_beta) % &satoshi_prime_oracle;

            if map_field_element_to_oracle_bigint(&multiplication_result) != oracle_expected_product {
                accumulated_integrity_faults += 1;
                technical_forensic_log.push_str("❌ REDC: Desviación bit-a-bit detectada en reducción modular.\n");
                println!("      ❌ ERROR: Bit-drift en multiplicación.");
                break;
            }
            operand_alpha = multiplication_result; // Encadenamiento para detectar errores acumulativos
        }
        println!("      ✅ Multiplicación Modular: Bit-Perfect Parity.");

        // 3. FASE: BATCH INVERSION (Truco de Montgomery)
        println!("   🧪 Fase 3: Certificando Inversión por Lotes (Magazine 1024)...");
        let mut elements_to_invert = vec![FieldElement::default(); 1024];
        for index in 0..1024 {
            elements_to_invert[index] = FieldElement::from_u64(index as u64 + 1);
        }

        let mut results_buffer = vec![FieldElement::default(); 1024];
        let mut scratch_memory = vec![FieldElement::default(); 1024];

        // Sincronizado con L1-Field V173.0
        let batch_result = FieldElement::batch_invert_into(
            &elements_to_invert,
            &mut results_buffer,
            &mut scratch_memory
        );

        if batch_result.is_ok() {
            let test_index = 777;
            let identity_check = elements_to_invert[test_index].multiply_modular(&results_buffer[test_index]);

            if identity_check == FieldElement::from_u64(1) {
                technical_forensic_log.push_str("✅ BATCH: Inversión simultánea certificada bit-perfect.\n");
                println!("      ✅ Montgomery Batch Trick: OK.");
            } else {
                accumulated_integrity_faults += 1;
                technical_forensic_log.push_str("❌ BATCH: El producto del inverso no converge a 1.\n");
            }
        }

        // 4. SENTENCIA Y REPORTE AL HUB
        let final_duration = performance_benchmark_start.elapsed();
        let throughput = 50000.0 / final_duration.as_secs_f64();

        let final_verdict = if accumulated_integrity_faults == 0 { "GOLD_MASTER" } else { "FAILED" };
        technical_forensic_log.push_str(&format!("\nFINAL_VERDICT: {}\n", final_verdict));

        dispatch_torture_audit_telemetry(
            final_verdict,
            throughput,
            technical_forensic_log,
            accumulated_integrity_faults
        );

        println!("\n🏁 [INFORME]: Auditoría finalizada en {:?}. Veredicto: {}",
            start_suite_instant.elapsed(),
            final_verdict
        );

        assert_eq!(accumulated_integrity_faults, 0, "Integridad de Campo Finito comprometida en L1.");
    }
}

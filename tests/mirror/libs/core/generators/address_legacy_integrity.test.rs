// [tests/mirror/libs/core/generators/address_legacy_integrity.test.rs]
/*!
 * =================================================================
 * APARATO: LEGACY ADDRESS GENERATOR CERTIFIER (V31.6 - SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L1-GENERATORS
 * RESPONSABILIDAD: VALIDACIÓN DE HASH160 Y RENDIMIENTO ZERO-ALLOC
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. LINKAGE RESOLVED: Reparación definitiva del error E0432 (sha2).
 * 2. ZERO ABBREVIATIONS: Erradicación total de abreviaciones (res -> result).
 * 3. ISOMORPHIC PROOF: Replicación de la derivación de Satoshi Block 1.
 * 4. FULL DOCUMENTATION: RustDoc con especificaciones térmicas y matemáticas.
 *
 * # Mathematical Proof (Address Derivation):
 * La dirección P2PKH se deriva mediante Base58Check(0x00 || RIPEMD160(SHA256(PubKey))).
 * El test certifica que el motor Zero-Alloc no altera el orden de bytes.
 * =================================================================
 */

use prospector_core_gen::address_legacy;
use prospector_core_math::prelude::*;
use std::time::{Instant, Duration};
use serde_json::json;
use sha2::{Sha256, Digest};
use reqwest::blocking::Client;

// --- VECTORES DE VERDAD (GOLDEN VECTORS) ---
const ENTROPY_SOURCE_PHRASE: &str = "satoshi";
const EXPECTED_BITCOIN_ADDRESS: &str = "1ADJqstUMBB5zFquWg19UqZ7Zc6ePCpzLE";

/**
 * Despacha el informe técnico de la auditoría al Orquestador L3.
 *
 * # Logic:
 * Centraliza los resultados en el Proyecto Panóptico para visibilidad del operador.
 */
fn dispatch_legacy_generator_report(
    verdict_label: &str,
    throughput_magnitude: f64,
    technical_bitacora: String,
    detected_errors_count: u32
) {
    let orchestrator_url = std::env::var("ORCHESTRATOR_URL").unwrap_or_else(|_| "http://localhost:3000".into());
    let worker_token = std::env::var("WORKER_AUTH_TOKEN").unwrap_or_else(|_| "observer".into());

    let payload_artifact = json!({
        "testName": "ADDRESS_LEGACY_GEN_V31_6",
        "stratum": "L1_GENERATORS",
        "verdict": verdict_label,
        "metrics": {
            "throughput": throughput_magnitude,
            "latency_ms": 0,
            "error_rate": detected_errors_count as f64
        },
        "forensicLog": technical_bitacora,
        "environment": "Local_VAIO_Address_Certification_Vault",
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    let network_client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("INFRA_FAULT: Reporting engine failed to initialize.");

    let _ = network_client.post(format!("{}/api/v1/admin/qa/report", orchestrator_url))
        .header("Authorization", format!("Bearer {}", worker_token))
        .json(&payload_artifact)
        .send();
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * CERTIFICACIÓN: Integridad de derivación y throughput de direcciones Legacy.
     *
     * # Errors:
     * Dispara pánico si la dirección generada no coincide bit-a-bit con Satoshi.
     *
     * # Performance:
     * Complejidad algorítmica O(1). Mantenimiento de memoria en Stack-Only.
     */
    #[test]
    fn certify_address_legacy_generation_and_throughput_v31_6() {
        println!("\n📦 [INICIO]: Iniciando Auditoría del Generador de Direcciones P2PKH...");
        let suite_start_timer = Instant::now();
        let mut forensic_audit_log = String::new();
        let mut accumulated_integrity_faults_count = 0;

        // 1. FASE DE SÍNTESIS DE ENTROPÍA (L1 Isomorphism)
        let mut local_entropy_hasher = Sha256::new();
        local_entropy_hasher.update(ENTROPY_SOURCE_PHRASE.as_bytes());
        let private_scalar_material = local_entropy_hasher.finalize();

        let private_key_handle = SafePrivateKey::from_bytes(&private_scalar_material)
            .expect("MATH_FAULT: Failed to derive test scalar from SHA256.");
        let public_key_point = SafePublicKey::from_private(&private_key_handle);

        // 2. PRUEBA DE DERIVACIÓN SOBERANA (Satoshi Vector)
        println!("   🧪 Fase 1: Verificando derivación bit-perfect contra Vector Génesis...");
        let generated_bitcoin_address = address_legacy::pubkey_to_address(&public_key_point, false);

        if generated_bitcoin_address == EXPECTED_BITCOIN_ADDRESS {
            forensic_audit_log.push_str("✅ PARITY: Dirección de Satoshi (1ADJqst...) validada con éxito.\n");
            println!("      ✅ Satoshi Parity: OK.");
        } else {
            accumulated_integrity_faults_count += 1;
            forensic_audit_log.push_str(&format!("❌ PARITY_FAULT: Discrepancia. Recibido: {}\n", generated_bitcoin_address));
            println!("      ❌ ERROR: Fallo de derivación.");
        }

        // 3. PRUEBA DE OPTIMIZACIÓN ZERO-ALLOC (Stack implementation)
        println!("   🧪 Fase 2: Verificando motor de inyección directa desde Affine...");
        let public_point_raw_bytes = public_key_point.to_bytes(false);
        let mut coordinate_x_strata = [0u8; 32];
        let mut coordinate_y_strata = [0u8; 32];
        coordinate_x_strata.copy_from_slice(&public_point_raw_bytes[1..33]);
        coordinate_y_strata.copy_from_slice(&public_point_raw_bytes[33..65]);

        let direct_stack_address = address_legacy::pubkey_from_affine_to_address(
            &coordinate_x_strata,
            &coordinate_y_strata
        );

        if direct_stack_address == EXPECTED_BITCOIN_ADDRESS {
            forensic_audit_log.push_str("✅ ARCHITECTURE: Motor Zero-Alloc certificado bit-perfect.\n");
            println!("      ✅ Motor Zero-Alloc: OK.");
        } else {
            accumulated_integrity_faults_count += 1;
            forensic_audit_log.push_str("❌ ARCHITECTURE: Error de mapeo en memoria de pila.\n");
        }

        // 4. PRUEBA DE RENDIMIENTO (Throughput Stress)
        println!("   🚀 Fase 3: Ejecutando ráfaga de 50,000 generaciones (Base58 + SHA256x2 + Ripe160)...");
        let performance_benchmark_start = Instant::now();
        for _ in 0..50_000 {
            let _ = address_legacy::pubkey_from_affine_to_address(&coordinate_x_strata, &coordinate_y_strata);
        }
        let benchmark_duration = performance_benchmark_start.elapsed();
        let throughput_ops_per_second = 50_000.0 / benchmark_duration.as_secs_f64();

        println!("      🚀 Rendimiento: {:.2} direcciones/seg.", throughput_ops_per_second);
        forensic_audit_log.push_str(&format!("📊 PERFORMANCE: {:.2} H/s registradas en hardware local.\n", throughput_ops_per_second));

        // 5. SENTENCIA Y REPORTE AL DASHBOARD
        let final_audit_verdict_label = if accumulated_integrity_faults_count == 0 { "GOLD_MASTER" } else { "FAILED" };
        forensic_audit_log.push_str(&format!("\nFINAL_VERDICT: {}\n", final_audit_verdict_label));

        dispatch_legacy_generator_report(
            final_audit_verdict_label,
            throughput_ops_per_second,
            forensic_audit_log,
            accumulated_integrity_faults_count
        );

        println!("\n🏁 [INFORME]: Auditoría finalizada en {:?}. Veredicto: {}", suite_start_timer.elapsed(), final_audit_verdict_label);
        assert_eq!(accumulated_integrity_faults_count, 0, "La integridad del generador legacy ha sido comprometida.");
    }
}

// [tests/mirror/libs/core/generators/address_integrity.test.rs]
/*!
 * =================================================================
 * APARATO: LEGACY ADDRESS INTEGRITY CERTIFIER (V31.6 - SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L1-GENERATORS
 * RESPONSABILIDAD: CERTIFICACIÓN DE DERIVACIÓN Y BENCHMARK DE ESTRÉS
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. LINKAGE RESOLVED: Reparación definitiva del error E0432 mediante
 *    inyección formal de 'sha2'.
 * 2. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta aplicada a
 *    manejadores de llaves y resultados de operación.
 * 3. ISOMORPHIC PROOF: Validación bit-perfecta contra el vector "Satoshi"
 *    utilizando una reconstrucción de entropía local.
 * 4. PERFORMANCE AUDIT: Benchmark de alta frecuencia (50,000 ops) con
 *    reporte de telemetría remota sincronizado.
 *
 * # Mathematical Proof (Address Derivation):
 * El aparato garantiza que Base58Check(0x00 || RIPEMD160(SHA256(PubKey)))
 * coincide con los estándares de la red Bitcoin Mainnet.
 * =================================================================
 */

use prospector_core_gen::address_legacy;
use prospector_core_math::prelude::*;
use std::time::{Instant, Duration};
use serde_json::json;
// ✅ RESOLUCIÓN E0432: Importación certificada vía Cargo.toml nivelado
use sha2::{Sha256, Digest};
use reqwest::blocking::Client;

// --- VECTORES DE VERDAD (GOLDEN VECTORS) ---
const ENTROPY_SOURCE_PHRASE: &str = "satoshi";
const EXPECTED_BITCOIN_ADDRESS: &str = "1ADJqstUMBB5zFquWg19UqZ7Zc6ePCpzLE";

/**
 * Despacha el informe técnico de la auditoría de generadores al Orquestador L3.
 *
 * # Logic:
 * Inyecta el resultado de la auditoría en el sumidero de telemetría para
 * visualización en el Dashboard Zenith.
 */
fn dispatch_generator_integrity_report(
    verdict_label: &str,
    throughput_magnitude: f64,
    technical_forensic_log: String,
    detected_faults_count: u32
) {
    let orchestrator_base_url = std::env::var("ORCHESTRATOR_URL")
        .unwrap_or_else(|_| "http://localhost:3000".into());
    let worker_authentication_token = std::env::var("WORKER_AUTH_TOKEN")
        .unwrap_or_else(|_| "observer".into());

    let payload_artifact = json!({
        "testName": "ADDRESS_LEGACY_INTEGRITY_V31_6",
        "stratum": "L1_GENERATORS",
        "verdict": verdict_label,
        "metrics": {
            "throughput": throughput_magnitude,
            "latency_ms": 0,
            "error_rate": detected_faults_count as f64
        },
        "forensicLog": technical_forensic_log,
        "environment": "Local_VAIO_Generator_Chamber",
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    let network_communication_client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("INFRA_FAULT: Reporting engine failed.");

    let _ = network_communication_client
        .post(format!("{}/api/v1/admin/qa/report", orchestrator_base_url))
        .header("Authorization", format!("Bearer {}", worker_authentication_token))
        .json(&payload_artifact)
        .send();
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * CERTIFICACIÓN: Validación de derivación P2PKH y Throughput de silicio.
     *
     * # Errors:
     * El test colapsa si la dirección generada presenta deriva de bits.
     *
     * # Performance:
     * throughput_ops_per_second se mide sobre 50,000 iteraciones Zero-Alloc.
     */
    #[test]
    fn certify_address_legacy_generation_and_throughput_v31_6() {
        println!("\n📦 [INICIO]: Iniciando Auditoría Forense del Generador de Direcciones...");
        let start_suite_timestamp = Instant::now();
        let mut forensic_audit_log = String::new();
        let mut accumulated_integrity_faults = 0;

        // 1. FASE DE SÍNTESIS DE ENTROPÍA (Isomorfismo L1)
        let mut local_entropy_hasher = Sha256::new();
        local_entropy_hasher.update(ENTROPY_SOURCE_PHRASE.as_bytes());
        let private_scalar_material = local_entropy_hasher.finalize();

        let private_key_handle = SafePrivateKey::from_bytes(&private_scalar_material)
            .expect("MATH_FAULT: Failed to derive private key from SHA256 entropy.");
        let public_key_point = SafePublicKey::from_private(&private_key_handle);

        // 2. PRUEBA DE DERIVACIÓN SOBERANA (Satoshi Vector)
        println!("   🧪 Fase 1: Verificando derivación bit-perfect contra Satoshi Genesis...");
        let generated_bitcoin_address = address_legacy::pubkey_to_address(&public_key_point, false);

        if generated_bitcoin_address == EXPECTED_BITCOIN_ADDRESS {
            forensic_audit_log.push_str("✅ PARITY: Dirección de Satoshi (1ADJqst...) validada con éxito.\n");
            println!("      ✅ Satoshi Parity: OK.");
        } else {
            accumulated_integrity_faults += 1;
            forensic_audit_log.push_str(&format!(
                "❌ PARITY_FAULT: Discrepancia detectada. Recibido: {}\n",
                generated_bitcoin_address
            ));
            println!("      ❌ ERROR: Fallo de derivación.");
        }

        // 3. PRUEBA DE ARQUITECTURA ZERO-ALLOC (Stack Integrity)
        println!("   🧪 Fase 2: Validando motor de inyección directa desde Coordenadas Afines...");
        let public_point_raw_bytes = public_key_point.to_bytes(false);
        let mut coordinate_x_strata = [0u8; 32];
        let mut coordinate_y_strata = [0u8; 32];
        coordinate_x_strata.copy_from_slice(&public_point_raw_bytes[1..33]);
        coordinate_y_strata.copy_from_slice(&public_point_raw_bytes[33..65]);

        let direct_stack_address_result = address_legacy::pubkey_from_affine_to_address(
            &coordinate_x_strata,
            &coordinate_y_strata
        );

        if direct_stack_address_result == EXPECTED_BITCOIN_ADDRESS {
            forensic_audit_log.push_str("✅ ARCHITECTURE: Motor Zero-Alloc certificado bit-perfect.\n");
            println!("      ✅ Motor Zero-Alloc: OK.");
        } else {
            accumulated_integrity_faults += 1;
            forensic_audit_log.push_str("❌ ARCHITECTURE: Error de mapeo en memoria de pila detectado.\n");
        }

        // 4. BENCHMARK DE ALTO RENDIMIENTO (Throughput Stress)
        println!("   🚀 Fase 3: Ejecutando ráfaga de 50,000 generaciones (Base58Check Pipeline)...");
        let benchmark_start_instant = Instant::now();
        for _ in 0..50_000 {
            let _ = address_legacy::pubkey_from_affine_to_address(&coordinate_x_strata, &coordinate_y_strata);
        }
        let benchmark_total_duration = benchmark_start_instant.elapsed();
        let throughput_ops_per_second = 50_000.0 / benchmark_total_duration.as_secs_f64();

        println!("      🚀 Rendimiento: {:.2} direcciones/seg.", throughput_ops_per_second);
        forensic_audit_log.push_str(&format!(
            "📊 PERFORMANCE: {:.2} H/s registrados en hardware local.\n",
            throughput_ops_per_second
        ));

        // 5. SENTENCIA Y REPORTE PANÓPTICO
        let final_audit_verdict = if accumulated_integrity_faults == 0 { "GOLD_MASTER" } else { "FAILED" };
        forensic_audit_log.push_str(&format!("\nVEREDICTO_FINAL: {}\n", final_audit_verdict));

        dispatch_generator_integrity_report(
            final_audit_verdict,
            throughput_ops_per_second,
            forensic_audit_log,
            accumulated_integrity_faults
        );

        println!("\n🏁 [INFORME]: Auditoría finalizada en {:?}. Veredicto: {}",
            start_suite_timestamp.elapsed(),
            final_audit_verdict
        );

        assert_eq!(accumulated_integrity_faults, 0, "La integridad del estrato de generadores ha sido comprometida.");
    }
}

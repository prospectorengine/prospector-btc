// [tests/mirror/libs/core/math_engine/arithmetic_integrity.test.rs]
/**
 * =================================================================
 * APARATO: CERTIFICADOR ARITMÉTICO SOBERANO (V120.0 - ADX READY)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L1-CORE
 * RESPONSABILIDAD: CERTIFICACIÓN DE ACARREOS PARALELOS Y SIMETRÍA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. HARDWARE AUDIT: Valida la detección de instrucciones ADX/BMI2 en el
 *    entorno de ejecución real (VAIO/Colab).
 * 2. NOMINAL CONVERSION: Certifica el método restaurado 'convert_u128_to_u256_be'.
 * 3. OVERFLOW PRECISION: Verifica que el acarreo ASM no corrompa los
 *    limbs superiores bajo estrés extremo.
 * 4. PANOPTICON SYNC: Reporte enriquecido con metadatos de optimización.
 * =================================================================
 */

use prospector_core_math::prelude::*;
use std::time::{Instant, Duration};
use serde_json::json;
use reqwest::blocking::Client;

// --- MOTOR DE REPORTE ESTRATÉGICO ---

/**
 * Transmite el veredicto técnico de la aritmética al Centro de Mando.
 * Inyecta el estado de optimización de hardware en el reporte forense.
 */
fn despachar_reporte_aritmetico_nivelado(
    identificador_de_prueba: &str,
    veredicto_final: &str,
    operaciones_por_segundo: f64,
    bitacora_forense: String,
    total_de_errores: u32,
    es_hardware_optimizado: bool
) {
    let url_del_orquestador = std::env::var("ORCHESTRATOR_URL").unwrap_or_else(|_| "http://localhost:3000".into());
    let token_de_autorizacion = std::env::var("WORKER_AUTH_TOKEN").unwrap_or_else(|_| "observer".into());

    let carga_util = json!({
        "testIdentifier": identificador_de_prueba,
        "targetStratum": "L1_MATH",
        "verdict": veredicto_final,
        "metrics": {
            "throughput": operaciones_por_segundo,
            "latency_ms": 0,
            "error_rate": total_de_errores as f64
        },
        "forensicLog": bitacora_forense,
        "environmentMetadata": {
            "bit_width": "256-bit",
            "optimization_level": if es_hardware_optimizado { "ADX_BMI2_Sovereign" } else { "Generic_Software_Fallback" },
            "instruction_set": if cfg!(target_arch = "x86_64") { "x86_64_AVX" } else { "Generic_Arch" }
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    let cliente_transporte = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("FALLO_CRÍTICO: No se pudo instanciar el cliente de reporte.");

    let _ = cliente_transporte.post(format!("{}/api/v1/admin/qa/report", url_del_orquestador))
        .header("Authorization", format!("Bearer {}", token_de_autorizacion))
        .json(&carga_util)
        .send();
}

// --- SUITE DE CERTIFICACIÓN DE INTEGRIDAD ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn certificar_integridad_aritmetica_u256_v120() {
        println!("\n🔢 [INICIO]: Iniciando Auditoría de Kernel Aritmético V120.0...");
        let marca_tiempo_inicio = Instant::now();
        let mut diario_forense = String::new();
        let mut errores_identificados = 0;

        // 1. FASE DE HARDWARE (Silicon Awareness)
        let adx_soportado = is_optimized_arithmetic_supported();
        println!("   🧪 Fase 1: Detectando soporte de optimización ADX/BMI2...");
        if adx_soportado {
            diario_forense.push_str("✅ Hardware: Extensiones ADX/BMI2 detectadas. Hot-Path activado.\n");
            println!("      ✅ Optimización: ACTIVA.");
        } else {
            diario_forense.push_str("⚠️ Hardware: ADX/BMI2 no detectado. Utilizando fallback de software.\n");
            println!("      ⚠️  Optimización: DESACTIVADA (Modo seguro).");
        }

        // 2. FASE DE CONVERSIÓN (restored u128 logic)
        println!("   🧪 Fase 2: Validando restauración de convert_u128_to_u256_be...");
        let valor_u128: u128 = 0xDEADC0DEBAADF00D1337BEEFCAFEBABE;
        let buffer_256 = convert_u128_to_u256_be(valor_u128);

        // El valor debe estar en la mitad baja del buffer (Big Endian)
        let match_low = &buffer_256[16..32] == &valor_u128.to_be_bytes();
        let match_high = &buffer_256[0..16] == &[0u8; 16];

        if match_low && match_high {
            diario_forense.push_str("✅ Conversión: Transformación u128 -> u256_be certificada.\n");
            println!("      ✅ Restauración u128: OK.");
        } else {
            errores_identificados += 1;
            diario_forense.push_str("❌ Conversión: Corrupción de bytes en la inyección de u128.\n");
        }

        // 3. FASE DE ACARREO (ASM/Fallback Carry)
        println!("   🧪 Fase 3: Verificando propagación de acarreo atómico...");
        let mut buffer_limite = [0xFFu8; 32]; // 2^256 - 1
        let resultado_overflow = add_u64_to_u256_be(&mut buffer_limite, 1);

        match resultado_overflow {
            Err(MathError::InvalidKeyFormat(msg)) if msg.contains("EXHAUSTED") || msg.contains("FALLBACK") => {
                diario_forense.push_str("✅ Acarreo: Detección de agotamiento de espacio escalar verificada.\n");
                println!("      ✅ Detección de Overflow: OK.");
            },
            _ => {
                errores_identificados += 1;
                diario_forense.push_str("❌ Acarreo: El motor no reportó el overflow crítico.\n");
                println!("      ❌ ERROR: Fallo de seguridad en acarreo.");
            }
        }

        // 4. BENCHMARK DE ALTO RENDIMIENTO
        println!("   🚀 Fase 4: Midiendo Throughput Aritmético (Stress 5M)...");
        let mut buffer_test = [0u8; 32];
        let inicio_rendimiento = Instant::now();
        for _ in 0..5_000_000 {
            let _ = add_u64_to_u256_be(&mut buffer_test, 1);
        }
        let duracion_rendimiento = inicio_rendimiento.elapsed();
        let ops_por_segundo = 5_000_000.0 / duracion_rendimiento.as_secs_f64();

        println!("      🚀 Rendimiento: {:.2} M-sumas/seg.", ops_por_segundo / 1_000_000.0);
        diario_forense.push_str(&format!("📊 Rendimiento medido: {:.2} ops/seg.\n", ops_por_segundo));

        // 5. SENTENCIA Y DESPACHO
        let veredicto = if errores_identificados == 0 { "GOLD_MASTER" } else { "DEGRADED" };
        diario_forense.push_str(&format!("\nVEREDICTO_FINAL: {}\n", veredicto));

        despachar_reporte_aritmetico_nivelado(
            "ARITHMETIC_CORE_V120",
            veredicto,
            ops_por_segundo,
            diario_forense,
            errores_identificados,
            adx_soportado
        );

        println!("\n🏁 [INFORME]: Auditoría finalizada en {:?}. Veredicto: {}", marca_tiempo_inicio.elapsed(), veredicto);
        assert_eq!(errores_identificados, 0, "Colapso de integridad en el Kernel Aritmético.");
    }
}

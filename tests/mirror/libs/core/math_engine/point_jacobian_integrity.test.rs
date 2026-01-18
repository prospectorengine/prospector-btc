// [tests/mirror/libs/core/math_engine/point_jacobian_integrity.test.rs]
/**
 * =================================================================
 * APARATO: CERTIFICADOR DE PUNTOS GEOMÉTRICOS (V48.2 - ELITE)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L1-CORE
 * OBJETIVO: Validación de Proyección Afín y Simetría Z
 * =================================================================
 */

use prospector_core_math::prelude::*;
use std::time::Instant;
use serde_json::json;
use std::fs;

// --- VECTORES DE PRUEBA: PUNTO G (BLOQUE GÉNESIS) ---
const G_X: [u64; 4] = [0x59F2815B16F81798, 0x029BFCDB2DCE28D9, 0x55A06295CE870B07, 0x79BE667EF9DCBBAC];
const G_Y: [u64; 4] = [0x9C47D08FFB10D4B8, 0xFD17B448A6855419, 0x5DA4FBFC0E1108A8, 0x483ADA7726A3C465];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn certificar_proyeccion_y_simetria_puntos() {
        println!("\n🌐 [INICIO]: Auditoría de Proyección Jacobiana...");
        let mut total_errors = 0;
        let start_suite = Instant::now();

        // 1. PRUEBA DE IDENTIDAD AFÍN (Roundtrip Check)
        // Escenario: Cargar G (Z=1), convertir a bytes y comparar con original.
        println!("   🧪 Verificando Roundtrip Afín -> Jacobiano -> Afín...");
        let punto_jacobian = JacobianPoint::from_affine(G_X, G_Y);

        match punto_jacobian.to_affine_bytes() {
            Ok((res_x, res_y)) => {
                let esperado_x = convert_limbs_u64_to_u256_be(&G_X);
                let esperado_y = convert_limbs_u64_to_u256_be(&G_Y);

                if res_x == esperado_x && res_y == esperado_y {
                    println!("   ✅ Simetría afín certificada bit-perfect.");
                } else {
                    println!("   ❌ FALLO: Discrepancia en la reconstrucción de coordenadas.");
                    total_errors += 1;
                }
            },
            Err(e) => {
                println!("   ❌ ERROR_PROYECCIÓN: {}", e);
                total_errors += 1;
            }
        }

        // 2. PRUEBA DE ESTRATO AL INFINITO (Neutral Point)
        println!("   🧪 Verificando integridad del Punto al Infinito...");
        let infinity = JacobianPoint::infinity();
        if infinity.is_infinity && infinity.to_affine_bytes().is_err() {
            println!("   ✅ Comportamiento de singularidad (Infinity) validado.");
        } else {
            println!("   ❌ FALLO: El punto al infinito no se comporta como singularidad.");
            total_errors += 1;
        }

        // 3. PRUEBA DE RENDIMIENTO (Projection Benchmark)
        // La proyección requiere 1 Inverso Modular (operación pesada).
        println!("   🚀 Ejecutando ráfaga de 10,000 proyecciones afines...");
        let start_perf = Instant::now();
        for _ in 0..10_000 {
            // Simulamos ráfaga de hallazgos
            let _ = punto_jacobian.to_affine_bytes();
        }
        let total_time = start_perf.elapsed();
        let ops_per_sec = 10_000.0 / total_time.as_secs_f64();

        println!("   ✅ Throughput de Proyección: {:.2} ops/seg.", ops_per_sec);

        // 4. PERSISTENCIA DE REPORTE FORENSE
        let report_path = "reports/qa/point_jacobian_report.json";
        let report_data = json!({
            "aparato": "geometric_point_engine",
            "veredicto": if total_errors == 0 { "GOLD_MASTER" } else { "INTEGRITY_COMPROMISED" },
            "metricas": {
                "proyecciones_por_segundo": ops_per_sec,
                "latencia_ms": total_time.as_millis(),
                "fallos_de_simetria": total_errors
            },
            "config": {
                "coordinate_space": "Jacobian(X,Y,Z)",
                "target_curve": "secp256k1"
            },
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        fs::create_dir_all("reports/qa").ok();
        fs::write(report_path, report_data.to_string()).expect("FALLO_ESCRITURA_REPORTE");

        println!("\n💾 Reporte de puntos cristalizado en: {}", report_path);
        println!("🏁 Auditoría finalizada en {:?}.", start_suite.elapsed());
    }
}

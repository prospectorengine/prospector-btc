// [tests/mirror/apps/miner_worker/hardware_telemetry.test.rs]
/**
 * =================================================================
 * APARATO: HARDWARE TELEMETRY INTEGRITY TEST
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar la captura de métricas de silicio.
 * =================================================================
 */

use prospector_miner_lib::cpu_manager::HardwareMonitor;

#[test]
fn certify_silicon_metrics_capture() {
    println!("\n🌡️ [QA_TEST]: Validating hardware sensor acquisition...");

    let metrics = HardwareMonitor::capture_instantaneous_metrics();

    // Verificamos que los valores no sean nulos (incluso en entornos virtualizados)
    assert!(metrics.cpu_frequency_megahertz > 0, "Fallo en lectura de reloj");
    assert!(metrics.core_temperature_celsius > 0.0, "Fallo en lectura térmica");

    println!("   ✅ CPU: {} MHz", metrics.cpu_frequency_megahertz);
    println!("   ✅ Temp: {:.2} °C", metrics.core_temperature_celsius);
}

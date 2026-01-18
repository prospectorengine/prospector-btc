// INICIO DEL ARCHIVO [tests/mirror/apps/orchestrator/state/ring_buffer_integrity.test.rs]
/**
 * =================================================================
 * APARATO: TELEMETRY RING BUFFER TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar comportamiento circular O(1) del log.
 * =================================================================
 */

use prospector_orchestrator::state::swarm_telemetry::SwarmTelemetryManager;
use prospector_domain_models::telemetry::ProvisioningLog;

#[test]
fn certify_ring_buffer_rotation() {
    let telemetry = SwarmTelemetryManager::new();
    let capacity_limit = 5000;

    // 1. Saturación del Buffer
    for i in 0..(capacity_limit + 10) {
        telemetry.push_navigation_trace(ProvisioningLog {
            node_index: i as u32,
            message: format!("LOG_SEQ_{}", i),
            level: "INFO".to_string(),
            timestamp: "2026-01-01T00:00:00Z".to_string(),
        });
    }

    // 2. Validación de Estado
    let logs = telemetry.provisioning_logs.read().unwrap();

    // Debe mantener el límite exacto
    assert_eq!(logs.len(), capacity_limit, "El buffer excedió su capacidad nominal.");

    // Debe haber rotado (El primer elemento debería ser el índice 10, no 0)
    let first_log = logs.front().unwrap();
    assert_eq!(first_log.node_index, 10, "La rotación FIFO no eliminó los elementos más antiguos.");

    println!("✅ RING_BUFFER: High-speed rotation certified.");
}
// FIN DEL ARCHIVO [tests/mirror/apps/orchestrator/state/ring_buffer_integrity.test.rs]

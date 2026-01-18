// [tests/mirror/libs/domain/models_rs/telemetry_parity.test.rs]
/**
 * =================================================================
 * APARATO: TELEMETRY CONTRACT PARITY TEST
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L2-MIRROR
 * OBJETIVO: Certificar la integridad de serialización de señales.
 * =================================================================
 */

use prospector_domain_models::telemetry::*;
use serde_json::json;

#[test]
fn certify_realtime_event_serialization_parity() {
    println!("\n📡 [QA_TEST]: Validating Telemetry Signal Stratification...");

    // 1. SETUP: Crear un evento de pulso de sistema
    let metrics = SystemMetrics {
        active_nodes_count: 300,
        cumulative_global_hashrate: 120_000_000,
        active_missions_in_flight: 15,
        timestamp_ms: 1736881200000,
    };
    let event = RealTimeEvent::SystemPulseUpdate(metrics);

    // 2. EXECUTION: Serializar a JSON (Simulando despacho SSE)
    let serialized = serde_json::to_string(&event).unwrap();
    println!("   📥 JSON Payload: {}", serialized);

    // 3. VALIDATION: Verificar tags y estructura discriminada
    assert!(serialized.contains("\"t\":\"sp\""), "Tag de tipo 'sp' extraviado.");
    assert!(serialized.contains("\"active_nodes_count\":300"), "Métrica de nodos corrupta.");

    println!("   ✅ [SUCCESS]: Telemetry event serialization matches L5 expectations.");
}

#[test]
fn certify_panopticon_log_structure() {
    let log = SystemLog {
        id: "test-uuid".into(),
        timestamp: "2026-01-14T00:00:00Z".into(),
        stratum: "L1_MATH".into(),
        severity: "CRITICAL".into(),
        message: "Elliptic Curve Discontinuity detected.".into(),
        metadata: None,
        trace_id: Some("tx-99".into()),
    };

    let serialized = serde_json::to_string(&log).unwrap();
    assert!(serialized.contains("L1_MATH"), "Estrato de log no preservado.");
    println!("   ✅ [SUCCESS]: Panopticon log structure verified.");
}

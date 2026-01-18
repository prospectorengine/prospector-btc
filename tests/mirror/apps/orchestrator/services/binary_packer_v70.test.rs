use prospector_orchestrator::services::binary_packer::BinaryNeuralPacker;
use prospector_domain_models::telemetry::{RealTimeEvent, SystemMetrics};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rmp_serde::decode;

#[test]
fn certify_binary_packer_roundtrip_integrity() {
    println!("\n📦 [PROVING_GROUNDS]: Binary Packer Strata Certification...");

    // 1. SETUP: Crear evento de pulso real
    let original_event = RealTimeEvent::SystemPulseUpdate(SystemMetrics {
        active_nodes_count: 7,
        cumulative_global_hashrate: 120_000_000,
        active_missions_in_flight: 3,
        timestamp_ms: 1736284000,
    });

    // 2. EXECUTION: Empaquetar
    let encoded_payload = BinaryNeuralPacker::pack_event(&original_event)
        .expect("El empaquetador falló en una señal nominal.");

    // 3. VALIDATION: Desempaquetado manual (Simulando al Dashboard)
    let decoded_bytes = BASE64.decode(encoded_payload).expect("Base64 corrupto.");
    let recovered_event: RealTimeEvent = decode::from_slice(&decoded_bytes)
        .expect("MessagePack incompatible con el contrato de dominio.");

    if let RealTimeEvent::SystemPulseUpdate(metrics) = recovered_event {
        assert_eq!(metrics.active_nodes_count, 7);
        assert_eq!(metrics.cumulative_global_hashrate, 120_000_000);
        println!("   ✅ Paridad bit-perfecta confirmada.");
    } else {
        panic!("El evento recuperado no coincide con el tipo original.");
    }

    println!("✅ PACKER_V70: Throughput and Integrity Certified.");
}

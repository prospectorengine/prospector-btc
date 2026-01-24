// [tests/mirror/libs/domain/models_rs/telemetry_visual_sync.test.rs]
/**
 * =================================================================
 * APARATO: TELEMETRY VISUAL SYNC TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L2-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE CONTRATO DE VIDEO PROXY
 * =================================================================
 */

 use prospector_domain_models::telemetry::RealTimeEvent;
 use serde_json::json;

 #[test]
 fn certify_visual_event_serialization_parity() {
     println!("\n📡 [PROVING_GROUNDS]: Auditing Visual Telemetry Contract...");

     // 1. SETUP: Crear evento de frame visual
     let visual_event = RealTimeEvent::NodeVisualFrameReady {
         worker_identifier: "node-01".into(),
         operational_status: "running".into(),
         snapshot_base64_data: "data:image/jpeg;base64,ABC".into(),
         system_timestamp: 1737700000000,
     };

     // 2. EXECUTION: Serialización a JSON (Simulando despacho Neural Link)
     let serialized = serde_json::to_string(&visual_event).unwrap();
     println!("   📥 JSON Output: {}", serialized);

     // 3. VALIDATION: Verificar que el campo 'snapshot_base64_data' reside en el payload 'p'
     assert!(serialized.contains("snapshot_base64_data"), "L2_CONTRACT_FAULT: Visual data field missing.");
     assert!(serialized.contains("data:image/jpeg;base64,ABC"));

     println!("   ✅ [SUCCESS]: Visual telemetry contract is level and synchronized.");
 }

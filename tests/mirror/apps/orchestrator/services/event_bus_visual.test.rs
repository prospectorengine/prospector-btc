// [tests/mirror/apps/orchestrator/services/event_bus_visual.test.rs]
/**
 * =================================================================
 * APARATO: EVENT BUS VISUAL SYNC TEST (V1.1 - RECOVERY READY)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L4-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE PROPAGACIÓN DE IMAGEN
 * =================================================================
 */

 use prospector_orchestrator::services::event_bus::EventBus;
 use prospector_domain_models::telemetry::RealTimeEvent;

 #[tokio::test]
 async fn certify_visual_frame_broadcasting_integrity_v87() {
     println!("\n📡 [PROVING_GROUNDS]: Auditing EventBus Visual Strata V87.1...");

     let bus = EventBus::new();
     let mut rx = bus.subscribe();

     let mock_image = "data:image/jpeg;base64,GOLD_MASTER_PAYLOAD".to_string();

     // 2. EXECUTION: Emitir señal con imagen y metadatos
     bus.emit_visual_frame_signal(
         "UNIT-01".into(),
         "running".into(),
         mock_image.clone(),
         1737700000000
     );

     // 3. VALIDATION
     let received = rx.recv().await.expect("Fallo al recibir señal");

     if let RealTimeEvent::NodeVisualFrameReady { snapshot_base64_data, .. } = received {
         assert_eq!(snapshot_base64_data, mock_image, "L4_BUS_FAULT: Visual data drift detected.");
         println!("   ✅ [SUCCESS]: Visual frame propagated bit-perfectly.");
     } else {
         panic!("Tipo de evento incorrecto recibido.");
     }
 }

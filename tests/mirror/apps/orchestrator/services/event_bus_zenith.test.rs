// [tests/mirror/apps/orchestrator/services/event_bus_zenith.test.rs]
/**
 * =================================================================
 * APARATO: EVENT BUS ZENITH TEST
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L4-MIRROR
 * OBJETIVO: Certificar la propagación íntegra de señales multicanal.
 * =================================================================
 */

use prospector_orchestrator::services::event_bus::EventBus;
use prospector_domain_models::telemetry::RealTimeEvent;

#[tokio::test]
async fn certify_multi_channel_signal_propagation() {
    println!("\n📡 [PROVING_GROUNDS]: Neural Event Bus Audit...");

    let bus = EventBus::new();
    let mut subscriber = bus.subscribe();

    // 1. EXECUTION: Emitir señal de Visual Frame (Nueva en V82)
    bus.emit_visual_frame_signal("UNIT-01".into(), "running".into(), 1736881200);

    // 2. VALIDATION
    let event = subscriber.recv().await.expect("Fallo al recibir evento del bus.");

    if let RealTimeEvent::NodeVisualFrameReady { worker_identifier, .. } = event {
        assert_eq!(worker_identifier, "UNIT-01");
        println!("   ✅ [SUCCESS]: Visual Frame signal captured accurately.");
    } else {
        panic!("Tipo de evento incorrecto recibido.");
    }

    // 3. EXECUTION: Emitir alerta de Ban Shield
    bus.emit_ban_shield_alert(prospector_domain_models::telemetry::BanShieldStatus {
        identities_in_vault: 1,
        safe_node_capacity: 3,
        is_ignition_authorized: false,
        restriction_reason: Some("DENSITY_FAULT".into()),
    });

    let event_shield = subscriber.recv().await.unwrap();
    if let RealTimeEvent::BanShieldUpdate(status) = event_shield {
        assert!(!status.is_ignition_authorized);
        println!("   ✅ [SUCCESS]: Ban Shield alert propagated.");
    }

    println!("✅ EVENT_BUS_V82: Multi-strata signaling certified.");
}

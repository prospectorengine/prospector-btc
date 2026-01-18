use prospector_orchestrator::state::operational_nexus::{OperationalNexusManager, SwarmOperationalMode};
use prospector_orchestrator::services::event_bus::EventBus;
use std::sync::Arc;

#[tokio::test]
async fn certify_nexus_reactivity() {
    println!("\n🛡️ [PROVING_GROUNDS]: Operational Nexus Reactivity Audit...");

    let event_bus = Arc::new(EventBus::new());
    let mut rx = event_bus.subscribe();
    let nexus = OperationalNexusManager::new(event_bus);

    // 1. Activar cambio de modo
    nexus.transition_mode(SwarmOperationalMode::SecurityHalt, "CRITICAL_VOLTAGE_DROP");

    // 2. Verificar que el Bus capturó la señal para el Dashboard
    let event = rx.recv().await.unwrap();
    if let prospector_domain_models::telemetry::RealTimeEvent::SystemLog(log) = event {
        assert!(log.message.contains("SecurityHalt"));
        assert!(log.message.contains("CRITICAL_VOLTAGE_DROP"));
        println!("✅ NEXUS_V190: State transition and event propagation certified.");
    } else {
        panic!("El Nexo no emitió el log de transición.");
    }
}

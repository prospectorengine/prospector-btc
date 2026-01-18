#[tokio::test]
async fn certify_ban_shield_prevents_overload() {
    let telemetry = SwarmTelemetryManager::new();

    // Escenario: 10 hilos solicitados, 2 cookies disponibles (Capacidad real = 6)
    let result = telemetry.validate_ignition_capacity(10, 2);

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("INSUFFICIENT_RESOURCES"));
    println!("✅ BAN_SHIELD: Prevented high-risk ignition.");
}

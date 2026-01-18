#[tokio::test]
async fn certify_routing_to_provisioning_log() {
    let state = setup_environment().await;
    let app = create_sovereign_router(state);

    // Simulación de ráfaga de red desde GitHub
    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/admin/provisioning/log")
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer test_master_token")
        .body(Body::from(json!({
            "node_index": 0,
            "message": "IGNITION_START",
            "level": "INFO",
            "timestamp": "2026-01-10T12:00:00Z"
        }).to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Verificación de existencia del endpoint
    assert_ne!(response.status(), StatusCode::NOT_FOUND, "Ruta /provisioning/log no registrada.");
    assert_eq!(response.status(), StatusCode::ACCEPTED, "El endpoint no aceptó el log legal.");
}

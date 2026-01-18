#[tokio::test]
async fn certify_provisioning_log_acceptance() {
    let state = setup_environment().await; // In-Memory DB
    let app = create_sovereign_router(state.clone());

    let payload = json!({
        "node_index": 1,
        "message": "TEST_LOG",
        "level": "INFO",
        "timestamp": "2026-01-10T12:00:00Z"
    });

    let response = app.oneshot(
        Request::builder()
            .method("POST")
            .uri("/api/v1/admin/provisioning/log")
            .header("Content-Type", "application/json")
            .header("Authorization", "Bearer test_token")
            .body(Body::from(payload.to_string()))
            .unwrap()
    ).await.unwrap();

    assert_eq!(response.status(), StatusCode::ACCEPTED);
    assert_eq!(state.swarm_telemetry.provisioning_logs.read().unwrap().len(), 1);
}

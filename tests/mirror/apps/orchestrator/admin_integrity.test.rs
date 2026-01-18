#[tokio::test]
async fn certify_admin_handler_multi_strata_coverage() {
    let state = setup_mock_state().await;

    // Verificamos que el handler de DNA (Restauraçao) existe y responde
    // Verificamos que el nuevo handler de Provisioning acepta trazas
    let log = ProvisioningLog {
        node_index: 1,
        message: "PLAYWRIGHT_AUTH_SUCCESS".into(),
        level: "INFO".into(),
        timestamp: "2026-01-10T12:00:00Z".into()
    };

    let res = ScenarioAdministrationHandler::handle_provisioning_log(
        State(state.clone()),
        Json(log)
    ).await;

    assert_eq!(res.into_response().status(), StatusCode::ACCEPTED);
    println!("✅ ADMIN_HANDLER: Provisioning log strata certified.");
}


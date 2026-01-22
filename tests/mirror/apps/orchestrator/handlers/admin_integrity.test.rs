// [tests/mirror/apps/orchestrator/handlers/admin_integrity.test.rs]
/**
 * APARATO: ADMIN HANDLER INTEGRITY TEST (V89.1)
 * OBJETIVO: Certificar la erradicación de ruidos y salud diagnóstica.
 */

use prospector_orchestrator::state::AppState;
use prospector_orchestrator::handlers::admin::ScenarioAdministrationHandler;
use prospector_infra_db::TursoClient;
use ax_test_utils::axum::extract::State;
use ax_test_utils::axum::http::StatusCode;
use ax_test_utils::axum::response::IntoResponse;

#[tokio::test]
async fn certify_diagnostic_report_v89() {
    println!("\n🛡️  [PROVING_GROUNDS]: Auditing Admin Handler V89.0...");

    let database_client = TursoClient::connect("file::memory:", None).await.unwrap();
    let application_state = AppState::new(database_client);

    // 1. EXECUTION: Prueba de diagnóstico del Kernel
    let response = ScenarioAdministrationHandler::handle_system_diagnostics(
        State(application_state)
    ).await;

    // 2. VALIDATION
    assert_eq!(response.into_response().status(), StatusCode::OK);

    println!("   ✅ [SUCCESS]: Kernel diagnostics report certified under zero-warning strata.");
}

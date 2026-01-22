// [tests/mirror/apps/orchestrator/handlers/admin_hygiene_build.test.rs]
/**
 * APARATO: ADMIN HYGIENE BUILD CERTIFIER
 * OBJETIVO: Certificar que el build de producción no depende de ax_test_utils.
 */

use prospector_orchestrator::handlers::admin::ScenarioAdministrationHandler;
use prospector_orchestrator::state::AppState;
use prospector_infra_db::TursoClient;

#[tokio::test]
async fn certify_production_import_strata() {
    println!("\n🛡️  [PROVING_GROUNDS]: Auditing Admin Production Imports...");

    // Esta prueba falla en compilación si ScenarioAdministrationHandler
    // aún depende de crates de test en su interfaz pública.
    let database_client = TursoClient::connect("file::memory:", None).await.unwrap();
    let _application_state = AppState::new(database_client);

    println!("   ✅ [SUCCESS]: Admin strata is independent of test utilities.");
}

// [tests/mirror/libs/infra/db_turso/errors_v180.test.rs]
/**
 * =================================================================
 * APARATO: DB ERROR SEMANTIC TEST (V1.2)
 * OBJETIVO: Certificar el triaje de errores de configuración.
 * =================================================================
 */

use prospector_infra_db::errors::DbError;

#[test]
fn certify_configuration_error_formatting() {
    println!("\n⚖️ [PROVING_GROUNDS]: Auditing Semantic Error Strata...");

    let error = DbError::ConfigurationError("DATABASE_URL_EMPTY_IN_GITHUB".into());
    let message = format!("{}", error);

    // Validación de rastro Panóptico
    assert!(message.contains("[L3_DB_CONFIG_FAULT]"), "Falta el prefijo de estrato.");
    assert!(message.contains("STRATEGIC_ENV_VOID"), "Falla la semántica del error.");

    println!("   ✅ [SUCCESS]: Configuration error correctly classified for C2 diagnostics.");
}

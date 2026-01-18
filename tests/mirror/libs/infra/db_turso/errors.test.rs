// [tests/mirror/libs/infra/db_turso/errors.test.rs]
#[cfg(test)]
mod tests {
    use prospector_infra_db::errors::DbError;

    #[test]
    fn certify_error_formatting_semantic() {
        let error = DbError::OwnershipConflict;
        let message = format!("{}", error);

        // Verificamos que el prefijo del estrato sea correcto para el Panóptico
        assert!(message.contains("[L3_MISSION_FAULT]"));
        assert!(message.contains("OWNERSHIP_VIOLATION"));
    }

    #[test]
    fn certify_governance_variants_existence() {
        // Esta prueba asegura que no hubo regresiones eliminando tipos de gobernanza
        let _error = DbError::IdentityLeaseExpired;
        let _error2 = DbError::IdentityNotFound;
        assert!(true);
    }
}

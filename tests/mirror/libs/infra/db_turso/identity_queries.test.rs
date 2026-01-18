// [tests/mirror/libs/infra/db_turso/identity_queries.test.rs]
#[cfg(test)]
mod tests {
    use prospector_infra_db::repositories::identity::queries::*;

    #[test]
    fn certify_query_placeholders_parity() {
        // Validación de firmas para evitar errores de bind en tiempo de ejecución
        assert!(LEASE_SOVEREIGN_IDENTITY.contains("?1")); // Platform
        assert!(LEASE_SOVEREIGN_IDENTITY.contains("?2")); // Minutes
        assert!(LEASE_SOVEREIGN_IDENTITY.contains("?3")); // UserAgent

        assert!(REFRESH_IDENTITY_CREDENTIALS.contains("?1")); // Email
        assert!(REFRESH_IDENTITY_CREDENTIALS.contains("?2")); // JSON

        println!("✅ SQL_PARITY: Query placeholders certified for V36.0.");
    }
}

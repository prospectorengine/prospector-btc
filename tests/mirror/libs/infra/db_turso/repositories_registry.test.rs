// [tests/mirror/libs/infra/db_turso/repositories_registry.test.rs]
#[cfg(test)]
mod tests {
    use prospector_infra_db::repositories::*;

    #[test]
    fn certify_repository_matrix_completeness() {
        // Esta prueba de compilación garantiza que todos los re-exports
        // nominales están presentes y son accesibles.
        println!("🚀 Verificando registro de repositorios...");

        // La simple existencia de estos tipos en el scope certifica el mod.rs
        let _ = std::any::TypeId::of::<MissionRepository>();
        let _ = std::any::TypeId::of::<IdentityRepository>();
        let _ = std::any::TypeId::of::<ScenarioRegistryRepository>();
        let _ = std::any::TypeId::of::<AuditRepository>();

        println!("✅ REPOSITORY_MATRIX: Todos los aparatos están registrados.");
    }
}

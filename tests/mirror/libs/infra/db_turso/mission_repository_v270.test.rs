// [tests/mirror/libs/infra/db_turso/mission_repository_v270.test.rs]
/*!
 * =================================================================
 * APARATO: MISSION REPOSITORY ADMIN TEST (V1.1 - SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L3-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE RESET ESTRUCTURAL Y PURGA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. NOMINAL PURITY: Erradicación de abreviaciones (repo -> mission_repository).
 * 2. ATOMIC VERIFICATION: Valida la transición 'Tabula Rasa' tras la purga.
 * 3. HYGIENE: Cero rastro residual en memoria compartida.
 * =================================================================
 */

#[cfg(test)]
mod tests {
    use prospector_infra_db::repositories::MissionRepository;
    use prospector_infra_db::TursoClient;

    /**
     * CERTIFICACIÓN: Lógica de purga administrativa y reset estructural.
     */
    #[tokio::test]
    async fn certify_administrative_purge_and_reset_logic() {
        println!("\n🗑️ [PROVING_GROUNDS]: Initiating Administrative Purge Audit...");

        // 1. SETUP: Inicialización de enlace táctico en RAM aislada
        let database_client = TursoClient::connect("file:mem_purge_v270?mode=memory&cache=shared", None)
            .await
            .expect("FALLO_CRÍTICO: No se pudo anclar el Ledger en RAM.");

        let mission_repository = MissionRepository::new(database_client.clone());
        let database_connection = database_client.get_connection().unwrap();

        // 2. ESCENARIO: Inyectar misiones en diversos estratos de estado
        println!("   🧪 Phase 1: Hydrating tactical strata with mock missions...");
        database_connection.execute(
            "INSERT INTO jobs (id, range_start, range_end, status) VALUES ('M1_QUEUED', '0', '1', 'queued')",
            ()
        ).await.unwrap();

        database_connection.execute(
            "INSERT INTO jobs (id, range_start, range_end, status) VALUES ('M2_ACTIVE', '0', '1', 'active')",
            ()
        ).await.unwrap();

        // 3. EXECUTION: Disparar incineración de registros (V300.8)
        println!("   🚀 Phase 2: Executing Tabula Rasa Protocol...");
        let records_purged_count = mission_repository.purge_and_reset_system().await
            .expect("PURGE_REJECTED: Administrative reset failed.");

        // 4. VALIDATION: Verificación de paridad bit-perfecta
        assert_eq!(
            records_purged_count,
            2,
            "INTEGRITY_FAULT: Purge count mismatch. Expected 2, found {}.",
            records_purged_count
        );

        // Verificación de rastro nulo en el Ledger
        let mut check_query = database_connection.query("SELECT count(*) FROM jobs", ()).await.unwrap();
        let remaining_count: i64 = check_query.next().await.unwrap().unwrap().get(0).unwrap();

        assert_eq!(remaining_count, 0, "SIGNAL_RESIDUE: Tactical strata still contains data after purge.");

        println!("   ✅ [SUCCESS]: Administrative reset certified. Strata is now lean.");
        println!("🏁 [COMPLETE]: Mission Repository V270 test finalized.\n");
    }
}

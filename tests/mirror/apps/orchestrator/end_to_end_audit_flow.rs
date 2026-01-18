// INICIO DEL ARCHIVO [apps/orchestrator/tests/end_to_end_audit_flow.rs]
#[cfg(test)]
mod e2e_tests {
    use prospector_orchestrator::state::AppState;
    use prospector_domain_strategy::ProjectiveSequentialEngine;
    use prospector_infra_db::TursoClient;
    use prospector_infra_db::repositories::MissionRepository;
    use prospector_core_probabilistic::sharded::ShardedFilter;
    // ✅ FIX: 'std::sync' imports eliminados por falta de uso
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicU64};

    struct MockHandler;
    impl prospector_domain_strategy::FindingHandler for MockHandler {
        fn on_finding(&self, _: String, _: prospector_core_math::private_key::SafePrivateKey, _: String) {}
    }

    #[tokio::test]
    async fn certify_full_audit_to_archival_cycle() {
        let client = TursoClient::connect("file::memory:", None).await.unwrap();
        let state = AppState::new(client.clone());
        let mission_repo = MissionRepository::new(client);

        let effort = Arc::new(AtomicU64::new(0));
        let filter = ShardedFilter::new(1, 100, 0.1);

        let checkpoint = ProjectiveSequentialEngine::execute_optimized_audit(
            "0000000000000000000000000000000000000000000000000000000000000001",
            1000,
            &filter,
            &AtomicBool::new(false),
            effort.clone(),
            &MockHandler
        );

        let report = prospector_domain_models::work::AuditReport {
            job_mission_identifier: "test-mission-001".into(),
            worker_node_identifier: "test-node".into(),
            total_wallets_audited: effort.load(std::sync::atomic::Ordering::SeqCst).to_string(),
            execution_duration_milliseconds: 100,
            final_mission_status: "completed".into(),
            audit_footprint_checkpoint: checkpoint,
            completed_at_timestamp: chrono::Utc::now().to_rfc3339(),
            average_computational_efficiency: 10.0,
        };

        state.database_client.get_connection().unwrap().execute(
            "INSERT INTO jobs (id, range_start, range_end, status) VALUES ('test-mission-001', '1', '1000', 'active')",
            ()
        ).await.unwrap();

        mission_repo.certify_mission_completion(&report).await.unwrap();

        let archival_repo = prospector_infra_db::repositories::ArchivalRepository::new(state.database_client.clone());
        let pending = archival_repo.fetch_pending_strategic_migration(10).await.unwrap();

        assert_eq!(pending.len(), 1, "OutboxRelay should see 1 pending mission");
        assert_eq!(pending[0]["computational_effort"], "1000", "Volume must match sequential effort");

        println!("✅ [CERTIFIED]: Sequential -> Tactical -> Archival flow verified.");
    }
}
// FIN DEL ARCHIVO [apps/orchestrator/tests/end_to_end_audit_flow.rs]

/**
 * =================================================================
 * APARATO: EXECUTOR DISPATCH TEST (V250.6 - SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * RESPONSABILIDAD: CERTIFICACIÓN DE ORQUESTACIÓN POLIMÓRFICA
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicBool};
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_core_math::prelude::*;
use prospector_domain_models::work::{WorkOrder, SearchStrategy, TargetStrata};
use prospector_domain_strategy::{StrategyExecutor, FindingHandler};

struct MockFindingSiphon;
impl FindingHandler for MockFindingSiphon {
    fn on_finding(&self, _: String, _: SafePrivateKey, _: String) {}
}

#[test]
fn certify_polymorphic_executor_report_integrity() {
    let mission_order = WorkOrder {
        job_mission_identifier: "test-mission-master-001".into(),
        lease_duration_seconds: 60,
        strategy: SearchStrategy::Sequential {
            start_index_hexadecimal: "0000000000000000000000000000000000000000000000000000000000000001".into(),
            end_index_hexadecimal: "000000000000000000000000000000000000000000000000000000000000FFFF".into(),
        },
        required_strata: TargetStrata::SatoshiEra,
    };

    let target_filter = ShardedFilter::new(1, 100, 0.01);
    let effort_accumulator = Arc::new(AtomicU64::new(0));
    let stop_signal = Arc::new(AtomicBool::new(false));
    let siphon_handler = MockFindingSiphon;

    let final_audit_report = StrategyExecutor::execute_mission_sequence(
        &mission_order,
        &target_filter,
        stop_signal,
        effort_accumulator.clone(),
        "unit-test-node-sovereign".into(),
        &siphon_handler,
        None
    );

    assert_eq!(final_audit_report.job_mission_identifier, "test-mission-master-001");
    assert_eq!(final_audit_report.final_mission_status, "completed");
    println!("✅ EXECUTOR: Polymorphic dispatch and report generation certified.");
}

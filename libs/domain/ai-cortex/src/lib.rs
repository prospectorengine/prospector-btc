// [libs/domain/ai-cortex/src/lib.rs]
pub mod errors;
pub mod decision_engine;
pub mod telemetry_observer;
pub mod optimization_controller;

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CognitiveVerdict {
    OptimalPerformance,
    OptimizationRequired,
    SuspiciousBehaviorDetected,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetrySnapshot {
    pub current_hashrate: u64,
    pub cpu_temperature_celsius: f32,
    pub cpu_load_percentage: f32,
    pub timestamp_ms: u64,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationDirective {
    pub target_worker_identifier: Option<String>,
    pub recommended_batch_size: u32,
    pub suggest_pacing_delay_milliseconds: u64,
    pub reasoning_metadata: String,
}

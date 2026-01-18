use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerHeartbeat {
    #[serde(rename = "worker_identifier")]
    pub worker_id: String,

    #[serde(rename = "hostname_identity")]
    pub hostname: String,

    #[serde(rename = "current_hashrate")]
    #[typeshare(serialized_as = "number")]
    pub hashrate: u64,

    #[serde(rename = "active_job_identifier")]
    pub current_job_id: Option<String>,

    #[serde(rename = "timestamp_utc")]
    // ✅ FIX: Instrucción explícita para que Typescript vea esto como string
    #[typeshare(serialized_as = "String")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "hardware_metrics")]
    pub hardware_stats: HardwareStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[typeshare]
pub struct HardwareStats {
    #[serde(rename = "cpu_frequency_megahertz")]
    pub cpu_frequency_mhz: u32,
    #[serde(rename = "cpu_load_percentage")]
    pub cpu_load_percent: f32,
    #[serde(rename = "cpu_temperature_celsius")]
    pub thermal_celsius: f32,
    #[serde(rename = "ram_usage_megabytes")]
    #[typeshare(serialized_as = "number")]
    pub memory_used_mb: u64,
    pub core_count: u32,
    #[serde(rename = "is_thermal_throttling_active")]
    pub is_throttling: bool,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerSnapshot {
    #[serde(rename = "worker_identifier")]
    pub worker_id: String,
    #[serde(rename = "operational_status")]
    pub status: String,
    #[serde(rename = "snapshot_base64_data")]
    pub snapshot_base64: String,
    #[serde(rename = "captured_at_timestamp")]
    pub timestamp: String,
    #[serde(rename = "hardware_metrics", skip_serializing_if = "Option::is_none")]
    pub hardware: Option<HardwareStats>,
}

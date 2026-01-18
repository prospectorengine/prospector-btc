// INICIO DEL ARCHIVO [apps/orchestrator/src/state/swarm_telemetry.rs]
/*!
 * =================================================================
 * APARATO: SWARM TELEMETRY & PANOPTICON STATE (V16.0 - DUAL BUFFER)
 * CLASIFICACIÓN: APPLICATION STATE (ESTRATO L3)
 * RESPONSABILIDAD: MEMORIA A CORTO PLAZO DE TRAZAS Y LOGS
 *
 * MEJORA TÁCTICA:
 * Se añade 'unified_system_logs' (VecDeque) para retener los últimos
 * 2000 eventos del sistema global, separado de los logs de provisionamiento.
 * =================================================================
 */

use std::collections::{HashMap, VecDeque};
use std::sync::RwLock;
use prospector_domain_models::worker::{WorkerHeartbeat, WorkerSnapshot};
use prospector_domain_models::telemetry::{ProvisioningLog, SystemLog};
use tracing::{warn, debug};

const PROVISIONING_LOG_CAPACITY: usize = 5000;
const SYSTEM_LOG_CAPACITY: usize = 2000; // ✅ Nueva capacidad para logs generales

pub struct SwarmTelemetryManager {
    pub active_nodes_telemetry: RwLock<HashMap<String, WorkerHeartbeat>>,
    pub visual_surveillance_frames: RwLock<HashMap<String, WorkerSnapshot>>,
    pub provisioning_logs: RwLock<VecDeque<ProvisioningLog>>,
    // ✅ NUEVO: Buffer para el Panóptico Global
    pub unified_system_logs: RwLock<VecDeque<SystemLog>>,
}

impl Default for SwarmTelemetryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SwarmTelemetryManager {
    pub fn new() -> Self {
        Self {
            active_nodes_telemetry: RwLock::new(HashMap::new()),
            visual_surveillance_frames: RwLock::new(HashMap::new()),
            provisioning_logs: RwLock::new(VecDeque::with_capacity(PROVISIONING_LOG_CAPACITY)),
            unified_system_logs: RwLock::new(VecDeque::with_capacity(SYSTEM_LOG_CAPACITY)),
        }
    }

    pub fn is_node_healthy(&self, worker_identifier: &str) -> bool {
        let telemetry_guard = self.active_nodes_telemetry.read().expect("LOCK_POISONED");
        if let Some(heartbeat) = telemetry_guard.get(worker_identifier) {
            let is_thermal_stable = heartbeat.hardware_stats.thermal_celsius < 90.0;
            let is_load_acceptable = heartbeat.hardware_stats.cpu_load_percent < 98.0;
            return is_thermal_stable && is_load_acceptable;
        }
        true
    }

    pub fn synchronize_heartbeat(&self, heartbeat: WorkerHeartbeat) {
        let mut telemetry_guard = self.active_nodes_telemetry.write().expect("LOCK_POISONED");
        if heartbeat.hardware_stats.thermal_celsius > 85.0 {
            warn!("🔥 [THERMAL_ALERT]: Node [{}] critical: {}°C",
                heartbeat.worker_id, heartbeat.hardware_stats.thermal_celsius);
        }
        debug!("📡 [TELEMETRY]: Pulse sync for unit [{}].", heartbeat.worker_id);
        telemetry_guard.insert(heartbeat.worker_id.clone(), heartbeat);
    }

    pub fn push_navigation_trace(&self, log: ProvisioningLog) {
        let mut logs_guard = self.provisioning_logs.write().expect("LOCK_POISONED");
        if logs_guard.len() >= PROVISIONING_LOG_CAPACITY {
            logs_guard.pop_front();
        }
        logs_guard.push_back(log);
    }

    // ✅ NUEVO: Ingesta O(1) para el Log Unificado
    pub fn push_system_log(&self, log: SystemLog) {
        let mut logs_guard = self.unified_system_logs.write().expect("LOCK_POISONED");
        if logs_guard.len() >= SYSTEM_LOG_CAPACITY {
            logs_guard.pop_front();
        }
        logs_guard.push_back(log);
    }

    pub fn validate_ignition_capacity(&self, requested_nodes: u32, available_identities: u32) -> Result<u32, String> {
        let safe_node_ratio = 3;
        let total_capacity = available_identities * safe_node_ratio;
        if requested_nodes > total_capacity && requested_nodes > 0 {
            return Err(format!("BAN_RISK: Requested {} > Capacity {}", requested_nodes, total_capacity));
        }
        Ok(total_capacity)
    }
}
// FIN DEL ARCHIVO [apps/orchestrator/src/state/swarm_telemetry.rs]

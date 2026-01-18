// [apps/orchestrator/src/state/swarm_telemetry.rs]
/**
 * =================================================================
 * APARATO: SWARM TELEMETRY & PANOPTICON STATE (V17.0 - SOBERANO)
 * CLASIFICACIÓN: APPLICATION STATE (ESTRATO L3)
 * RESPONSABILIDAD: MEMORIA CIRCULAR DE TRAZAS Y BIOMETRÍA DE NODOS
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * Actúa como la memoria a corto plazo del Orquestador.
 * 1. O(1) STABILITY: Pre-alocación de buffers circulares para evitar re-alocaciones.
 * 2. PANOPTICON SYNC: Gestión segregada de logs de aprovisionamiento y sistema.
 * 3. HEALTH VETO: Lógica de evaluación de hardware con reporte semántico.
 * 4. HYGIENE: Cero abreviaciones y documentación técnica nivel Tesis.
 * =================================================================
 */

use std::collections::{HashMap, VecDeque};
use std::sync::RwLock;
use prospector_domain_models::worker::{WorkerHeartbeat, WorkerSnapshot};
use prospector_domain_models::telemetry::{ProvisioningLog, SystemLog};
use tracing::{warn, debug, instrument};

/// Capacidad máxima del buffer de navegación (L6).
const PROVISIONING_LOG_CAPACITY: usize = 5000;
/// Capacidad máxima del buffer unificado del Panóptico (L1-L6).
const SYSTEM_LOG_CAPACITY: usize = 2000;

pub struct SwarmTelemetryManager {
    /// Telemetría activa de los nodos identificados en la rejilla.
    pub active_nodes_telemetry: RwLock<HashMap<String, WorkerHeartbeat>>,
    /// Almacenamiento volátil de fragmentos visuales (Video-Stream Proxy).
    pub visual_surveillance_frames: RwLock<HashMap<String, WorkerSnapshot>>,
    /// Rastro de navegación del automatizador C2 (Sentinel).
    pub provisioning_logs: RwLock<VecDeque<ProvisioningLog>>,
    /// Sumidero circular de eventos del sistema global.
    pub unified_system_logs: RwLock<VecDeque<SystemLog>>,
}

impl Default for SwarmTelemetryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SwarmTelemetryManager {
    /**
     * Construye una nueva instancia del gestor de telemetría.
     *
     * # Performance:
     * Realiza pre-alocación de memoria (VecDeque::with_capacity) para garantizar
     * que la inserción de logs sea una operación de tiempo constante puro.
     */
    pub fn new() -> Self {
        Self {
            active_nodes_telemetry: RwLock::new(HashMap::with_capacity(300)),
            visual_surveillance_frames: RwLock::new(HashMap::with_capacity(300)),
            provisioning_logs: RwLock::new(VecDeque::with_capacity(PROVISIONING_LOG_CAPACITY)),
            unified_system_logs: RwLock::new(VecDeque::with_capacity(SYSTEM_LOG_CAPACITY)),
        }
    }

    /**
     * Evalúa la viabilidad operativa de un nodo basándose en su biometría.
     *
     * # Logic:
     * Un nodo es considerado 'No Saludable' si:
     * 1. Supera los 90.0°C (Riesgo de degradación de silicio).
     * 2. Mantiene una carga de CPU > 98% (Saturación de hilos).
     */
    pub fn is_node_healthy(&self, worker_identifier: &str) -> bool {
        let telemetry_guard = self.active_nodes_telemetry.read()
            .expect("LOCK_POISONED: Swarm telemetry strata is compromised.");

        if let Some(heartbeat) = telemetry_guard.get(worker_identifier) {
            let is_thermal_stable = heartbeat.hardware_stats.thermal_celsius < 90.0;
            let is_load_acceptable = heartbeat.hardware_stats.cpu_load_percent < 98.0;

            if !is_thermal_stable {
                warn!("🛡️ [HEALTH_VETO]: Node {} rejected. Critical temperature: {}°C",
                    worker_identifier, heartbeat.hardware_stats.thermal_celsius);
            }

            return is_thermal_stable && is_load_acceptable;
        }
        true // Los nodos nuevos son aceptados por defecto
    }

    /**
     * Sincroniza el latido de un worker con el estado global.
     */
    #[instrument(skip(self, heartbeat), fields(node = %heartbeat.worker_id))]
    pub fn synchronize_heartbeat(&self, heartbeat: WorkerHeartbeat) {
        let mut telemetry_guard = self.active_nodes_telemetry.write()
            .expect("LOCK_POISONED: Swarm telemetry write failed.");

        if heartbeat.hardware_stats.thermal_celsius > 85.0 {
            warn!("🔥 [THERMAL_ALERT]: Node [{}] approaching critical strata: {}°C",
                heartbeat.worker_id, heartbeat.hardware_stats.thermal_celsius);
        }

        debug!("📡 [TELEMETRY_SYNC]: Pulse registered for unit [{}].", heartbeat.worker_id);
        telemetry_guard.insert(heartbeat.worker_id.clone(), heartbeat);
    }

    /**
     * Inserta una traza de navegación del automatizador C2.
     * Implementa rotación de buffer circular FIFO.
     */
    pub fn push_navigation_trace(&self, provisioning_log: ProvisioningLog) {
        let mut logs_guard = self.provisioning_logs.write()
            .expect("LOCK_POISONED: Provisioning buffer inaccessible.");

        if logs_guard.len() >= PROVISIONING_LOG_CAPACITY {
            logs_guard.pop_front();
        }
        logs_guard.push_back(provisioning_log);
    }

    /**
     * Inyecta una entrada en el sumidero unificado del Panóptico.
     *
     * # Performance:
     * Operación O(1). Esencial para la visibilidad 360° del Dashboard Zenith.
     */
    pub fn push_system_log(&self, system_log: SystemLog) {
        let mut logs_guard = self.unified_system_logs.write()
            .expect("LOCK_POISONED: System log buffer inaccessible.");

        if logs_guard.len() >= SYSTEM_LOG_CAPACITY {
            logs_guard.pop_front();
        }
        logs_guard.push_back(system_log);
    }

    /**
     * Valida la capacidad de ignición para prevenir baneos por densidad de red.
     *
     * # Mathematical Proof:
     * El ratio de seguridad Hydra es de 1 identidad : 3 nodos. Superar este límite
     * aumenta exponencialmente la probabilidad de detección por comportamiento no humano.
     */
    pub fn validate_ignition_capacity(&self, requested_nodes: u32, available_identities: u32) -> Result<u32, String> {
        const SAFE_NODE_PER_IDENTITY_RATIO: u32 = 3;
        let total_safe_capacity = available_identities * SAFE_NODE_PER_IDENTITY_RATIO;

        if requested_nodes > total_safe_capacity && requested_nodes > 0 {
            return Err(format!("BAN_RISK_DETECTED: Requested {} exceeds safe capacity of {}.",
                requested_nodes, total_safe_capacity));
        }

        Ok(total_safe_capacity)
    }

    /**
     * Recupera una instantánea de los logs unificados para el oráculo GraphQL.
     */
    pub fn get_panopticon_snapshot(&self, limit: usize) -> Vec<SystemLog> {
        let logs_guard = self.unified_system_logs.read()
            .expect("LOCK_POISONED");

        logs_guard.iter()
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }
}

// [apps/orchestrator/src/state/mod.rs]
/**
 * =================================================================
 * APARATO: SOVEREIGN STATE ORCHESTRATOR (V224.3 - ZENITH ABSOLUTE)
 * CLASIFICACIÓN: APPLICATION STATE (ESTRATO L1-APP)
 * RESPONSABILIDAD: ORQUESTACIÓN DE ESTADOS, PERSISTENCIA Y ORÁCULO
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. NOMINAL ALIGNMENT: Restaura 'set_mode' y asegura 'workers()' para
 *    sanar los errores E0599 detectados en Reaper y Bootstrap.
 * 2. ATOMIC INTEGRITY: Uso de 'Arc<T>' y cerrojos tácticos para coherencia
 *    en entornos multi-hilo de alta frecuencia.
 * 3. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta aplicada a cada miembro.
 * 4. PANOPTICON TRACING: Instrumentación #[instrument] en transiciones críticas.
 * =================================================================
 */

pub mod mission_control;
pub mod swarm_telemetry;
pub mod operational_nexus;
pub mod finding_vault;

use std::sync::{Arc, RwLock, Mutex};
use std::collections::HashMap;
use prospector_infra_db::TursoClient;
use crate::services::event_bus::EventBus;
use prospector_domain_models::worker::WorkerHeartbeat;

// --- SINAPSIS INTERNA (ZENITH ALIGNMENT) ---
use crate::graphql::{build_neural_schema, NeuralSchema};
use crate::state::operational_nexus::SwarmOperationalMode;
use tracing::{info, warn, instrument, debug, error};

/// Modos de salud del sistema para la interceptación de middleware HTTP.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemMode {
    /// El sistema procesa ráfagas de red y asigna misiones.
    Operational,
    /// El sistema ha suspendido el despacho por anomalía técnica o mando C2.
    Maintenance(String),
}

/**
 * Contenedor de estado compartido (Thread-Safe) para el Orquestador.
 * Actúa como la placa base neural del sistema.
 */
#[derive(Clone)]
pub struct AppState {
    /// Cliente táctico para el Motor A (Turso Cloud / libSQL).
    pub database_client: TursoClient,
    /// Bus de eventos para la difusión de señales en tiempo real al Dashboard.
    pub event_bus: Arc<EventBus>,
    /// Gestor de inventario de misiones en RAM (Acceso O1).
    pub mission_control: Arc<mission_control::MissionControlManager>,
    /// Centro de telemetría y vigilancia visual del enjambre.
    pub swarm_telemetry: Arc<swarm_telemetry::SwarmTelemetryManager>,
    /// Máquina de estados soberana de integridad y mando.
    pub operational_nexus: Arc<operational_nexus::OperationalNexusManager>,
    /// Bóveda de tránsito para hallazgos criptográficos confirmados.
    pub finding_vault: Arc<finding_vault::FindingVaultManager>,
    /// Instancia viva del oráculo de datos GraphQL (Academia).
    pub graphql_schema: NeuralSchema,
    /// Estado de liveness para guardias perimetrales de la API.
    pub current_system_mode: Arc<RwLock<SystemMode>>,
    /// Buffer de latidos para el protocolo 'Write-Behind' (Protección de Motor A).
    pub heartbeat_buffer: Arc<Mutex<HashMap<String, WorkerHeartbeat>>>,
}

impl AppState {
    /**
     * Forja una nueva instancia del Estado Maestro inyectando dependencias.
     */
    pub fn new(database_client: TursoClient) -> Self {
        debug!("🧬 [APP_STATE]: Executing sovereign ignition sequence V224.3...");

        let event_bus_instance = Arc::new(EventBus::new());

        let graphql_oracle_schema = build_neural_schema(
            database_client.clone(),
            event_bus_instance.clone()
        );

        Self {
            database_client: database_client.clone(),
            event_bus: event_bus_instance.clone(),
            mission_control: Arc::new(mission_control::MissionControlManager::new()),
            swarm_telemetry: Arc::new(swarm_telemetry::SwarmTelemetryManager::new()),
            operational_nexus: Arc::new(operational_nexus::OperationalNexusManager::new(event_bus_instance)),
            finding_vault: Arc::new(finding_vault::FindingVaultManager::new()),
            graphql_schema: graphql_oracle_schema,
            current_system_mode: Arc::new(RwLock::new(SystemMode::Operational)),
            heartbeat_buffer: Arc::new(Mutex::new(HashMap::with_capacity(300))),
        }
    }

    /**
     * Provee acceso directo al cliente de base de datos táctica (Motor A).
     */
    pub fn db(&self) -> TursoClient {
        self.database_client.clone()
    }

    /**
     * Provee acceso al gestor de telemetría de trabajadores de la flota.
     * ✅ RESOLUCIÓN E0599: Método explícito y público para el servicio Reaper.
     */
    pub fn workers(&self) -> &Arc<swarm_telemetry::SwarmTelemetryManager> {
        &self.swarm_telemetry
    }

    /**
     * Sincroniza el modo operativo del servidor para el control de acceso.
     * ✅ RESOLUCIÓN E0599 (Bootstrap): Restaurado nombre 'set_mode' para paridad de sistema.
     *
     * @param target_system_mode El nuevo estado de salud del servidor.
     */
    #[instrument(skip(self, target_system_mode))]
    pub fn set_mode(&self, target_system_mode: SystemMode) {
        match self.current_system_mode.write() {
            Ok(mut mode_guard) => {
                info!("🔄 [STATE_SHIFT]: System transitioning to mode: {:?}", target_system_mode);
                *mode_guard = target_system_mode;
            }
            Err(poison_error) => {
                error!("💀 [KERNEL_CRASH]: System mode lock poisoned: {}", poison_error);
            }
        }
    }

    /**
     * Alias para compatibilidad transicional.
     */
    pub fn set_system_mode(&self, mode: SystemMode) {
        self.set_mode(mode);
    }

    /**
     * Evalúa la capacidad operativa ante ráfagas HTTP entrantes.
     */
    #[instrument(skip(self), level = "debug")]
    pub fn is_operational(&self) -> Result<(), String> {
        let mode_guard = self.current_system_mode.read()
            .map_err(|e| format!("LOCK_POISON_FAULT: {}", e))?;

        match &*mode_guard {
            SystemMode::Operational => Ok(()),
            SystemMode::Maintenance(rejection_reason) => {
                warn!("⛔ [ACCESS_DENIED]: Strata maintenance active: {}", rejection_reason);
                Err(rejection_reason.clone())
            },
        }
    }

    /**
     * Determina si el despacho de misiones está autorizado bit-a-bit.
     */
    pub fn is_mission_acquisition_authorized(&self) -> bool {
        let current_nexus_state = self.operational_nexus.get_current_snapshot();
        current_nexus_state.mode == SwarmOperationalMode::FullExecution
    }

    /**
     * Purga ráfagas visuales obsoletas para proteger la integridad de la RAM.
     *
     * # Performance:
     * Operación O(N) ejecutada por el servicio 'Reaper'.
     */
    #[instrument(skip(self))]
    pub fn prune_stale_snapshots(&self, expiration_timeout_seconds: i64) -> usize {
        let mut visual_frames_guard = self.swarm_telemetry.visual_surveillance_frames.write()
            .expect("FATAL: Visual Frames Lock poisoned.");

        let initial_frame_count = visual_frames_guard.len();
        let expiration_threshold = chrono::Utc::now() - chrono::Duration::seconds(expiration_timeout_seconds);

        visual_frames_guard.retain(|_, snapshot_artifact| {
            if let Ok(parsed_ts) = chrono::DateTime::parse_from_rfc3339(&snapshot_artifact.timestamp) {
                parsed_ts.with_timezone(&chrono::Utc) > expiration_threshold
            } else {
                false
            }
        });

        let purged_count = initial_frame_count - visual_frames_guard.len();
        if purged_count > 0 {
            info!("💀 [STATE_CLEANUP]: Purged {} visual frames from ephemeral strata.", purged_count);
        }

        purged_count
    }
}

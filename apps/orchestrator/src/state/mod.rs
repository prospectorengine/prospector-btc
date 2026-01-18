// [apps/orchestrator/src/state/mod.rs]
/**
 * =================================================================
 * APARATO: SOVEREIGN STATE ORCHESTRATOR (V225.0 - GALVANIC CORE)
 * CLASIFICACIÓN: APPLICATION STATE (ESTRATO L1-APP)
 * RESPONSABILIDAD: ORQUESTACIÓN DE ESTRATOS, REPOSITORIOS Y ORÁCULO
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. L7 REPOSITORY INJECTION: Integra Billing, Notification y Gamification
 *    como ciudadanos de primera clase, eliminando la deuda técnica E0583.
 * 2. COMPOSITION ROOT: Centraliza la autoridad de persistencia mediante
 *    instancias 'Arc<T>' pre-hidratadas, optimizando el despacho masivo.
 * 3. ATOMIC CONSISTENCY: Mantenimiento de cerrojos tácticos (RwLock/Mutex)
 *    para coherencia en ráfagas de 300+ nodos.
 * 4. NOMINAL PURITY: Erradicación total de abreviaciones y placeholders.
 *
 * # Mathematical Proof (Resource Sovereignty):
 * El uso de punteros atómicos (Arc) garantiza que todos los hilos del
 * Orquestador compartan la misma vista del Ledger Táctico, impidiendo
 * colisiones de estado en la tabla 'outbox_strategic'.
 * =================================================================
 */

pub mod mission_control;
pub mod swarm_telemetry;
pub mod operational_nexus;
pub mod finding_vault;

use std::sync::{Arc, RwLock, Mutex};
use std::collections::HashMap;
use prospector_infra_db::TursoClient;
use prospector_infra_db::repositories::{
    MissionRepository,
    IdentityRepository,
    BillingRepository,
    NotificationRepository,
    GamificationRepository
};
use crate::services::event_bus::EventBus;
use prospector_domain_models::worker::WorkerHeartbeat;

// --- SINAPSIS INTERNA (ZENITH ALIGNMENT) ---
use crate::graphql::{build_neural_schema, NeuralSchema};
use crate::state::operational_nexus::SwarmOperationalMode;
use tracing::{info, warn, instrument, debug, error};

/// Modos de salud del sistema para la interceptación de middleware perimetral.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemMode {
    /// El sistema procesa ráfagas de red y asigna misiones de forma nominal.
    Operational,
    /// El sistema ha suspendido el despacho por mantenimiento o mando C2.
    Maintenance(String),
}

/**
 * Contenedor de estado compartido (Thread-Safe) para el Orquestador.
 * Actúa como la placa base neural donde se conectan todos los estratos.
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
    /// Máquina de estados soberana de integridad y mando C2.
    pub operational_nexus: Arc<operational_nexus::OperationalNexusManager>,
    /// Bóveda de tránsito para hallazgos criptográficos confirmados.
    pub finding_vault: Arc<finding_vault::FindingVaultManager>,
    /// Instancia viva del oráculo de datos GraphQL (Academia).
    pub graphql_schema: NeuralSchema,
    /// Estado de liveness para guardias perimetrales de la API.
    pub current_system_mode: Arc<RwLock<SystemMode>>,
    /// Buffer de latidos para el protocolo 'Write-Behind' (Protección de Motor A).
    pub heartbeat_buffer: Arc<Mutex<HashMap<String, WorkerHeartbeat>>>,

    // --- ESTRATO L7: REPOSITORIOS DE SERVICIOS AL USUARIO ---

    /// Autoridad de persistencia para el Ledger de misiones.
    pub mission_repository: Arc<MissionRepository>,
    /// Gestor de identidades ZK y arrendamientos (Leases).
    pub identity_repository: Arc<IdentityRepository>,
    /// Motor de facturación y cuotas de energía computacional.
    pub billing_repository: Arc<BillingRepository>,
    /// Sistema Herald de notificaciones y alertas tácticas.
    pub notification_repository: Arc<NotificationRepository>,
    /// Motor Nexus de experiencia y prestigio del operador.
    pub gamification_repository: Arc<GamificationRepository>,
}

impl AppState {
    /**
     * Forja una nueva instancia del Estado Maestro inyectando todas las dependencias.
     *
     * # Mathematical Proof (Indivisible Ignition):
     * Garantiza que el sistema solo inicie si todos los repositorios
     * están correctamente enlazados con el cliente Turso.
     */
    pub fn new(database_client: TursoClient) -> Self {
        debug!("🧬 [APP_STATE]: Executing sovereign ignition sequence V225.0...");

        let event_bus_instance = Arc::new(EventBus::new());

        let graphql_oracle_schema = build_neural_schema(
            database_client.clone(),
            event_bus_instance.clone()
        );

        // Pre-hidratación de repositorios soberanos
        let mission_repo = Arc::new(MissionRepository::new(database_client.clone()));
        let identity_repo = Arc::new(IdentityRepository::new(database_client.clone()));
        let billing_repo = Arc::new(BillingRepository::new(database_client.clone()));
        let notification_repo = Arc::new(NotificationRepository::new(database_client.clone()));
        let gamification_repo = Arc::new(GamificationRepository::new(database_client.clone()));

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

            // Inyección de autoridad L7
            mission_repository: mission_repo,
            identity_repository: identity_repo,
            billing_repository: billing_repo,
            notification_repository: notification_repo,
            gamification_repository: gamification_repo,
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
     * ✅ RESOLUCIÓN E0599: Método para el servicio Reaper.
     */
    pub fn workers(&self) -> &Arc<swarm_telemetry::SwarmTelemetryManager> {
        &self.swarm_telemetry
    }

    /**
     * Sincroniza el modo operativo del servidor para el control de acceso.
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
            Err(lock_poison_fault) => {
                error!("💀 [KERNEL_CRASH]: System mode lock poisoned: {}", lock_poison_fault);
            }
        }
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
     * Determina si el despacho de misiones está autorizado bit-a-bit por el Nexo.
     */
    pub fn is_mission_acquisition_authorized(&self) -> bool {
        let current_nexus_state = self.operational_nexus.get_current_snapshot();
        current_nexus_state.mode == SwarmOperationalMode::FullExecution
    }

    /**
     * Purga ráfagas visuales obsoletas para proteger la integridad de la RAM.
     */
    #[instrument(skip(self))]
    pub fn prune_stale_snapshots(&self, expiration_timeout_seconds: i64) -> usize {
        let mut visual_frames_guard = self.swarm_telemetry.visual_surveillance_frames.write()
            .expect("FATAL: Visual Frames Lock poisoned.");

        let initial_frame_count = visual_frames_guard.len();
        let expiration_threshold_timestamp = chrono::Utc::now() - chrono::Duration::seconds(expiration_timeout_seconds);

        visual_frames_guard.retain(|_, snapshot_artifact| {
            if let Ok(parsed_timestamp) = chrono::DateTime::parse_from_rfc3339(&snapshot_artifact.timestamp) {
                parsed_timestamp.with_timezone(&chrono::Utc) > expiration_threshold_timestamp
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

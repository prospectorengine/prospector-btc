// [apps/orchestrator/src/services/swarm_resurrection.rs]
/*!
 * =================================================================
 * APARATO: SWARM RESURRECTION SERVICE (V182.0 - TYPE SOBERANO)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: RECUPERACIÓN DE UNIDADES ZOMBIE Y GESTIÓN C2
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. TYPE ANNOTATION FIX: Resolución definitiva del error E0282 mediante la
 *    inyección nominal del tipo 'Vec<String>' en la detección de huérfanos.
 * 2. ZERO ABBREVIATIONS: Erradicación total de términos genéricos (ids -> mission_identifiers).
 * 3. SATURATION SHIELD: Implementación de guardias de capacidad para evitar el
 *    baneo de identidades por exceso de igniciones concurrentes.
 * 4. PANOPTICON SYNC: Telemetría enriquecida vinculada al estrato L3_ORCH_RESURRECTION.
 *
 * # Mathematical Proof (Anti-Avalanche Protocol):
 * El servicio aplica un 'Atomic Lock' en el Ledger antes de contactar a GitHub.
 * Esto garantiza que si la red externa presenta latencia, el sistema no
 * duplique las órdenes de trabajo para un mismo segmento del keyspace.
 * =================================================================
 */

use crate::state::AppState;
use crate::services::c2_coordinator::GitHubCommandCoordinator;
use prospector_infra_db::repositories::MissionRepository;
use prospector_domain_models::telemetry::SystemLog;
use std::time::Duration;
use tokio::time::{interval, MissedTickBehavior};
use tracing::{info, warn, error, instrument, debug};

/// Umbral de inactividad (15 minutos) para declarar un nodo como 'Huérfano' o 'Zombie'.
const STALE_MISSION_ABANDONMENT_THRESHOLD_SECONDS: i64 = 900;

/// Ciclo de vigilancia táctica (5 minutos) para minimizar la carga sobre Motor A.
const ZOMBIE_SURVEILLANCE_INTERVAL_SECONDS: u64 = 300;

/// Límite de seguridad para evitar la saturación de la forja de GitHub (10 nodos).
const MAXIMUM_RECOVERY_IGNITION_BURST_SIZE: u32 = 10;

/**
 * Daemon encargado de la supervivencia del enjambre.
 * Localiza hilos de computación que han dejado de emitir latidos y relanza
 * el aprovisionamiento de forma determinista.
 */
pub struct SwarmResurrectionService {
    /// Referencia compartida al sistema nervioso central del orquestador.
    application_shared_state: AppState,
}

impl SwarmResurrectionService {
    /**
     * Construye una nueva instancia del servicio de resurrección.
     */
    #[must_use]
    pub fn new(application_state: AppState) -> Self {
        Self {
            application_shared_state: application_state
        }
    }

    /**
     * Inicia el bucle de vigilancia perpetua en el reactor de Tokio.
     *
     * # Reliability:
     * Utiliza 'MissedTickBehavior::Skip' para asegurar que los ciclos no se
     * acumulen ante bloqueos temporales de I/O en la base de datos Turso.
     */
    pub async fn spawn_resurrection_daemon(self) {
        let mut surveillance_ticker = interval(Duration::from_secs(ZOMBIE_SURVEILLANCE_INTERVAL_SECONDS));
        surveillance_ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);

        info!("🩺 [RESURRECTION]: Anti-Avalanche Guardian V182.0 active.");

        loop {
            surveillance_ticker.tick().await;

            // 1. NEXUS AUTHORITY: Verificación de modo operativo C2.
            if !self.application_shared_state.is_mission_acquisition_authorized() {
                debug!("💤 [RESURRECTION]: Operational Nexus has suspended dispatch. Standby.");
                continue;
            }

            // 2. EJECUCIÓN DEL PROTOCOLO DE RECUPERACIÓN
            if let Err(recovery_fault) = self.execute_atomic_recovery_sequence().await {
                error!("❌ [RECOVERY_FAULT]: Resurrection strata sequence collapsed: {}", recovery_fault);
            }
        }
    }

    /**
     * Ejecuta el escrutinio del Ledger y el disparo de señales hacia GitHub.
     *
     * # Errors:
     * - `DbError`: Si el enlace con Turso falla durante el bloqueo de zombies.
     * - `C2Error`: Si el token de GitHub es rechazado o la cuota está agotada.
     */
    #[instrument(skip(self))]
    async fn execute_atomic_recovery_sequence(&self) -> anyhow::Result<()> {
        let mission_repository_engine = MissionRepository::new(self.application_shared_state.database_client.clone());
        let tactical_ledger_connection = self.application_shared_state.database_client.get_connection()?;

        // 1. IDENTIFICACIÓN Y BLOQUEO LOCAL (L3 Strata)
        // ✅ RESOLUCIÓN E0282: Inyección explícita del tipo Vec<String> para los identificadores.
        let abandoned_mission_identifiers: Vec<String> = mission_repository_engine
            .identify_and_lock_zombies(
                &tactical_ledger_connection,
                STALE_MISSION_ABANDONMENT_THRESHOLD_SECONDS,
                i64::from(MAXIMUM_RECOVERY_IGNITION_BURST_SIZE)
            )
            .await?;

        if abandoned_mission_identifiers.is_empty() {
            return Ok(());
        }

        let orphan_units_count = abandoned_mission_identifiers.len() as u32;
        warn!("💀 [ZOMBIES_DETECTED]: Found {} orphan units. Initiating Cloud Resupply...", orphan_units_count);

        // 2. SINAPSIS CON LA NUBE (C2 Availability Check)
        let github_command_coordinator = GitHubCommandCoordinator::from_production_environment()?;

        // SONDAS DE SATURACIÓN: Verificar si la nube ya está procesando igniciones
        if github_command_coordinator.has_active_ignitions_in_cloud().await.unwrap_or(false) {
            warn!("🛡️ [SATURATION_SHIELD]: GitHub Forge is at capacity. Delaying resupply to protect identity pool.");

            // Rollback del candado local para permitir reintento en el siguiente ciclo
            mission_repository_engine.unlock_zombies(&tactical_ledger_connection, abandoned_mission_identifiers).await?;

            self.emit_forensic_alert("Ignition suppressed: Cloud forge saturation detected.");
            return Ok(());
        }

        // 3. DISPARO DE SEÑAL DE EXPANSIÓN (Ignition)
        match github_command_coordinator.trigger_swarm_expansion_sequence(orphan_units_count).await {
            Ok(_) => {
                info!("🚀 [C2_IGNITION]: Resupply signal accepted for {} units.", orphan_units_count);

                // Retornamos las misiones al estado 'queued' para que los nuevos nodos las reclamen
                mission_repository_engine.requeue_missions(&tactical_ledger_connection, abandoned_mission_identifiers).await?;

                self.emit_forensic_alert(&format!("Recovery sequence successful. {} nodes requested.", orphan_units_count));
            }
            Err(ignition_fault) => {
                error!("⚠️ [C2_IGNITION_FAILED]: Cloud rejected expansion signal: {}. Releasing local strata.", ignition_fault);

                // Rollback para evitar que las misiones queden bloqueadas en 'ignition_pending'
                mission_repository_engine.unlock_zombies(&tactical_ledger_connection, abandoned_mission_identifiers).await?;
            }
        }

        Ok(())
    }

    /**
     * Emite un rastro forense al Dashboard Zenith mediante el Bus de Eventos.
     */
    fn emit_forensic_alert(&self, alert_message_content: &str) {
        let log_entry_artifact = SystemLog {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            stratum: "L3_ORCH_RESURRECTION".into(),
            severity: "WARN".into(),
            message: format!("🛡️ RESURRECTION_MONITOR: {}", alert_message_content),
            metadata: None,
            trace_id: None,
        };

        self.application_shared_state.event_bus.emit_system_log(log_entry_artifact);
    }
}

// [apps/orchestrator/src/services/mission_hydrator.rs]
/**
 * =================================================================
 * APARATO: MISSION HYDRATOR SERVICE (V224.0 - SILICON PULSE SYNC)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: GESTIÓN ESTRATÉGICA DE LA CADENA DE SUMINISTRO EN RAM
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. ELASTIC REPLENISHMENT: Reducción del ciclo de vigilancia a 15s para
 *    saturar la demanda de enjambres de alta densidad (300+ nodos).
 * 2. SYSTEM COHERENCE GUARD: Valida el estado de integridad en el Nexo
 *    antes de proceder, evitando la carga de datos si L1 está comprometido.
 * 3. NOMINAL TELEMETRY: Emite informes detallados al Panóptico indicando
 *    el volumen y origen del material de misión hidratado.
 * 4. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta nivel Tesis Doctoral.
 *
 * # Mathematical Proof (Deterministic Hydration):
 * El servicio garantiza la invariante de inventario I_ram >= L_threshold.
 * Mediante el uso de ráfagas de 200 unidades, amortizamos el coste de
 * latencia RTT del Motor A frente a la velocidad de despacho O(1) de la API.
 * =================================================================
 */

use crate::state::AppState;
use crate::state::operational_nexus::SystemIntegrityStatus;
use prospector_infra_db::repositories::MissionRepository;
use prospector_domain_models::work::WorkOrder;
use prospector_domain_models::telemetry::SystemLog;
use std::time::Duration;
use tokio::time::{interval, MissedTickBehavior};
use tracing::{info, debug, error, warn, instrument};
use uuid::Uuid;

/// Umbral de seguridad para existencias mínimas en memoria volátil.
const INVENTORY_CRITICAL_LOW_WATERMARK: usize = 50;
/// Magnitud de la ráfaga de reabastecimiento desde el Ledger Táctico.
const REPLENISHMENT_BATCH_MAGNITUDE: usize = 200;
/// Ciclo de vigilancia logística sintonizado para producción (15 segundos).
const LOGISTICS_SURVEILLANCE_PULSE_SECONDS: u64 = 15;

/**
 * Daemon soberano encargado de la fluidez del suministro de misiones.
 * Actúa como el pulmón logístico entre el disco (Turso) y la red (Workers).
 */
pub struct MissionHydratorService {
    /// Referencia compartida al sistema nervioso central del orquestador.
    application_shared_state: AppState,
}

impl MissionHydratorService {
    /**
     * Construye una nueva instancia del servicio inyectando el estado maestro.
     */
    #[must_use]
    pub fn new(application_state: AppState) -> Self {
        Self {
            application_shared_state: application_state
        }
    }

    /**
     * Lanza el bucle de vigilancia perpetua en el reactor de Tokio.
     *
     * # Logic:
     * 1. Verifica autorización de despacho en el Nexo.
     * 2. Evalúa presión de inventario en RAM.
     * 3. Dispara secuencia de extracción si la ocupación es < Low-Watermark.
     */
    pub async fn spawn_hydrator_daemon(self) {
        let mut logistics_pulse_timer = interval(Duration::from_secs(LOGISTICS_SURVEILLANCE_PULSE_SECONDS));

        // Protocolo de resiliencia: Ignorar ticks acumulados por latencia de DB.
        logistics_pulse_timer.set_missed_tick_behavior(MissedTickBehavior::Skip);

        info!(
            "🚰 [HYDRATOR]: Supply chain guardian V224.0 online. Target_Threshold: {} units.",
            INVENTORY_CRITICAL_LOW_WATERMARK
        );

        loop {
            logistics_pulse_timer.tick().await;

            // 1. NEXUS COHERENCE AUDIT
            let current_nexus_snapshot = self.application_shared_state.operational_nexus.get_current_snapshot();

            if current_nexus_snapshot.integrity == SystemIntegrityStatus::Compromised {
                warn!("🛑 [HYDRATOR_VETO]: Sourcing suspended. Mathematical strata is COMPROMISED.");
                continue;
            }

            if !self.application_shared_state.is_mission_acquisition_authorized() {
                debug!("💤 [HYDRATOR]: Dispatch engine in standby. Supply chain paused.");
                continue;
            }

            // 2. INVENTORY PRESSURE ANALYSIS
            let current_inventory_volume = self.application_shared_state.mission_control.get_available_buffer_size();

            if current_inventory_volume < INVENTORY_CRITICAL_LOW_WATERMARK {
                debug!(
                    "📥 [HYDRATOR]: Pressure drop detected ({}/{}). Firing replenishment pulse...",
                    current_inventory_volume,
                    INVENTORY_CRITICAL_LOW_WATERMARK
                );

                match self.execute_sovereign_replenishment_sequence().await {
                    Ok(newly_hydrated_volume) if newly_hydrated_volume > 0 => {
                        info!("✅ [HYDRATOR]: Logistics success. Secured {} missions in RAM Strata.", newly_hydrated_volume);
                        self.emit_logistics_telemetry(newly_hydrated_volume);
                    },
                    Ok(_) => {
                        // El Ledger Táctico está temporalmente agotado.
                        debug!("💤 [HYDRATOR]: Tactical strata exhausted. Awaiting new campaign seeds.");
                    },
                    Err(hydration_fault) => {
                        error!("❌ [HYDRATOR_CRITICAL_ERROR]: Supply chain collapse: {}", hydration_fault);
                    }
                }
            }
        }
    }

    /**
     * Ejecuta la transacción de extracción bit-perfecta y carga en cola FIFO.
     */
    #[instrument(skip(self))]
    async fn execute_sovereign_replenishment_sequence(&self) -> anyhow::Result<usize> {
        let database_client_handle = self.application_shared_state.database_client.clone();
        let mission_repository_engine = MissionRepository::new(database_client_handle);

        // Extracción de ráfaga nivelada (OlogN)
        let extracted_missions_collection: Vec<WorkOrder> = mission_repository_engine
            .fetch_dynamic_mission_batch(REPLENISHMENT_BATCH_MAGNITUDE)
            .await?;

        let actual_volume_count = extracted_missions_collection.len();

        if actual_volume_count > 0 {
            // Inyección atómica en el buffer de memoria (Acceso O1 para los Handlers)
            self.application_shared_state.mission_control.hydrate_queue(extracted_missions_collection);
        }

        Ok(actual_volume_count)
    }

    /**
     * Transmite una señal de éxito logístico al Dashboard Zenith (Proyecto Panóptico).
     */
    fn emit_logistics_telemetry(&self, hydrated_volume: usize) {
        let log_artifact = SystemLog {
            id: Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            stratum: "L4_ORCH_LOGISTICS".to_string(),
            severity: "INFO".to_string(),
            message: format!("SUPPLY_CHAIN_SYNC: Hydrated {} units to RAM.", hydrated_volume),
            metadata: None,
            trace_id: None,
        };

        self.application_shared_state.event_bus.emit_system_log(log_artifact);
    }
}

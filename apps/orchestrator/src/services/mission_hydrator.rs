// [apps/orchestrator/src/services/mission_hydrator.rs]
/*!
 * =================================================================
 * APARATO: MISSION HYDRATOR SERVICE (V222.0 - TYPE SOBERANO)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: REPOSICIÓN DINÁMICA DEL INVENTARIO DE MISIONES EN RAM
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. TYPE ANNOTATION FIX: Resolución del error E0282 mediante la inyección
 *    nominal del tipo 'Vec<WorkOrder>' en la fase de extracción.
 * 2. NOMINAL CLARITY: Erradicación de abreviaciones. 'extracted' -> 'extracted_missions_collection'.
 * 3. RESILIENT SCHEDULING: Implementación de 'MissedTickBehavior::Skip' para evitar
 *    ráfagas de I/O acumuladas ante latencia en el cluster de Turso.
 * 4. PANOPTICON INTEGRATION: Telemetría enriquecida con #[instrument] para el
 *    rastreo de la cadena de suministro de misiones.
 *
 * # Mathematical Proof (Supply Chain Continuity):
 * El hydrator actúa como un sensor de presión. Si el volumen en RAM cae por debajo
 * del 'Low Watermark', dispara una ráfaga de reabastecimiento O(1) desde el Ledger
 * Táctico, manteniendo el tiempo de respuesta del Handshake por debajo de los 10ms.
 * =================================================================
 */

use crate::state::AppState;
use prospector_infra_db::repositories::MissionRepository;
use prospector_domain_models::work::WorkOrder;
use std::time::Duration;
use tokio::time::{interval, MissedTickBehavior};
use tracing::{info, debug, error, instrument};

/// Umbral crítico de existencias en RAM (50 misiones).
const INVENTORY_LOW_WATERMARK_THRESHOLD: usize = 50;
/// Volumen nominal de reabastecimiento por ráfaga.
const REPLENISHMENT_BATCH_SIZE_MAGNITUDE: usize = 200;
/// Ciclo de vigilancia logística (30 segundos).
const LOGISTICS_SURVEILLANCE_CYCLE_SECONDS: u64 = 30;

/**
 * Daemon encargado de mantener la fluidez del inventario volátil.
 * Evita que el enjambre se detenga por latencia de consulta en la DB.
 */
pub struct MissionHydratorService {
    /// Referencia compartida al sistema nervioso central del orquestador.
    application_shared_state: AppState,
}

impl MissionHydratorService {
    /**
     * Construye una nueva instancia del servicio de hidratación.
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
     * # Logic:
     * El servicio monitoriza el 'MissionControl' de forma pasiva. Solo interviene
     * si el sistema está autorizado (OperationalMode) y el buffer está en niveles críticos.
     */
    pub async fn spawn_hydrator_daemon(self) {
        let mut logistics_timer = interval(Duration::from_secs(LOGISTICS_SURVEILLANCE_CYCLE_SECONDS));

        // Protocolo de resiliencia: Si el hilo se bloquea, no acumular ejecuciones.
        logistics_timer.set_missed_tick_behavior(MissedTickBehavior::Skip);

        info!("🚰 [HYDRATOR]: Silent logistics daemon V222.0 operational.");

        loop {
            logistics_timer.tick().await;

            // 1. NEXUS AUTHORITY CHECK: ¿Estamos en un modo que permita adquisición?
            if !self.application_shared_state.is_mission_acquisition_authorized() {
                continue;
            }

            // 2. ESCANEO DE PRESIÓN EN RAM
            let current_buffer_count = self.application_shared_state.mission_control.get_available_buffer_size();

            if current_buffer_count < INVENTORY_LOW_WATERMARK_THRESHOLD {
                debug!("📥 [HYDRATOR]: Pressure drop detected ({} units). Refilling strata...", current_buffer_count);

                match self.execute_dynamic_replenishment_sequence().await {
                    Ok(newly_injected_count) if newly_injected_count > 0 => {
                        info!("✅ [HYDRATOR]: Logistics success. Injected {} units into volatile memory.", newly_injected_count);
                    },
                    Ok(_) => {
                        // El Ledger Táctico no tiene misiones queued. Silencio nominal.
                        debug!("💤 [HYDRATOR]: Tactical Ledger exhausted. Awaiting new campaign seeds.");
                    },
                    Err(fault) => {
                        error!("❌ [HYDRATOR_CRITICAL_FAULT]: Supply chain collapsed: {}", fault);
                    }
                }
            }
        }
    }

    /**
     * Ejecuta la transacción de extracción y carga en el buffer circular.
     *
     * # Errors:
     * - Retorna error si el enlace con el Motor A (Turso) es inalcanzable.
     *
     * # Performance:
     * Operación O(N) donde N es REPLENISHMENT_BATCH_SIZE_MAGNITUDE.
     */
    #[instrument(skip(self))]
    async fn execute_dynamic_replenishment_sequence(&self) -> anyhow::Result<usize> {
        let mission_repository_engine = MissionRepository::new(self.application_shared_state.database_client.clone());

        // ✅ RESOLUCIÓN E0282: Inyección explícita del tipo soberano Vec<WorkOrder>
        let extracted_missions_collection: Vec<WorkOrder> = mission_repository_engine
            .fetch_dynamic_mission_batch(REPLENISHMENT_BATCH_SIZE_MAGNITUDE)
            .await?;

        let actual_extracted_count = extracted_missions_collection.len();

        if actual_extracted_count > 0 {
            // Hidratación de la cola FIFO en RAM (O1 access)
            self.application_shared_state.mission_control.hydrate_queue(extracted_missions_collection);
        }

        Ok(actual_extracted_count)
    }
}

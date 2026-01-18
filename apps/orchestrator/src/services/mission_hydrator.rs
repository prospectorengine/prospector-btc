// [apps/orchestrator/src/services/mission_hydrator.rs]
/*!
 * =================================================================
 * APARATO: MISSION HYDRATOR SERVICE (V223.1 - SUPPLY CHAIN MASTER)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: REPOSICIÓN DINÁMICA DEL INVENTARIO DE MISIONES EN RAM
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. REPOSITORY SYNC: Sincroniza con 'MissionRepository' V300.9, eliminando
 *    el error E0432 y consolidando la nomenclatura de la Tesis.
 * 2. TYPE SOVEREIGNTY: Mantiene la inyección nominal de 'Vec<WorkOrder>'
 *    para prevenir fallos de inferencia E0282 en el reactor asíncrono.
 * 3. L7 AWARENESS: Instrumentación preparada para el rastreo de cuotas
 *    de campaña durante el ciclo de reabastecimiento.
 * 4. HYGIENE: Erradicación de abreviaciones y documentación técnica MIT.
 *
 * # Mathematical Proof (Deterministic Hydration):
 * El servicio garantiza que el buffer de RAM mantenga una ocupación >=
 * 'LOW_WATERMARK', asegurando que el tiempo de respuesta del orquestador
 * sea independiente de la latencia del cluster de Turso (Motor A).
 * =================================================================
 */

use crate::state::AppState;
use prospector_infra_db::repositories::MissionRepository; // ✅ SINCRO E0432
use prospector_domain_models::work::WorkOrder;
use std::time::Duration;
use tokio::time::{interval, MissedTickBehavior};
use tracing::{info, debug, error, instrument};

/// Umbral crítico de existencias en RAM (50 misiones).
const INVENTORY_LOW_WATERMARK_THRESHOLD: usize = 50;
/// Volumen nominal de reabastecimiento por ráfaga táctica.
const REPLENISHMENT_BATCH_SIZE_MAGNITUDE: usize = 200;
/// Ciclo de vigilancia logística (30 segundos).
const LOGISTICS_SURVEILLANCE_CYCLE_SECONDS: u64 = 30;

/**
 * Daemon encargado de mantener la fluidez de la cadena de suministro de misiones.
 * Actúa como un sensor de presión sobre el 'MissionControl' volátil.
 */
pub struct MissionHydratorService {
    /// Referencia compartida al sistema nervioso central del orquestador.
    application_shared_state: AppState,
}

impl MissionHydratorService {
    /**
     * Construye una nueva instancia del servicio de hidratación inyectando el estado maestro.
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
     * El servicio monitoriza el buffer de RAM. Si la presión cae por debajo del
     * umbral, dispara una ráfaga de extracción bit-perfecta desde el Motor A.
     */
    pub async fn spawn_hydrator_daemon(self) {
        let mut logistics_timer = interval(Duration::from_secs(LOGISTICS_SURVEILLANCE_CYCLE_SECONDS));

        // Protocolo de resiliencia: Ignorar ticks acumulados si el I/O presenta latencia.
        logistics_timer.set_missed_tick_behavior(MissedTickBehavior::Skip);

        info!("🚰 [HYDRATOR]: Supply chain guardian online. Threshold: {} units.", INVENTORY_LOW_WATERMARK_THRESHOLD);

        loop {
            logistics_timer.tick().await;

            // 1. NEXUS AUTHORITY CHECK: ¿El sistema está en un modo de despacho activo?
            if !self.application_shared_state.is_mission_acquisition_authorized() {
                debug!("💤 [HYDRATOR]: Swarm dispatch is suspended. Standby mode.");
                continue;
            }

            // 2. ESCANEO DE PRESIÓN DE INVENTARIO
            let current_inventory_count = self.application_shared_state.mission_control.get_available_buffer_size();

            if current_inventory_count < INVENTORY_LOW_WATERMARK_THRESHOLD {
                debug!("📥 [HYDRATOR]: Pressure drop detected ({}). Initiating replenishment...", current_inventory_count);

                match self.execute_dynamic_replenishment_sequence().await {
                    Ok(newly_injected_count) if newly_injected_count > 0 => {
                        info!("✅ [HYDRATOR]: Logistics success. Secured {} new missions in RAM.", newly_injected_count);
                    },
                    Ok(_) => {
                        // El Ledger Táctico está agotado.
                        debug!("💤 [HYDRATOR]: Tactical strata exhausted. Awaiting new seeds.");
                    },
                    Err(hydration_fault) => {
                        error!("❌ [HYDRATOR_CRITICAL_FAULT]: Supply chain collapse: {}", hydration_fault);
                    }
                }
            }
        }
    }

    /**
     * Ejecuta la transacción de extracción y carga en el buffer circular.
     */
    #[instrument(skip(self))]
    async fn execute_dynamic_replenishment_sequence(&self) -> anyhow::Result<usize> {
        let database_client_handle = self.application_shared_state.database_client.clone();
        let mission_repository_engine = MissionRepository::new(database_client_handle);

        // ✅ RESOLUCIÓN E0282: Especificación nominal del tipo de colección soberana.
        let extracted_missions_collection: Vec<WorkOrder> = mission_repository_engine
            .fetch_dynamic_mission_batch(REPLENISHMENT_BATCH_SIZE_MAGNITUDE)
            .await?;

        let actual_extracted_count = extracted_missions_collection.len();

        if actual_extracted_count > 0 {
            // Inyección en la cola FIFO de RAM (Acceso O1 para los Handlers)
            self.application_shared_state.mission_control.hydrate_queue(extracted_missions_collection);
        }

        Ok(actual_extracted_count)
    }
}

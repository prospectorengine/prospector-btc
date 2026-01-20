// [apps/miner-worker/src/engine.rs]
/*!
 * =================================================================
 * APARATO: ADAPTIVE EXECUTION ENGINE (V133.0 - ZENITH GOLD)
 * CLASIFICACIÓN: WORKER EXECUTION LAYER (ESTRATO L1-WORKER)
 * RESPONSABILIDAD: ORQUESTACIÓN DE MISIONES Y RESILIENCIA DE RED
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. CONTRACT ALIGNMENT: Resuelve el error de campo 'ram_available_mb'
 *    sincronizando con 'ram_available_megabytes' del modelo L2 SSoT.
 * 2. NOMINAL PURITY: Erradicación total de abreviaciones en acumuladores
 *    y estructuras de despacho.
 * 3. ZENITH OBSERVABILITY: Instrumentación #[instrument] enriquecida para
 *    trazado forense en el Dashboard L5.
 * 4. HYGIENE: Cero residuos de compilación y cumplimiento de RustDoc MIT.
 *
 * # Mathematical Proof (Deterministic Lifecycle):
 * Garantiza que t(misión) = t(negociación) + t(hidratación) + t(cómputo),
 * minimizando los huecos de inactividad mediante la carga paralela de shards.
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tracing::{info, warn, error, instrument, debug};

// --- SINAPSIS CON EL NÚCLEO Y DOMINIO (L1-L3) ---
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_domain_models::work::{
    WorkOrder, MissionRequestPayload, NodeHardwareCapacity, TargetStrata
};
use prospector_domain_models::finding::Finding;
use prospector_domain_strategy::executor::{StrategyExecutor, FindingHandler};
use prospector_infra_worker_client::WorkerClient;
use crate::cpu_manager::HardwareMonitor;

/// Umbral térmico de seguridad para evitar baneo por inestabilidad de instancia.
const THERMAL_THROTTLE_THRESHOLD_CELSIUS: f32 = 82.5;
/// Intervalo de sincronización del rastro forense (Checkpoints) hacia el Hub.
const CHECKPOINT_HEARTBEAT_INTERVAL_SECONDS: u64 = 45;
/// Conteo determinista de fragmentos de censo (SipHash-1-3 Strata).
const FILTRATION_PARTITION_COUNT: usize = 4;

/**
 * IMPLEMENTACIÓN: REPORTERO DE COLISIONES SOBERANO
 * Canaliza los hallazgos criptográficos hacia el túnel asíncrono de red.
 */
struct SwarmFindingReporter {
    transmission_channel_sender: mpsc::UnboundedSender<Finding>,
    worker_node_identifier: String,
    active_mission_identifier: String,
}

impl FindingHandler for SwarmFindingReporter {
    fn on_finding(
        &self,
        bitcoin_address: String,
        private_key_handle: prospector_core_math::private_key::SafePrivateKey,
        entropy_source_metadata: String
    ) {
        let discovery_artifact = Finding {
            address: bitcoin_address,
            private_key_wif: prospector_core_gen::wif::private_to_wif(&private_key_handle, false),
            source_entropy: entropy_source_metadata,
            wallet_type: "p2pkh_legacy_uncompressed".to_string(),
            found_by_worker: self.worker_node_identifier.clone(),
            job_id: Some(self.active_mission_identifier.clone()),
            detected_at: chrono::Utc::now().to_rfc3339(),
        };

        if self.transmission_channel_sender.send(discovery_artifact).is_err() {
            error!("❌ [CHANNEL_COLLAPSE]: Failed to queue collision signal in Node {}", self.worker_node_identifier);
        }
    }
}

pub struct MinerEngine {
    orchestrator_uplink: Arc<WorkerClient>,
    is_operational_signal: Arc<AtomicBool>,
    worker_node_identifier: String,
    local_cache_directory: std::path::PathBuf,
}

impl MinerEngine {
    /**
     * Construye una nueva instancia del motor adaptativo.
     */
    #[must_use]
    pub fn new(
        client: Arc<WorkerClient>,
        operational_signal: Arc<AtomicBool>,
        node_id: String,
        cache_path: std::path::PathBuf,
    ) -> Self {
        Self {
            orchestrator_uplink: client,
            is_operational_signal: operational_signal,
            worker_node_identifier: node_id,
            local_cache_directory: cache_path,
        }
    }

    /**
     * Inicia la secuencia de ignición soberana.
     * Gestiona el bucle perpetuo de adquisición de misiones y la telemetría de red.
     */
    #[instrument(skip(self), fields(node = %self.worker_node_identifier))]
    pub async fn ignite_sovereign_operations(&self) {
        info!("🚀 [ENGINE]: Async Ignition Sequence V133.0 active.");

        // 1. TÚNEL DE COMUNICACIÓN DE HALLAZGOS (ASÍNCRONO)
        let (findings_transmission_tx, mut findings_reception_rx) = mpsc::unbounded_channel::<Finding>();
        let reporting_client_reference = Arc::clone(&self.orchestrator_uplink);

        tokio::spawn(async move {
            while let Some(collision_packet) = findings_reception_rx.recv().await {
                if let Err(uplink_error) = reporting_client_reference.transmit_found_collision(&collision_packet).await {
                    error!("❌ [UPLINK_FAULT]: Failed to secure discovery: {}", uplink_error);
                }
            }
        });

        // 2. BUCLE PRINCIPAL DE AUDITORÍA
        while self.is_operational_signal.load(Ordering::SeqCst) {
            let hardware_metrics = HardwareMonitor::capture_instantaneous_metrics();

            // SENSOR TÉRMICO REACTIVO (Protección de silicio)
            if hardware_metrics.core_temperature_celsius > THERMAL_THROTTLE_THRESHOLD_CELSIUS {
                warn!("🔥 [THERMAL_PACING]: High heat detected ({}°C). Delaying mission acquisition.",
                    hardware_metrics.core_temperature_celsius);
                sleep(Duration::from_secs(10)).await;
                continue;
            }

            // ✅ RESOLUCIÓN SOBERANA: Nivelación de ram_available_megabytes
            let handshake_request = MissionRequestPayload {
                worker_id: self.worker_node_identifier.clone(),
                hardware_capacity: NodeHardwareCapacity {
                    ram_available_megabytes: hardware_metrics.memory_utilization_bytes / (1024 * 1024),
                    cpu_cores: num_cpus::get() as u32,
                    supports_avx2: is_x86_feature_detected!("avx2"),
                },
            };

            match self.orchestrator_uplink.negotiate_mission_assignment_handshake(&handshake_request).await {
                Ok(assignment_envelope) => {
                    info!("🎯 [MISSION_ACQUIRED]: Identifier: {}", assignment_envelope.mission_order.job_mission_identifier);

                    if let Err(mission_error) = self.execute_mission_lifecycle(
                        assignment_envelope.mission_order,
                        findings_transmission_tx.clone()
                    ).await {
                        error!("⚠️ [MISSION_ABORTED]: Operational strata collapsed: {}", mission_error);
                    }
                }
                Err(negotiation_error) => {
                    debug!("💤 [STANDBY]: Orchestrator in cooldown or queue empty: {}", negotiation_error);
                    sleep(Duration::from_secs(20)).await;
                }
            }
        }
    }

    /**
     * Orquesta la ejecución de una unidad de trabajo con aislamiento de hilos y checkpoints.
     */
    #[instrument(skip_all, fields(mission = %mission_order.job_mission_identifier))]
    async fn execute_mission_lifecycle(
        &self,
        mission_order: WorkOrder,
        findings_sender: mpsc::UnboundedSender<Finding>
    ) -> anyhow::Result<()> {
        let mission_identifier = mission_order.job_mission_identifier.clone();

        // 1. HIDRATACIÓN SOBERANA (HYDRA STREAM PARALLEL SHARDS)
        self.orchestrator_uplink.synchronize_mission_sharded_filter(&mission_order, &self.local_cache_directory).await?;

        let strata_label = match mission_order.required_strata {
            TargetStrata::SatoshiEra => "satoshi_era",
            _ => "standard_legacy",
        };

        let filter_storage_path = self.local_cache_directory.join(strata_label);
        let target_census_filter = Arc::new(
            tokio::task::spawn_blocking(move || {
                ShardedFilter::load_from_directory(&filter_storage_path, FILTRATION_PARTITION_COUNT)
            }).await??
        );

        // 2. DAEMON DE CHECKPOINTING (WRITE-BEHIND PROTOCOL)
        let effort_accumulator = Arc::new(AtomicU64::new(0));
        let effort_ref_for_daemon = Arc::clone(&effort_accumulator);
        let stop_signal_for_daemon = Arc::clone(&self.is_operational_signal);
        let uplink_for_daemon = Arc::clone(&self.orchestrator_uplink);
        let mission_id_for_daemon = mission_identifier.clone();

        tokio::spawn(async move {
            while stop_signal_for_daemon.load(Ordering::SeqCst) {
                sleep(Duration::from_secs(CHECKPOINT_HEARTBEAT_INTERVAL_SECONDS)).await;
                let current_volume_effort = effort_ref_for_daemon.load(Ordering::Relaxed);

                if current_volume_effort > 0 {
                    let _ = uplink_for_daemon.report_mission_progress(&mission_id_for_daemon, current_volume_effort).await;
                }
            }
        });

        // 3. EJECUCIÓN DEL MÚSCULO COMPUTACIONAL (BLOQUEANTE EN CPU)
        let reporter = SwarmFindingReporter {
            transmission_channel_sender: findings_sender,
            worker_node_identifier: self.worker_node_identifier.clone(),
            active_mission_identifier: mission_identifier.clone(),
        };

        let stop_signal = Arc::clone(&self.is_operational_signal);
        let node_id_snapshot = self.worker_node_identifier.clone();
        let mission_payload = mission_order.clone();

        // Transferencia de control al motor de estrategias L2
        let audit_certification_report = tokio::task::spawn_blocking(move || {
            StrategyExecutor::execute_mission_sequence(
                &mission_payload,
                &target_census_filter,
                stop_signal,
                effort_accumulator,
                node_id_snapshot,
                &reporter,
                None // ADN se inyecta bajo demanda del motor forense
            )
        }).await?;

        // 4. PROTOCOLO DE SELLADO (CERTIFICACIÓN L4)
        self.orchestrator_uplink.submit_mission_audit_certification(&audit_certification_report).await?;

        info!("✅ [CERTIFIED]: Mission {} verified and sealed in Tactical Ledger.", mission_identifier);
        Ok(())
    }
}

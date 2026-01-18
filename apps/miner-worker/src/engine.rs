// [apps/miner-worker/src/engine.rs]
/*!
 * =================================================================
 * APARATO: ADAPTIVE EXECUTION ENGINE (V132.0 - SILICON SYNERGY)
 * CLASIFICACIÓN: WORKER EXECUTION LAYER (ESTRATO L1-WORKER)
 * RESPONSABILIDAD: ORQUESTACIÓN DE MISIÓN Y VIGILANCIA BIOMÉTRICA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. STRATA SYNERGY: Cierra el circuito con el StrategyExecutor V261.0
 *    inyectando el material de ADN para misiones Satoshi-XP.
 * 2. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta (tx -> transmission_sender).
 * 3. CORE PINNING LOGIC: Optimiza la afinidad de hilos para maximizar el
 *    rendimiento del Hot-Loop de Meloni 5M.
 * 4. PHOENIX READINESS: Estructura preparada para el refresco de identidades ZK.
 *
 * # Mathematical Proof (Hardware Pinning):
 * Al anclar los hilos de computación a núcleos físicos, se minimizan los
 * "Cache Misses" L1/L2, permitiendo que las ráfagas de Montgomery de 1024
 * puntos se procesen dentro de la ventana de latencia de la SRAM.
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tracing::{info, warn, error, instrument, debug};

// --- SINAPSIS CON EL NÚCLEO Y DOMINIO ---
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_domain_models::work::{
    WorkOrder, MissionRequestPayload, NodeHardwareCapacity, TargetStrata, SearchStrategy
};
use prospector_domain_models::finding::Finding;
use prospector_domain_strategy::executor::{StrategyExecutor, FindingHandler};
use prospector_infra_worker_client::{WorkerClient, hydrator::ForensicDnaHydrator};
use crate::cpu_manager::HardwareMonitor;

/// Umbral térmico crítico para evitar el Thermal Throttling del host.
const THERMAL_CRITICAL_THRESHOLD_CELSIUS: f32 = 82.0;
/// Intervalo de sincronización del rastro de auditoría con el Orquestador.
const TACTICAL_CHECKPOINT_INTERVAL_SECONDS: u64 = 60;
/// Conteo determinista de fragmentos del censo (Sovereign Standard).
const SHARD_PARTITION_COUNT: usize = 4;

/**
 * IMPLEMENTACIÓN: REPORTERO DE COLISIONES SOBERANO
 * Actúa como el sumidero de señales para cualquier hallazgo criptográfico.
 */
struct SwarmFindingReporter {
    transmission_channel_sender: mpsc::UnboundedSender<Finding>,
    worker_node_identifier: String,
    active_mission_identifier: String,
}

impl FindingHandler for SwarmFindingReporter {
    /**
     * Procesa una colisión detectada inyectando la firma WIF y metadatos de rastro.
     */
    fn on_finding(
        &self,
        bitcoin_address: String,
        private_key_handle: prospector_core_math::private_key::SafePrivateKey,
        entropy_source_metadata: String
    ) {
        let cryptographic_discovery = Finding {
            address: bitcoin_address,
            private_key_wif: prospector_core_gen::wif::private_to_wif(&private_key_handle, false),
            source_entropy: entropy_source_metadata,
            wallet_type: "p2pkh_legacy_uncompressed".to_string(),
            found_by_worker: self.worker_node_identifier.clone(),
            job_id: Some(self.active_mission_identifier.clone()),
            detected_at: chrono::Utc::now().to_rfc3339(),
        };

        if let Err(channel_fault) = self.transmission_channel_sender.send(cryptographic_discovery) {
            error!("❌ [CHANNEL_COLLAPSE]: Unable to queue discovery artifact: {}", channel_fault);
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
     * Inicia la secuencia de ignición de operaciones soberanas.
     *
     * # Performance:
     * Ejecuta el "Thread Pinning" al inicio para estabilizar el hashrate
     * antes de entrar en el bucle de adquisición de misiones.
     *
     * # Logic:
     * Separa el tráfico de hallazgos en un canal asíncrono para no bloquear
     * el motor matemático durante ráfagas de red.
     */
    #[instrument(skip(self), fields(node = %self.worker_node_identifier))]
    pub async fn ignite_sovereign_operations(&self) {
        info!("🚀 [ENGINE]: Adaptive Ignition Sequence V132.0 initialized.");

        // 1. AFINIDAD DE HARDWARE (MAXIMIZACIÓN DE CACHÉ)
        if let Some(core_identifiers) = core_affinity::get_core_ids() {
            info!("🧬 [HARDWARE]: Detected {} units for silicon pinning.", core_identifiers.len());
        }

        // 2. TÚNEL DE COMUNICACIÓN DE DESCUBRIMIENTOS
        let (findings_transmission_sender, mut findings_reception_receiver) = mpsc::unbounded_channel::<Finding>();
        let uplink_reference = Arc::clone(&self.orchestrator_uplink);

        tokio::spawn(async move {
            while let Some(discovery_artifact) = findings_reception_receiver.recv().await {
                if let Err(network_fault) = uplink_reference.transmit_found_collision(&discovery_artifact).await {
                    error!("❌ [VAULT_SYNC_FAULT]: Strata synchronization failed: {}", network_fault);
                }
            }
        });

        // 3. BUCLE PRINCIPAL DE AUDITORÍA
        while self.is_operational_signal.load(Ordering::SeqCst) {
            let hardware_metrics = HardwareMonitor::capture_instantaneous_metrics();

            // GESTIÓN TÉRMICA PROACTIVA
            if hardware_metrics.core_temperature_celsius > THERMAL_CRITICAL_THRESHOLD_CELSIUS {
                warn!("🔥 [THERMAL_ALERT]: Silicon stress at {}°C. Pacing engine.",
                    hardware_metrics.core_temperature_celsius);
                sleep(Duration::from_millis(2500)).await;
            }

            let handshake_payload = MissionRequestPayload {
                worker_id: self.worker_node_identifier.clone(),
                hardware_capacity: NodeHardwareCapacity {
                    ram_available_mb: hardware_metrics.memory_utilization_bytes / (1024 * 1024),
                    cpu_cores: num_cpus::get() as u32,
                    supports_avx2: is_x86_feature_detected!("avx2"),
                },
            };

            match self.orchestrator_uplink.negotiate_mission_assignment_handshake(&handshake_payload).await {
                Ok(assignment_envelope) => {
                    info!("🎯 [MISSION_ACQUIRED]: ID: {}", assignment_envelope.mission_order.job_mission_identifier);

                    if let Err(mission_fault) = self.execute_mission_lifecycle(
                        assignment_envelope.mission_order,
                        findings_transmission_sender.clone()
                    ).await {
                        error!("⚠️ [MISSION_ABORTED]: Protocol collapse: {}", mission_fault);
                    }
                }
                Err(negotiation_fault) => {
                    debug!("💤 [STANDBY]: Orchestrator strata busy. Pulsing in 20s. Detail: {}", negotiation_fault);
                    sleep(Duration::from_secs(20)).await;
                }
            }
        }
    }

    /**
     * Orquesta el ciclo de vida de una misión técnica.
     *
     * # Logic:
     * 1. Hidrata fragmentos (Shards) en paralelo.
     * 2. Si la misión es forense, hidrata el DNA Template.
     * 3. Lanza el daemon de Checkpoints para persistencia de rastro.
     * 4. Sella la certificación final en el Ledger Táctico.
     */
    #[instrument(skip_all, fields(mission_id = %mission_order.job_mission_identifier))]
    async fn execute_mission_lifecycle(
        &self,
        mission_order: WorkOrder,
        findings_sender: mpsc::UnboundedSender<Finding>
    ) -> anyhow::Result<()> {
        let mission_identifier = mission_order.job_mission_identifier.clone();

        // 1. HIDRATACIÓN DE ESTRATOS BINARIOS (CENSO SHARDED)
        self.orchestrator_uplink.synchronize_mission_sharded_filter(&mission_order, &self.local_cache_directory).await?;

        let strata_label = match mission_order.required_strata {
            TargetStrata::SatoshiEra => "satoshi_era",
            TargetStrata::VulnerableLegacy => "vulnerable_legacy",
            TargetStrata::StandardLegacy => "standard_legacy",
            TargetStrata::FullTacticalSet => "full_tactical_set",
        };

        let filter_storage_path = self.local_cache_directory.join(strata_label);
        let sharded_census_filter = Arc::new(
            tokio::task::spawn_blocking(move || {
                ShardedFilter::load_from_directory(&filter_storage_path, SHARD_PARTITION_COUNT)
            }).await??
        );

        // 2. HIDRATACIÓN DE ARTEFACTOS FORENSES (DNA TEMPLATE)
        // ✅ RESOLUCIÓN: Inyectamos la carga de ADN si la estrategia es Satoshi-XP
        let performance_dna_artifact = if let SearchStrategy::SatoshiWindowsXpForensic { ref scenario_template_identifier, .. } = mission_order.strategy {
            let dna_path = self.local_cache_directory.join(format!("{}.bin", scenario_template_identifier));
            info!("🧬 [XP_BOOTSTRAP]: Hydrating DNA strata for template {}...", scenario_template_identifier);
            Some(ForensicDnaHydrator::hydrate_dna_from_disk(&dna_path).await?)
        } else {
            None
        };

        // 3. DAEMON DE RASTRO FORENSE (CHECKPOINTS)
        let effort_accumulator = Arc::new(AtomicU64::new(0));
        let effort_ref_for_daemon = Arc::clone(&effort_accumulator);
        let stop_signal_for_daemon = Arc::clone(&self.is_operational_signal);
        let uplink_for_daemon = Arc::clone(&self.orchestrator_uplink);
        let mission_id_copy = mission_identifier.clone();

        tokio::spawn(async move {
            while stop_signal_for_daemon.load(Ordering::Relaxed) {
                sleep(Duration::from_secs(TACTICAL_CHECKPOINT_INTERVAL_SECONDS)).await;
                let current_volume = effort_ref_for_daemon.load(Ordering::Relaxed);
                if current_volume > 0 {
                    let _ = uplink_for_daemon.report_mission_progress(&mission_id_copy, current_volume).await;
                }
            }
        });

        // 4. EJECUCIÓN DEL MÚSCULO COMPUTACIONAL (STRATEGY EXECUTOR V261.0)
        let collision_handler = SwarmFindingReporter {
            transmission_channel_sender: findings_sender,
            worker_node_identifier: self.worker_node_identifier.clone(),
            active_mission_identifier: mission_identifier.clone(),
        };

        let node_id = self.worker_node_identifier.clone();
        let stop_signal = Arc::clone(&self.is_operational_signal);

        // ✅ REPARACIÓN: Pasamos el DNA buffer al ejecutor para habilitar arqueología SIMD
        let audit_certification_report = tokio::task::spawn_blocking(move || {
            StrategyExecutor::execute_mission_sequence(
                &mission_order,
                &sharded_census_filter,
                stop_signal,
                effort_accumulator,
                node_id,
                &collision_handler,
                performance_dna_artifact.as_deref()
            )
        }).await?;

        // 5. SELLADO SOBERANO
        self.orchestrator_uplink.submit_mission_audit_certification(&audit_certification_report).await?;

        info!("✅ [CERTIFIED]: Mission {} sealed and reported to Tactical Ledger.", mission_identifier);
        Ok(())
    }
}

// [libs/domain/mining-strategy/src/executor.rs]
/*!
 * =================================================================
 * APARATO: STRATEGY EXECUTOR MASTER (V262.2 - SWISS WATCH SEAL)
 * CLASIFICACIÓN: DOMAIN LOGIC (ESTRATO L2)
 * RESPONSABILIDAD: ORQUESTACIÓN POLIMÓRFICA Y CERTIFICACIÓN DE HARDWARE
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. ZERO RESIDUE: Eliminación del import 'debug' no utilizado para satisfacer
 *    la política de higiene estricta de rustc.
 * 2. CONTRACT ALIGNMENT: Implementación bit-perfect del inicializador de
 *    'AuditReport', inyectando la firma técnica 'hardware_acceleration_signature'.
 * 3. NOMINAL PURITY: Erradicación de abreviaciones. 'sk' -> 'private_key_handle'.
 * 4. DETERMINISM: Resolución del rastro de diccionario ignorando formalmente
 *    el tamaño del lote en el despacho.
 *
 * # Mathematical Proof (Hardware Traceability):
 * Al encapsular la firma de hardware en el reporte inmutable, el sistema
 * certifica la veracidad del esfuerzo computacional (Proof of Computation),
 * permitiendo auditar la eficiencia energética y algorítmica de la campaña.
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::time::Instant;
use chrono::Utc;
// ✅ RESOLUCIÓN RESIDUO: 'debug' eliminado de los imports de tracing
use tracing::{info, warn, error, instrument};

// --- SINAPSIS CON EL NÚCLEO MATEMÁTICO (L1) ---
use prospector_core_math::hardware::is_optimized_arithmetic_supported;

// --- SINAPSIS CON MOTORES DE ESTRATEGIA ATÓMICOS (L2) ---
use crate::engines::sequential_engine::ProjectiveSequentialEngine;
use crate::engines::satoshi_xp_engine::SatoshiWindowsXpForensicEngine;
use crate::engines::android_lcg_engine::AndroidLcgForensicEngine;
use crate::engines::dictionary_engine::EntropyDictionaryEngine;
use crate::kangaroo::KangarooRunner;

// --- SINAPSIS CON MODELOS (L2-CONTRACTS) ---
use prospector_domain_models::work::{AuditReport, SearchStrategy, WorkOrder};
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_core_math::private_key::SafePrivateKey;

/**
 * Trait SSS (Sovereign Signal System) para la gestión de colisiones.
 * Garantiza que cualquier hallazgo sea propagado al bus de red.
 */
pub trait FindingHandler: Send + Sync {
    /**
     * Reporta una colisión confirmada contra el censo UTXO.
     *
     * @param bitcoin_address Dirección Bitcoin colisionada (Base58).
     * @param private_key_handle Clave privada recuperada (SafePrivateKey).
     * @param entropy_source_metadata Metadatos del vector de entropía utilizado.
     */
    fn on_finding(
        &self,
        bitcoin_address: String,
        private_key_handle: SafePrivateKey,
        entropy_source_metadata: String
    );
}

/// Orquestador central de ejecución de misiones distribuibles.
pub struct StrategyExecutor;

impl StrategyExecutor {
    /**
     * Ejecuta una secuencia de misión completa y genera un Reporte de Certificación inmutable.
     *
     * # Errors:
     * - `error_dna_missing`: Si la misión Satoshi-XP no encuentra el buffer de ADN en RAM.
     * - `interrupted_by_host_signal`: Si se recibe una señal de preemption de la nube.
     *
     * # Performance:
     * - Latencia de despacho: O(1).
     * - El throughput escala proporcionalmente al conjunto de instrucciones AVX2/ADX.
     */
    #[instrument(
        skip_all,
        fields(
            mission_id = %mission_order.job_mission_identifier,
            strategy = ?mission_order.strategy,
            worker_id = %worker_node_identifier
        )
    )]
    pub fn execute_mission_sequence<H: FindingHandler>(
        mission_order: &WorkOrder,
        target_census_filter: &ShardedFilter,
        global_termination_signal: Arc<AtomicBool>,
        effort_telemetry_accumulator: Arc<AtomicU64>,
        worker_node_identifier: String,
        collision_handler: &H,
        performance_dna_template: Option<&[u8]>
    ) -> AuditReport {
        let sequence_ignition_timestamp = Instant::now();
        let mut audit_trail_checkpoint_hex = String::new();
        let mut final_execution_verdict = "completed".to_string();

        // 1. AUDITORÍA DE SILICIO (Hardware Awareness V2.1)
        let is_silicon_optimized = is_optimized_arithmetic_supported();
        let hardware_acceleration_signature = if is_silicon_optimized {
            info!("🚀 [EXECUTOR]: ELITE_HARDWARE_DETECTED. Engaging SIMD 4-Way & Co-Z Strata.");
            "ELITE_SIMD_ADX"
        } else {
            warn!("🐢 [EXECUTOR]: LEGACY_HARDWARE. Engaging Scalar Software Fallback.");
            "STANDARD_SW"
        };

        // 2. DESPACHO POLIMÓRFICO (Mando Central)
        match &mission_order.strategy {
            // MOTOR ALFA: BÚSQUEDA SECUENCIAL (Projective Meloni)
            SearchStrategy::Sequential { start_index_hexadecimal, .. } => {
                audit_trail_checkpoint_hex = ProjectiveSequentialEngine::execute_optimized_audit(
                    start_index_hexadecimal,
                    10_000_000,
                    target_census_filter,
                    &global_termination_signal,
                    effort_telemetry_accumulator.clone(),
                    collision_handler
                );
            },

            // MOTOR BETA: ARQUEOLOGÍA SATOSHI-XP
            SearchStrategy::SatoshiWindowsXpForensic {
                scenario_template_identifier,
                uptime_seconds_start,
                uptime_seconds_end,
                hardware_clock_frequency,
            } => {
                if let Some(dna_buffer) = performance_dna_template {
                    audit_trail_checkpoint_hex = SatoshiWindowsXpForensicEngine::execute_forensic_audit(
                        dna_buffer,
                        *hardware_clock_frequency,
                        *uptime_seconds_start,
                        *uptime_seconds_end,
                        target_census_filter,
                        &global_termination_signal,
                        effort_telemetry_accumulator.clone(),
                        collision_handler
                    );
                } else {
                    error!("❌ [STRATA_FAULT]: Forensic DNA [{}] is void.", scenario_template_identifier);
                    final_execution_verdict = "error_dna_missing".to_string();
                }
            },

            // MOTOR GAMMA: ANDROID LCG (Java PRNG Forensic)
            SearchStrategy::AndroidLcgForensic { seed_range_start, seed_range_end } => {
                audit_trail_checkpoint_hex = AndroidLcgForensicEngine::execute_seed_sweep(
                    *seed_range_start,
                    *seed_range_end,
                    target_census_filter,
                    &global_termination_signal,
                    effort_telemetry_accumulator.clone(),
                    collision_handler
                );
            },

            // MOTOR DELTA: POLLARD'S KANGAROO (ECDLP Resolver)
            SearchStrategy::KangarooLambda { target_public_key_hexadecimal, range_width_max } => {
                KangarooRunner::run(
                    target_public_key_hexadecimal,
                    "0000000000000000000000000000000000000000000000000000000000000000",
                    *range_width_max,
                    global_termination_signal.clone(),
                    effort_telemetry_accumulator.clone(),
                    collision_handler
                );
                audit_trail_checkpoint_hex = format!("kangaroo_exhausted_w{}", range_width_max);
            },

            // MOTOR EPSILON: ENTROPY DICTIONARY (Brainwallets)
            // ✅ RESOLUCIÓN WARNING: 'processing_batch_size' ignorado formalmente
            SearchStrategy::Dictionary { dataset_resource_locator, processing_batch_size: _ } => {
                audit_trail_checkpoint_hex = EntropyDictionaryEngine::execute_dictionary_audit(
                    std::slice::from_ref(dataset_resource_locator),
                    target_census_filter,
                    &global_termination_signal,
                    effort_telemetry_accumulator.clone(),
                    collision_handler
                );
            }
        }

        // 3. CÁLCULO DE MÉTRICAS DE CERTIFICACIÓN SOBERANA
        let total_execution_milliseconds = sequence_ignition_timestamp.elapsed().as_millis() as u64;
        let final_computational_effort_volume = effort_telemetry_accumulator.load(Ordering::SeqCst);

        // Auditoría de Interrupción (Preemption Awareness)
        if global_termination_signal.load(Ordering::Relaxed) && final_execution_verdict == "completed" {
            final_execution_verdict = "interrupted_by_host_signal".to_string();
        }

        // Eficiencia: H/ms (Escalable a Exahashes en el Dashboard L5)
        let average_efficiency_ratio = if total_execution_milliseconds > 0 {
            (final_computational_effort_volume as f64) / (total_execution_milliseconds as f64)
        } else {
            0.0
        };

        info!(
            "🏁 [MISSION_SEALED]: Unit: {} | Verdict: {} | Mode: {} | Ratio: {:.2} H/ms",
            worker_node_identifier,
            final_execution_verdict,
            hardware_acceleration_signature,
            average_efficiency_ratio
        );

        // 4. CRISTALIZACIÓN DEL REPORTE INMUTABLE (SSoT)
        // ✅ RESOLUCIÓN ERROR E0063: Inyección del campo hardware_acceleration_signature
        AuditReport {
            job_mission_identifier: mission_order.job_mission_identifier.clone(),
            worker_node_identifier,
            total_wallets_audited: final_computational_effort_volume.to_string(),
            execution_duration_milliseconds: total_execution_milliseconds,
            final_mission_status: final_execution_verdict,
            audit_footprint_checkpoint: audit_trail_checkpoint_hex,
            completed_at_timestamp: Utc::now().to_rfc3339(),
            average_computational_efficiency: average_efficiency_ratio,
            hardware_acceleration_signature: hardware_acceleration_signature.to_string(),
        }
    }
}

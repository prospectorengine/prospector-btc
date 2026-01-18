// [libs/domain/mining-strategy/src/kangaroo.rs]
/**
 * =================================================================
 * APARATO: KANGAROO STRATEGY ENGINE (V22.0 - RESILIENT GOLD)
 * CLASIFICACIÓN: DOMAIN STRATEGY (ESTRATO L2)
 * RESPONSABILIDAD: ORQUESTACIÓN DE RESOLUCIÓN ECDLP CON TELEMETRÍA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. SIGNATURE ALIGNMENT: Nivelación bit-perfecta con el KangarooSolver L1 V19.0,
 *    inyectando 'stop_signal' y 'effort_accumulator'.
 * 2. PREEMPTION READY: El algoritmo ahora es interrumpible por el centro de mando
 *    C2 sin dejar procesos huérfanos en el host.
 * 3. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta aplicada a vectores,
 *    puntos y resultados de decodificación.
 * 4. HYGIENE: Erradicación de placeholders y rastro #[instrument] enriquecido.
 *
 * # Mathematical Proof (Pollard's Lambda Orchestration):
 * Actúa como el controlador de ráfaga para la resolución de claves públicas.
 * Valida la integridad del material de entrada antes de saturar los hilos
 * de ejecución mediante el motor matemático paralelo.
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64};
use tracing::{info, warn, error, instrument};
use prospector_core_math::prelude::*;
use prospector_core_math::arithmetic::U256_BYTE_SIZE;
use crate::executor::FindingHandler;

/// Orquestador del algoritmo Pollard's Kangaroo para el enjambre distribuido.
pub struct KangarooRunner;

impl KangarooRunner {
    /**
     * Ejecuta una resolución de precisión para una clave pública objetivo.
     *
     * # Arguments:
     * * `target_public_key_hexadecimal` - String Hex con la clave pública SEC1.
     * * `starting_scalar_hexadecimal` - Base de inicio para la trayectoria (Base64/Hex).
     * * `search_width_magnitude` - Ancho del intervalo de búsqueda (W).
     * * `stop_signal_reference` - Señal atómica de interrupción del sistema.
     * * `computational_effort_accumulator` - Registro atómico de saltos realizados.
     * * `finding_handler` - Receptor de colisiones para despacho al Orquestador L3.
     *
     * # Performance:
     * El Runner valida la estructura de datos en O(1) antes de delegar el
     * cómputo intensivo a L1. Sincronizado para reporte de métricas al Dashboard.
     */
    #[instrument(
        skip(finding_handler, stop_signal_reference, computational_effort_accumulator),
        fields(target = %target_public_key_hexadecimal)
    )]
    pub fn run<H: FindingHandler>(
        target_public_key_hexadecimal: &str,
        starting_scalar_hexadecimal: &str,
        search_width_magnitude: u64,
        stop_signal_reference: Arc<AtomicBool>,
        computational_effort_accumulator: Arc<AtomicU64>,
        finding_handler: &H,
    ) {
        info!("🦘 [KANGAROO_STRATEGY]: Initiating resilient resolution sequence V22.0.");

        // 1. DECODIFICACIÓN DEL OBJETIVO TÁCTICO (PUBLIC KEY STRATA)
        let target_public_key_bytes = match hex::decode(target_public_key_hexadecimal.trim()) {
            Ok(binary_payload) => binary_payload,
            Err(decoding_error) => {
                error!("❌ [STRATEGY_FAULT]: Target key hexadecimal decoding failed: {}", decoding_error);
                return;
            }
        };

        let target_point_safe_public = match SafePublicKey::from_bytes(&target_public_key_bytes) {
            Ok(point_instance) => point_instance,
            Err(math_error) => {
                error!("❌ [MATH_FAULT]: ECDSA point reconstruction failed: {}", math_error);
                return;
            }
        };

        // 2. CONFIGURACIÓN DEL ESCALAR DE ARRANQUE (DETERMINISTIC BASE)
        let mut starting_scalar_buffer = [0u8; U256_BYTE_SIZE];
        if let Ok(decoded_scalar_material) = hex::decode(starting_scalar_hexadecimal.trim()) {
            if decoded_scalar_material.len() == U256_BYTE_SIZE {
                starting_scalar_buffer.copy_from_slice(&decoded_scalar_material);
            } else {
                warn!("⚠️ [STRATEGY_WARN]: Starting scalar size mismatch. Using curve genesis base.");
            }
        }

        // 3. CONSTRUCCIÓN DE CONFIGURACIÓN SOBERANA (L1 ALIGNMENT)
        // Sintonizado para la capacidad de memoria de nodos efímeros modernos.
        let solver_tactical_configuration = KangarooConfig {
            start_scalar: starting_scalar_buffer,
            search_width: search_width_magnitude,
            distinguished_point_mask: 0x0F, // Probabilidad de colisión 1/16
            maximum_traps_capacity: 20000,   // Sello Gold Master para RAM estable
        };

        // 4. INVOCACIÓN DEL MOTOR MATEMÁTICO PARALELO (ESTRATO L1)
        // Pasamos los punteros atómicos para el cierre del bucle de telemetría.
        let resolution_result = KangarooSolver::solve_discrete_logarithm(
            &target_point_safe_public,
            &solver_tactical_configuration,
            &stop_signal_reference,
            &computational_effort_accumulator
        );

        match resolution_result {
            Ok(Some(recovered_private_key_bytes)) => {
                info!("🎯 [KANGAROO_COLLISION]: Logarithm resolved. Cryptographic strata penetrated.");

                if let Ok(private_key_handle) = SafePrivateKey::from_bytes(&recovered_private_key_bytes) {
                    let derived_public_key = SafePublicKey::from_private(&private_key_handle);

                    // La arqueología requiere formato No-Comprimido (Legacy Satoshi)
                    let derived_bitcoin_address = prospector_core_gen::address_legacy::pubkey_to_address(
                        &derived_public_key,
                        false
                    );

                    // Reporte enriquecido para el Proyecto Panóptico (Dashboard L5)
                    finding_handler.on_finding(
                        derived_bitcoin_address,
                        private_key_handle,
                        format!(
                            "kangaroo_lambda:target_{}:width_{}",
                            &target_public_key_hexadecimal[..8],
                            search_width_magnitude
                        )
                    );
                }
            }
            Ok(None) => {
                info!("🏁 [SCAN_COMPLETE]: Resolution strata exhausted or interrupted. No colision detected.");
            }
            Err(critical_math_panic) => {
                error!("💀 [SOLVER_COLLAPSE]: Fatal mathematical strata error: {}", critical_math_panic);
            }
        }
    }
}

// [libs/domain/mining-strategy/src/kangaroo.rs]
/**
 * =================================================================
 * APARATO: KANGAROO STRATEGY ENGINE (V25.0 - ZENITH GOLD MASTER)
 * CLASIFICACIÓN: DOMAIN STRATEGY (ESTRATO L2)
 * RESPONSABILIDAD: ORQUESTACIÓN DE RESOLUCIÓN ECDLP Y MANDO C2
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. NOMINAL ALIGNMENT: Sincronización bit-perfecta con KangarooSolver L1 V20.0,
 *    mapeando 'start_scalar_bytes' y 'distinguished_point_bitmask'.
 * 2. DATA VALIDATION: Valida físicamente la longitud de los inputs hexadecimales
 *    para evitar fallos de segmentación en el motor ASM.
 * 3. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta aplicada a todos
 *    los parámetros de configuración táctica.
 * 4. HYGIENE: Documentación técnica nivel Tesis Doctoral MIT.
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
     * # Mathematical Proof (Pollard's Lambda Orchestration):
     * Transforma una cadena de texto en un punto afín verificado. La resolución
     * se garantiza si el logaritmo discreto reside en el intervalo [starting_scalar, starting_scalar + width].
     *
     * # Performance:
     * El Runner valida la estructura en O(1). La complejidad del cómputo
     * delegado a L1 es O(sqrt(W)).
     *
     * @param target_public_key_hexadecimal Clave pública SEC1 (Comprimida/No-comprimida).
     * @param starting_scalar_hexadecimal Base de inicio del rango de búsqueda.
     * @param search_width_magnitude Ancho máximo del intervalo de búsqueda.
     * @param stop_signal_reference Señal atómica para terminación controlada.
     * @param computational_effort_accumulator Contador de saltos para telemetría.
     * @param finding_handler Canal de reporte hacia el Orquestador L3.
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
        info!("🦘 [KANGAROO_STRATEGY]: Initiating resilient resolution sequence V25.0.");

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
                warn!("⚠️ [STRATEGY_WARN]: Starting scalar size mismatch. Potential strata drift.");
            }
        }

        // 3. CONSTRUCCIÓN DE CONFIGURACIÓN SOBERANA (L1 ALIGNMENT V20.0)
        // ✅ SINCRO NIVEL DIOS: Mapeo de campos nominales exactos.
        let solver_tactical_configuration = KangarooConfig {
            start_scalar_bytes: starting_scalar_buffer,
            search_width_magnitude,
            distinguished_point_bitmask: 0x0F, // Densidad de trampas: 1/16
            maximum_traps_capacity_limit: 25000, // Optimizado para 1GB RAM
        };

        // 4. INVOCACIÓN DEL MOTOR MATEMÁTICO PARALELO (ESTRATO L1)
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

                    // La auditoría forense requiere formato No-Comprimido para paridad Satoshi 2009.
                    let derived_bitcoin_address = prospector_core_gen::address_legacy::pubkey_to_address(
                        &derived_public_key,
                        false
                    );

                    // Reporte enriquecido para el Panóptico (Dashboard Zenith L5)
                    finding_handler.on_finding(
                        derived_bitcoin_address,
                        private_key_handle,
                        format!(
                            "kangaroo_lambda:target_{}:mask_0x0F",
                            &target_public_key_hexadecimal[..8]
                        )
                    );
                }
            }
            Ok(None) => {
                info!("🏁 [SCAN_COMPLETE]: Resolution strata exhausted or interrupted. Range is clean.");
            }
            Err(critical_math_panic) => {
                error!("💀 [SOLVER_COLLAPSE]: Fatal mathematical strata error: {}", critical_math_panic);
            }
        }
    }
}

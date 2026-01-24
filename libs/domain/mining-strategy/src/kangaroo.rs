// [libs/domain/mining-strategy/src/kangaroo.rs]
/**
 * =================================================================
 * APARATO: KANGAROO STRATEGY ENGINE (V25.1 - MEMORY ALIGNED)
 * CLASIFICACIÓN: DOMAIN STRATEGY (ESTRATO L2)
 * RESPONSABILIDAD: ORQUESTACIÓN DE RESOLUCIÓN ECDLP Y MANDO C2
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. MEMORY BUDGET ALIGNMENT: Resuelve E0560 sincronizando con el contrato
 *    KangarooConfig V23.0. Sustituye límites estáticos por 'memory_budget_mb'.
 * 2. QUANTUM CONFIGURATION: Fija la densidad de puntos distinguidos en 0x0F (1/16)
 *    para equilibrar el hashrate y el consumo de RAM en Colab.
 * 3. NOMINAL PURITY: Erradicación total de abreviaciones. 'config' -> 'tactical_configuration'.
 * 4. HYGIENE: Documentación técnica MIT y rastro forense #[instrument].
 *
 * # Mathematical Proof (Pollard's Lambda Orchestration):
 * El Runner materializa el punto objetivo Q y el escalar de inicio k_0.
 * La resolución k = k_0 + (dist_tame - dist_wild) se garantiza si el 
 * logaritmo reside en el ancho de búsqueda definido.
 * =================================================================
 */

 use std::sync::Arc;
 use std::sync::atomic::{AtomicBool, AtomicU64};
 use tracing::{info, warn, error, instrument, debug};
 use prospector_core_math::prelude::*;
 use prospector_core_math::arithmetic::U256_BYTE_SIZE;
 use crate::executor::FindingHandler;
 
 /// Orquestador del algoritmo Pollard's Kangaroo para el enjambre distribuido.
 pub struct KangarooRunner;
 
 impl KangarooRunner {
     /**
      * Ejecuta una resolución de precisión para una clave pública objetivo.
      *
      * # Performance:
      * El orquestador opera en O(1) memoria. El esfuerzo computacional
      * real ocurre en L1 con complejidad O(sqrt(W)).
      *
      * @param target_public_key_hexadecimal Clave pública SEC1.
      * @param starting_scalar_hexadecimal Base de inicio del rango.
      * @param search_width_magnitude Ancho máximo del intervalo.
      * @param stop_signal_reference Señal atómica para terminación.
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
         info!("🦘 [KANGAROO_STRATEGY]: Initiating resilient resolution sequence V25.1.");
 
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
 
         // 3. CONSTRUCCIÓN DE CONFIGURACIÓN SOBERANA (L1 ALIGNMENT V23.0)
         // ✅ RESOLUCIÓN E0560: Inyección de memory_budget_mb para heurística de RAM L1
         let solver_tactical_configuration = KangarooConfig {
             start_scalar_bytes: starting_scalar_buffer,
             search_width_magnitude,
             distinguished_point_bitmask: 0x0F, // Densidad de trampas: 1/16
             memory_budget_mb: 1024,            // Presupuesto de 1GB para silos efímeros
         };
 
         debug!("⚙️ [KANGAROO_CONFIG]: Stratum levelized. Budget: 1024MB.");
 
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
 
                     // Formato forense: Satoshi Standard (No-comprimido)
                     let derived_bitcoin_address = prospector_core_gen::address_legacy::pubkey_to_address(
                         &derived_public_key,
                         false
                     );
 
                     // Reporte enriquecido para el Panóptico L5
                     finding_handler.on_finding(
                         derived_bitcoin_address,
                         private_key_handle,
                         format!(
                             "kangaroo_lambda:target_{}:mask_0x0F:budget_1GB",
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
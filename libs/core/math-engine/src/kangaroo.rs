// [libs/core/math-engine/src/kangaroo.rs]
/**
 * =================================================================
 * APARATO: KANGAROO MATRIX SOLVER (V19.2 - DOCUMENTATION SEALED)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: RESOLUCIÓN PARALELA DE ECDLP CON RESILIENCIA C2
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. FULL RUSTDOC: Sella el error de 'missing_docs' (Severity 8) inyectando
 *    especificaciones de Tesis en todos los tipos públicos.
 * 2. NOMINAL SYNC: Consistencia absoluta con 'arithmetic.rs' V121.0
 *    utilizando el estándar 'big_endian'.
 * 3. PREEMPTION READY: Monitoreo de señales de interrupción para detener
 *    el enjambre de saltos de forma determinista.
 * 4. HYGIENE: Erradicación de abreviaciones y rastro forense #[instrument].
 *
 * # Mathematical Proof (Pollard's Lambda with DP):
 * El algoritmo busca colisiones entre una trayectoria conocida (Tame) y
 * una desconocida (Wild) en un rango $w$. La probabilidad de colisión
 * se optimiza mediante el uso de Puntos Distinguidos para reducir
 * la ocupación de memoria en el Orquestador.
 * =================================================================
 */

use crate::prelude::*;
use crate::arithmetic::{
    add_u256_big_endian,
    subtract_u256_big_endian,
    convert_u128_to_u256_big_endian
};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use rayon::prelude::*;
use tracing::{info, debug, warn, instrument};

/// Configuración operativa para la ráfaga de resolución del algoritmo Canguro.
pub struct KangarooConfig {
    /// Escalar inicial hexadecimal del rango de búsqueda.
    pub start_scalar: [u8; 32],
    /// Ancho total del espacio de búsqueda (Rango $W$).
    pub search_width: u64,
    /// Máscara binaria para la detección de Puntos Distinguidos (ej: 0x0F para 1/16).
    pub distinguished_point_mask: u8,
    /// Capacidad máxima del almacén de trampas (traps) en memoria RAM.
    pub maximum_traps_capacity: usize,
}

#[derive(Clone, Copy)]
struct LeapTableEntry {
    /// Escalar de salto precomputado en formato de bytes.
    scalar_step: [u8; 32],
    /// Distancia lógica recorrida en la curva tras el salto.
    distance_weight: u128,
}

#[derive(Clone)]
struct KangarooUnit {
    /// Punto actual en la curva secp256k1 (Coordenada Afín).
    current_point: SafePublicKey,
    /// Distancia acumulada desde el origen de la trayectoria actual.
    cumulative_distance: [u8; 32],
}

impl KangarooUnit {
    /**
     * Ejecuta un salto estocástico determinista basado en la posición actual en la curva.
     *
     * # Logic
     * El índice de salto se deriva de la coordenada X del punto actual,
     * garantizando que ambos canguros (Tame y Wild) sigan la misma trayectoria
     * al colisionar.
     */
    #[inline(always)]
    fn perform_leap(
        &mut self,
        jump_matrix: &[LeapTableEntry; 32],
        effort_accumulator: &AtomicU64
    ) -> Result<(), MathError> {
        let point_bytes = self.current_point.to_bytes(true);
        // Determinismo de salto: Utilizamos el último byte como selector de matriz
        let jump_index = (point_bytes[32] % 32) as usize;
        let entry = &jump_matrix[jump_index];

        self.current_point = self.current_point.add_scalar(&entry.scalar_step)?;
        let leap_distance_u256 = convert_u128_to_u256_big_endian(entry.distance_weight);
        self.cumulative_distance = add_u256_big_endian(&self.cumulative_distance, &leap_distance_u256)?;

        // Reporte de esfuerzo para el HUD de telemetría
        effort_accumulator.fetch_add(1, Ordering::Relaxed);

        Ok(())
    }

    /**
     * Evalúa si el punto actual cumple con la máscara de Puntos Distinguidos.
     */
    #[inline(always)]
    fn is_at_distinguished_coordinates(&self, bit_mask: u8) -> bool {
        let point_bytes = self.current_point.to_bytes(true);
        (point_bytes[31] & bit_mask) == 0
    }
}

/// Solucionador de alto rendimiento para el Problema del Logaritmo Discreto (ECDLP).
pub struct KangarooSolver;

impl KangarooSolver {
    /**
     * Ejecuta la resolución criptográfica de un punto público con conciencia de sistema.
     *
     * # Mathematical Proof
     * Implementa el algoritmo de los Canguros de Pollard. La trayectoria 'Tame'
     * siembra trampas en el espacio de búsqueda. La trayectoria 'Wild' (iniciada
     * desde el punto objetivo) busca caer en una trampa para revelar el logaritmo.
     *
     * # Performance
     * Complejidad media de $2 \cdot \sqrt{W}$ operaciones de grupo. Utiliza
     * paralelismo masivo vía Rayon para la fase 'Wild'.
     *
     * # Errors
     * - Retorna `MathError` ante fallos de aritmética U256 o geometría de curva.
     * - Retorna `None` si el rango es agotado o si se recibe señal de interrupción C2.
     */
    #[instrument(skip_all, fields(width = config.search_width))]
    pub fn solve_discrete_logarithm(
        target_public_key: &SafePublicKey,
        config: &KangarooConfig,
        global_stop_signal: &AtomicBool,
        computational_effort_accumulator: &AtomicU64,
    ) -> Result<Option<[u8; 32]>, MathError> {
        // 1. GENERACIÓN DE MATRIZ DE SALTOS (DETERMINISTA)
        let mut jump_matrix = [LeapTableEntry { scalar_step: [0; 32], distance_weight: 0 }; 32];
        for (index, entry) in jump_matrix.iter_mut().enumerate() {
            let exponent_weight = 1u128 << (index / 2);
            entry.distance_weight = exponent_weight;
            entry.scalar_step = convert_u128_to_u256_big_endian(exponent_weight);
        }

        let start_private_key = SafePrivateKey::from_bytes(&config.start_scalar)?;
        let base_point = SafePublicKey::from_private(&start_private_key);
        let width_as_u256 = convert_u128_to_u256_big_endian(config.search_width as u128);

        // 2. TAME KANGAROO: Fase de sembrado de trampas
        debug!("🦘 [KANGAROO]: Deploying Tame Unit (The Hunter)...");
        let tame_start_point = base_point.add_scalar(&width_as_u256)?;
        let mut tame_unit = KangarooUnit {
            current_point: tame_start_point,
            cumulative_distance: width_as_u256,
        };

        let mut trap_vault: HashMap<Vec<u8>, [u8; 32]> = HashMap::with_capacity(config.maximum_traps_capacity);
        let max_steps_threshold = (config.search_width as f64).sqrt() as usize * 4;

        for step_index in 0..max_steps_threshold {
            // Sensor de preemption
            if step_index % 1024 == 0 && global_stop_signal.load(Ordering::Relaxed) {
                warn!("🛑 [KANGAROO]: Tame sequence interrupted.");
                return Ok(None);
            }

            tame_unit.perform_leap(&jump_matrix, computational_effort_accumulator)?;

            if tame_unit.is_at_distinguished_coordinates(config.distinguished_point_mask) {
                trap_vault.insert(tame_unit.current_point.to_bytes(true), tame_unit.cumulative_distance);
                if trap_vault.len() >= config.maximum_traps_capacity { break; }
            }
        }

        // 3. WILD KANGAROO: Enjambre paralelo de búsqueda
        info!("🦘 [KANGAROO]: Searching with Wild Units ({} traps set)...", trap_vault.len());
        let shared_trap_vault = Arc::new(trap_vault);

        let result = (0..rayon::current_num_threads()).into_par_iter().find_map_any(|thread_identifier| {
            let initial_offset_u256 = convert_u128_to_u256_big_endian(thread_identifier as u128);
            let wild_start_point = target_public_key.add_scalar(&initial_offset_u256).ok()?;

            let mut wild_unit = KangarooUnit {
                current_point: wild_start_point,
                cumulative_distance: initial_offset_u256,
            };

            for step_index in 0..max_steps_threshold {
                if step_index % 1024 == 0 && global_stop_signal.load(Ordering::Relaxed) {
                    return None;
                }

                if wild_unit.perform_leap(&jump_matrix, computational_effort_accumulator).is_err() {
                    break;
                }

                if wild_unit.is_at_distinguished_coordinates(config.distinguished_point_mask) {
                    let point_signature = wild_unit.current_point.to_bytes(true);
                    if let Some(tame_distance_stored) = shared_trap_vault.get(&point_signature) {

                        // ¡COLISIÓN! k = tame_dist - wild_dist
                        if let Ok(distance_delta) = subtract_u256_big_endian(tame_distance_stored, &wild_unit.cumulative_distance) {
                            if let Ok(final_private_scalar) = add_u256_big_endian(&config.start_scalar, &distance_delta) {
                                info!("🎯 [KANGAROO_MATCH]: Target located in thread {}.", thread_identifier);
                                return Some(final_private_scalar);
                            }
                        }
                    }
                }
            }
            None
        });

        Ok(result)
    }
}

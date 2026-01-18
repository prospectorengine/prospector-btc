// [libs/core/math-engine/src/kangaroo.rs]
/**
 * =================================================================
 * APARATO: KANGAROO MATRIX SOLVER (V19.0 - RESILIENT GOLD)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: RESOLUCIÓN PARALELA DE ECDLP CON RESILIENCIA C2
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. RESILIENCE INJECTION: Implementa el monitoreo de 'global_stop_signal' en
 *    ambas fases (Tame & Wild), previniendo bloqueos de hilo en preemption.
 * 2. EFFORT TELEMETRY: Sincroniza el rastro de auditoría con 'computational_effort_accumulator'
 *    para reporte de Hashrate en tiempo real al Dashboard.
 * 3. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta aplicada a matrices y pesos.
 * 4. HYGIENE: Documentación técnica nivel Tesis Doctoral y rastro #[instrument].
 *
 * # Mathematical Proof (Pollard's Lambda with DP):
 * El algoritmo utiliza Distinguished Points (DP) para encontrar una colisión entre
 * el conejo "Tame" (trayectoria conocida) y el "Wild" (trayectoria del objetivo).
 * La complejidad media es de 2 * sqrt(search_width) operaciones.
 * =================================================================
 */

use crate::prelude::*;
use crate::arithmetic::{add_u256_be, subtract_u256_be, convert_u128_to_u256_be};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use rayon::prelude::*;
use tracing::{info, debug, warn, instrument};

/// Configuración operativa para la ráfaga de resolución del Canguro.
pub struct KangarooConfig {
    /// Escalar inicial hexadecimal del rango.
    pub start_scalar: [u8; 32],
    /// Ancho del espacio de búsqueda (W).
    pub search_width: u64,
    /// Máscara para la detección de Distinguished Points (ej: 0x0F).
    pub distinguished_point_mask: u8,
    /// Capacidad máxima del almacén de trampas en RAM.
    pub maximum_traps_capacity: usize,
}

#[derive(Clone, Copy)]
struct LeapTableEntry {
    /// Escalar de salto precomputado.
    scalar_step: [u8; 32],
    /// Distancia lógica recorrida en la curva.
    distance_weight: u128,
}

#[derive(Clone)]
struct KangarooUnit {
    /// Punto actual en la curva secp256k1.
    current_point: SafePublicKey,
    /// Distancia acumulada desde el origen de la trayectoria.
    cumulative_distance: [u8; 32],
}

impl KangarooUnit {
    /**
     * Ejecuta un salto estocástico determinista basado en la posición actual.
     */
    #[inline(always)]
    fn perform_leap(
        &mut self,
        jump_matrix: &[LeapTableEntry; 32],
        effort_accumulator: &AtomicU64
    ) -> Result<(), MathError> {
        let point_bytes = self.current_point.to_bytes(true);
        // Determinismo de salto: Utilizamos el último byte de la coordenada X
        let jump_index = (point_bytes[32] % 32) as usize;
        let entry = &jump_matrix[jump_index];

        self.current_point = self.current_point.add_scalar(&entry.scalar_step)?;
        let leap_distance_u256 = convert_u128_to_u256_be(entry.distance_weight);
        self.cumulative_distance = add_u256_be(&self.cumulative_distance, &leap_distance_u256)?;

        // Registro de esfuerzo para telemetría L5
        effort_accumulator.fetch_add(1, Ordering::Relaxed);

        Ok(())
    }

    #[inline(always)]
    fn is_at_distinguished_coordinates(&self, bit_mask: u8) -> bool {
        let point_bytes = self.current_point.to_bytes(true);
        // Distinguished Point: El punto cumple con tener N ceros en el rastro binario
        (point_bytes[31] & bit_mask) == 0
    }
}

pub struct KangarooSolver;

impl KangarooSolver {
    /**
     * Ejecuta la resolución criptográfica de un punto público con conciencia de sistema.
     *
     * # Errors:
     * - Retorna `MathError` ante fallos de aritmética U256 o geometría de curva.
     * - Retorna `None` si el rango es agotado o si se recibe señal de interrupción.
     *
     * # Performance:
     * El throughput es monitoreado en tiempo real. Utiliza 'find_map_any' de Rayon
     * para abortar todos los hilos inmediatamente después de localizar la colisión.
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
            entry.scalar_step = convert_u128_to_u256_be(exponent_weight);
        }

        let start_private_key = SafePrivateKey::from_bytes(&config.start_scalar)?;
        let base_point = SafePublicKey::from_private(&start_private_key);
        let width_as_u256 = convert_u128_to_u256_be(config.search_width as u128);

        // 2. TAME KANGAROO: Colocación de trampas Distinguished Points
        debug!("🦘 [KANGAROO]: Deploying Tame Unit (The Hunter)...");
        let tame_start_point = base_point.add_scalar(&width_as_u256)?;
        let mut tame_unit = KangarooUnit {
            current_point: tame_start_point,
            cumulative_distance: width_as_u256,
        };

        let mut trap_vault: HashMap<Vec<u8>, [u8; 32]> = HashMap::with_capacity(config.maximum_traps_capacity);
        let max_steps_threshold = (config.search_width as f64).sqrt() as usize * 4;

        for step_index in 0..max_steps_threshold {
            // SENSOR DE INTERRUPCIÓN: Fase Tame
            if step_index % 1024 == 0 && global_stop_signal.load(Ordering::Relaxed) {
                warn!("🛑 [KANGAROO]: Tame sequence interrupted by host signal.");
                return Ok(None);
            }

            tame_unit.perform_leap(&jump_matrix, computational_effort_accumulator)?;

            if tame_unit.is_at_distinguished_coordinates(config.distinguished_point_mask) {
                trap_vault.insert(tame_unit.current_point.to_bytes(true), tame_unit.cumulative_distance);
                if trap_vault.len() >= config.maximum_traps_capacity { break; }
            }
        }

        // 3. WILD KANGAROO: El enjambre paralelo busca la trampa
        info!("🦘 [KANGAROO]: Searching with Wild Units ({} traps set)...", trap_vault.len());
        let shared_trap_vault = Arc::new(trap_vault);

        // Paralelización por hilos de CPU (Rayon)
        let result = (0..rayon::current_num_threads()).into_par_iter().find_map_any(|thread_identifier| {
            let initial_offset_u256 = convert_u128_to_u256_be(thread_identifier as u128);
            let wild_start_point = target_public_key.add_scalar(&initial_offset_u256).ok()?;

            let mut wild_unit = KangarooUnit {
                current_point: wild_start_point,
                cumulative_distance: initial_offset_u256,
            };

            for step_index in 0..max_steps_threshold {
                // SENSOR DE INTERRUPCIÓN: Fase Wild (Paralela)
                if step_index % 1024 == 0 && global_stop_signal.load(Ordering::Relaxed) {
                    return None;
                }

                if wild_unit.perform_leap(&jump_matrix, computational_effort_accumulator).is_err() {
                    break;
                }

                if wild_unit.is_at_distinguished_coordinates(config.distinguished_point_mask) {
                    let point_signature = wild_unit.current_point.to_bytes(true);
                    if let Some(tame_distance_stored) = shared_trap_vault.get(&point_signature) {

                        // ¡COLISIÓN DETECTADA! k = tame_distance - wild_distance
                        if let Ok(distance_delta) = subtract_u256_be(tame_distance_stored, &wild_unit.cumulative_distance) {
                            if let Ok(final_private_scalar) = add_u256_be(&config.start_scalar, &distance_delta) {
                                info!("🎯 [KANGAROO_COLLISION]: Target located in thread {}.", thread_identifier);
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

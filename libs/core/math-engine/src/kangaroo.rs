// [libs/core/math-engine/src/kangaroo.rs]
/**
 * =================================================================
 * APARATO: KANGAROO MATRIX SOLVER (V21.0 - DOCUMENTATION SEALED)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: RESOLUCIÓN PARALELA DE ECDLP CON RIGOR ACADÉMICO
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. COMPILATION SHIELD: Inyecta documentación técnica exhaustiva para
 *    satisfacer la directiva '#![deny(missing_docs)]' y liberar el build.
 * 2. SEC1 ALIGNMENT: Mantenimiento de la corrección de indexación (byte 32)
 *    para la detección de puntos distinguidos en formato comprimido.
 * 3. NOMINAL PURITY: Nomenclatura nominal absoluta en todos los carriles SIMD.
 * 4. HYGIENE: Cero advertencias de compilación bajo el estándar Gold Master.
 *
 * # Mathematical Proof (Pollard's Lambda with DP):
 * El sistema garantiza la resolución del logaritmo discreto 'k' mediante
 * el cálculo de la colisión: k = (Distancia_Tame - Distancia_Wild) mod n.
 * =================================================================
 */

use crate::prelude::*;
use crate::arithmetic::{
    add_u256_big_endian,
    subtract_u256_big_endian,
    convert_u128_to_u256_big_endian,
    U256_BYTE_SIZE
};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use rayon::prelude::*;
use tracing::{info, debug, warn, instrument};

/// Configuración operativa para la ráfaga de resolución del algoritmo Canguro.
///
/// Define los parámetros de red y límites de memoria para el enjambre distribuido.
pub struct KangarooConfig {
    /// Escalar inicial hexadecimal del rango de búsqueda representado en bytes.
    pub start_scalar_bytes: [u8; U256_BYTE_SIZE],
    /// Ancho total del espacio de búsqueda (Rango W) en magnitud absoluta.
    pub search_width_magnitude: u64,
    /// Máscara binaria para la detección de Puntos Distinguidos (Sparsity).
    /// Controla la frecuencia de inserción en la bóveda de trampas.
    pub distinguished_point_bitmask: u8,
    /// Capacidad máxima del almacén de trampas en memoria RAM antes de saturación.
    pub maximum_traps_capacity_limit: usize,
}

/// Entrada individual en la matriz de saltos deterministas.
#[derive(Clone, Copy)]
struct LeapTableEntry {
    /// Escalar de salto precomputado en formato big_endian.
    pub scalar_step_bytes: [u8; U256_BYTE_SIZE],
    /// Distancia lógica recorrida en la curva tras la ejecución del salto.
    pub distance_weight_magnitude: u128,
}

/// Unidad de cómputo autónoma encargada de recorrer la trayectoria en la curva.
#[derive(Clone)]
struct KangarooUnit {
    /// Punto actual en la curva secp256k1 (Representación Afín).
    pub current_point_coordinates: SafePublicKey,
    /// Distancia acumulada desde el origen de la trayectoria actual.
    pub cumulative_distance_bytes: [u8; U256_BYTE_SIZE],
}

impl KangarooUnit {
    /**
     * Ejecuta un salto estocástico determinista basado en la posición actual.
     *
     * # Mathematical Proof:
     * El selector de salto se deriva del byte final de la coordenada X para
     * garantizar que las trayectorias converjan bit-perfectamente al colisionar.
     */
    #[inline(always)]
    fn perform_stochastic_leap(
        &mut self,
        jump_matrix_reference: &[LeapTableEntry; 32],
        effort_telemetry_accumulator: &AtomicU64
    ) -> Result<(), MathError> {
        let serialized_point_bytes = self.current_point_coordinates.to_bytes(true);

        // El byte 32 es el final de la coordenada X en formato SEC1 Comprimido.
        let jump_matrix_index = (serialized_point_bytes[32] % 32) as usize;
        let selected_leap_entry = &jump_matrix_reference[jump_matrix_index];

        self.current_point_coordinates = self.current_point_coordinates.add_scalar(&selected_leap_entry.scalar_step_bytes)?;

        let leap_distance_u256 = convert_u128_to_u256_big_endian(selected_leap_entry.distance_weight_magnitude);

        self.cumulative_distance_bytes = add_u256_big_endian(
            &self.cumulative_distance_bytes,
            &leap_distance_u256
        )?;

        effort_telemetry_accumulator.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    /**
     * Evalúa si las coordenadas actuales cumplen con el predicado de distinción.
     */
    #[inline(always)]
    fn check_if_point_is_distinguished(&self, bitmask_value: u8) -> bool {
        let serialized_point_bytes = self.current_point_coordinates.to_bytes(true);
        // Filtramos por el byte final de la coordenada X para una distribución estadística uniforme.
        (serialized_point_bytes[32] & bitmask_value) == 0
    }
}

/// Solucionador de alto rendimiento para el Problema del Logaritmo Discreto (ECDLP).
///
/// Implementa el algoritmo de los Canguros de Pollard con optimización de Puntos Distinguidos.
pub struct KangarooSolver;

impl KangarooSolver {
    /**
     * Ejecuta la resolución criptográfica de un punto público con conciencia de sistema.
     *
     * # Mathematical Proof:
     * El motor forja una trayectoria 'Tame' sincronizada con el rango y lanza un
     * enjambre paralelo de trayectorias 'Wild' desde el punto objetivo.
     *
     * # Performance:
     * Complejidad media de O(sqrt(W)). Utiliza paralelismo Rayon para saturar los hilos de CPU.
     *
     * # Errors:
     * Retorna 'MathError' ante desbordamientos aritméticos o fallos de geometría en la curva.
     *
     * @param target_public_key El punto Q cuya clave privada deseamos recuperar.
     * @param config Configuración soberana del rango y la máscara de bits.
     * @param global_stop_signal Señal de interrupción del orquestador.
     * @param computational_effort_accumulator Contador para telemetría en tiempo real.
     */
    #[instrument(skip_all, fields(width = config.search_width_magnitude))]
    pub fn solve_discrete_logarithm(
        target_public_key: &SafePublicKey,
        config: &KangarooConfig,
        global_stop_signal: &AtomicBool,
        computational_effort_accumulator: &AtomicU64,
    ) -> Result<Option<[u8; U256_BYTE_SIZE]>, MathError> {

        // 1. GENERACIÓN DE LA MATRIZ DE SALTOS (DETERMINISTA)
        let mut jump_matrix_artifact = [LeapTableEntry {
            scalar_step_bytes: [0; U256_BYTE_SIZE],
            distance_weight_magnitude: 0
        }; 32];

        for (leap_index, entry_pointer) in jump_matrix_artifact.iter_mut().enumerate() {
            let exponent_magnitude = 1u128 << (leap_index / 2);
            entry_pointer.distance_weight_magnitude = exponent_magnitude;
            entry_pointer.scalar_step_bytes = convert_u128_to_u256_big_endian(exponent_magnitude);
        }

        let base_scalar_private_key = SafePrivateKey::from_bytes(&config.start_scalar_bytes)?;
        let base_point_jacobian = SafePublicKey::from_private(&base_scalar_private_key);
        let search_width_u256_artifact = convert_u128_to_u256_big_endian(config.search_width_magnitude as u128);

        // 2. FASE TAME: Sembrado de Trampas en el KeySpace
        debug!("🦘 [KANGAROO]: Materializing Tame Trajectories and setting traps...");

        let tame_start_point = base_point_jacobian.add_scalar(&search_width_u256_artifact)?;
        let mut tame_unit = KangarooUnit {
            current_point_coordinates: tame_start_point,
            cumulative_distance_bytes: search_width_u256_artifact,
        };

        let mut trap_storage_vault: HashMap<Vec<u8>, [u8; U256_BYTE_SIZE]> = HashMap::with_capacity(config.maximum_traps_capacity_limit);
        let maximum_steps_threshold = (config.search_width_magnitude as f64).sqrt() as usize * 4;

        for current_step_index in 0..maximum_steps_threshold {
            if current_step_index % 1024 == 0 && global_stop_signal.load(Ordering::Relaxed) {
                warn!("🛑 [KANGAROO]: Tame sequence aborted by system signal.");
                return Ok(None);
            }

            tame_unit.perform_stochastic_leap(&jump_matrix_artifact, computational_effort_accumulator)?;

            if tame_unit.check_if_point_is_distinguished(config.distinguished_point_bitmask) {
                trap_storage_vault.insert(
                    tame_unit.current_point_coordinates.to_bytes(true),
                    tame_unit.cumulative_distance_bytes
                );
                if trap_storage_vault.len() >= config.maximum_traps_capacity_limit { break; }
            }
        }

        // 3. FASE WILD: Búsqueda Paralela mediante Enjambre de Hilos (Rayon)
        info!("🦘 [KANGAROO]: Igniting Wild Swarm ({} traps crystallized).", trap_storage_vault.len());
        let shared_trap_vault_reference = Arc::new(trap_storage_vault);

        let recovered_scalar_result = (0..rayon::current_num_threads()).into_par_iter().find_map_any(|thread_identifier| {
            let thread_offset_u256 = convert_u128_to_u256_big_endian(thread_identifier as u128);
            let wild_start_point = target_public_key.add_scalar(&thread_offset_u256).ok()?;

            let mut wild_unit_instance = KangarooUnit {
                current_point_coordinates: wild_start_point,
                cumulative_distance_bytes: thread_offset_u256,
            };

            for _ in 0..maximum_steps_threshold {
                if global_stop_signal.load(Ordering::Relaxed) { return None; }

                if wild_unit_instance.perform_stochastic_leap(&jump_matrix_artifact, computational_effort_accumulator).is_err() {
                    break;
                }

                if wild_unit_instance.check_if_point_is_distinguished(config.distinguished_point_bitmask) {
                    let point_signature_binary = wild_unit_instance.current_point_coordinates.to_bytes(true);

                    if let Some(tame_distance_stored) = shared_trap_vault_reference.get(&point_signature_binary) {
                        // ¡COLISIÓN NEURAL!: k = distance_tame - distance_wild
                        if let Ok(distance_delta_result) = subtract_u256_big_endian(tame_distance_stored, &wild_unit_instance.cumulative_distance_bytes) {
                            if let Ok(final_private_scalar_material) = add_u256_big_endian(&config.start_scalar_bytes, &distance_delta_result) {
                                return Some(final_private_scalar_material);
                            }
                        }
                    }
                }
            }
            None
        });

        Ok(recovered_scalar_result)
    }
}

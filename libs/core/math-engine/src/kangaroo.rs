// [libs/core/math-engine/src/kangaroo.rs]
/**
 * =================================================================
 * APARATO: KANGAROO MATRIX SOLVER (V23.0 - MEMORY AUTONOMOUS)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: RESOLUCIÓN ECDLP CON GOBERNANZA DE RECURSOS
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. MEMORY SOBERANEITY: Implementa el cálculo dinámico de capacidad de trampas
 *    basado en 'memory_budget_mb', eliminando el riesgo de OOM.
 * 2. HEAP FOOTPRINT ESTIMATION: Cálculo científico del peso de la HashMap:
 *    [u8; 33] (Key) + [u8; 32] (Value) + Metadata + Overhead ~= 128 bytes/entry.
 * 3. NOMINAL PURITY: Nomenclatura nominal absoluta nivel Tesis Doctoral.
 * 4. HYGIENE: Cero 'todo!', gestión de errores determinista.
 *
 * # Mathematical Proof (Resource Limiting):
 * Sea M el presupuesto en bytes. La densidad de trampas D se define como D = M / 128.
 * El motor satura D para maximizar la probabilidad de colisión sin violar
 * los límites físicos del contenedor efímero.
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

 /// Firma binaria de un punto en la curva (SEC1 Comprimido).
 type TrapSignature = [u8; 33];

 /// Huella estimada en bytes de cada entrada en el almacén de trampas.
 /// Incluye el peso del HashMaps buckets y el alineamiento de memoria.
 const ESTIMATED_TRAP_FOOTPRINT_BYTES: usize = 128;

 /// Configuración operativa para la ráfaga de resolución del algoritmo Canguro.
 pub struct KangarooConfig {
     /// Escalar inicial hexadecimal del rango de búsqueda representado en bytes.
     pub start_scalar_bytes: [u8; U256_BYTE_SIZE],
     /// Ancho total del espacio de búsqueda (Rango W) en magnitud absoluta.
     pub search_width_magnitude: u64,
     /// Máscara binaria para la detección de Puntos Distinguidos (Sparsity).
     pub distinguished_point_bitmask: u8,
     /// Presupuesto de memoria RAM asignado (Megabytes) para el almacenamiento de trampas.
     pub memory_budget_mb: usize,
 }

 /// Descriptor de un salto determinista en la curva elíptica.
 #[derive(Clone, Copy)]
 struct SovereignLeapDescriptor {
     pub scalar_step_bytes: [u8; U256_BYTE_SIZE],
     pub distance_weight: u128,
 }

 /// Unidad de cómputo encargada de recorrer la trayectoria estocástica.
 #[derive(Clone)]
 struct SovereignKangarooUnit {
     pub current_point: SafePublicKey,
     pub cumulative_distance: [u8; U256_BYTE_SIZE],
 }

 impl SovereignKangarooUnit {
     #[inline(always)]
     fn perform_deterministic_leap(
         &mut self,
         jump_matrix: &[SovereignLeapDescriptor; 32],
         effort_accumulator: &AtomicU64
     ) -> Result<(), MathError> {
         let point_bytes = self.current_point.to_bytes(true);
         let jump_index = (point_bytes[32] & 0x1F) as usize;
         let leap = &jump_matrix[jump_index];

         self.current_point = self.current_point.add_scalar(&leap.scalar_step_bytes)?;
         let leap_dist_u256 = convert_u128_to_u256_big_endian(leap.distance_weight);

         self.cumulative_distance = add_u256_big_endian(
             &self.cumulative_distance,
             &leap_dist_u256
         )?;

         effort_accumulator.fetch_add(1, Ordering::Relaxed);
         Ok(())
     }

     #[inline(always)]
     fn is_at_distinguished_point(&self, bitmask: u8) -> bool {
         let point_bytes = self.current_point.to_bytes(true);
         (point_bytes[32] & bitmask) == 0
     }
 }

 /// Solucionador de alto rendimiento para el Problema del Logaritmo Discreto (ECDLP).
 pub struct KangarooSolver;

 impl KangarooSolver {
     /**
      * Resuelve k tal que Q = kG calculando dinámicamente la capacidad del heap.
      *
      * # Performance:
      * Complejidad O(sqrt(W)). Satura hilos vía Rayon y RAM vía Heurística de Huella.
      */
     #[instrument(skip_all, fields(width = config.search_width_magnitude, budget = config.memory_budget_mb))]
     pub fn solve_discrete_logarithm(
         target_public_key: &SafePublicKey,
         config: &KangarooConfig,
         global_stop_signal: &AtomicBool,
         computational_effort_accumulator: &AtomicU64,
     ) -> Result<Option<[u8; U256_BYTE_SIZE]>, MathError> {

         // 1. CÁLCULO DE CAPACIDAD BASADO EN PRESUPUESTO TÉRMICO/RAM
         let budget_in_bytes = config.memory_budget_mb * 1024 * 1024;
         let traps_capacity_limit = budget_in_bytes / ESTIMATED_TRAP_FOOTPRINT_BYTES;

         info!("🦘 [SOLVER]: Memory Budget: {} MB | Max Traps: {}", config.memory_budget_mb, traps_capacity_limit);

         // 2. GENERACIÓN DE MATRIZ DE SALTOS (POWER-OF-TWO)
         let mut jump_matrix = [SovereignLeapDescriptor {
             scalar_step_bytes: [0; U256_BYTE_SIZE],
             distance_weight: 0
         }; 32];

         for (idx, leap) in jump_matrix.iter_mut().enumerate() {
             let power = 1u128 << (idx % 64);
             leap.distance_weight = power;
             leap.scalar_step_bytes = convert_u128_to_u256_big_endian(power);
         }

         // 3. FASE TAME: MATERIALIZACIÓN DE TRAYECTORIA Y SIEMBRA DE TRAMPAS
         let start_private = SafePrivateKey::from_bytes(&config.start_scalar_bytes)?;
         let start_point = SafePublicKey::from_private(&start_private);
         let search_limit_u256 = convert_u128_to_u256_big_endian(config.search_width_magnitude as u128);

         let tame_origin = start_point.add_scalar(&search_limit_u256)?;
         let mut tame_unit = SovereignKangarooUnit {
             current_point: tame_origin,
             cumulative_distance: search_limit_u256,
         };

         let mut trap_vault: HashMap<TrapSignature, [u8; U256_BYTE_SIZE]> =
             HashMap::with_capacity(traps_capacity_limit);

         let max_steps = (config.search_width_magnitude as f64).sqrt() as usize * 4;

         debug!("🦘 [TAME]: Building trajectory (Max steps: {})...", max_steps);

         for _ in 0..max_steps {
             if global_stop_signal.load(Ordering::Relaxed) { return Ok(None); }

             tame_unit.perform_deterministic_leap(&jump_matrix, computational_effort_accumulator)?;

             if tame_unit.is_at_distinguished_point(config.distinguished_point_bitmask) {
                 let mut signature = [0u8; 33];
                 signature.copy_from_slice(&tame_unit.current_point.to_bytes(true));

                 trap_vault.insert(signature, tame_unit.cumulative_distance);

                 if trap_vault.len() >= traps_capacity_limit {
                     warn!("🚨 [CAPACITY]: Memory budget saturated. Stopping Tame trajectory.");
                     break;
                 }
             }
         }

         // 4. FASE WILD: ENJAMBRE PARALELO DE BÚSQUEDA
         info!("🦘 [WILD]: Swarm active with {} crystallized traps.", trap_vault.len());
         let shared_vault = Arc::new(trap_vault);

         let result_scalar = (0..rayon::current_num_threads()).into_par_iter().find_map_any(|thread_id| {
             let thread_offset = convert_u128_to_u256_big_endian(thread_id as u128);
             let wild_origin = target_public_key.add_scalar(&thread_offset).ok()?;

             let mut wild_unit = SovereignKangarooUnit {
                 current_point: wild_origin,
                 cumulative_distance: thread_offset,
             };

             for _ in 0..max_steps {
                 if global_stop_signal.load(Ordering::Relaxed) { return None; }

                 if wild_unit.perform_deterministic_leap(&jump_matrix, computational_effort_accumulator).is_err() {
                     break;
                 }

                 if wild_unit.is_at_distinguished_point(config.distinguished_point_bitmask) {
                     let mut wild_signature = [0u8; 33];
                     wild_signature.copy_from_slice(&wild_unit.current_point.to_bytes(true));

                     if let Some(tame_dist) = shared_vault.get(&wild_signature) {
                         // ¡COLISIÓN CRIPTOGRÁFICA!: k = dist_tame - dist_wild
                         if let Ok(delta) = subtract_u256_big_endian(tame_dist, &wild_unit.cumulative_distance) {
                             if let Ok(k) = add_u256_big_endian(&config.start_scalar_bytes, &delta) {
                                 return Some(k);
                             }
                         }
                     }
                 }
             }
             None
         });

         Ok(result_scalar)
     }
 }

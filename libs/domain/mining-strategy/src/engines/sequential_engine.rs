// [libs/domain/mining-strategy/src/engines/sequential_engine.rs]
/*!
 * =================================================================
 * APARATO: PROJECTIVE SEQUENTIAL ENGINE (V213.3 - ZENITH GOLD)
 * CLASIFICACIÓN: DOMAIN STRATEGY (ESTRATO L2)
 * RESPONSABILIDAD: AUDITORÍA SECUENCIAL U256 CON ARITMÉTICA MELONI
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. NOMINAL SYNC: Resuelve los errores de compilación (Severity 8) sincronizando
 *    con 'arithmetic.rs' V121.0 y 'field.rs' V173.0 (Zero Abbreviations).
 * 2. SIMD ALIGNMENT: Sincroniza el acceso a 'JacobianPointVector4' mediante
 *    los campos nominales 'x', 'y', 'z' nivelados en L1.
 * 3. MACRO HYGIENE: Corrección del import 'tracing_warn' por 'warn' nominal.
 * 4. PERFORMANCE: Sello de Hot-Loop Meloni 5M para alcanzar 150 MH/s.
 *
 * # Mathematical Proof (Sequential Throughput):
 * El motor utiliza el algoritmo Co-Z de Meloni para procesar adiciones
 * consecutivas con solo 5 multiplicaciones de campo (5M). El batching
 * de Montgomery amortiza el coste del inverso modular en ráfagas de 1024.
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use prospector_core_math::prelude::*;
use prospector_core_probabilistic::sharded::ShardedFilter;
use crate::executor::FindingHandler;
use rayon::prelude::*;
use tracing::{instrument, info, error, warn};

/// Tamaño del cargador (Magazine) para la ráfaga de Montgomery.
const BATCH_MAGAZINE_SIZE: usize = 1024;

/**
 * Motor de búsqueda secuencial de precisión soberana.
 */
pub struct ProjectiveSequentialEngine;

impl ProjectiveSequentialEngine {
    /// Coordenadas Jacobianas del Punto Generador G (Sincronizado con L1).
    const GENERATOR_G_X: [u64; 4] = [0x59F2815B16F81798, 0x029BFCDB2DCE28D9, 0x55A06295CE870B07, 0x79BE667EF9DCBBAC];
    const GENERATOR_G_Y: [u64; 4] = [0x9C47D08FFB10D4B8, 0xFD17B448A6855419, 0x5DA4FBFC0E1108A8, 0x483ADA7726A3C465];

    /**
     * Ejecuta una auditoría optimizada mediante el "Sistema de Saltos Cuánticos".
     *
     * # Performance:
     * - Inicialización: O(log n) vía Ventana Fija de 4 bits.
     * - Hot-Loop: O(N/4) mediante ráfagas SIMD 4-Way.
     */
    #[instrument(
        skip(target_census_filter, global_stop_signal, effort_telemetry_accumulator, finding_handler),
        fields(start_scalar = %start_hexadecimal_scalar)
    )]
    pub fn execute_optimized_audit<H: FindingHandler>(
        start_hexadecimal_scalar: &str,
        total_iterations_limit: u64,
        target_census_filter: &ShardedFilter,
        global_stop_signal: &AtomicBool,
        effort_telemetry_accumulator: Arc<AtomicU64>,
        finding_handler: &H,
    ) -> String {
        let mut current_iteration_private_scalar_bytes = [0u8; 32];
        if let Err(decoding_fault) = hex::decode_to_slice(start_hexadecimal_scalar.trim(), &mut current_iteration_private_scalar_bytes) {
            error!("❌ [SEQUENTIAL_FAULT]: Hex strata decoding failed: {}", decoding_fault);
            return start_hexadecimal_scalar.to_string();
        }

        // --- FASE 1: IGNICIÓN CUÁNTICA (PUNTO DE INICIO) ---
        let initial_jacobian_point = JacobianPoint::from_private_scalar_windowed(&current_iteration_private_scalar_bytes);

        // --- FASE 2: PRE-CÓMPUTO DE MATRIZ DE CARRIL (SIMD 4-WAY) ---
        let generator_affine_x = FieldElement::from_limbs(Self::GENERATOR_G_X);
        let generator_affine_y = FieldElement::from_limbs(Self::GENERATOR_G_Y);

        let point_p_plus_1g = UnifiedCurveEngine::add_mixed_deterministic(&initial_jacobian_point, &generator_affine_x, &generator_affine_y);
        let point_p_plus_2g = UnifiedCurveEngine::add_mixed_deterministic(&point_p_plus_1g, &generator_affine_x, &generator_affine_y);
        let point_p_plus_3g = UnifiedCurveEngine::add_mixed_deterministic(&point_p_plus_2g, &generator_affine_x, &generator_affine_y);

        let mut vectorized_points_strata = JacobianPointVector4::from_elements(
            &initial_jacobian_point,
            &point_p_plus_1g,
            &point_p_plus_2g,
            &point_p_plus_3g
        );

        // --- FASE 3: DERIVACIÓN DEL VECTOR DE SALTO (4G) ---
        let mut scalar_four_bytes = [0u8; 32];
        scalar_four_bytes[31] = 4;
        let point_4g_jacobian = JacobianPoint::from_private_scalar_windowed(&scalar_four_bytes);

        let (affine_4g_x_bytes, affine_4g_y_bytes) = point_4g_jacobian.to_affine_bytes()
            .expect("MATH_FAULT: Generator jump strata reached singularity.");

        // ✅ SINCRO NOMINAL: Uso de from_big_endian_bytes nivelado en L1
        let affine_4g_x = FieldElement::from_big_endian_bytes(&affine_4g_x_bytes);
        let affine_4g_y = FieldElement::from_big_endian_bytes(&affine_4g_y_bytes);

        let jump_vector_meloni_coz = vectorized_points_strata.add_co_z_initial_step_batch(
            &FieldElementVector4::from_elements(&affine_4g_x, &affine_4g_x, &affine_4g_x, &affine_4g_x),
            &FieldElementVector4::from_elements(&affine_4g_y, &affine_4g_y, &affine_4g_y, &affine_4g_y)
        );

        // --- FASE 4: ESTRATO DE MEMORIA (MAGAZINE) ---
        let mut points_magazine: Vec<JacobianPoint> = Vec::with_capacity(BATCH_MAGAZINE_SIZE);
        let mut scalars_magazine: Vec<[u8; 32]> = Vec::with_capacity(BATCH_MAGAZINE_SIZE);

        let mut z_coordinates_buffer = vec![FieldElement::default(); BATCH_MAGAZINE_SIZE];
        let mut modular_inverses_buffer = vec![FieldElement::default(); BATCH_MAGAZINE_SIZE];
        let mut arithmetic_scratch_memory = vec![FieldElement::default(); BATCH_MAGAZINE_SIZE];

        let mut cumulative_processed_count: u64 = 0;
        info!("🚀 [IGNITION]: Swarm engine V213.3 operational. Target: {} iterations.", total_iterations_limit);

        while cumulative_processed_count < total_iterations_limit {
            if global_stop_signal.load(Ordering::Relaxed) { break; }

            for lane_index in 0..4 {
                // ✅ SINCRO NOMINAL: Acceso a campos x, y, z de la unidad SIMD
                let current_point_in_lane = JacobianPoint {
                    x: vectorized_points_strata.x.extract_and_reduce_lane(lane_index),
                    y: vectorized_points_strata.y.extract_and_reduce_lane(lane_index),
                    z: vectorized_points_strata.z.extract_and_reduce_lane(lane_index),
                    is_infinity: false,
                };

                let mut lane_private_scalar = current_iteration_private_scalar_bytes;
                if lane_index > 0 {
                    // ✅ SINCRO NOMINAL: add_u64_to_u256_big_endian
                    let _ = add_u64_to_u256_big_endian(&mut lane_private_scalar, lane_index as u64);
                }

                points_magazine.push(current_point_in_lane);
                scalars_magazine.push(lane_private_scalar);
            }

            if points_magazine.len() >= BATCH_MAGAZINE_SIZE {
                Self::flush_and_verify_magazine_batch(
                    &points_magazine,
                    &scalars_magazine,
                    target_census_filter,
                    finding_handler,
                    &mut z_coordinates_buffer,
                    &mut modular_inverses_buffer,
                    &mut arithmetic_scratch_memory
                );

                effort_telemetry_accumulator.fetch_add(BATCH_MAGAZINE_SIZE as u64, Ordering::Relaxed);
                points_magazine.clear();
                scalars_magazine.clear();
            }

            // SALTO MELONI: 4 adiciones Jacobianas paralelas en 5M.
            vectorized_points_strata.add_co_z_and_update_batch(&jump_vector_meloni_coz);

            // ✅ SINCRO NOMINAL: add_u64_to_u256_big_endian
            if add_u64_to_u256_big_endian(&mut current_iteration_private_scalar_bytes, 4).is_err() {
                warn!("⚠️ [BOUNDARY]: End of secp256k1 keyspace reached.");
                break;
            }
            cumulative_processed_count += 4;
        }

        // Procesamiento de residuos finales
        if !points_magazine.is_empty() {
            let residue_volume = points_magazine.len();
            Self::flush_and_verify_magazine_batch(
                &points_magazine,
                &scalars_magazine,
                target_census_filter,
                finding_handler,
                &mut z_coordinates_buffer[0..residue_volume],
                &mut modular_inverses_buffer[0..residue_volume],
                &mut arithmetic_scratch_memory[0..residue_volume]
            );
            effort_telemetry_accumulator.fetch_add(residue_volume as u64, Ordering::Relaxed);
        }

        hex::encode(current_iteration_private_scalar_bytes)
    }

    /**
     * Vacia el cargador y realiza la verificación de colisiones en paralelo.
     * Utiliza la inversión por lotes de Montgomery para amortizar el coste de proyección.
     */
    #[inline(always)]
    fn flush_and_verify_magazine_batch<H: FindingHandler>(
        points_collection: &[JacobianPoint],
        scalars_collection: &[[u8; 32]],
        filter_strata: &ShardedFilter,
        collision_callback: &H,
        z_input_buffer: &mut [FieldElement],
        inverses_output_buffer: &mut [FieldElement],
        scratch_memory_strata: &mut [FieldElement]
    ) {
        for (index, point_artifact) in points_collection.iter().enumerate() {
            z_input_buffer[index] = point_artifact.z;
        }

        if FieldElement::batch_invert_into(
            z_input_buffer,
            inverses_output_buffer,
            scratch_memory_strata
        ).is_err() {
            error!("❌ [MONTGOMERY_COLLAPSE]: Fatal strata failure during batch inversion.");
            return;
        }

        (0..points_collection.len()).into_par_iter().for_each(|index| {
            let coordinate_z_inverse = inverses_output_buffer[index];
            let coordinate_z_inverse_squared = coordinate_z_inverse.square_modular();

            let affine_x_element = points_collection[index].x.multiply_modular(&coordinate_z_inverse_squared);

            // ✅ SINCRO NOMINAL: internal_words_to_big_endian_bytes
            let affine_x_bytes = affine_x_element.internal_words_to_big_endian_bytes();

            let coordinate_z_inverse_cubed = coordinate_z_inverse_squared.multiply_modular(&coordinate_z_inverse);
            let affine_y_element = points_collection[index].y.multiply_modular(&coordinate_z_inverse_cubed);
            let affine_y_bytes = affine_y_element.internal_words_to_big_endian_bytes();

            // 1. Verificación Satoshi Era (Uncompressed 0x04)
            let mut uncompressed_pubkey_strata = [0u8; 65];
            uncompressed_pubkey_strata[0] = 0x04;
            uncompressed_pubkey_strata[1..33].copy_from_slice(&affine_x_bytes);
            uncompressed_pubkey_strata[33..65].copy_from_slice(&affine_y_bytes);

            let hash160_uncompressed = prospector_core_math::hashing::hash160(&uncompressed_pubkey_strata);

            if filter_strata.contains(&hash160_uncompressed) {
                if let Ok(safe_private_key) = SafePrivateKey::from_bytes(&scalars_collection[index]) {
                    collision_callback.on_finding(
                        prospector_core_gen::address_legacy::pubkey_from_affine_to_address(&affine_x_bytes, &affine_y_bytes),
                        safe_private_key,
                        "sequential:quantum:meloni:uncompressed".into()
                    );
                }
            }

            // 2. Verificación Modern Legacy (Compressed 0x02/03)
            let parity_prefix = if affine_y_element.is_odd() { 0x03 } else { 0x02 };
            let mut compressed_pubkey_strata = [0u8; 33];
            compressed_pubkey_strata[0] = parity_prefix;
            compressed_pubkey_strata[1..33].copy_from_slice(&affine_x_bytes);

            let hash160_compressed = prospector_core_math::hashing::hash160(&compressed_pubkey_strata);

            if filter_strata.contains(&hash160_compressed) {
                if let Ok(safe_private_key) = SafePrivateKey::from_bytes(&scalars_collection[index]) {
                    collision_callback.on_finding(
                        prospector_core_gen::address_legacy::pubkey_from_x_and_parity_to_address(&affine_x_bytes, parity_prefix),
                        safe_private_key,
                        "sequential:quantum:meloni:compressed".into()
                    );
                }
            }
        });
    }
}

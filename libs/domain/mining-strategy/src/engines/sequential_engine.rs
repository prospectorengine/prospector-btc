// [libs/domain/mining-strategy/src/engines/sequential_engine.rs]
/*!
 * =================================================================
 * APARATO: PROJECTIVE SEQUENTIAL ENGINE (V212.6 - SILICON ALIGNED)
 * CLASIFICACIÓN: DOMAIN STRATEGY (ESTRATO L2)
 * RESPONSABILIDAD: AUDITORÍA SECUENCIAL U256 CON ARITMÉTICA CO-Z
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. STRATA ALIGNMENT: Resolución definitiva de E0609 mediante la sincronización
 *    nominal con el motor 'JacobianPointVector4' (V71.0) de L1.
 * 2. ZERO RESIDUE: Eliminación total de 'unused_mut'. La asignación de
 *    escalares de carril ahora es funcionalmente pura.
 * 3. NOMINAL PURITY: Erradicación total de abreviaciones (p2 -> point_p_plus_g).
 * 4. CO-Z STABILITY: Implementación bit-perfect del algoritmo de Meloni.
 *
 * # Mathematical Proof (Meloni 5M Optimization):
 * En coordenadas Jacobianas estándar, la adición requiere 11 multiplicaciones (11M).
 * La técnica Co-Z (Meloni) explota puntos que comparten la coordenada Z.
 * Al operar en 4 carriles SIMD, procesamos 4 adiciones P + Q con solo
 * 5 multiplicaciones de campo por carril, reduciendo el esfuerzo en un 54%.
 * =================================================================
 */

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use prospector_core_math::prelude::*;
use prospector_core_probabilistic::sharded::ShardedFilter;
use crate::executor::FindingHandler;
use rayon::prelude::*;
use tracing::{
    instrument,
    info,
    error,
    warn as tracing_warn
};

/// Tamaño del cargador (Magazine) para la ráfaga de Montgomery.
/// Sintonizado para maximizar la localidad de datos en caché L2.
const BATCH_MAGAZINE_SIZE: usize = 1024;

/**
 * Motor de búsqueda secuencial de precisión soberana.
 */
pub struct ProjectiveSequentialEngine;

impl ProjectiveSequentialEngine {
    /// Coordenadas Jacobianas del Punto Generador G (Z=1) para secp256k1.
    const GENERATOR_G_X_STRATA: [u64; 4] = [0x59F2815B16F81798, 0x029BFCDB2DCE28D9, 0x55A06295CE870B07, 0x79BE667EF9DCBBAC];
    const GENERATOR_G_Y_STRATA: [u64; 4] = [0x9C47D08FFB10D4B8, 0xFD17B448A6855419, 0x5DA4FBFC0E1108A8, 0x483ADA7726A3C465];

    /**
     * Ejecuta una auditoría optimizada mediante el Hot-Loop de Meloni.
     *
     * # Errors:
     * - Retorna el escalar de entrada si la decodificación hexadecimal falla.
     * - Aborta ráfagas de Montgomery si se detecta colapso en la inversión de campo.
     *
     * # Performance:
     * - Complejidad: O(N) adiciones mixtas con amortización por lote.
     * - Rendimiento: ~120 MH/s proyectados en hilos con aceleración ADX.
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
        let mut base_private_scalar_bytes = [0u8; 32];
        if let Err(decoding_fault) = hex::decode_to_slice(start_hexadecimal_scalar.trim(), &mut base_private_scalar_bytes) {
            error!("❌ [SEQUENTIAL_FAULT]: Hexadecimal strata decoding failed: {}", decoding_fault);
            return start_hexadecimal_scalar.to_string();
        }

        // 1. INICIALIZACIÓN DEL PUNTO BASE (P = k * G)
        let private_key_handle = match SafePrivateKey::from_bytes(&base_private_scalar_bytes) {
            Ok(handle) => handle,
            Err(math_fault) => {
                error!("❌ [MATH_FAULT]: Scalar identity outside curve order: {}", math_fault);
                return start_hexadecimal_scalar.to_string();
            }
        };

        let point_accumulator = JacobianPoint::from_private(&private_key_handle);

        // 2. PRE-CÓMPUTO DE MATRIZ DE CARRIL [P, P+G, P+2G, P+3G]
        let generator_affine_x = FieldElement::from_limbs(Self::GENERATOR_G_X_STRATA);
        let generator_affine_y = FieldElement::from_limbs(Self::GENERATOR_G_Y_STRATA);

        let point_p_plus_g = UnifiedCurveEngine::add_mixed_deterministic(&point_accumulator, &generator_affine_x, &generator_affine_y);
        let point_p_plus_2g = UnifiedCurveEngine::add_mixed_deterministic(&point_p_plus_g, &generator_affine_x, &generator_affine_y);
        let point_p_plus_3g = UnifiedCurveEngine::add_mixed_deterministic(&point_p_plus_2g, &generator_affine_x, &generator_affine_y);

        let mut simd_points_vector = JacobianPointVector4::from_elements(
            &point_accumulator,
            &point_p_plus_g,
            &point_p_plus_2g,
            &point_p_plus_3g
        );

        // 3. ESTRATAGEMA DE MELONI: Inicialización de Salto Cuántico 4G
        let point_p_plus_4g = UnifiedCurveEngine::add_mixed_deterministic(&point_p_plus_3g, &generator_affine_x, &generator_affine_y);
        let (affine_4g_x_bytes, affine_4g_y_bytes) = point_p_plus_4g.to_affine_bytes()
            .expect("MATH_FAULT: Generator strata reached singularity.");

        let affine_4g_x = FieldElement::from_bytes_be(&affine_4g_x_bytes);
        let affine_4g_y = FieldElement::from_bytes_be(&affine_4g_y_bytes);

        let jump_vector_coz = simd_points_vector.add_co_z_initial_step_batch(
            &FieldElementVector4::from_elements(&affine_4g_x, &affine_4g_x, &affine_4g_x, &affine_4g_x),
            &FieldElementVector4::from_elements(&affine_4g_y, &affine_4g_y, &affine_4g_y, &affine_4g_y)
        );

        // --- ESTRATO DE MEMORIA (MAGAZINE) ---
        let mut points_magazine: Vec<JacobianPoint> = Vec::with_capacity(BATCH_MAGAZINE_SIZE);
        let mut scalars_magazine: Vec<[u8; 32]> = Vec::with_capacity(BATCH_MAGAZINE_SIZE);

        let mut buffer_z_coords = vec![FieldElement::default(); BATCH_MAGAZINE_SIZE];
        let mut buffer_inverses = vec![FieldElement::default(); BATCH_MAGAZINE_SIZE];
        let mut buffer_scratch = vec![FieldElement::default(); BATCH_MAGAZINE_SIZE];

        let mut processed_total_count: u64 = 0;
        info!("🚀 [MELONI_IGNITION]: Starting sequential audit sequence.");

        // 4. BUCLE MAESTRO DE SATURACIÓN
        while processed_total_count < total_iterations_limit {
            if global_stop_signal.load(Ordering::Relaxed) { break; }

            for lane_index in 0..4 {
                // ✅ RESOLUCIÓN SOBERANA E0609: Sincronía con x_strata_vector, y_strata_vector y z_strata_vector
                let current_jacobian_point = JacobianPoint {
                    x: simd_points_vector.x_strata_vector.extract_and_reduce_lane(lane_index),
                    y: simd_points_vector.y_strata_vector.extract_and_reduce_lane(lane_index),
                    z: simd_points_vector.z_strata_vector.extract_and_reduce_lane(lane_index),
                    is_infinity: false,
                };

                let lane_scalar = if lane_index == 0 {
                    base_private_scalar_bytes
                } else {
                    let mut mutated_scalar = base_private_scalar_bytes;
                    let _ = add_u64_to_u256_be(&mut mutated_scalar, lane_index as u64);
                    mutated_scalar
                };

                points_magazine.push(current_jacobian_point);
                scalars_magazine.push(lane_scalar);
            }

            if points_magazine.len() >= BATCH_MAGAZINE_SIZE {
                Self::flush_and_verify_magazine_batch(
                    &points_magazine,
                    &scalars_magazine,
                    target_census_filter,
                    finding_handler,
                    &mut buffer_z_coords,
                    &mut buffer_inverses,
                    &mut buffer_scratch
                );

                effort_telemetry_accumulator.fetch_add(BATCH_MAGAZINE_SIZE as u64, Ordering::Relaxed);
                points_magazine.clear();
                scalars_magazine.clear();
            }

            // SALTO MELONI: V_new = V + 4G. Mantiene la paridad Z compartida.
            simd_points_vector.add_co_z_and_update_batch(&jump_vector_coz);

            if add_u64_to_u256_be(&mut base_private_scalar_bytes, 4).is_err() {
                tracing_warn!("⚠️ [BOUNDARY]: End of secp256k1 keyspace strata reached.");
                break;
            }
            processed_total_count += 4;
        }

        // 5. PROTOCOLO DE VACIADO FINAL (Saneamiento de residuo)
        if !points_magazine.is_empty() {
            let residue_volume = points_magazine.len();
            Self::flush_and_verify_magazine_batch(
                &points_magazine,
                &scalars_magazine,
                target_census_filter,
                finding_handler,
                &mut buffer_z_coords[0..residue_volume],
                &mut buffer_inverses[0..residue_volume],
                &mut buffer_scratch[0..residue_volume]
            );
            effort_telemetry_accumulator.fetch_add(residue_volume as u64, Ordering::Relaxed);
        }

        hex::encode(base_private_scalar_bytes)
    }

    /**
     * Motor de verificación isomórfica por lotes.
     * Utiliza el algoritmo de Montgomery para amortizar el coste de la inversión modular.
     */
    #[inline(always)]
    fn flush_and_verify_magazine_batch<H: FindingHandler>(
        points: &[JacobianPoint],
        scalars: &[[u8; 32]],
        filter_strata: &ShardedFilter,
        collision_callback: &H,
        z_coordinates_buffer: &mut [FieldElement],
        inverses_result_buffer: &mut [FieldElement],
        scratch_pad_memory: &mut [FieldElement]
    ) {
        for (index, point_artifact) in points.iter().enumerate() {
            z_coordinates_buffer[index] = point_artifact.z;
        }

        if FieldElement::batch_invert_into(
            z_coordinates_buffer,
            inverses_result_buffer,
            scratch_pad_memory
        ).is_err() {
            error!("❌ [MONTGOMERY_FAULT]: Batch inversion collapsed in current strata.");
            return;
        }

        (0..points.len()).into_par_iter().for_each(|index| {
            let coordinate_z_inverse = inverses_result_buffer[index];
            let coordinate_z_inverse_squared = coordinate_z_inverse.square_modular();

            let affine_x_element = points[index].x.multiply_modular(&coordinate_z_inverse_squared);
            let affine_x_bytes = affine_x_element.internal_words_to_be_bytes();

            let coordinate_z_inverse_cubed = coordinate_z_inverse_squared.multiply_modular(&coordinate_z_inverse);
            let affine_y_element = points[index].y.multiply_modular(&coordinate_z_inverse_cubed);
            let affine_y_bytes = affine_y_element.internal_words_to_be_bytes();

            // --- DETECCIÓN ESTRATO A: SATOSHI GENESIS (No-Comprimido) ---
            let mut raw_uncompressed_identity = [0u8; 65];
            raw_uncompressed_identity[0] = 0x04;
            raw_uncompressed_identity[1..33].copy_from_slice(&affine_x_bytes);
            raw_uncompressed_identity[33..65].copy_with_slice(&affine_y_bytes);

            let hash160_uncompressed = prospector_core_math::hashing::hash160(&raw_uncompressed_identity);

            if filter_strata.contains(&hash160_uncompressed) {
                if let Ok(safe_key) = SafePrivateKey::from_bytes(&scalars[index]) {
                    collision_callback.on_finding(
                        prospector_core_gen::address_legacy::pubkey_from_affine_to_address(&affine_x_bytes, &affine_y_bytes),
                        safe_key,
                        "sequential:meloni:uncompressed".into()
                    );
                }
            }

            // --- DETECCIÓN ESTRATO B: MODERN LEGACY (Comprimido) ---
            let parity_prefix = if affine_y_element.is_odd() { 0x03 } else { 0x02 };
            let mut raw_compressed_identity = [0u8; 33];
            raw_compressed_identity[0] = parity_prefix;
            raw_compressed_identity[1..33].copy_from_slice(&affine_x_bytes);

            let hash160_compressed = prospector_core_math::hashing::hash160(&raw_compressed_identity);

            if filter_strata.contains(&hash160_compressed) {
                if let Ok(safe_key) = SafePrivateKey::from_bytes(&scalars[index]) {
                    collision_callback.on_finding(
                        prospector_core_gen::address_legacy::pubkey_from_x_and_parity_to_address(&affine_x_bytes, parity_prefix),
                        safe_key,
                        "sequential:meloni:compressed".into()
                    );
                }
            }
        });
    }
}

/// Extensión técnica para la duplicación bit-perfecta de buffers.
trait CopyFromSliceExt {
    fn copy_with_slice(&mut self, src: &[u8]);
}
impl CopyFromSliceExt for [u8] {
    #[inline(always)]
    fn copy_with_slice(&mut self, src: &[u8]) {
        self.copy_from_slice(src);
    }
}

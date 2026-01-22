// [libs/core/math-engine/src/secp256k1.rs]
/*!
 * =================================================================
 * APARATO: SOVEREIGN GEOMETRIC HUB (V140.0 - ZENITH UNIFICATION)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: AUTORIDAD GEOMÉTRICA Y DERIVACIÓN ESCALAR O(1)
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. LUT SATURATION: Implementa el acceso real a 'GENERATOR_TABLE',
 *    eliminando la derivación dinámica de la versión V132.0.
 * 2. NOMINAL PURITY: Erradicación total de 'be/le'. Uso estricto de
 *    'big_endian' para paridad con el protocolo Bitcoin.
 * 3. ARITHMETIC SYNERGY: Optimización de adición mixta (8M) mediante
 *    el uso de puntos afines pre-computados (Z=1).
 * 4. HYGIENE: Documentación técnica MIT nivel Tesis Doctoral.
 * =================================================================
 */

use crate::prelude::*;
use crate::generator_table::GENERATOR_TABLE;
use tracing::{instrument, trace};

/// Coordenada X del Punto Generador G (Fuente: SEC 2 v2).
pub const GENERATOR_G_X: [u64; 4] = [
    0x59F2815B16F81798, 0x029BFCDB2DCE28D9, 0x55A06295CE870B07, 0x79BE667EF9DCBBAC
];

/// Coordenada Y del Punto Generador G (Fuente: SEC 2 v2).
pub const GENERATOR_G_Y: [u64; 4] = [
    0x9C47D08FFB10D4B8, 0xFD17B448A6855419, 0x5DA4FBFC0E1108A8, 0x483ADA7726A3C465
];

impl JacobianPoint {
    /**
     * Materializa el punto generador G en el espacio proyectivo.
     *
     * # Mathematical Proof:
     * El punto se inicializa con Z=1, asumiendo una base afín para
     * optimizar las operaciones de grupo subsiguientes.
     */
    #[inline(always)]
    #[must_use]
    pub fn point_generator_g() -> Self {
        Self::from_affine(GENERATOR_G_X, GENERATOR_G_Y)
    }

    /**
     * Derivación Escalar Maestra (Q = k * G) vía Ventana Fija de 4 bits.
     *
     * # Performance:
     * Realiza exactamente 64 adiciones Jacobianas mixtas consultando la LUT.
     * Elimina el 100% de las duplicaciones escalares en el Hot-Path.
     *
     * @param private_scalar_big_endian_bytes Clave privada de 32 bytes.
     */
    #[instrument(level = "trace", skip(private_scalar_big_endian_bytes))]
    pub fn from_private_scalar_windowed(private_scalar_big_endian_bytes: &[u8; 32]) -> Self {
        trace!("🧬 [GEOMETRY]: Deriving public point via Fixed-Base Windowing (Static LUT).");

        let mut point_accumulator = Self::infinity();

        // Procesamos los 32 bytes del escalar (de más significativo a menos)
        for byte_position in 0..32 {
            let current_byte = private_scalar_big_endian_bytes[byte_position];

            // Calculamos el índice base de ventana (2 ventanas de 4 bits por cada byte)
            let base_window_index = (31 - byte_position) * 2;

            // 1. Procesamiento de Nibble Bajo (Bits 0-3)
            let low_nibble_value = current_byte & 0x0F;
            if low_nibble_value > 0 {
                point_accumulator = Self::execute_lookup_addition(
                    &point_accumulator,
                    base_window_index,
                    low_nibble_value as usize
                );
            }

            // 2. Procesamiento de Nibble Alto (Bits 4-7)
            let high_nibble_value = (current_byte >> 4) & 0x0F;
            if high_nibble_value > 0 {
                point_accumulator = Self::execute_lookup_addition(
                    &point_accumulator,
                    base_window_index + 1,
                    high_nibble_value as usize
                );
            }
        }

        point_accumulator
    }

    /**
     * Consulta la matriz de silicio y ejecuta la ley de grupo.
     * Implementa la adición mixta: Jacobian + Affine (Z=1).
     */
    #[inline(always)]
    fn execute_lookup_addition(
        current_accumulator: &Self,
        window_index: usize,
        value_index: usize
    ) -> Self {
        // Acceso O(1) a la tabla pre-computada
        let static_point_artifact = &GENERATOR_TABLE[window_index][value_index];

        let affine_x_element = FieldElement::from_limbs(static_point_artifact.x_limbs);
        let affine_y_element = FieldElement::from_limbs(static_point_artifact.y_limbs);

        UnifiedCurveEngine::add_mixed_deterministic(
            current_accumulator,
            &affine_x_element,
            &affine_y_element
        )
    }

    /**
     * Duplicación Jacobiana Determinista ($2P$).
     *
     * # Mathematical Proof:
     * Optimizada para secp256k1 ($a=0$). Coste: 3M + 4S.
     */
    #[inline(always)]
    #[instrument(level = "trace", skip_all)]
    pub fn double_deterministic(&self) -> Self {
        if self.is_infinity || self.y.is_zero() {
            return Self::infinity();
        }

        let x_coordinate_squared = self.x.square_modular();
        let tangent_slope_m = x_coordinate_squared
            .add_modular(&x_coordinate_squared)
            .add_modular(&x_coordinate_squared);

        let y_coordinate_squared = self.y.square_modular();
        let internal_term_s = self.x.multiply_modular(&y_coordinate_squared).multiply_by_u64(4);

        let m_squared = tangent_slope_m.square_modular();
        let s_doubled = internal_term_s.add_modular(&internal_term_s);
        let result_x = m_squared.subtract_modular(&s_doubled);

        let y_times_z_accumulator = self.y.multiply_modular(&self.z);
        let result_z = y_times_z_accumulator.add_modular(&y_times_z_accumulator);

        let y_fourth_power = y_coordinate_squared.square_modular();
        let scaled_y_fourth = y_fourth_power.multiply_by_u64(8);

        let s_minus_x3 = internal_term_s.subtract_modular(&result_x);
        let result_y = tangent_slope_m
            .multiply_modular(&s_minus_x3)
            .subtract_modular(&scaled_y_fourth);

        Self {
            x: result_x,
            y: result_y,
            z: result_z,
            is_infinity: false,
        }
    }

    /**
     * Adición Jacobiana Estándar ($P_1 + P_2$).
     * Utilizada en misiones de rango arbitrario o colisiones.
     */
    #[inline(always)]
    #[instrument(level = "trace", skip_all)]
    pub fn add_deterministic(&self, other_point: &Self) -> Self {
        if self.is_infinity { return *other_point; }
        if other_point.is_infinity { return *self; }

        let z1_squared = self.z.square_modular();
        let z2_squared = other_point.z.square_modular();

        let u1 = self.x.multiply_modular(&z2_squared);
        let u2 = other_point.x.multiply_modular(&z1_squared);

        let s1 = self.y.multiply_modular(&other_point.z.multiply_modular(&z2_squared));
        let s2 = other_point.y.multiply_modular(&self.z.multiply_modular(&z1_squared));

        if u1 == u2 {
            if s1 == s2 {
                return self.double_deterministic();
            } else {
                return Self::infinity();
            }
        }

        let h_distance = u2.subtract_modular(&u1);
        let r_slope = s2.subtract_modular(&s1);

        let h_squared = h_distance.square_modular();
        let h_cubed = h_squared.multiply_modular(&h_distance);
        let v_projection = u1.multiply_modular(&h_squared);

        let r_squared = r_slope.square_modular();
        let v_doubled = v_projection.add_modular(&v_projection);
        let result_x = r_squared.subtract_modular(&h_cubed).subtract_modular(&v_doubled);

        let v_minus_x3 = v_projection.subtract_modular(&result_x);
        let result_y = r_slope
            .multiply_modular(&v_minus_x3)
            .subtract_modular(&s1.multiply_modular(&h_cubed));

        let result_z = self.z.multiply_modular(&other_point.z).multiply_modular(&h_distance);

        Self {
            x: result_x,
            y: result_y,
            z: result_z,
            is_infinity: false,
        }
    }
}

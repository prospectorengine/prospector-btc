// [libs/core/math-engine/src/curve.rs]
/*!
 * =================================================================
 * APARATO: STANDARD JACOBIAN CURVE ENGINE (V131.0 - ZENITH GOLD)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: LEYES DE GRUPO PROYECTIVAS PARA SECP256K1
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. NOMINAL ALIGNMENT: Sincronización total con JacobianPoint V63.0 (x, y, z, is_infinity).
 * 2. ARITHMETIC OPTIMIZATION: Sustitución de multiplicaciones genéricas por square_modular()
 *    y multiply_by_u64() para reducir ciclos de reloj en el Hot-Path.
 * 3. ZERO REGRESSIONS: Mantenimiento de la lógica de trazado forense #[instrument].
 * 4. MATHEMATICAL RIGOR: Implementación bit-perfect de las fórmulas de adición mixta
 *    optimizadas para curvas de Weierstrass con a=0.
 *
 * # Mathematical Proof (Jacobian Strategy):
 * El uso de coordenadas Jacobianas (X, Y, Z) donde x = X/Z^2 e y = Y/Z^3 permite
 * realizar la ley de grupo sin inversiones modulares costosas (O(log p)).
 * =================================================================
 */

use crate::prelude::*;
use tracing::instrument;

/// Motor unificado para la ejecución de leyes de grupo en coordenadas Jacobianas.
pub struct UnifiedCurveEngine;

impl UnifiedCurveEngine {
    /**
     * Realiza la adición de un punto Jacobiano P1 y un punto Afín P2 (Z2=1).
     *
     * # Mathematical Proof:
     * Al asumir Z2 = 1, el coste computacional se reduce de 11 a 8 multiplicaciones de campo (8M).
     * Es la operación fundamental del barrido secuencial escalar.
     *
     * # Errors:
     * Retorna el punto infinito si la suma resulta en una singularidad geométrica.
     */
    #[inline(always)]
    #[instrument(level = "trace", skip_all)]
    pub fn add_mixed_deterministic(
        point_alpha: &JacobianPoint,
        point_beta_x: &FieldElement,
        point_beta_y: &FieldElement,
    ) -> JacobianPoint {
        // Gestión de Elemento Neutro: INF + P = P
        if point_alpha.is_infinity {
            return JacobianPoint::from_affine(
                point_beta_x.internal_words,
                point_beta_y.internal_words
            );
        }

        // 1. DERIVACIÓN DE COMPONENTES PROYECTIVAS
        // U2 = x2 * Z1^2
        let z1_squared = point_alpha.z.square_modular();
        let u2 = point_beta_x.multiply_modular(&z1_squared);

        // S2 = y2 * Z1^3
        let s2 = point_beta_y.multiply_modular(&point_alpha.z.multiply_modular(&z1_squared));

        // 2. CÁLCULO DE DIFERENCIAS (Distancias de campo)
        // h = U2 - X1
        let horizontal_distance = u2.subtract_modular(&point_alpha.x);
        // r = S2 - Y1
        let vertical_slope = s2.subtract_modular(&point_alpha.y);

        // 3. VALIDACIÓN DE SINGULARIDADES
        if horizontal_distance.is_zero() {
            if vertical_slope.is_zero() {
                // P1 == P2: Proceder a duplicación puntual
                return Self::double_point_jacobian(point_alpha);
            } else {
                // P1 == -P2: El resultado es la identidad
                return JacobianPoint::infinity();
            }
        }

        // 4. GENERACIÓN DE COORDENADAS RESULTANTES (X3, Y3, Z3)
        let h_squared = horizontal_distance.square_modular();
        let h_cubed = h_squared.multiply_modular(&horizontal_distance);
        let v_term = point_alpha.x.multiply_modular(&h_squared);

        // X3 = r^2 - h^3 - 2V
        let r_squared = vertical_slope.square_modular();
        let v_doubled = v_term.multiply_by_u64(2);

        let x3 = r_squared
            .subtract_modular(&h_cubed)
            .subtract_modular(&v_doubled);

        // Y3 = r * (V - X3) - Y1 * h^3
        let v_minus_x3 = v_term.subtract_modular(&x3);
        let first_y_segment = vertical_slope.multiply_modular(&v_minus_x3);
        let second_y_segment = point_alpha.y.multiply_modular(&h_cubed);
        let y3 = first_y_segment.subtract_modular(&second_y_segment);

        // Z3 = Z1 * h
        let z3 = point_alpha.z.multiply_modular(&horizontal_distance);

        JacobianPoint { x: x3, y: y3, z: z3, is_infinity: false }
    }

    /**
     * Implementa la duplicación Jacobiana (P + P) optimizada para secp256k1.
     *
     * # Mathematical Proof:
     * Dado que a = 0 en secp256k1, la fórmula se simplifica a 3M + 4S (Multiplicaciones/Cuadrados).
     *
     * # Performance:
     * Utiliza duplicaciones por adición para evitar multiplicaciones por 2 innecesarias.
     */
    #[inline(always)]
    #[instrument(level = "trace", skip_all)]
    pub fn double_point_jacobian(point: &JacobianPoint) -> JacobianPoint {
        if point.is_infinity || point.y.is_zero() {
            return JacobianPoint::infinity();
        }

        // 1. CÁLCULO DE PENDIENTE (M)
        // M = 3 * X1^2
        let x1_squared = point.x.square_modular();
        let slope_m = x1_squared
            .add_modular(&x1_squared)
            .add_modular(&x1_squared);

        // 2. CÁLCULO DE TÉRMINO DE SOPORTE (S)
        // S = 4 * X1 * Y1^2
        let y1_squared = point.y.square_modular();
        let s_term = point.x.multiply_modular(&y1_squared).multiply_by_u64(4);

        // 3. GENERACIÓN DE X3
        // X3 = M^2 - 2S
        let m_squared = slope_m.square_modular();
        let s_doubled = s_term.add_modular(&s_term);
        let x3 = m_squared.subtract_modular(&s_doubled);

        // 4. GENERACIÓN DE Z3
        // Z3 = 2 * Y1 * Z1
        let y1_z1 = point.y.multiply_modular(&point.z);
        let z3 = y1_z1.add_modular(&y1_z1);

        // 5. GENERACIÓN DE Y3
        // Y3 = M * (S - X3) - 8 * Y1^4
        let y1_fourth = y1_squared.square_modular();
        let support_y_segment = y1_fourth.multiply_by_u64(8);

        let s_minus_x3 = s_term.subtract_modular(&x3);
        let y3 = slope_m.multiply_modular(&s_minus_x3).subtract_modular(&support_y_segment);

        JacobianPoint { x: x3, y: y3, z: z3, is_infinity: false }
    }

    /**
     * ADICIÓN CO-Z (FASE INICIAL): ESCALADO DE COORDENADA Z.
     *
     * # Mathematical Proof:
     * Transforma un punto afín Q para que comparta la coordenada Z del punto P.
     * Q' = (x_q * Z_p^2, y_q * Z_p^3, Z_p).
     * Esto es el requisito fundamental para el motor Meloni 5M (L2-Strategy).
     */
    #[inline(always)]
    #[instrument(level = "trace", skip_all)]
    pub fn add_co_z_initial_step(
        point_p: &JacobianPoint,
        point_q_x: &FieldElement,
        point_q_y: &FieldElement
    ) -> JacobianPoint {
        let z_squared = point_p.z.square_modular();
        let z_cubed = point_p.z.multiply_modular(&z_squared);

        let scaled_x = point_q_x.multiply_modular(&z_squared);
        let scaled_y = point_q_y.multiply_modular(&z_cubed);

        JacobianPoint {
            x: scaled_x,
            y: scaled_y,
            z: point_p.z,
            is_infinity: false,
        }
    }
}

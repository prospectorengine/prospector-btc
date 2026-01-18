// [libs/core/math-engine/src/curve.rs]
/*!
 * =================================================================
 * APARATO: STANDARD JACOBIAN CURVE ENGINE (V130.2 - TRACING FIXED)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: LEYES DE GRUPO PROYECTIVAS PARA SECP256K1
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. MACRO RESOLUTION: Inyecta 'use tracing::instrument' para sanar el
 *    error de compilación E0433 detectado en el entorno de despliegue.
 * 2. FIXED SYNERGY: Mantiene el enlace bit-perfect con 'subtract_modular'
 *    y 'multiply_by_u64' del motor Montgomery real (field.rs V160.7).
 * 3. CO-Z SOBERANO: Implementa la transformación Meloni real para escalado
 *    de coordenada Z, optimizando el despacho secuencial en Fase 3.
 * 4. NOMINAL PURITY: Nomenclatura descriptiva absoluta (H -> horizontal_distance).
 * =================================================================
 */

use crate::prelude::*;
// ✅ RESOLUCIÓN DEFINITIVA: Inyección de macros de observabilidad
use tracing::instrument;

/// Motor unificado para la ejecución de leyes de grupo en coordenadas Jacobianas.
pub struct UnifiedCurveEngine;

impl UnifiedCurveEngine {
    /**
     * Realiza la adición de un punto Jacobiano (X1, Y1, Z1) y un punto Afín (x2, y2, Z=1).
     *
     * # Mathematical Proof:
     * Al ser Z2 = 1, la fórmula se optimiza de 11 a 8 Multiplicaciones.
     * Es el Hot-Path del barrido secuencial escalar.
     */
    #[inline(always)]
    #[instrument(level = "trace", skip_all)]
    pub fn add_mixed_deterministic(
        point_alpha_jacobian: &JacobianPoint,
        point_beta_affine_x: &FieldElement,
        point_beta_affine_y: &FieldElement,
    ) -> JacobianPoint {
        // Gestión de Elemento Neutro (Punto al Infinito)
        if point_alpha_jacobian.is_infinity {
            return JacobianPoint::from_affine(
                point_beta_affine_x.internal_words,
                point_beta_affine_y.internal_words
            );
        }

        // 1. DERIVACIÓN DE COMPONENTES PROYECTIVAS
        // U2 = x2 * Z1^2
        let z1_coordinate_squared = point_alpha_jacobian.z.square_modular();
        let projective_u2 = point_beta_affine_x.multiply_modular(&z1_coordinate_squared);

        // S2 = y2 * Z1^3
        let z1_coordinate_cubed = point_alpha_jacobian.z.multiply_modular(&z1_coordinate_squared);
        let projective_s2 = point_beta_affine_y.multiply_modular(&z1_coordinate_cubed);

        // 2. CÁLCULO DE DIFERENCIAS (Distancias de campo)
        // horizontal_distance (H) = U2 - X1
        let horizontal_distance = projective_u2.subtract_modular(&point_alpha_jacobian.x);
        // vertical_slope_r (R) = S2 - Y1
        let vertical_slope_r = projective_s2.subtract_modular(&point_alpha_jacobian.y);

        // 3. VALIDACIÓN DE SINGULARIDADES
        if horizontal_distance.is_zero() {
            if vertical_slope_r.is_zero() {
                // Los puntos colisionan en el plano: Proceder a duplicación técnica
                return Self::double_point_jacobian(point_alpha_jacobian);
            } else {
                // Puntos inversos: El resultado es la identidad (Infinito)
                return JacobianPoint::infinity();
            }
        }

        // 4. GENERACIÓN DE COORDENADAS RESULTANTES (X3, Y3, Z3)
        let horizontal_distance_squared = horizontal_distance.square_modular();
        let horizontal_distance_cubed = horizontal_distance_squared.multiply_modular(&horizontal_distance);
        let intermediate_v_term = point_alpha_jacobian.x.multiply_modular(&horizontal_distance_squared);

        // X3 = R^2 - H^3 - 2V
        let slope_r_squared = vertical_slope_r.square_modular();
        let intermediate_v_doubled = intermediate_v_term.add_modular(&intermediate_v_term);

        let output_x = slope_r_squared
            .subtract_modular(&horizontal_distance_cubed)
            .subtract_modular(&intermediate_v_doubled);

        // Y3 = R * (V - X3) - Y1 * H^3
        let distance_v_x3 = intermediate_v_term.subtract_modular(&output_x);
        let slope_r_times_v_x3 = vertical_slope_r.multiply_modular(&distance_v_x3);
        let y1_times_h_cubed = point_alpha_jacobian.y.multiply_modular(&horizontal_distance_cubed);
        let output_y = slope_r_times_v_x3.subtract_modular(&y1_times_h_cubed);

        // Z3 = Z1 * H
        let output_z = point_alpha_jacobian.z.multiply_modular(&horizontal_distance);

        JacobianPoint {
            x: output_x,
            y: output_y,
            z: output_z,
            is_infinity: false,
        }
    }

    /**
     * Implementa la duplicación Jacobiana optimizada para secp256k1 (a=0).
     *
     * # Algoritmo:
     * Utiliza la fórmula simplificada 3M + 4S.
     */
    #[inline(always)]
    #[instrument(level = "trace", skip_all)]
    pub fn double_point_jacobian(point: &JacobianPoint) -> JacobianPoint {
        if point.is_infinity || point.y.is_zero() {
            return JacobianPoint::infinity();
        }

        // x_coordinate_squared = X^2
        let x_coordinate_squared = point.x.square_modular();

        // slope_m = 3 * X^2
        let slope_m = x_coordinate_squared
            .add_modular(&x_coordinate_squared)
            .add_modular(&x_coordinate_squared);

        // y_coordinate_squared = Y^2
        let y_coordinate_squared = point.y.square_modular();

        // term_s = 4 * X * Y^2
        let x_times_y_squared = point.x.multiply_modular(&y_coordinate_squared);
        let term_s = x_times_y_squared
            .add_modular(&x_times_y_squared)
            .add_modular(&x_times_y_squared)
            .add_modular(&x_times_y_squared);

        // X3 = M^2 - 2*S
        let slope_m_squared = slope_m.square_modular();
        let term_s_doubled = term_s.add_modular(&term_s);
        let output_x = slope_m_squared.subtract_modular(&term_s_doubled);

        // Z3 = 2 * Y * Z
        let y_times_z = point.y.multiply_modular(&point.z);
        let output_z = y_times_z.add_modular(&y_times_z);

        // Y3 = M * (S - X3) - 8 * Y^4
        let y_coordinate_fourth = y_coordinate_squared.square_modular();
        let eight_y_fourth = y_coordinate_fourth.multiply_by_u64(8);

        let s_minus_x3 = term_s.subtract_modular(&output_x);
        let output_y = slope_m
            .multiply_modular(&s_minus_x3)
            .subtract_modular(&eight_y_fourth);

        JacobianPoint {
            x: output_x,
            y: output_y,
            z: output_z,
            is_infinity: false,
        }
    }

    /**
     * ADICIÓN CO-Z (MELONI): ESCALADO DE COORDENADA Z.
     *
     * # Mathematical Proof:
     * Transforma P y Q para que compartan la coordenada Z.
     * Si P=(X1, Y1, Z1) y Q=(X2, Y2, 1), entonces escalamos Q:
     * Q' = (X2*Z1^2, Y2*Z1^3, Z1).
     * Ahora P y Q' tienen Z_new = Z1, permitiendo adiciones de 5 multiplicaciones.
     */
    #[inline(always)]
    #[instrument(level = "trace", skip_all)]
    pub fn add_co_z_initial_step(
        p: &JacobianPoint,
        q_affine_x: &FieldElement,
        q_affine_y: &FieldElement
    ) -> JacobianPoint {
        let z_coordinate_squared = p.z.square_modular();

        // Scaled X = x2 * Z1^2
        let scaled_x = q_affine_x.multiply_modular(&z_coordinate_squared);

        // Scaled Y = y2 * Z1^3
        let z_coordinate_cubed = p.z.multiply_modular(&z_coordinate_squared);
        let scaled_y = q_affine_y.multiply_modular(&z_coordinate_cubed);

        JacobianPoint {
            x: scaled_x,
            y: scaled_y,
            z: p.z,
            is_infinity: false,
        }
    }
}

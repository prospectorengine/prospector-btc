// [libs/core/math-engine/src/point.rs]
#![allow(unsafe_code)]
#![deny(missing_docs)]

/*!
 * =================================================================
 * APARATO: GEOMETRIC POINT ENGINE (V64.1 - NOMINAL SYNCED)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: GESTIÓN DE PUNTOS PROYECTIVOS Y DERIVACIÓN O(1)
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. NOMINAL ALIGNMENT: Sincronización bit-perfecta con 'generator_table.rs' V1.4.
 *    Resuelve E0609 mediante el uso de 'x_limbs' y 'y_limbs'.
 * 2. QUANTUM DERIVATION: Implementa la multiplicación escalar de base fija
 *    reduciendo 256 duplicaciones a 64 adiciones de tabla.
 * 3. ZERO ABBREVIATIONS: Erradicación total de 'ax', 'res', 'sk'.
 * 4. HYGIENE: RustDoc MIT completo y rastro forense #[instrument].
 * =================================================================
 */

use crate::field::FieldElement;
use crate::errors::MathError;
use crate::private_key::SafePrivateKey;
use crate::curve::UnifiedCurveEngine;
use crate::generator_table::GENERATOR_TABLE;
use tracing::{instrument, trace};

/// Representa un punto en la curva secp256k1 utilizando coordenadas Jacobianas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JacobianPoint {
    /// Coordenada X en el espacio proyectivo.
    pub x: FieldElement,
    /// Coordenada Y en el espacio proyectivo.
    pub y: FieldElement,
    /// Coordenada Z (denominador compartido) en el espacio proyectivo.
    pub z: FieldElement,
    /// Indicador de singularidad (Identidad del Grupo).
    pub is_infinity: bool,
}

impl JacobianPoint {
    /**
     * Construye un punto Jacobiano a partir de componentes afines (Z = 1).
     *
     * # Mathematical Proof:
     * Al fijar Z = 1, el punto reside inicialmente en el plano afín de Bitcoin,
     * permitiendo adiciones mixtas optimizadas de 8 multiplicaciones (8M).
     */
    #[inline(always)]
    #[must_use]
    pub fn from_affine(x_raw_limbs: [u64; 4], y_raw_limbs: [u64; 4]) -> Self {
        Self {
            x: FieldElement::from_limbs(x_raw_limbs),
            y: FieldElement::from_limbs(y_raw_limbs),
            z: FieldElement::from_u64(1),
            is_infinity: false,
        }
    }

    /**
     * Punto de entrada de alto nivel para la derivación de clave pública (Q = kG).
     */
    #[inline(always)]
    #[must_use]
    pub fn from_private(private_key_handle: &SafePrivateKey) -> Self {
        let private_scalar_bytes = private_key_handle.to_bytes();
        Self::from_private_scalar_windowed(&private_scalar_bytes)
    }

    /**
     * Derivación Escalar Maestra mediante Ventana Fija de 4 bits.
     *
     * # Mathematical Proof:
     * Divide el escalar de 256 bits en 64 nibbles. El punto resultante es la 
     * suma de los puntos pre-computados en la matriz de silicio:
     * Q = sum(GENERATOR_TABLE[nibble_index][nibble_value]).
     *
     * # Performance:
     * Operación O(1) respecto al número de bits. Elimina el bottleneck 
     * de las duplicaciones sucesivas del algoritmo 'Double-and-Add'.
     */
    #[instrument(level = "trace", skip(private_scalar_bytes_big_endian))]
    pub fn from_private_scalar_windowed(private_scalar_bytes_big_endian: &[u8; 32]) -> Self {
        trace!("🧬 [POINT_ENGINE]: Deriving public point via Fixed-Base Windowing (64 steps).");

        let mut point_accumulator = Self::infinity();

        // Procesamos los 32 bytes (del más significativo al menos)
        for byte_index in 0..32 {
            let current_byte = private_scalar_bytes_big_endian[byte_index];
            let base_window_index = (31 - byte_index) * 2;

            // 1. Procesamiento del Nibble Bajo (Bits 0-3)
            let low_nibble_value = current_byte & 0x0F;
            if low_nibble_value > 0 {
                point_accumulator = Self::lookup_and_add_mixed(
                    &point_accumulator,
                    base_window_index,
                    low_nibble_value as usize
                );
            }

            // 2. Procesamiento del Nibble Alto (Bits 4-7)
            let high_nibble_value = (current_byte >> 4) & 0x0F;
            if high_nibble_value > 0 {
                point_accumulator = Self::lookup_and_add_mixed(
                    &point_accumulator,
                    base_window_index + 1,
                    high_nibble_value as usize
                );
            }
        }

        point_accumulator
    }

    /**
     * Consulta la matriz pre-computada y ejecuta una adición Jacobiana Mixta.
     *
     * # Performance (Elite):
     * Al estar los puntos de la tabla en formato Afín (Z=1), el coste es de solo 8M.
     */
    #[inline(always)]
    fn lookup_and_add_mixed(accumulator: &Self, window_index: usize, value_index: usize) -> Self {
        // ✅ RESOLUCIÓN E0609: Sincronía nominal con generator_table.rs V1.4
        let static_point_data = &GENERATOR_TABLE[window_index][value_index];

        let affine_x_element = FieldElement::from_limbs(static_point_data.x_limbs);
        let affine_y_element = FieldElement::from_limbs(static_point_data.y_limbs);

        UnifiedCurveEngine::add_mixed_deterministic(accumulator, &affine_x_element, &affine_y_element)
    }

    /**
     * Proyecta el punto Jacobiano de vuelta al plano afín de Bitcoin (x, y).
     *
     * # Mathematical Proof:
     * x = X * Z^-2 (mod p)
     * y = Y * Z^-3 (mod p)
     *
     * # Errors:
     * - Retorna 'InvalidKeyFormat' si el punto reside en el infinito.
     */
    #[instrument(level = "trace", skip(self))]
    pub fn to_affine_bytes(&self) -> Result<([u8; 32], [u8; 32]), MathError> {
        if self.is_infinity {
            return Err(MathError::InvalidKeyFormat("POINT_AT_INFINITY_CANNOT_PROJECT".into()));
        }

        // 1. Inversión modular del denominador compartido Z
        let coordinate_z_inverse = self.z.invert()?;

        // 2. Factores de proyección
        let z_inverse_squared = coordinate_z_inverse.square_modular();
        let z_inverse_cubed = z_inverse_squared.multiply_modular(&coordinate_z_inverse);

        // 3. Extracción de coordenadas normalizadas (Big-Endian para red)
        let affine_x_bytes = self.x.multiply_modular(&z_inverse_squared).internal_words_to_big_endian_bytes();
        let affine_y_bytes = self.y.multiply_modular(&z_inverse_cubed).internal_words_to_big_endian_bytes();

        Ok((affine_x_bytes, affine_y_bytes))
    }

    /**
     * Retorna el elemento neutro del grupo (Identidad).
     */
    #[inline(always)]
    #[must_use]
    pub fn infinity() -> Self {
        Self {
            x: FieldElement::from_u64(0),
            y: FieldElement::from_u64(0),
            z: FieldElement::from_u64(0),
            is_infinity: true,
        }
    }
}

impl Default for JacobianPoint {
    fn default() -> Self {
        Self::infinity()
    }
}
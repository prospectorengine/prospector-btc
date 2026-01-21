// [libs/core/math-engine/src/point.rs]
#![allow(unsafe_code)]
#![deny(missing_docs)]

/*!
 * =================================================================
 * APARATO: GEOMETRIC POINT ENGINE (V64.0 - STATIC LUT ENABLED)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: GESTIÓN DE PUNTOS PROYECTIVOS Y DERIVACIÓN O(1)
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. STATIC LUT INTEGRATION: Implementa el acceso real a la tabla de
 *    ventana de base fija (GENERATOR_TABLE), eliminando la simulación
 *    dinámica de la versión V63.3.
 * 2. QUANTUM DERIVATION: Optimización del motor 'from_private_scalar_windowed'
 *    mediante la reducción de 256 duplicaciones Jacobianas a 64 adiciones mixtas.
 * 3. NOMINAL PURITY: Erradicación total de abreviaciones. 'ax' -> 'affine_x',
 *    'res' -> 'result_point', 'sk' -> 'private_key_handle'.
 * 4. HYGIENE: Documentación técnica MIT completa para cada método público.
 *
 * # Mathematical Proof (Jacobian Projection):
 * El sistema opera en el espacio proyectivo Jacobian $(X, Y, Z)$ que mapea
 * al plano afín $(x, y)$ de Bitcoin mediante $x = X \cdot Z^{-2} \pmod p$.
 * Esta arquitectura permite que la ley de grupo (adición/duplicación) sea
 * una función de multiplicaciones de campo, difiriendo la inversión modular.
 * =================================================================
 */

use crate::field::FieldElement;
use crate::errors::MathError;
use crate::private_key::SafePrivateKey;
use crate::curve::UnifiedCurveEngine;
// ✅ SINCRO SOBERANA: Importación de la tabla pre-computada de 960 puntos
use crate::generator_table::GENERATOR_TABLE;
use tracing::{instrument, trace};

/// Representa un punto en la curva secp256k1 utilizando coordenadas Jacobianas.
///
/// El elemento identidad (Punto al Infinito) se representa mediante el flag `is_infinity`.
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
     * # Mathematical Proof
     * Al fijar Z = 1, el punto proyectivo reside inicialmente en el plano afín,
     * facilitando las adiciones mixtas subsiguientes en el motor secuencial.
     *
     * @param x_raw_limbs Palabras de 64 bits para la coordenada X.
     * @param y_raw_limbs Palabras de 64 bits para la coordenada Y.
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
     * Punto de entrada de alto nivel para la derivación de clave pública.
     *
     * @param private_key_handle Instancia validada de la clave privada (L1-L2 Bridge).
     */
    #[inline(always)]
    #[must_use]
    pub fn from_private(private_key_handle: &SafePrivateKey) -> Self {
        let private_scalar_bytes = private_key_handle.to_bytes();
        Self::from_private_scalar_windowed(&private_scalar_bytes)
    }

    /**
     * IGNICIÓN CUÁNTICA: Multiplicación de Base Fija mediante Ventana de 4 bits.
     *
     * # Mathematical Proof
     * Divide el escalar de 256 bits en 64 ventanas de 4 bits (nibbles).
     * El punto resultante se calcula como la suma de 64 términos pre-computados:
     * $Q = \sum_{i=0}^{63} [v_i] \cdot (16^i \cdot G)$
     *
     * # Performance
     * Operación O(1) respecto a la duplicación. Reduce el esfuerzo computacional
     * de 256 duplicaciones + 128 adiciones a exactamente 64 adiciones de tabla.
     */
    #[instrument(level = "trace", skip(private_scalar_bytes_big_endian))]
    pub fn from_private_scalar_windowed(private_scalar_bytes_big_endian: &[u8; 32]) -> Self {
        trace!("🧬 [POINT_ENGINE]: Deriving public point via Fixed-Base Windowing.");

        let mut point_accumulator = Self::infinity();

        // Procesamos los 32 bytes del escalar (de más significativo a menos)
        for byte_index in 0..32 {
            let current_byte = private_scalar_bytes_big_endian[byte_index];

            // Calculamos el índice base de ventana (2 ventanas por byte)
            let base_window_index = (31 - byte_index) * 2;

            // 1. Ventana Baja (4 bits inferiores)
            let low_nibble_value = current_byte & 0x0F;
            if low_nibble_value > 0 {
                point_accumulator = Self::lookup_and_add_mixed(
                    &point_accumulator,
                    base_window_index,
                    low_nibble_value as usize
                );
            }

            // 2. Ventana Alta (4 bits superiores)
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
     * Consulta la tabla estática y ejecuta una adición mixta (Jacobian + Affine).
     *
     * # Performance (Elite):
     * Al estar los puntos de la tabla en formato afín ($Z=1$), la fórmula de
     * adición se reduce a 8 multiplicaciones de campo (8M), maximizando el hashrate.
     */
    #[inline(always)]
    fn lookup_and_add_mixed(accumulator: &Self, window_index: usize, value_index: usize) -> Self {
        // Acceso O(1) a la memoria pre-computada
        let static_point_data = &GENERATOR_TABLE[window_index][value_index];

        let affine_x_element = FieldElement::from_limbs(static_point_data.x_limbs);
        let affine_y_element = FieldElement::from_limbs(static_point_data.y_limbs);

        UnifiedCurveEngine::add_mixed_deterministic(accumulator, &affine_x_element, &affine_y_element)
    }

    /**
     * Proyecta el punto Jacobiano al plano afín de Bitcoin ($X/Z^2, Y/Z^3$).
     *
     * # Errors
     * - `MathError::InvalidKeyFormat`: Si el punto reside en el infinito.
     *
     * # Performance
     * Requiere una inversión modular costosa. Se utiliza como paso final
     * tras el barrido secuencial o ante una detección positiva del filtro.
     */
    #[instrument(level = "trace", skip(self))]
    pub fn to_affine_bytes(&self) -> Result<([u8; 32], [u8; 32]), MathError> {
        if self.is_infinity {
            return Err(MathError::InvalidKeyFormat("POINT_AT_INFINITY_CANNOT_PROJECT".into()));
        }

        // 1. Inversión modular del denominador compartido Z
        let coordinate_z_inverse = self.z.invert()?;

        // 2. Cálculo de factores de proyección
        let z_inverse_squared = coordinate_z_inverse.square_modular();
        let z_inverse_cubed = z_inverse_squared.multiply_modular(&coordinate_z_inverse);

        // 3. Extracción de coordenadas afines normalizadas
        let affine_x_bytes = self.x.multiply_modular(&z_inverse_squared).internal_words_to_big_endian_bytes();
        let affine_y_bytes = self.y.multiply_modular(&z_inverse_cubed).internal_words_to_big_endian_bytes();

        Ok((affine_x_bytes, affine_y_bytes))
    }

    /**
     * Retorna la identidad del grupo (Punto al Infinito).
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
    /**
     * Inicializa el punto Jacobiano como el elemento neutro por defecto.
     */
    fn default() -> Self {
        Self::infinity()
    }
}

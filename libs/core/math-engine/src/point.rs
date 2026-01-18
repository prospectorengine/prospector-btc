// [libs/core/math-engine/src/point.rs]
/*!
 * =================================================================
 * APARATO: GEOMETRIC POINT ENGINE (V60.0 - ZENITH GOLD MASTER)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: MANIPULACIÓN DE PUNTOS EN ESPACIO PROYECTIVO
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. SCALAR ASCENSION: Implementa 'from_private' para sanar los errores
 *    0-3 de compilación, permitiendo la ignición de motores forenses.
 * 2. NOMINAL SYMMETRY: Sincroniza con 'field.rs' (V170.0) utilizando
 *    'internal_words_to_be_bytes' para paridad absoluta.
 * 3. ZERO ABBREVIATIONS: Nomenclatura nominal (fe -> field_element).
 * 4. HYGIENE: Documentación de Tesis MIT y rastro #[instrument].
 *
 * # Mathematical Proof (Projective Jacobian):
 * Un punto (x, y) en el plano afín se representa como (X, Y, Z) tal que
 * x = X/Z^2 y y = Y/Z^3. Al fijar Z=1, el punto reside originalmente en
 * el plano afín, permitiendo adiciones mixtas ultra-veloces.
 * =================================================================
 */

use crate::field::FieldElement;
use crate::errors::MathError;
use crate::private_key::SafePrivateKey;
use crate::public_key::SafePublicKey;
use tracing::{trace, instrument};

/// Punto en la curva secp256k1 utilizando coordenadas Jacobianas (X, Y, Z).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JacobianPoint {
    /// Coordenada X proyectiva.
    pub x: FieldElement,
    /// Coordenada Y proyectiva.
    pub y: FieldElement,
    /// Coordenada Z proyectiva (Denominador compartido).
    pub z: FieldElement,
    /// Indica si el punto es la identidad aditiva (Infinito).
    pub is_infinity: bool,
}

impl JacobianPoint {
    /**
     * Crea un punto Jacobiano a partir de coordenadas afines (X, Y).
     * Se utiliza típicamente para cargar el Punto Generador G o llaves base.
     */
    #[inline(always)]
    #[must_use]
    pub fn from_affine(x_raw_words: [u64; 4], y_raw_words: [u64; 4]) -> Self {
        Self {
            x: FieldElement::from_limbs(x_raw_words),
            y: FieldElement::from_limbs(y_raw_words),
            z: FieldElement::from_u64(1), // Z=1 define el plano afín inicial
            is_infinity: false,
        }
    }

    /**
     * Asciende una Clave Privada al espacio Jacobiano (P = k * G).
     *
     * # Mathematical Proof:
     * Utiliza la tabla de multiplicación escalar pre-computada del contexto
     * global para derivar la clave pública y transformarla a coordenadas Jacobianas.
     *
     * # Performance:
     * Operación O(1) con aceleración por tabla. Esencial para la ignición
     * de los motores Satoshi-XP y Android LCG.
     */
    #[instrument(level = "trace", skip(private_key_handle))]
    pub fn from_private(private_key_handle: &SafePrivateKey) -> Self {
        trace!("🧬 [GEOMETRY]: Ascending private scalar to Jacobian space.");

        // 1. Derivación de Clave Pública Afín
        let public_key_instance = SafePublicKey::from_private(private_key_handle);

        // 2. Extracción de coordenadas (SEC1 Uncompressed: 0x04 + X + Y)
        let public_key_bytes = public_key_instance.to_bytes(false);

        // 3. Mapeo a Elementos de Campo L1
        let mut x_bytes = [0u8; 32];
        let mut y_bytes = [0u8; 32];
        x_bytes.copy_from_slice(&public_key_bytes[1..33]);
        y_bytes.copy_from_slice(&public_key_bytes[33..65]);

        let field_x = FieldElement::from_bytes_be(&x_bytes);
        let field_y = FieldElement::from_bytes_be(&y_bytes);

        Self::from_affine(field_x.internal_words, field_y.internal_words)
    }

    /**
     * Transforma el punto Jacobiano a coordenadas afines.
     * Requiere una inversión modular de Z, seguido de multiplicaciones de campo.
     *
     * # Errors:
     * Retorna 'MathError' si el punto está en el infinito (no invertible).
     *
     * # Performance:
     * Complejidad: 1 Inversión + 4 Multiplicaciones + 2 Cuadrados.
     */
    #[instrument(level = "trace", skip(self))]
    pub fn to_affine_bytes(&self) -> Result<([u8; 32], [u8; 32]), MathError> {
        if self.is_infinity {
            return Err(MathError::InvalidKeyFormat("POINT_AT_INFINITY_COLLAPSE".into()));
        }

        // 1. Calcular Z^-1 mod p utilizando Pequeño Teorema de Fermat
        let z_inverse = self.z.invert()?;
        let z_inverse_squared = z_inverse.square_modular();
        let z_inverse_cubed = z_inverse_squared.multiply_modular(&z_inverse);

        // 2. Recuperar x = X/Z^2, y = Y/Z^3
        let x_affine_element = self.x.multiply_modular(&z_inverse_squared);
        let y_affine_element = self.y.multiply_modular(&z_inverse_cubed);

        // 3. Serialización Sincronizada (Big-Endian)
        Ok((
            x_affine_element.internal_words_to_be_bytes(),
            y_affine_element.internal_words_to_be_bytes()
        ))
    }

    /**
     * Genera la identidad aditiva de la curva (Punto al Infinito).
     */
    #[inline(always)]
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

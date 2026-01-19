// [libs/core/math-engine/src/public_key.rs]
/*!
 * =================================================================
 * APARATO: PUBLIC KEY ENGINE (V17.1 - DOCUMENTATION SEALED)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: GESTIÓN DE PUNTOS AFINES Y SERIALIZACIÓN SEC1
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. FULL RUSTDOC: Resuelve los errores de 'missing_docs' (Severity 8) inyectando
 *    especificaciones de Tesis en el struct y todos sus métodos públicos.
 * 2. MATHEMATICAL RIGOR: Documentación de la ley de grupo para incrementos
 *    y ajustes escalares (tweaks).
 * 3. ZERO REGRESSIONS: Preserva el uso de 'secp256k1::PublicKey' optimizado
 *    con el contexto global pre-computado.
 * 4. PERFORMANCE: Mantenimiento de los marcadores #[inline(always)] para el Hot-Path.
 * =================================================================
 */

use crate::context::global_context;
use crate::errors::MathError;
use crate::private_key::SafePrivateKey;
use secp256k1::{PublicKey, Scalar};

/// Representa una Clave Pública en la curva secp256k1.
///
/// Es un punto $(x, y)$ que satisface la ecuación de Weierstrass $y^2 = x^3 + 7 \pmod p$.
/// Actúa como el identificador soberano para la derivación de direcciones Bitcoin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SafePublicKey {
    /// El punto de la curva elíptica gestionado por la librería criptográfica subyacente.
    internal_point: PublicKey,
}

impl SafePublicKey {
    /**
     * Deriva una clave pública a partir de una clave privada ($Q = k \cdot G$).
     *
     * # Mathematical Proof
     * Realiza la multiplicación escalar del punto generador $G$ por el escalar secreto $k$.
     * Utiliza las tablas de pre-cómputo del contexto global para máxima velocidad.
     *
     * # Performance
     * Operación de alto coste amortizada por el Singleton del contexto.
     */
    #[inline(always)]
    #[must_use]
    pub fn from_private(private_key_handle: &SafePrivateKey) -> Self {
        let context = global_context();
        let point = PublicKey::from_secret_key(context, private_key_handle.as_inner());
        Self { internal_point: point }
    }

    /**
     * Reconstruye una clave pública a partir de una ráfaga de bytes en formato SEC1.
     *
     * # Errors
     * Retorna `MathError::EllipticCurveError` si los bytes no representan un punto válido
     * en la curva o si el formato (comprimido/no-comprimido) es ilegal.
     */
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, MathError> {
        // El contexto se requiere para la validación de integridad del punto.
        let _context = global_context();
        let point = PublicKey::from_slice(bytes)
            .map_err(MathError::EllipticCurveError)?;

        Ok(Self { internal_point: point })
    }

    /**
     * Incrementa la clave pública sumándole el punto generador ($Q' = Q + G$).
     *
     * # Mathematical Proof
     * Equivale a incrementar el escalar privado original en una unidad ($k + 1$)
     * sin conocer el valor de $k$. Utiliza la propiedad homomórfica de la curva.
     *
     * # Performance
     * Operación crítica para el motor secuencial. Optimizada mediante 'tweak' aditivo.
     */
    #[inline(always)]
    pub fn increment(&self) -> Result<Self, MathError> {
        let context = global_context();
        let mut one_scalar_bytes = [0u8; 32];
        one_scalar_bytes[31] = 1;

        // Transformación a escalar de la librería (Big-Endian)
        let scalar_one = Scalar::from_be_bytes(one_scalar_bytes)
            .map_err(|_| MathError::InvalidKeyFormat("INTERNAL_SCALAR_ERROR".into()))?;

        let updated_point = self.internal_point.add_exp_tweak(context, &scalar_one)
            .map_err(MathError::EllipticCurveError)?;

        Ok(Self { internal_point: updated_point })
    }

    /**
     * Ajusta la clave pública sumándole un escalar arbitrario ($Q' = Q + s \cdot G$).
     *
     * # Mathematical Proof
     * Utilizado para saltos cuánticos en la curva durante misiones de búsqueda
     * distribuida y algoritmos de tipo Kangaroo.
     *
     * # Errors
     * Retorna error si el escalar `scalar_bytes` desborda el orden $n$ de la curva.
     */
    #[inline(always)]
    pub fn add_scalar(&self, scalar_bytes: &[u8; 32]) -> Result<Self, MathError> {
        let context = global_context();
        let scalar_value = Scalar::from_be_bytes(*scalar_bytes)
            .map_err(|_| MathError::InvalidKeyFormat("SCALAR_OVERFLOW".into()))?;

        let updated_point = self.internal_point.add_exp_tweak(context, &scalar_value)
            .map_err(MathError::EllipticCurveError)?;

        Ok(Self { internal_point: updated_point })
    }

    /**
     * Serializa la clave pública al formato binario estándar SEC1.
     *
     * # Arguments
     * * `use_compression` - Si es true, genera 33 bytes (prefijo + X).
     *                       Si es false, genera 65 bytes (0x04 + X + Y).
     */
    #[inline(always)]
    #[must_use]
    pub fn to_bytes(&self, use_compression: bool) -> Vec<u8> {
        if use_compression {
            self.internal_point.serialize().to_vec()
        } else {
            self.internal_point.serialize_uncompressed().to_vec()
        }
    }

    /**
     * Provee acceso por referencia al tipo de dato nativo de la librería criptográfica.
     * Reservado para operaciones de interoperabilidad interna en L1/L2.
     */
    #[inline(always)]
    #[must_use]
    pub fn as_inner(&self) -> &PublicKey {
        &self.internal_point
    }
}

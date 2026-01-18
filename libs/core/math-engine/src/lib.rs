// [libs/core/math-engine/src/lib.rs]
/*!
 * =================================================================
 * APARATO: CORE MATH MASTER HUB (V33.0 - ZENITH ALIGNED)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: EXPOSICIÓN DE PRIMITIVAS Y DETECCIÓN DE HARDWARE
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. HARDWARE AWARENESS: Inyecta utilidades para detectar soporte de
 *    ADX y BMI2 en tiempo de ejecución, facilitando el balanceo de
 *    carga en el enjambre Hydra-Zero.
 * 2. MODULE HIERARCHY: Mantiene el aislamiento galvánico de los
 *    sub-aparatos (Arithmetic, Field, Curve, etc.).
 * 3. NOMINAL PRELUDE: Re-exportación exhaustiva de la aritmética
 *    U256 nivelada para evitar ruido de importación en el worker.
 * 4. HYGIENE: Cero advertencias y documentación de grado Tesis MIT.
 * =================================================================
 */

pub mod arithmetic;
pub mod context;
pub mod curve;
pub mod curve_simd;
pub mod errors;
pub mod field;
pub mod field_simd;
pub mod hashing;
pub mod point;
pub mod private_key;
pub mod public_key;
pub mod scalar;
pub mod kangaroo;

/**
 * ESTRATO DE PROSPECCIÓN DE HARDWARE
 *
 * Permite a los motores de búsqueda (L2) ajustar su agresividad
 * basándose en la arquitectura física detectada.
 */
pub mod hardware {
    /**
     * Determina si la CPU actual posee las extensiones ADX y BMI2.
     * Vital para autorizar el uso de cadenas de acarreo paralelas (ADCX/ADOX).
     */
    #[must_use]
    pub fn is_optimized_arithmetic_supported() -> bool {
        #[cfg(target_arch = "x86_64")]
        {
            // ADX y BMI2 son requisitos para el Hot-Path de V120.0
            is_x86_feature_detected!("adx") && is_x86_feature_detected!("bmi2")
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            false
        }
    }
}

/**
 * PRELUDIO MATEMÁTICO SOBERANO
 *
 * Colección de tipos y funciones de alta frecuencia para inyección
 * directa en el kernel de minería.
 */
pub mod prelude {
    pub use crate::arithmetic::{
        add_u64_to_u256_be,
        add_u256_be,
        subtract_u256_be,
        compare_u256_be,
        convert_u256_be_to_limbs_u64,
        convert_limbs_u64_to_u256_be,
        convert_u128_to_u256_be,
        fast_hex_encode,
        U256_BYTE_SIZE
    };

    pub use crate::field::{FieldElement, SECP256K1_FIELD_PRIME};
    pub use crate::point::JacobianPoint;
    pub use crate::curve::UnifiedCurveEngine;
    pub use crate::field_simd::FieldElementVector4;
    pub use crate::curve_simd::JacobianPointVector4;
    pub use crate::private_key::SafePrivateKey;
    pub use crate::public_key::SafePublicKey;
    pub use crate::errors::MathError;
    pub use crate::hashing::hash160;
    pub use crate::kangaroo::{KangarooSolver, KangarooConfig};
    pub use crate::hardware::is_optimized_arithmetic_supported;
}

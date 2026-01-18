// [libs/core/probabilistic/src/lib.rs]

#![deny(missing_docs)]
// Autorizamos el uso de código inseguro a nivel de crate raíz para permitir
// que los sub-módulos gestionen el mapeo de memoria (mmap) de forma soberana.
#![allow(unsafe_code)]

//! # APARATO: PROBABILISTIC STRATUM ROOT (V32.2 - ZENITH GOLD)
//! CLASIFICACIÓN: CORE INFRASTRUCTURE (ESTRATO L1)
//! RESPONSABILIDAD: ORQUESTACIÓN Y EXPOSICIÓN DEL MOTOR PROBABILÍSTICO
//!
//! ## Visión Holística 2026
//! Actúa como la autoridad suprema de la matriz de búsqueda. Esta versión
//! sella la integración entre el envoltorio atómico y el orquestador
//! fragmentado (Sharding), garantizando que los motores de búsqueda L2
//! operen sobre una base determinista y de ultra-bajo consumo de memoria.
//!
//! ## Características de Élite
//! 1. **Zero-Copy Architecture:** Habilita el acceso directo a censo UTXO vía MMAP.
//! 2. **Nominal Symmetry:** Sincroniza los tipos 'RichListFilter' y 'ShardedFilter'
//!    con las exigencias del contrato del enjambre.
//! 3. **Poison Shielding:** Propaga la resiliencia de cerrojos (locks) a toda la crate.
//! 4. **Higiene L1:** Erradicación total de advertencias del compilador y residuos.

/// Definiciones de errores semánticos para el triaje programático de fallos.
pub mod errors;

/// Envoltorio atómico para la matriz de Bloom de 160 bits (secp256k1).
pub mod filter_wrapper;

/// Orquestador de fragmentación determinista para auditoría paralela masiva.
pub mod sharded;

/**
 * RE-EXPORTACIONES SOBERANAS (NOMINAL ACCESS)
 *
 * Centralizamos los tipos fundamentales para eliminar el ruido de importación
 * en los estratos superiores (L2-Strategy y L3-Orchestrator).
 */

pub use crate::errors::FilterError;
pub use crate::filter_wrapper::RichListFilter;
pub use crate::sharded::ShardedFilter;

/**
 * PRELUDIO PROBABILÍSTICO
 *
 * Colección de tipos de alta frecuencia para inyección directa en motores.
 */
pub mod prelude {
    pub use crate::errors::FilterError;
    pub use crate::filter_wrapper::RichListFilter;
    pub use crate::sharded::ShardedFilter;
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * CERTIFICACIÓN DE VISIBILIDAD SOBERANA:
     * Garantiza que los tipos clave son accesibles y que el linkado es íntegro.
     *
     * # Logic:
     * Utiliza reflexión estática de tipos para certificar que la jerarquía
     * de módulos no ha sufrido regresiones de encapsulamiento.
     */
    #[test]
    fn certify_stratum_visibility_v32_2() {
        // Validación de existencia nominal de los pilares del estrato
        let _rich_filter_id = std::any::TypeId::of::<RichListFilter>();
        let _sharded_filter_id = std::any::TypeId::of::<ShardedFilter>();
        let _error_id = std::any::TypeId::of::<FilterError>();

        println!("✅ STRATUM_L1: Integrity and visibility contracts certified.");
    }
}

// [libs/domain/forensics/src/lib.rs]
#![deny(missing_docs)]

/*!
 * =================================================================
 * APARATO: FORENSICS DOMAIN BARREL (V17.2 - LINKAGE REPAIRED)
 * CLASIFICACIÓN: DOMAIN LAYER (ESTRATO L2)
 * RESPONSABILIDAD: AUTORIDAD SUPREMA DE ARQUEOLOGÍA CRIPTOGRÁFICA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. LINKAGE RESOLUTION: Resuelve el error E0583 mapeando los nombres
 *    nominales a los archivos físicos de rastro reducido (rng.rs).
 * 2. ZERO ABBREVIATIONS: La API pública utiliza nombres completos
 *    ('android_random_generator') para el rigor de la Tesis Doctoral.
 * 3. NOMINAL PRELUDE: Centraliza el acceso para el StrategyExecutor.
 * 4. HYGIENE: Documentación técnica MIT y validación de tipos atómica.
 * =================================================================
 */

/// Motor de reconstrucción para la vulnerabilidad LCG de Java en Android (2013).
/// ✅ SINCRO: Mapeo físico a android_rng.rs para resolver E0583.
#[path = "android_rng.rs"]
pub mod android_random_generator;

/// Motor de reconstrucción para el fallo de entropía de OpenSSL en Debian (2008).
/// ✅ SINCRO: Mapeo físico a debian_rng.rs para resolver E0583.
#[path = "debian_rng.rs"]
pub mod debian_random_generator;

/// Motor de reconstrucción temporal para la vulnerabilidad Luno/Blockchain.info (2014).
/// ✅ SINCRO: Mapeo físico a luno_rng.rs para resolver E0583.
#[path = "luno_rng.rs"]
pub mod luno_random_generator;

// --- RE-EXPORTACIONES SOBERANAS (NOMINAL ACCESS) ---
// Garantizan que el StrategyExecutor (L2) no sufra regresiones.

pub use crate::android_random_generator::AndroidLcgIterator;
pub use crate::debian_random_generator::DebianForensicIterator;
pub use crate::luno_random_generator::LunoForensicIterator;

/**
 * PRELUDIO DE ARQUEOLOGÍA DIGITAL
 *
 * Colección nominal de tipos esenciales para la inyección de motores 
 * forenses en el StrategyExecutor Maestro.
 */
pub mod prelude {
    pub use crate::android_random_generator::AndroidLcgIterator;
    pub use crate::debian_random_generator::DebianForensicIterator;
    pub use crate::luno_random_generator::LunoForensicIterator;
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * CERTIFICACIÓN DE ENLACE ESTRUCTURAL:
     * Valida que la jerarquía de módulos y el preludio son íntegros tras el mapeo.
     */
    #[test]
    fn certify_forensics_barrel_linkage_v17_2() {
        let _android_id = std::any::TypeId::of::<AndroidLcgIterator>();
        let _debian_id = std::any::TypeId::of::<DebianForensicIterator>();
        let _luno_id = std::any::TypeId::of::<LunoForensicIterator>();

        println!("✅ FORENSICS_L2: Structural link and nominal visibility certified.");
    }
}
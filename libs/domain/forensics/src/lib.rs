// [libs/domain/forensics/src/lib.rs]
#![deny(missing_docs)]

/*!
 * =================================================================
 * APARATO: FORENSICS DOMAIN BARREL (V17.1 - SINGULARITY GOLD)
 * CLASIFICACIÓN: DOMAIN LAYER (ESTRATO L2)
 * RESPONSABILIDAD: AUTORIDAD SUPREMA DE ARQUEOLOGÍA CRIPTOGRÁFICA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. ENTROPY ARCHAEOLOGY: Centraliza los motores de reconstrucción de 
 *    generadores de números pseudo-aleatorios (PRNG) históricamente defectuosos.
 * 2. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta nivel Tesis Doctoral.
 *    'android_rng' -> 'android_random_generator'.
 * 3. LUNO REAL-TIME SYNC: Integra el motor de reconstrucción temporal 2014, 
 *    sellando la brecha de seguridad de Blockchain.info.
 * 4. HYGIENE: Documentación técnica MIT y rastro de visibilidad estricto.
 *
 * # Mathematical Proof (Deterministic Archaeology):
 * La librería expone iteradores que mapean subespacios de baja entropía 
 * (2^15, 2^35, 2^48) hacia el espacio soberano de secp256k1, permitiendo 
 * auditorías exhaustivas imposibles mediante fuerza bruta tradicional.
 * =================================================================
 */

/// Motor de reconstrucción para la vulnerabilidad LCG de Java en Android (2013).
pub mod android_random_generator;

/// Motor de reconstrucción para el fallo de entropía de OpenSSL en Debian (2008).
pub mod debian_random_generator;

/// Motor de reconstrucción temporal para la vulnerabilidad Luno/Blockchain.info (2014).
pub mod luno_random_generator;

// --- RE-EXPORTACIONES SOBERANAS (NOMINAL ACCESS) ---

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
     * Valida que la jerarquía de módulos y el preludio son íntegros.
     */
    #[test]
    fn certify_forensics_barrel_visibility_v17() {
        let _android_id = std::any::TypeId::of::<AndroidLcgIterator>();
        let _debian_id = std::any::TypeId::of::<DebianForensicIterator>();
        let _luno_id = std::any::TypeId::of::<LunoForensicIterator>();

        println!("✅ FORENSICS_L2: Structural link and nominal visibility certified.");
    }
}
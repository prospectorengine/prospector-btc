// [apps/miner-worker/src/lib.rs]
/*!
 * =================================================================
 * APARATO: MINER LIBRARY ROOT (V1.1 - SOBERANO)
 * CLASIFICACIÓN: ESTRATO L1-WORKER
 * RESPONSABILIDAD: EXPOSICIÓN NOMINAL Y PRELUDIO DE IGNICIÓN
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. NOMINAL EXPOSURE: Re-exporta los componentes nucleares para
 *    sanar el rastro de importación en el binario principal.
 * 2. PRELUDE PATTERN: Centraliza los tipos de alta frecuencia.
 * 3. HYGIENE: Erradica la opacidad de los sub-módulos privados.
 * =================================================================
 */

pub mod cpu_manager;
pub mod engine;

/**
 * PRELUDIO DEL TRABAJADOR
 *
 * Colección soberana de tipos necesaria para la ignición de la Shell.
 */
pub mod prelude {
    pub use crate::engine::MinerEngine;
    pub use crate::cpu_manager::HardwareMonitor;
    pub use crate::cpu_manager::NodeHardwareMetrics;
}

// Re-exportación nominal para consumidores externos (Tests/Apps)
pub use engine::MinerEngine;
pub use cpu_manager::HardwareMonitor;

#![deny(missing_docs)]

/*!
 * =================================================================
 * APARATO: CENSUS TAKER KERNEL (V1.0 - SOBERANO)
 * CLASIFICACIÓN: ESTRATO L6 - OPS INFRASTRUCTURE
 * RESPONSABILIDAD: EXPOSICIÓN DE LA API DE CARTOGRAFÍA CRIPTOGRÁFICA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * Actúa como la autoridad única de exportación para el motor ETL.
 * 1. STRUCTURAL INTEGRITY: Resuelve el fallo 'os error 2' al proveer
 *    el punto de entrada de librería exigido por el Cargo.toml.
 * 2. MODULE ORCHESTRATION: Centraliza el rastro de los sub-aparatos
 *    (Pipeline, Partitioner, Generator) eliminando la redundancia en el binario.
 * 3. PUBLIC INTERFACE: Facilita la certificación forense mediante la
 *    exposición nominal de los motores de ingesta para la suite de pruebas.
 *
 * # Mathematical Proof (Deterministic Extraction):
 * La librería garantiza que el flujo de datos [CSV -> Bloom Filter] sea
 * reproducible y auditable bit-a-bit en cualquier instancia del enjambre.
 * =================================================================
 */

/// Motor de orquestación del flujo de ingesta masiva.
pub mod pipeline;

/// Lógica de segmentación cronológica y particionamiento de estratos.
pub mod partitioner;

/// Generador sintético de ADN de sistema operativo para la Tesis.
pub mod forensic_generator;

// --- RE-EXPORTACIONES SOBERANAS (NOMINAL ACCESS) ---

pub use crate::pipeline::IngestionPipeline;
pub use crate::partitioner::ForensicPartitioner;
pub use crate::forensic_generator::ForensicTemplateGenerator;

/**
 * PRELUDIO DEL CARTÓGRAFO
 *
 * Colección de tipos esenciales para la manipulación del censo UTXO.
 */
pub mod prelude {
    pub use crate::pipeline::IngestionPipeline;
    pub use crate::partitioner::ForensicPartitioner;
    pub use crate::forensic_generator::ForensicTemplateGenerator;
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * CERTIFICACIÓN DE ENLACE ESTRUCTURAL:
     * Valida que la jerarquía de módulos es íntegra y accesible.
     */
    #[test]
    fn certify_census_kernel_visibility() {
        let _pipeline_id = std::any::TypeId::of::<IngestionPipeline>();
        let _partitioner_id = std::any::TypeId::of::<ForensicPartitioner>();
        let _generator_id = std::any::TypeId::of::<ForensicTemplateGenerator>();

        println!("✅ CENSUS_LIB: Structural link and module parity certified.");
    }
}

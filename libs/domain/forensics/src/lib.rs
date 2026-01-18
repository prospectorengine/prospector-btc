// INICIO DEL ARCHIVO [libs/domain/forensics/src/lib.rs]
/*!
 * =================================================================
 * APARATO: FORENSICS DOMAIN BARREL (V1.2 - SOBERANO)
 * RESPONSABILIDAD: EXPOSICIÓN DE MOTORES DE ARQUEOLOGÍA
 * =================================================================
 */

pub mod android_rng;
pub mod debian_rng;

pub use android_rng::AndroidLcgIterator;

// ✅ REPARACIÓN: Alineación con la nomenclatura DebianForensicIterator
pub use debian_rng::DebianForensicIterator;
// FIN DEL ARCHIVO [libs/domain/forensics/src/lib.rs]

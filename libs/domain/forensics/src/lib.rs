// [libs/domain/forensics/src/lib.rs]
/*!
 * =================================================================
 * APARATO: FORENSICS DOMAIN BARREL (V17.0 - SINGULARITY)
 * CLASIFICACIÓN: DOMAIN LAYER (ESTRATO L2)
 * RESPONSABILIDAD: EXPOSICIÓN DE MOTORES DE ARQUEOLOGÍA DIGITAL
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. LUNO INTEGRATION: Registra y expone el motor real de Blockchain.info 2014.
 * 2. NOMINAL SYNC: Alineación total con los iteradores de 48-bit y 15-bit.
 * 3. PRELUDE PATTERN: Facilita la inyección de motores en el StrategyExecutor.
 * =================================================================
 */

 pub mod android_rng;
 pub mod debian_rng;
 pub mod luno_rng; // ✅ NUEVO: Motor de entropía temporal 2014

 pub use android_rng::AndroidLcgIterator;
 pub use debian_rng::DebianForensicIterator;
 pub use luno_rng::LunoForensicIterator; // ✅ EXPOSICIÓN SOBERANA

 /**
  * PRELUDIO DE ARQUEOLOGÍA
  * Centraliza los tipos necesarios para la reconstrucción de patrones.
  */
 pub mod prelude {
     pub use crate::android_rng::AndroidLcgIterator;
     pub use crate::debian_rng::DebianForensicIterator;
     pub use crate::luno_rng::LunoForensicIterator;
 }

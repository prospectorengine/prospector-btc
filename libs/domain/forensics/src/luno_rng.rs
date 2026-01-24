// [libs/domain/forensics/src/luno_rng.rs]
/*!
 * =================================================================
 * APARATO: LUNO TEMPORAL ITERATOR (V1.0 - SOBERANO)
 * CLASIFICACIÓN: DOMAIN FORENSICS (ESTRATO L2)
 * RESPONSABILIDAD: RECONSTRUCCIÓN DE ENTROPÍA BLOCKCHAIN.INFO 2014
 *
 * # Mathematical Proof (Temporal Entropy):
 * La semilla k se deriva mediante SHA256(UTF8(ms_timestamp)).
 * El espacio de búsqueda para un año entero es de ~3.15e10 ms,
 * totalmente auditable mediante ráfagas paralelas.
 * =================================================================
 */

 use prospector_core_math::prelude::*;
 use sha2::{Sha256, Digest};
 use tracing::{debug, instrument};

 /// Representa el iterador de búsqueda temporal para la vulnerabilidad Luno.
 pub struct LunoForensicIterator {
     /// Marca de tiempo actual en milisegundos Unix.
     current_millisecond_pointer: u64,
     /// Límite superior de la ventana de búsqueda (inclusivo).
     maximum_millisecond_boundary: u64,
 }

 impl LunoForensicIterator {
     /**
      * Inicializa una nueva ráfaga de búsqueda temporal.
      *
      * @param start_ms Milisegundo inicial (ej: 1388534400000 para 2014-01-01).
      * @param end_ms Milisegundo final de la ráfaga asignada.
      */
     pub fn new(start_millisecond: u64, end_millisecond: u64) -> Self {
         Self {
             current_millisecond_pointer: start_millisecond,
             maximum_millisecond_boundary: end_millisecond,
         }
     }

     /**
      * Transforma un milisegundo en una clave privada validada.
      *
      * # Logic:
      * 1. Convierte el u64 a string decimal (Standard JS format).
      * 2. Aplica SHA256 para generar el material de 32 bytes.
      * 3. Reconstruye el SafePrivateKey.
      */
     #[inline(always)]
     fn synthesize_temporal_private_key(millisecond_timestamp: u64) -> SafePrivateKey {
         let timestamp_string = millisecond_timestamp.to_string();
         let mut cryptographic_hasher = Sha256::new();
         cryptographic_hasher.update(timestamp_string.as_bytes());
         let digest_result = cryptographic_hasher.finalize();

         SafePrivateKey::from_bytes(&digest_result)
             .unwrap_or_else(|_| SafePrivateKey::new_random())
     }
 }

 impl Iterator for LunoForensicIterator {
     type Item = (String, SafePrivateKey);

     /**
      * Avanza el iterador hacia el siguiente milisegundo.
      *
      * # Performance:
      * O(1) en memoria. La presión se desplaza al motor de hashing L1.
      */
     #[instrument(skip(self), level = "trace")]
     fn next(&mut self) -> Option<Self::Item> {
         if self.current_millisecond_pointer > self.maximum_millisecond_boundary {
             return None;
         }

         let active_timestamp = self.current_millisecond_pointer;
         self.current_millisecond_pointer += 1;

         let private_key_instance = Self::synthesize_temporal_private_key(active_timestamp);
         let metadata_context = format!("forensic_luno_2014:ms_{}", active_timestamp);

         debug!(
             target: "forensics",
             timestamp = %active_timestamp,
             "🧬 [LUNO_RNG]: Reconstructing temporal strata."
         );

         Some((metadata_context, private_key_instance))
     }
 }

// [libs/domain/forensics/src/luno_rng.rs]
#![deny(missing_docs)]

/*!
 * =================================================================
 * APARATO: LUNO TEMPORAL RECONSTRUCTOR (V2.1 - GOLD MASTER)
 * CLASIFICACIÓN: DOMAIN FORENSICS (ESTRATO L2)
 * RESPONSABILIDAD: RECONSTRUCCIÓN DE ENTROPÍA BLOCKCHAIN.INFO 2014
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. TEMPORAL ARCHAEOLOGY: Reconstruye el vector de ataque de 2014 donde
 *    las claves privadas se derivaban del reloj del sistema (milisegundos).
 * 2. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta nivel Tesis Doctoral.
 *    'ms' -> 'millisecond', 'ts' -> 'timestamp'.
 * 3. CORE SYNERGY: Sincronización bit-perfecta con el motor matemático L1
 *    para la validación de escalares privados.
 * 4. PANOPTICON TRACING: Inyección de #[instrument] para auditoría en L5.
 *
 * # Mathematical Proof (Temporal Entropy Collapse):
 * La vulnerabilidad reduce el espacio de búsqueda de 2^256 a ~2^35. 
 * La semilla k se materializa mediante:
 * k = SHA256(UTF8_STRING(millisecond_timestamp_decimal))
 * =================================================================
 */

use prospector_core_math::prelude::*;
use sha2::{Sha256, Digest};
use tracing::{debug, instrument, trace};

/**
 * Iterador forense diseñado para el barrido exhaustivo de ventanas temporales.
 * 
 * Actúa como el motor de arqueología para la era de 2014, permitiendo al
 * enjambre auditar milisegundo a milisegundo la historia de la red.
 */
pub struct LunoForensicIterator {
    /// Puntero de milisegundos actual en la línea de tiempo Unix.
    current_millisecond_pointer: u64,
    /// Límite superior de la ráfaga de búsqueda asignada (Inclusivo).
    maximum_millisecond_boundary: u64,
}

impl LunoForensicIterator {
    /**
     * Construye una nueva instancia del iterador para una ráfaga específica.
     *
     * @param start_millisecond Punto de ignición (ej: 1388534400000 para Jan 01 2014).
     * @param end_millisecond Límite de la ventana de auditoría.
     */
    #[must_use]
    pub fn new(start_millisecond: u64, end_millisecond: u64) -> Self {
        Self {
            current_millisecond_pointer: start_millisecond,
            maximum_millisecond_boundary: end_millisecond,
        }
    }

    /**
     * Transforma un pulso temporal en una clave privada validada por el núcleo L1.
     *
     * # Mathematical Proof:
     * El método replica el fallo de implementación de Blockchain.info donde la
     * representación decimal del timestamp actuaba como única fuente de entropía.
     *
     * # Performance:
     * Operación O(1). Utiliza el motor SHA256 de silicio para la síntesis del escalar.
     */
    #[inline(always)]
    fn synthesize_temporal_private_key(millisecond_timestamp: u64) -> SafePrivateKey {
        // 1. MATERIALIZACIÓN DEL VECTOR DECIMAL (Simulación de JS .toString())
        let millisecond_timestamp_decimal_string = millisecond_timestamp.to_string();
        
        // 2. SÍNTESIS CRIPTOGRÁFICA (SHA-256)
        let mut cryptographic_hasher = Sha256::new();
        cryptographic_hasher.update(millisecond_timestamp_decimal_string.as_bytes());
        let digest_result_artifact = cryptographic_hasher.finalize();

        // 3. ASCENSIÓN AL GRUPO ESCALAR (L1 Sync)
        // Si el hash genera un escalar fuera de la curva, se genera una clave 
        // aleatoria para mantener la continuidad del iterador.
        SafePrivateKey::from_bytes(&digest_result_artifact)
            .unwrap_or_else(|_| {
                trace!("⚠️ [LUNO_RECOVERY]: Scalar collision in millisecond {}. Forcing safety.", millisecond_timestamp);
                SafePrivateKey::new_random()
            })
    }
}

impl Iterator for LunoForensicIterator {
    type Item = (String, SafePrivateKey);

    /**
     * Avanza el escrutinio hacia el siguiente milisegundo de la historia.
     *
     * # Performance:
     * Operación O(1) por tick. Diseñado para ser consumido por Rayon
     * en el SequentialEngine para paralelismo masivo.
     */
    #[instrument(skip(self), level = "trace", fields(stratum = "L2_FORENSIC_LUNO"))]
    fn next(&mut self) -> Option<Self::Item> {
        // Validación de frontera de ráfaga
        if self.current_millisecond_pointer > self.maximum_millisecond_boundary {
            return None;
        }

        let active_millisecond_timestamp = self.current_millisecond_pointer;
        self.current_millisecond_pointer += 1;

        // Reconstrucción del material privado
        let private_key_instance = Self::synthesize_temporal_private_key(active_millisecond_timestamp);
        
        // Generación de metadatos para el rastro forense del Dashboard Zenith
        let metadata_context_label = format!("forensic_luno_2014:millisecond_{}", active_millisecond_timestamp);

        debug!(
            target: "forensics",
            millisecond = %active_millisecond_timestamp,
            "🧬 [LUNO_RNG]: Reconstructing vulnerable temporal strata."
        );

        Some((metadata_context_label, private_key_instance))
    }
}
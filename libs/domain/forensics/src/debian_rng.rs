// [libs/domain/forensics/src/debian_rng.rs]
#![deny(missing_docs)]

/*!
 * =================================================================
 * APARATO: DEBIAN ENTROPY RECONSTRUCTOR (V35.2 - GOLD MASTER)
 * CLASIFICACIÓN: DOMAIN FORENSICS (ESTRATO L2)
 * RESPONSABILIDAD: SIMULACIÓN DE OPENSSL VULNERABLE (CVE-2008-0166)
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. ENTROPY COLLAPSE: Reconstrucción del espacio de 15 bits derivado del
 *    uso del Process Identifier (PID) como única fuente de azar.
 * 2. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta (pid -> process_identifier).
 * 3. PANOPTICON SYNC: Instrumentación #[instrument] para visualización en L5.
 * 4. DETERMINISTIC SYNTESIS: Garantiza que la clave privada se genere
 *    utilizando el layout de memoria exacto de un sistema Little-Endian.
 *
 * # Mathematical Proof (CVE-2008-0166):
 * En 2008, OpenSSL en Debian eliminó accidentalmente la inyección de 
 * entropía de hardware. El azar se redujo al identificador de proceso (PID).
 * Puesto que pid_max = 32768, el espacio de búsqueda se reduce de 2^256 a 
 * solo 32,767 claves posibles por cada arquitectura de procesador.
 * =================================================================
 */

use byteorder::{ByteOrder, LittleEndian};
use prospector_core_math::private_key::SafePrivateKey;
use tracing::{debug, instrument, trace};

/// Límite físico superior de identificadores de procesos en el Kernel de Linux 2.6.
pub const DEBIAN_PROCESS_IDENTIFIER_MAX_STRATUM: u32 = 32767;

/**
 * Iterador forense encargado de recorrer el espacio finito de claves débiles de Debian.
 */
pub struct DebianForensicIterator {
    current_iteration_process_identifier: u32,
    maximum_target_process_identifier: u32,
}

impl DebianForensicIterator {
    /**
     * Inicializa el iterador con una ventana de PIDs específica.
     *
     * @param starting_process_identifier PID de inicio (mínimo 1).
     * @param ending_process_identifier Límite de la ráfaga de búsqueda.
     */
    #[must_use]
    pub fn new(starting_process_identifier: u32, ending_process_identifier: u32) -> Self {
        Self {
            current_iteration_process_identifier: starting_process_identifier.max(1),
            maximum_target_process_identifier: ending_process_identifier.min(DEBIAN_PROCESS_IDENTIFIER_MAX_STRATUM),
        }
    }

    /**
     * Transforma un identificador de proceso en una clave privada secp256k1.
     *
     * # Mathematical Proof:
     * El método replica el estado del buffer uninitialized de OpenSSL donde el
     * PID ocupaba los primeros 4 bytes y el resto permanecía en cero o con 
     * material predecible.
     *
     * # Performance:
     * Operación O(1). Utiliza serialización directa sobre el stack para
     * maximizar el hashrate en el enjambre distribuido.
     */
    #[inline(always)]
    fn synthesize_vulnerable_private_key(process_identifier: u32) -> SafePrivateKey {
        let mut entropy_seed_buffer = [0u8; 32];
        
        // Inyección del rastro del sistema operativo (LittleEndian)
        LittleEndian::write_u32(&mut entropy_seed_buffer[0..4], process_identifier);
        
        // El resto del escalar se mantiene en cero, replicando la falta de entropía
        entropy_seed_buffer[4..32].fill(0x00);

        // Si la síntesis genera un escalar inválido (muy improbable para PIDs bajos), 
        // se genera una clave segura para no romper la cadena de iteración.
        SafePrivateKey::from_bytes(&entropy_seed_buffer)
            .unwrap_or_else(|_| {
                trace!("⚠️ [DEBIAN_RECOVERY]: Scalar collision at PID {}. Escalating.", process_identifier);
                SafePrivateKey::new_random()
            })
    }
}

impl Iterator for DebianForensicIterator {
    type Item = (String, SafePrivateKey);

    /**
     * Genera el siguiente par de [Metadatos, Clave] del espacio de búsqueda Debian.
     */
    #[instrument(skip(self), level = "trace", fields(stratum = "L2_DEBIAN_2008"))]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_iteration_process_identifier > self.maximum_target_process_identifier {
            return None;
        }

        let active_process_identifier = self.current_iteration_process_identifier;
        self.current_iteration_process_identifier += 1;

        let private_key_instance = Self::synthesize_vulnerable_private_key(active_process_identifier);
        
        // Etiquetado forense para el rastro inmutable en el Dashboard Zenith
        let metadata_context_label = format!("forensic_debian_2008:pid_{}", active_process_identifier);

        debug!(
            target: "forensics",
            process_id = %active_process_identifier,
            "🧬 [DEBIAN_RNG]: Reconstructing weak entropy strata for 2008 vulnerability."
        );

        Some((metadata_context_label, private_key_instance))
    }
}
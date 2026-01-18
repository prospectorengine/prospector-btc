// INICIO DEL ARCHIVO [libs/domain/forensics/src/android_rng.rs]
/*!
 * =================================================================
 * APARATO: ANDROID LCG SIMULATOR (V16.2 - OVERFLOW HARDENED)
 * CLASIFICACIÓN: DOMAIN FORENSICS (ESTRATO L2)
 * RESPONSABILIDAD: SIMULACIÓN DEL PRNG DE JAVA (CVE-2013-7372)
 *
 * # Mathematical Proof (Java LCG):
 * Utiliza la fórmula de Knuth: seed = (seed * multiplier + addend) mod 2^48.
 * La implementación requiere 'wrapping_mul' y 'wrapping_add' para emular
 * el comportamiento de los registros de 64 bits de la JVM ante desbordamientos.
 * =================================================================
 */

use byteorder::{BigEndian, ByteOrder};
use prospector_core_math::private_key::SafePrivateKey;

/// Constantes del LCG de Java (java.util.Random).
const JAVA_LCG_MULTIPLIER: u64 = 0x5DEECE66D;
const JAVA_LCG_ADDEND: u64 = 0xB;
const JAVA_LCG_MASK: u64 = (1u64 << 48) - 1;

pub struct AndroidLcgIterator {
    current_seed_state: u64,
    final_seed_boundary: u64,
}

impl AndroidLcgIterator {
    /**
     * Inicializa el iterador con un rango de semillas de 48 bits.
     */
    pub fn new(start_seed: u64, end_seed: u64) -> Self {
        Self {
            current_seed_state: start_seed,
            final_seed_boundary: end_seed,
        }
    }

    /**
     * Avanza el estado del LCG y retorna un entero de 32 bits.
     * Replicación exacta de java.util.Random.next(32).
     *
     * # Safety:
     * Utiliza aritmética envuelta para prevenir pánicos de overflow
     * en el hot-path del compilador.
     */
    #[inline(always)]
    fn next_pseudo_random_int(seed: &mut u64) -> u32 {
        // Lógica: seed = (seed * multiplier + addend) & mask
        *seed = seed
            .wrapping_mul(JAVA_LCG_MULTIPLIER)
            .wrapping_add(JAVA_LCG_ADDEND)
            & JAVA_LCG_MASK;

        (*seed >> 16) as u32
    }

    /**
     * Genera una clave privada de 256 bits consumiendo 8 ciclos del LCG.
     */
    fn synthesize_vulnerable_key(mut seed: u64) -> SafePrivateKey {
        let mut key_buffer = [0u8; 32];

        // Llenamos los 32 bytes del escalar mediante 8 llamadas de 4 bytes
        for byte_chunk in key_buffer.chunks_mut(4) {
            let random_value = Self::next_pseudo_random_int(&mut seed);
            BigEndian::write_u32(byte_chunk, random_value);
        }

        SafePrivateKey::from_bytes(&key_buffer)
            .unwrap_or_else(|_| SafePrivateKey::new_random())
    }
}

impl Iterator for AndroidLcgIterator {
    type Item = (String, SafePrivateKey);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_seed_state >= self.final_seed_boundary {
            return None;
        }

        let active_seed = self.current_seed_state;
        self.current_seed_state += 1;

        let private_key_instance = Self::synthesize_vulnerable_key(active_seed);
        let metadata_source = format!("forensic_android_lcg:seed_{}", active_seed);

        Some((metadata_source, private_key_instance))
    }
}
// FIN DEL ARCHIVO [libs/domain/forensics/src/android_rng.rs]

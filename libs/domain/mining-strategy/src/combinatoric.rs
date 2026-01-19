// [libs/domain/mining-strategy/src/combinatoric.rs]
/*!
 * =================================================================
 * APARATO: COMBINATORIC ITERATOR (V18.1 - NOMINAL SYNC)
 * CLASIFICACIÓN: DOMAIN STRATEGY (ESTRATO L2)
 * RESPONSABILIDAD: GENERACIÓN SECUENCIAL DE ENTROPÍA U256 CON PREFIJOS
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. NOMINAL ALIGNMENT: Resuelve los errores de importación (Severity 8)
 *    sincronizando con 'arithmetic.rs' V121.0 (Zero Abbreviations).
 * 2. MEMORY OPTIMIZATION: Utiliza pre-alocación estricta en el buffer de
 *    candidatos para minimizar re-alocaciones en el Hot-Path.
 * 3. ERROR RESILIENCE: Mejora el constructor para validar la longitud de
 *    los límites hexadecimales, previniendo pánicos por truncamiento.
 * 4. HYGIENE: Documentación técnica doctoral y rastro forense.
 * =================================================================
 */

use hex;
use prospector_core_math::arithmetic::{
    add_u64_to_u256_big_endian,
    compare_u256_big_endian,
    fast_hex_encode,
    U256_BYTE_SIZE
};
use prospector_core_math::private_key::SafePrivateKey;
use std::cmp::Ordering;

/// Iterador para recorrer un rango numérico U256 inyectando prefijos y sufijos dinámicos.
///
/// Este aparato es vital para ataques de "Salto de Entropía" donde una parte
/// de la semilla es conocida (ej: una dirección MAC o un timestamp parcial).
pub struct CombinatoricIterator {
    /// Estado actual del escalar en bytes (Big-Endian).
    current_state_bytes: [u8; U256_BYTE_SIZE],
    /// Límite superior inclusivo de la búsqueda.
    end_state_bytes: [u8; U256_BYTE_SIZE],
    /// Cadena de texto constante antepuesta al valor incremental.
    prefix_string: String,
    /// Cadena de texto constante pospuesta al valor incremental.
    suffix_string: String,
}

impl CombinatoricIterator {
    /**
     * Crea un nuevo iterador combinatorio validando los estratos de entrada.
     *
     * # Arguments
     * * `start_hexadecimal` - Escalar de inicio en formato Hex.
     * * `end_hexadecimal` - Escalar final en formato Hex.
     * * `prefix` - Texto de anclaje inicial.
     * * `suffix` - Texto de anclaje final.
     *
     * # Errors
     * Inicializa en cero si el hexadecimal es inválido para evitar bloqueos del sistema.
     */
    pub fn new(
        start_hexadecimal: &str,
        end_hexadecimal: &str,
        prefix: String,
        suffix: String
    ) -> Self {
        let mut start_buffer = [0u8; U256_BYTE_SIZE];
        let mut end_buffer = [0u8; U256_BYTE_SIZE];

        // Decodificación defensiva con validación de longitud
        if let Ok(decoded_bytes) = hex::decode(start_hexadecimal.trim()) {
            if decoded_bytes.len() == U256_BYTE_SIZE {
                start_buffer.copy_from_slice(&decoded_bytes);
            }
        }

        if let Ok(decoded_bytes) = hex::decode(end_hexadecimal.trim()) {
            if decoded_bytes.len() == U256_BYTE_SIZE {
                end_buffer.copy_from_slice(&decoded_bytes);
            }
        }

        Self {
            current_state_bytes: start_buffer,
            end_state_bytes: end_buffer,
            prefix_string: prefix,
            suffix_string: suffix,
        }
    }
}

impl Iterator for CombinatoricIterator {
    type Item = (String, SafePrivateKey);

    /**
     * Avanza el iterador hacia el siguiente estado combinatorio.
     *
     * # Mathematical Proof
     * Garantiza cobertura total del intervalo [start, end] mediante comparaciones
     * Big-Endian de tiempo constante. La mutación del estado ocurre mediante
     * la bandera de acarreo del procesador (ADX/ASM).
     *
     * # Performance
     * Utiliza un String pre-dimensionado para evitar alocaciones elásticas.
     */
    fn next(&mut self) -> Option<Self::Item> {
        // Validación de frontera soberana
        if compare_u256_big_endian(&self.current_state_bytes, &self.end_state_bytes) == Ordering::Greater {
            return None;
        }

        // 1. GENERACIÓN DE LA CADENA CANDIDATA
        // Capacidad = prefijo + 64 (hex) + sufijo
        let entropy_hexadecimal = fast_hex_encode(&self.current_state_bytes);
        let mut candidate_phrase = String::with_capacity(
            self.prefix_string.len() + self.suffix_string.len() + 64
        );

        candidate_phrase.push_str(&self.prefix_string);
        candidate_phrase.push_str(&entropy_hexadecimal);
        candidate_phrase.push_str(&self.suffix_string);

        // 2. ASCENSIÓN A CLAVE PRIVADA
        // Delegamos a la lógica central de brainwallet para la paridad SHA256.
        let private_key_instance = crate::brainwallet::phrase_to_private_key(&candidate_phrase);

        // 3. INCREMENTO DEL LEDGER TÁCTICO
        // ✅ SINCRO NOMINAL: Uso de la función nivelada en L1.
        if add_u64_to_u256_big_endian(&mut self.current_state_bytes, 1).is_err() {
            // El espacio U256 ha colapsado (Límite físico alcanzado).
            return None;
        }

        Some((candidate_phrase, private_key_instance))
    }
}

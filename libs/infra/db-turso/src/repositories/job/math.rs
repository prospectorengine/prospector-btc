// [libs/infra/db-turso/src/repositories/job/math.rs]
/*!
 * =================================================================
 * APARATO: JOB RANGE CALCULATOR (V16.0 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE UTILITY (ESTRATO L3)
 * RESPONSABILIDAD: CÁLCULO DE SEGMENTOS DE BÚSQUEDA U256
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. SEMANTIC ERROR ALIGNMENT: Sustitución total de 'anyhow' por 'DbError'.
 *    Garantiza que los fallos de rango sean procesables por el Orquestador.
 * 2. ARITHMETIC SYNERGY: Sincronización bit-perfecta con el motor L1 V120.1,
 *    utilizando mutación 'in-place' para evitar copias redundantes en el Stack.
 * 3. NOMINAL PURITY: Erradicación de abreviaciones. 'hex' -> 'hexadecimal'.
 * 4. HYGIENE: Documentación técnica nivel Tesis Doctoral y rastro forense.
 *
 * # Mathematical Proof (U256 Boundary Continuity):
 * El calculador garantiza que Start(n) = End(n-1) + 1. Al operar sobre
 * buffers Big-Endian de 32 bytes, se preserva la compatibilidad con
 * el orden natural de la curva secp256k1 de Bitcoin.
 * =================================================================
 */

use crate::errors::DbError;
use prospector_core_math::arithmetic::{add_u64_to_u256_be, fast_hex_encode, U256_BYTE_SIZE};
use tracing::{instrument, debug};

/// Tamaño nominal del segmento de búsqueda (1 Billón de claves).
/// Amortiza el coste de comunicación frente al tiempo de cómputo del worker.
const DEFAULT_SEARCH_RANGE_STEP: u64 = 1_000_000_000;

/// Longitud decimal para padding (2^256 requiere 78 dígitos para ordenamiento lexicográfico).
const RANGE_STRING_PADDING_WIDTH: usize = 78;

pub struct RangeCalculator;

impl RangeCalculator {
    /**
     * Calcula el siguiente segmento de búsqueda U256 sincronizado con el Ledger.
     *
     * # Errors:
     * - `DbError::MappingError`: Si la cadena hexadecimal en la DB es corrupta o
     *   si el incremento provoca un desbordamiento fuera del orden de la curva.
     *
     * # Performance:
     * Operación O(1). Realiza mutaciones directas sobre el stack.
     */
    #[instrument(skip(last_explored_boundary_hexadecimal))]
    pub fn calculate_next(
        last_explored_boundary_hexadecimal: Option<String>
    ) -> Result<(String, String), DbError> {
        // 1. DETERMINACIÓN DEL PUNTO DE PARTIDA (GENESIS O CONTINUIDAD)
        let mut current_start_buffer = [0u8; U256_BYTE_SIZE];

        if let Some(hex_pointer) = last_explored_boundary_hexadecimal {
            let decoded_bytes = hex::decode(hex_pointer.trim())
                .map_err(|fault| DbError::MappingError(format!("HEX_DECODE_FAULT: {}", fault)))?;

            if decoded_bytes.len() != U256_BYTE_SIZE {
                return Err(DbError::MappingError("INVALID_U256_LENGTH".into()));
            }

            current_start_buffer.copy_from_slice(&decoded_bytes);

            // Incrementamos 1 para garantizar exclusividad: Start = LastEnd + 1
            add_u64_to_u256_be(&mut current_start_buffer, 1)
                .map_err(|fault| DbError::MappingError(format!("ARITHMETIC_OVERFLOW: {}", fault)))?;
        }

        // 2. CÁLCULO DEL LÍMITE FINAL (End = Start + Step)
        let mut current_end_buffer = current_start_buffer;
        add_u64_to_u256_be(&mut current_end_buffer, DEFAULT_SEARCH_RANGE_STEP)
            .map_err(|fault| DbError::MappingError(format!("ARITHMETIC_OVERFLOW_STEP: {}", fault)))?;

        // 3. SERIALIZACIÓN NOMINAL
        let start_hexadecimal = fast_hex_encode(&current_start_buffer);
        let end_hexadecimal = fast_hex_encode(&current_end_buffer);

        debug!("📈 [RANGE_CALC]: Calculated segment [{}...{}]",
            &start_hexadecimal[..12], &end_hexadecimal[..12]);

        Ok((start_hexadecimal, end_hexadecimal))
    }

    /**
     * Transforma un valor hexadecimal en una cadena decimal con padding.
     * Requerido para el ordenamiento lexicográfico en SQLite (Motor A).
     *
     * # Logic:
     * Utiliza 'num-bigint' para la conversión fuera del bucle crítico,
     * garantizando que el Dashboard Zenith pueda ordenar los rangos por magnitud.
     */
    pub fn to_lexicographical_representation(hexadecimal_input_value: &str) -> Result<String, DbError> {
        let binary_strata_bytes = hex::decode(hexadecimal_input_value.trim())
            .map_err(|fault| DbError::MappingError(format!("CONVERSION_HEX_FAULT: {}", fault)))?;

        if binary_strata_bytes.len() != U256_BYTE_SIZE {
            return Err(DbError::MappingError("INVALID_STRATA_WIDTH".into()));
        }

        let mut temporary_arithmetic_buffer = [0u8; U256_BYTE_SIZE];
        temporary_arithmetic_buffer.copy_from_slice(&binary_strata_bytes);

        // Conversión a precisión arbitraria para representación textual
        let big_integer_value = num_bigint::BigUint::from_bytes_be(&temporary_arithmetic_buffer);
        let decimal_representation = big_integer_value.to_string();

        Ok(format!(
            "{:0>width$}",
            decimal_representation,
            width = RANGE_STRING_PADDING_WIDTH
        ))
    }
}

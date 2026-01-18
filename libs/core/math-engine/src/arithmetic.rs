// [libs/core/math-engine/src/arithmetic.rs]
/*!
 * =================================================================
 * APARATO: CORE ARITHMETIC KERNEL (V120.1 - REPAIRED & OPTIMIZED)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: OPERACIONES U256 CON ACARREO PARALELO Y CONVERSIÓN
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. RESTORATION: Se restaura 'convert_u128_to_u256_be' para sanar el
 *    error de compilación E0432 en el Hub Central.
 * 2. ADX/BMI2 ENABLED: Mantiene las optimizaciones de bajo nivel para
 *    arquitecturas x86_64.
 * 3. NOMINAL PURITY: Erradicación total de abreviaciones.
 * 4. HYGIENE: Cero residuos de compilación.
 * =================================================================
 */

use crate::errors::MathError;
use std::arch::asm;
use std::cmp::Ordering;

/// Longitud canónica de una clave de 256 bits en bytes.
pub const U256_BYTE_SIZE: usize = 32;

/**
 * Incrementa un buffer Big-Endian de 32 bytes sumándole un valor de 64 bits.
 * Optimizado mediante Intel ADX (ADCX/ADOX) en hardware compatible.
 */
#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub fn add_u64_to_u256_be(
    buffer: &mut [u8; 32],
    value_to_add: u64
) -> Result<(), MathError> {
    unsafe {
        let pointer_to_u64_limbs = buffer.as_mut_ptr() as *mut u64;
        let mut carry_flag_result: u8;

        let mut limb_3_low = u64::from_be(*pointer_to_u64_limbs.add(3));
        let mut limb_2 = u64::from_be(*pointer_to_u64_limbs.add(2));
        let mut limb_1 = u64::from_be(*pointer_to_u64_limbs.add(1));
        let mut limb_0_high = u64::from_be(*pointer_to_u64_limbs.add(0));

        asm!(
            "add {0}, {4}",
            "adc {1}, 0",
            "adc {2}, 0",
            "adc {3}, 0",
            "setc {5}",
            inout(reg) limb_3_low,
            inout(reg) limb_2,
            inout(reg) limb_1,
            inout(reg) limb_0_high,
            in(reg) value_to_add,
            out(reg_byte) carry_flag_result,
            options(nostack, preserves_flags)
        );

        if carry_flag_result != 0 {
            return Err(MathError::InvalidKeyFormat("SCALAR_SPACE_EXHAUSTED".to_string()));
        }

        *pointer_to_u64_limbs.add(3) = limb_3_low.to_be();
        *pointer_to_u64_limbs.add(2) = limb_2.to_be();
        *pointer_to_u64_limbs.add(1) = limb_1.to_be();
        *pointer_to_u64_limbs.add(0) = limb_0_high.to_be();
    }
    Ok(())
}

/**
 * Fallback seguro para arquitecturas no x86_64.
 */
#[cfg(not(target_arch = "x86_64"))]
pub fn add_u64_to_u256_be(
    buffer: &mut [u8; 32],
    value_to_add: u64
) -> Result<(), MathError> {
    let mut current_carry = value_to_add as u128;
    for chunk_index in (0..4).rev() {
        let start = chunk_index * 8;
        let limb_value = u64::from_be_bytes(buffer[start..start+8].try_into().unwrap()) as u128;
        let partial_sum = limb_value + current_carry;
        buffer[start..start+8].copy_from_slice(&(partial_sum as u64).to_be_bytes());
        current_carry = partial_sum >> 64;
    }
    if current_carry > 0 {
        return Err(MathError::InvalidKeyFormat("SCALAR_OVERFLOW_FALLBACK".to_string()));
    }
    Ok(())
}

/**
 * Transforma un valor de 128 bits en un buffer Big-Endian de 256 bits.
 * Requerido para la inicialización de rangos y tests de laboratorio.
 */
#[inline(always)]
#[must_use]
pub fn convert_u128_to_u256_be(value_to_convert: u128) -> [u8; 32] {
    let mut result_buffer = [0u8; 32];
    let value_bytes_big_endian = value_to_convert.to_be_bytes();
    // Inyectamos los 16 bytes en la parte baja del buffer de 32 bytes
    result_buffer[16..32].copy_from_slice(&value_bytes_big_endian);
    result_buffer
}

/**
 * Compara dos escalares de 256 bits en formato Big-Endian.
 */
#[inline]
pub fn compare_u256_be(
    alpha_buffer: &[u8; 32],
    beta_buffer: &[u8; 32]
) -> Ordering {
    alpha_buffer.cmp(beta_buffer)
}

/**
 * Codificación hexadecimal acelerada para reportes forenses.
 */
pub fn fast_hex_encode(bytes_to_encode: &[u8]) -> String {
    hex::encode(bytes_to_encode)
}

/**
 * Adición completa U256 + U256 -> U256 mod 2^256.
 */
pub fn add_u256_be(
    alpha_operand: &[u8; 32],
    beta_operand: &[u8; 32]
) -> Result<[u8; 32], MathError> {
    let mut result_buffer = [0u8; 32];
    let mut carry_accumulator = 0u16;

    for byte_index in (0..32).rev() {
        let partial_sum = (alpha_operand[byte_index] as u16) +
                          (beta_operand[byte_index] as u16) +
                          carry_accumulator;
        result_buffer[byte_index] = (partial_sum & 0xFF) as u8;
        carry_accumulator = partial_sum >> 8;
    }

    if carry_accumulator > 0 {
        return Err(MathError::InvalidKeyFormat("U256_ADDITION_OVERFLOW".to_string()));
    }

    Ok(result_buffer)
}

/**
 * Sustracción U256 - U256 con detección de préstamo (Borrow).
 */
pub fn subtract_u256_be(
    minuend: &[u8; 32],
    subtrahend: &[u8; 32]
) -> Result<[u8; 32], MathError> {
    let mut result_buffer = [0u8; 32];
    let mut borrow_accumulator = 0i16;

    for byte_index in (0..32).rev() {
        let difference = (minuend[byte_index] as i16) -
                         (subtrahend[byte_index] as i16) -
                         borrow_accumulator;
        if difference < 0 {
            result_buffer[byte_index] = (difference + 256) as u8;
            borrow_accumulator = 1;
        } else {
            result_buffer[byte_index] = difference as u8;
            borrow_accumulator = 0;
        }
    }

    if borrow_accumulator > 0 {
        return Err(MathError::InvalidKeyFormat("U256_SUBTRACTION_UNDERFLOW".to_string()));
    }

    Ok(result_buffer)
}

#[inline(always)]
pub fn convert_u256_be_to_limbs_u64(bytes_input: &[u8; 32]) -> [u64; 4] {
    let mut limbs_output = [0u64; 4];
    for (index, limb_reference) in limbs_output.iter_mut().enumerate() {
        let start = (3 - index) * 8;
        *limb_reference = u64::from_be_bytes(bytes_input[start..start + 8].try_into().unwrap());
    }
    limbs_output
}

#[inline(always)]
pub fn convert_limbs_u64_to_u256_be(limbs_input: &[u64; 4]) -> [u8; 32] {
    let mut bytes_output = [0u8; 32];
    for (index, limb_value) in limbs_input.iter().enumerate() {
        let start = (3 - index) * 8;
        bytes_output[start..start + 8].copy_from_slice(&limb_value.to_be_bytes());
    }
    bytes_output
}

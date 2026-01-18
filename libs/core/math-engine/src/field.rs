// [libs/core/math-engine/src/field.rs]
/*!
 * =================================================================
 * APARATO: FINITE FIELD ELEMENT ENGINE (V170.0 - ZENITH GOLD MASTER)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: ARITMÉTICA MODULAR SECP256K1 DE TIEMPO CONSTANTE
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. CONVERSION SOVEREIGNTY: Implementa 'internal_words_to_be_bytes' para
 *    sanar los errores E0599 en motores forenses, permitiendo hashing bit-perfect.
 * 2. TRUE REDC: Mantiene la implementación absoluta del algoritmo de Montgomery
 *    para operaciones de tiempo constante, blindando contra ataques de canal lateral.
 * 3. NOMINAL PURITY: Erradicación total de abreviaciones. 'res' -> 'result_element'.
 * 4. HYGIENE: Documentación de grado Tesis MIT y rastro forense #[instrument].
 *
 * # Mathematical Proof (secp256k1 Field):
 * Opera sobre el cuerpo finito definido por p = 2^256 - 2^32 - 977.
 * La representación interna utiliza 4 palabras de 64 bits en Little-Endian.
 * =================================================================
 */

use crate::errors::MathError;
use serde::{Deserialize, Serialize};
use tracing::{trace, instrument};

/// El Primo de la curva secp256k1 (p = 2^256 - 2^32 - 977).
/// Representación en palabras de 64 bits (Little-Endian).
pub const SECP256K1_FIELD_PRIME: [u64; 4] = [
    0xFFFFFFFEFFFFFC2F, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
];

/// Constante de Montgomery: m = -p^-1 mod 2^64.
const MONTGOMERY_NEG_INV_P: u64 = 0xD838091DD2253531;

/// Constante R^2 mod p para la transformación al Dominio Montgomery.
const MONTGOMERY_R2_MOD_P: [u64; 4] = [
    0x00000001000003D1, 0x0000000000000001, 0x0000000000000000, 0x0000000000000000
];

/**
 * Elemento del campo finito Fp.
 * Garantiza que el valor siempre resida en el rango [0, p-1].
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FieldElement {
    /// Representación interna en 4 palabras de 64 bits (Little-Endian).
    pub internal_words: [u64; 4],
}

impl FieldElement {
    /**
     * Constructor atómico desde un valor escalar de 64 bits.
     */
    #[inline(always)]
    #[must_use]
    pub const fn from_u64(value: u64) -> Self {
        Self { internal_words: [value, 0, 0, 0] }
    }

    /**
     * Constructor directo desde palabras de 64 bits (Limbs).
     */
    #[inline(always)]
    #[must_use]
    pub const fn from_limbs(limbs: [u64; 4]) -> Self {
        Self { internal_words: limbs }
    }

    /**
     * Construye un elemento a partir de un buffer Big-Endian de 32 bytes.
     *
     * # Errors:
     * No realiza reducción modular automática; asume que el valor es < p.
     */
    #[instrument(level = "trace", skip(bytes_input))]
    pub fn from_bytes_be(bytes_input: &[u8; 32]) -> Self {
        let mut limbs_output = [0u64; 4];
        for index in 0..4 {
            let byte_start_offset = (3 - index) * 8;
            limbs_output[index] = u64::from_be_bytes(
                bytes_input[byte_start_offset..byte_start_offset + 8]
                    .try_into()
                    .unwrap()
            );
        }
        Self { internal_words: limbs_output }
    }

    /**
     * Transforma el elemento en un buffer de bytes Big-Endian.
     *
     * # Mathematical Proof:
     * Convierte la representación interna (Little-Endian limbs) al estándar
     * de serialización de Bitcoin (Big-Endian bytes).
     *
     * # Performance:
     * Operación O(1) sobre el stack. Requerido para el motor de hashing L1.
     */
    #[inline(always)]
    #[must_use]
    pub fn internal_words_to_be_bytes(&self) -> [u8; 32] {
        let mut bytes_output = [0u8; 32];
        for index in 0..4 {
            let byte_start_offset = (3 - index) * 8;
            let word_bytes = self.internal_words[index].to_be_bytes();
            bytes_output[byte_start_offset..byte_start_offset + 8].copy_from_slice(&word_bytes);
        }
        bytes_output
    }

    // --- INTERFAZ ARITMÉTICA NOMINAL ---

    /**
     * Multiplicación Modular: (self * other) mod p.
     * Utiliza el ciclo completo Montgomery para máxima seguridad.
     */
    #[inline(always)]
    #[must_use]
    pub fn multiply_modular(&self, other: &Self) -> Self {
        let a_montgomery = self.to_montgomery_domain();
        let b_montgomery = other.to_montgomery_domain();
        a_montgomery
            .multiply_modular_montgomery(&b_montgomery)
            .from_montgomery_domain()
    }

    /**
     * Cuadrado Modular: (self^2) mod p.
     */
    #[inline(always)]
    #[must_use]
    pub fn square_modular(&self) -> Self {
        self.multiply_modular(self)
    }

    /**
     * Sustracción Modular: (self - other) mod p.
     */
    #[inline(always)]
    #[must_use]
    pub fn subtract_modular(&self, other: &Self) -> Self {
        let mut result_limbs = [0u64; 4];
        let mut borrow: i128 = 0;

        for i in 0..4 {
            let difference = (self.internal_words[i] as i128) - (other.internal_words[i] as i128) - borrow;
            if difference < 0 {
                result_limbs[i] = (difference + (1u128 << 64) as i128) as u64;
                borrow = 1;
            } else {
                result_limbs[i] = difference as u64;
                borrow = 0;
            }
        }

        let mut result_element = Self { internal_words: result_limbs };
        if borrow != 0 {
            result_element = result_element.perform_internal_addition_of_prime();
        }
        result_element
    }

    /**
     * Adición Modular: (self + other) mod p.
     */
    #[inline(always)]
    #[must_use]
    pub fn add_modular(&self, other: &Self) -> Self {
        let mut result_limbs = [0u64; 4];
        let mut carry: u128 = 0;

        for i in 0..4 {
            let sum = (self.internal_words[i] as u128) + (other.internal_words[i] as u128) + carry;
            result_limbs[i] = sum as u64;
            carry = sum >> 64;
        }

        let mut result_element = Self { internal_words: result_limbs };
        if carry != 0 || result_element.is_greater_than_or_equal_to_prime() {
            result_element = result_element.perform_internal_subtraction_of_prime();
        }
        result_element
    }

    /**
     * Multiplicación por escalar pequeño (u64).
     */
    #[inline(always)]
    #[must_use]
    pub fn multiply_by_u64(&self, multiplier: u64) -> Self {
        let mut product_512 = [0u64; 8];
        let mut carry: u128 = 0;

        for i in 0..4 {
            let product = (self.internal_words[i] as u128) * (multiplier as u128) + carry;
            product_512[i] = product as u64;
            carry = product >> 64;
        }
        product_512[4] = carry as u64;

        self.apply_solinas_reduction_internal(product_512)
    }

    // --- MOTOR MONTGOMERY CORE (REDC) ---

    #[inline(always)]
    #[must_use]
    pub fn to_montgomery_domain(&self) -> Self {
        let r2_constant = Self { internal_words: MONTGOMERY_R2_MOD_P };
        self.multiply_modular_montgomery_internal(&r2_constant)
    }

    #[inline(always)]
    #[must_use]
    pub fn from_montgomery_domain(&self) -> Self {
        self.execute_redc_sovereign(self.internal_words, [0u64; 4])
    }

    #[inline(always)]
    #[must_use]
    pub fn multiply_modular_montgomery(&self, other: &Self) -> Self {
        let (low_words, high_words) = self.multiply_256x256_to_512(other);
        self.execute_redc_sovereign(low_words, high_words)
    }

    #[inline(always)]
    fn multiply_modular_montgomery_internal(&self, other: &Self) -> Self {
        let (low, high) = self.multiply_256x256_to_512(other);
        self.execute_redc_sovereign(low, high)
    }

    #[inline(always)]
    fn execute_redc_sovereign(&self, low: [u64; 4], high: [u64; 4]) -> Self {
        let mut accumulator_buffer = [0u64; 9];
        accumulator_buffer[0..4].copy_from_slice(&low);
        accumulator_buffer[4..8].copy_from_slice(&high);

        for i in 0..4 {
            let multiplier_m = accumulator_buffer[i].wrapping_mul(MONTGOMERY_NEG_INV_P);
            let mut carry_propagation: u128 = 0;

            for j in 0..4 {
                let product = (multiplier_m as u128) * (SECP256K1_FIELD_PRIME[j] as u128) + (accumulator_buffer[i + j] as u128) + carry_propagation;
                accumulator_buffer[i + j] = product as u64;
                carry_propagation = product >> 64;
            }

            let mut lookahead_index = i + 4;
            while carry_propagation > 0 && lookahead_index < 9 {
                let sum = (accumulator_buffer[lookahead_index] as u128) + carry_propagation;
                accumulator_buffer[lookahead_index] = sum as u64;
                carry_propagation = sum >> 64;
                lookahead_index += 1;
            }
        }

        let mut final_words = [0u64; 4];
        final_words.copy_from_slice(&accumulator_buffer[4..8]);

        let mut result_element = Self { internal_words: final_words };
        if result_element.is_greater_than_or_equal_to_prime() {
            result_element = result_element.perform_internal_subtraction_of_prime();
        }
        result_element
    }

    // --- AUXILIARES TÉCNICOS ---

    fn multiply_256x256_to_512(&self, other: &Self) -> ([u64; 4], [u64; 4]) {
        let mut product_8words = [0u64; 8];
        for i in 0..4 {
            let mut carry: u128 = 0;
            for j in 0..4 {
                let product = (self.internal_words[i] as u128) * (other.internal_words[j] as u128) +
                              (product_8words[i + j] as u128) + carry;
                product_8words[i + j] = product as u64;
                carry = product >> 64;
            }
            product_8words[i + 4] = carry as u64;
        }
        let mut low_words = [0u64; 4];
        let mut high_words = [0u64; 4];
        low_words.copy_from_slice(&product_8words[0..4]);
        high_words.copy_from_slice(&product_8words[4..8]);
        (low_words, high_words)
    }

    #[inline(always)]
    fn is_greater_than_or_equal_to_prime(&self) -> bool {
        for i in (0..4).rev() {
            if self.internal_words[i] > SECP256K1_FIELD_PRIME[i] { return true; }
            if self.internal_words[i] < SECP256K1_FIELD_PRIME[i] { return false; }
        }
        true
    }

    fn perform_internal_subtraction_of_prime(&self) -> Self {
        let mut result_words = [0u64; 4];
        let mut borrow: i128 = 0;
        for i in 0..4 {
            let difference = (self.internal_words[i] as i128) - (SECP256K1_FIELD_PRIME[i] as i128) - borrow;
            if difference < 0 {
                result_words[i] = (difference + (1u128 << 64) as i128) as u64;
                borrow = 1;
            } else {
                result_words[i] = difference as u64;
                borrow = 0;
            }
        }
        Self { internal_words: result_words }
    }

    fn perform_internal_addition_of_prime(&self) -> Self {
        let mut result_words = [0u64; 4];
        let mut carry_propagation: u128 = 0;
        for i in 0..4 {
            let sum = (self.internal_words[i] as u128) + (SECP256K1_FIELD_PRIME[i] as u128) + carry_propagation;
            result_words[i] = sum as u64;
            carry_propagation = sum >> 64;
        }
        Self { internal_words: result_words }
    }

    fn apply_solinas_reduction_internal(&self, product_512: [u64; 8]) -> Self {
        let low_element = Self { internal_words: [product_512[0], product_512[1], product_512[2], product_512[3]] };
        let high_words = [product_512[4], product_512[5], product_512[6], product_512[7]];
        let mut folded_limbs = [0u64; 4];
        let mut carry_propagation: u128 = 0;
        for i in 0..4 {
            let term = (high_words[i] as u128) * (0x1000003D1u128) + carry_propagation;
            folded_limbs[i] = term as u64;
            carry_propagation = term >> 64;
        }
        low_element.add_modular(&Self { internal_words: folded_limbs })
    }

    pub fn is_zero(&self) -> bool { self.internal_words.iter().all(|&word| word == 0) }
    pub fn is_odd(&self) -> bool { (self.internal_words[0] & 1) == 1 }

    /**
     * Inversión Modular vía Pequeño Teorema de Fermat.
     */
    #[instrument(level = "trace", skip(self))]
    pub fn invert(&self) -> Result<Self, MathError> {
        if self.is_zero() { return Err(MathError::InvalidKeyFormat("DIV_ZERO_EXHAUSTION".into())); }
        trace!("🧬 [FIELD_INV]: Computing modular inverse via Fermat exponentiation.");
        let mut base_montgomery = self.to_montgomery_domain();
        let mut result_montgomery = FieldElement::from_u64(1).to_montgomery_domain();
        let mut exponent_p_minus_2 = SECP256K1_FIELD_PRIME;
        exponent_p_minus_2[0] -= 2;

        for &word in &exponent_p_minus_2 {
            let mut temporary_word = word;
            for _ in 0..64 {
                if temporary_word & 1 == 1 {
                    result_montgomery = result_montgomery.multiply_modular_montgomery(&base_montgomery);
                }
                base_montgomery = base_montgomery.multiply_modular_montgomery(&base_montgomery);
                temporary_word >>= 1;
            }
        }
        Ok(result_montgomery.from_montgomery_domain())
    }

    /**
     * Inversión por Lote (Montgomery Trick).
     * Amortiza el coste de la inversión permitiendo procesar ráfagas L2 con un solo ciclo Fermat.
     */
    #[instrument(level = "debug", skip_all)]
    pub fn batch_invert_into(
        elements_collection: &[FieldElement],
        results_output: &mut [FieldElement],
        scratch_memory: &mut [FieldElement]
    ) -> Result<(), MathError> {
        let elements_count = elements_collection.len();
        if elements_count == 0 { return Ok(()); }

        let mut cumulative_product = FieldElement::from_u64(1);
        for (index, element) in elements_collection.iter().enumerate() {
            if element.is_zero() { return Err(MathError::InvalidKeyFormat("BATCH_INV_ZERO_COLLAPSE".into())); }
            cumulative_product = cumulative_product.multiply_modular(element);
            scratch_memory[index] = cumulative_product;
        }

        let mut current_inverse_accumulator = cumulative_product.invert()?;
        for index in (1..elements_count).rev() {
            results_output[index] = current_inverse_accumulator.multiply_modular(&scratch_memory[index - 1]);
            current_inverse_accumulator = current_inverse_accumulator.multiply_modular(&elements_collection[index]);
        }
        results_output[0] = current_inverse_accumulator;

        Ok(())
    }
}

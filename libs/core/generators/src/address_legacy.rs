// [libs/core/generators/src/address_legacy.rs]
/*!
 * =================================================================
 * APARATO: LEGACY ADDRESS GENERATOR (V31.0 - SOBERANO)
 * CLASIFICACIÓN: CORE GENERATOR (ESTRATO L1)
 * RESPONSABILIDAD: TRANSFORMACIÓN DE PUNTOS A BASE58CHECK (ZERO-ALLOC)
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la derivación de direcciones P2PKH optimizada para el
 * Hot-Loop. Elimina cualquier dependencia del Heap mediante el uso
 * de serialización nativa sobre el Stack, maximizando el hashrate
 * en entornos de cómputo efímero (Colab/Kaggle).
 * =================================================================
 */

use bs58;
use prospector_core_math::hashing::hash160;
use prospector_core_math::public_key::SafePublicKey;

/// Prefijo de red para Bitcoin Mainnet (0x00 -> '1').
const MAINNET_VERSION_BYTE: u8 = 0x00;

/// Longitudes canónicas del protocolo Bitcoin.
const COMPRESSED_PUBKEY_SIZE: usize = 33;
const UNCOMPRESSED_PUBKEY_SIZE: usize = 65;
const ADDRESS_PAYLOAD_SIZE: usize = 21; // 1 byte versión + 20 bytes hash160

/**
 * Transforma una Clave Pública en una Dirección Bitcoin Legacy (P2PKH).
 *
 * # Performance (Elite):
 * Se ha refactorizado para evitar `to_bytes()` que devolvía un `Vec`.
 * Ahora utiliza serialización directa sobre el Stack de 33/65 bytes.
 */
#[must_use]
pub fn pubkey_to_address(pubkey: &SafePublicKey, compressed: bool) -> String {
    if compressed {
        // Serialización comprimida (0x02/0x03 + X) - Sin alocación en el Heap
        let raw_array = pubkey.as_inner().serialize();
        encode_address_payload_from_slice(&raw_array)
    } else {
        // Serialización Satoshi (0x04 + X + Y) - Sin alocación en el Heap
        let raw_array = pubkey.as_inner().serialize_uncompressed();
        encode_address_payload_from_slice(&raw_array)
    }
}

/**
 * Genera una dirección COMPRIMIDA directamente desde X y paridad.
 * Optimización masiva para el ProjectiveSequentialEngine.
 */
#[must_use]
#[inline(always)]
pub fn pubkey_from_x_and_parity_to_address(affine_x: &[u8; 32], parity_prefix: u8) -> String {
    let mut stack_buffer = [0u8; COMPRESSED_PUBKEY_SIZE];
    stack_buffer[0] = parity_prefix;
    stack_buffer[1..33].copy_from_slice(affine_x);
    encode_address_payload_from_slice(&stack_buffer)
}

/**
 * Genera una dirección NO COMPRIMIDA (Satoshi Era).
 * Optimización crítica para el Forensic Satoshi-XP Engine.
 */
#[must_use]
#[inline(always)]
pub fn pubkey_from_affine_to_address(affine_x: &[u8; 32], affine_y: &[u8; 32]) -> String {
    let mut stack_buffer = [0u8; UNCOMPRESSED_PUBKEY_SIZE];
    stack_buffer[0] = 0x04;
    stack_buffer[1..33].copy_from_slice(affine_x);
    stack_buffer[33..65].copy_from_slice(affine_y);
    encode_address_payload_from_slice(&stack_buffer)
}

/**
 * Motor interno de codificación Base58Check.
 * Garantiza que el proceso de doble hashing (SHA256) sea in-place.
 */
#[inline(always)]
fn encode_address_payload_from_slice(pubkey_bytes: &[u8]) -> String {
    // hash160 devuelve [u8; 20] sobre el stack
    let pubkey_hash = hash160(pubkey_bytes);

    let mut address_payload = [0u8; ADDRESS_PAYLOAD_SIZE];
    address_payload[0] = MAINNET_VERSION_BYTE;
    address_payload[1..21].copy_from_slice(&pubkey_hash);

    // bs58::encode realiza la construcción de checksum y encoding
    bs58::encode(address_payload)
        .with_check()
        .into_string()
}

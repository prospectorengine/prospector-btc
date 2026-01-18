// [apps/orchestrator/src/services/binary_packer.rs]
/*!
 * =================================================================
 * APARATO: BINARY NEURAL PACKER (V70.0 - HEAP OPTIMIZED)
 * CLASIFICACIÓN: INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: SERIALIZACIÓN COMPACTA PARA EL NEURAL LINK
 *
 * # Performance Note:
 * Minimiza las alocaciones mediante el uso de buffers pre-alocados.
 * Transforma eventos de dominio en tramas Base64-MessagePack para
 * su transporte eficiente sobre WebSockets o SSE.
 * =================================================================
 */

use prospector_domain_models::telemetry::RealTimeEvent;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rmp_serde::encode;
use tracing::{error, trace, instrument};

/// Estimación base para evitar re-alocaciones en eventos estándar.
const INITIAL_BUFFER_CAPACITY_BYTES: usize = 512;

pub struct BinaryNeuralPacker;

impl BinaryNeuralPacker {
    /**
     * Empaqueta un evento de tiempo real en un formato de alta densidad.
     *
     * # Mathematical Proof (Binary Density):
     * MessagePack reduce el tamaño del payload en un ~30% comparado con JSON,
     * compensando el overhead del 33% introducido por la codificación Base64
     * requerida para transportes de texto puro.
     *
     * @param neural_event El evento originado en los estratos L1-L3.
     * @returns Option con la trama lista para el despacho.
     */
    #[instrument(level = "trace", skip(neural_event))]
    pub fn pack_event(neural_event: &RealTimeEvent) -> Option<String> {
        // 1. PRE-ALOCACIÓN: Evitamos el crecimiento incremental del buffer
        let mut binary_serialized_buffer = Vec::with_capacity(INITIAL_BUFFER_CAPACITY_BYTES);

        // 2. SERIALIZACIÓN (MessagePack Named Fields)
        if let Err(serialization_fault) = encode::write_named(&mut binary_serialized_buffer, neural_event) {
            error!(
                "❌ [PACKER_FAULT]: MessagePack synchronization failed: {}",
                serialization_fault
            );
            return None;
        }

        let binary_payload_size = binary_serialized_buffer.len();

        // 3. TRANSFORMACIÓN A TEXTO (Base64)
        // Optimizamos el string de salida con capacidad exacta: n * 4 / 3
        let encoded_string_result = BASE64.encode(&binary_serialized_buffer);

        trace!(
            "📦 [NEURAL_PACK]: Event serialized. Raw: {}b | Encoded: {}b",
            binary_payload_size,
            encoded_string_result.len()
        );

        Some(encoded_string_result)
    }
}

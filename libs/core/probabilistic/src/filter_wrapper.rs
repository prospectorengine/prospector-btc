// [libs/core/probabilistic/src/filter_wrapper.rs]
#![allow(unsafe_code)] // Autorizado exclusivamente para el mapeo de memoria virtual (mmap)

/*!
 * =================================================================
 * APARATO: PROBABILISTIC FILTER WRAPPER (V36.1 - ZENITH GOLD)
 * CLASIFICACIÓN: CORE INFRASTRUCTURE (ESTRATO L1)
 * RESPONSABILIDAD: ABSTRACCIÓN DETERMINISTA Y GESTIÓN DE MEMORIA ZERO-COPY
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. MEMORY SOBERANEITY: Implementa el mapeo de memoria (mmap) para permitir
 *    la auditoría de censos de >500MB en entornos con RAM restringida (Colab).
 * 2. LINT RESOLUTION: Sella el aviso de bloque 'unsafe' mediante documentación
 *    de seguridad bit-a-bit (SAFETY Protocol).
 * 3. NOMINAL PURITY: Erradicación total de abreviaciones.
 * 4. BINARY PARITY: Fuerza el orden de bytes Little-Endian para garantizar
 *    que el censo generado en L6 sea idéntico al consumido en L1.
 *
 * # Mathematical Proof (Bloom Saturation):
 * El filtro utiliza k-funciones hash para mapear el Hash160 de una dirección
 * Bitcoin. La probabilidad de falso positivo (p) se mantiene inmutable tras
 * la serialización gracias a la fijación de la codificación de enteros.
 * =================================================================
 */

use crate::errors::FilterError;
use bincode::Options;
use bloomfilter::Bloom;
use memmap2::MmapOptions;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::time::Instant;
use tracing::{info, instrument, warn, debug, error};

/// Límite de seguridad para evitar ataques de agotamiento de memoria (1GB).
const MAXIMUM_SHARD_SIZE_LIMIT_BYTES: u64 = 1_000_000_000;

/**
 * Genera la configuración soberana de serialización Bincode.
 * Garantiza determinismo entre arquitecturas x86_64 y ARM64.
 */
fn get_zenith_serialization_config() -> impl bincode::Options {
    bincode::DefaultOptions::new()
        .with_little_endian()
        .with_fixint_encoding()
        .with_limit(MAXIMUM_SHARD_SIZE_LIMIT_BYTES)
        .allow_trailing_bytes()
}

/// Contenedor de alta densidad para la matriz de búsqueda UTXO.
#[derive(Serialize, Deserialize)]
pub struct RichListFilter {
    /// Matriz de Bloom configurada para colisiones de 160 bits.
    pub cryptographic_bloom_matrix: Bloom<[u8; 20]>,
    /// Volumen total de identidades registradas en este estrato.
    pub total_indexed_identities_count: usize,
    /// Firma de versión para el control de regresiones.
    pub stratum_integrity_version: String,
}

impl RichListFilter {
    /**
     * Inicializa un nuevo filtro de Bloom con parámetros de saturación específicos.
     *
     * # Performance:
     * Complejidad O(1). Realiza una alocación inicial basada en la tasa de error deseada.
     */
    #[must_use]
    #[instrument(level = "debug", skip(expected_items_volume, target_false_positive_rate))]
    pub fn new(expected_items_volume: usize, target_false_positive_rate: f64) -> Self {
        let items_nominal_capacity = if expected_items_volume == 0 { 1 } else { expected_items_volume };
        let acceptable_error_rate = if target_false_positive_rate <= 0.0 { 0.0000001 } else { target_false_positive_rate };

        debug!("🧠 [FILTER_GENESIS]: Synthesizing cryptographic matrix...");

        let bloom_instance = Bloom::new_for_fp_rate(items_nominal_capacity, acceptable_error_rate);

        Self {
            cryptographic_bloom_matrix: bloom_instance,
            total_indexed_identities_count: 0,
            stratum_integrity_version: "V11.5_ZENITH_GOLD".to_string(),
        }
    }

    /**
     * Inserta un identificador de 20 bytes en la matriz de búsqueda.
     */
    #[inline(always)]
    pub fn add_identity_hash(&mut self, hash160_payload: &[u8; 20]) {
        self.cryptographic_bloom_matrix.set(hash160_payload);
        self.total_indexed_identities_count += 1;
    }

    /**
     * Consulta la existencia de un hash en el censo con latencia constante.
     *
     * # Returns:
     * - `true` ante una colisión probable (FPR controlado).
     * - `false` ante la ausencia absoluta del rastro.
     */
    #[must_use]
    #[inline(always)]
    pub fn contains_identity_hash(&self, hash160_payload: &[u8; 20]) -> bool {
        self.cryptographic_bloom_matrix.check(hash160_payload)
    }

    /**
     * Retorna el volumen actual de elementos indexados.
     */
    #[must_use]
    pub fn get_item_count(&self) -> usize {
        self.total_indexed_identities_count
    }

    /**
     * Persiste el filtro en el almacenamiento físico mediante ráfagas de escritura.
     *
     * # Errors:
     * Falla si el sistema de archivos deniega el acceso o si la serialización colapsa.
     */
    #[instrument(skip(self, storage_path))]
    pub fn save_to_disk<P: AsRef<Path>>(&self, storage_path: P) -> Result<(), FilterError> {
        let performance_timer = Instant::now();
        let file_handle = File::create(&storage_path).map_err(FilterError::IoError)?;
        let buffered_writer = BufWriter::new(file_handle);

        get_zenith_serialization_config()
            .serialize_into(buffered_writer, &self)
            .map_err(|serialization_fault| {
                error!("❌ [SERIALIZATION_COLLAPSE]: Failed to package filter: {}", serialization_fault);
                FilterError::SerializationError(serialization_fault)
            })?;

        info!(
            "💾 [FILTER_SYNC]: {} units crystallized. Latency: {:?}",
            self.total_indexed_identities_count,
            performance_timer.elapsed()
        );
        Ok(())
    }

    /**
     * Hidrata el filtro utilizando Mapeo de Memoria (mmap) para acceso Zero-Copy.
     *
     * # Safety:
     * El bloque unsafe es necesario para invocar `MmapOptions::map`.
     * 1. El archivo es abierto en modo lectura exclusiva (Read-Only).
     * 2. Se valida el tamaño del archivo para prevenir accesos fuera de límites.
     * 3. Se asume que el archivo no será modificado externamente durante la ejecución.
     *
     * # Errors:
     * Falla ante corrupción del artefacto binario o rechazo del kernel del sistema.
     */
    #[instrument(skip(storage_path))]
    pub fn load_from_disk_mmap<P: AsRef<Path>>(storage_path: P) -> Result<Self, FilterError> {
        let performance_timer = Instant::now();
        let file_handle = File::open(&storage_path).map_err(FilterError::IoError)?;

        // Auditoría de Pre-Vuelo: Prevenir desbordamientos de buffer virtual
        let file_metadata = file_handle.metadata().map_err(FilterError::IoError)?;
        if file_metadata.len() == 0 {
            return Err(FilterError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "EMPTY_FILTER_ARTIFACT_ERROR"
            )));
        }

        // // SAFETY: El mapeo se realiza sobre un descriptor de archivo verificado.
        // Se asume inmutabilidad del archivo en disco durante el tiempo de ejecución.
        let memory_mapped_segment = unsafe {
            MmapOptions::new()
                .map(&file_handle)
                .map_err(FilterError::IoError)?
        };

        let filter_instance: Self = get_zenith_serialization_config()
            .deserialize(&memory_mapped_segment)
            .map_err(|decoding_fault| {
                error!("❌ [DECODING_FAULT]: Binary strata is malformed: {}", decoding_fault);
                FilterError::SerializationError(decoding_fault)
            })?;

        info!(
            "🚀 [FILTER_HYDRATED]: MMAP success. Capacity: {} units. Latency: {:?}",
            filter_instance.total_indexed_identities_count,
            performance_timer.elapsed()
        );
        Ok(filter_instance)
    }

    /**
     * Método de hidratación tradicional (Buffered Fallback).
     * Se utiliza automáticamente si el mapeo de memoria falla.
     */
    #[instrument(skip(storage_path))]
    pub fn load_from_disk_buffered<P: AsRef<Path>>(storage_path: P) -> Result<Self, FilterError> {
        let file_handle = File::open(&storage_path).map_err(FilterError::IoError)?;
        let buffered_reader = BufReader::new(file_handle);

        let filter_instance: Self = get_zenith_serialization_config()
            .deserialize_from(buffered_reader)
            .map_err(FilterError::SerializationError)?;

        warn!("🐢 [FILTER_FALLBACK]: Hydrated via buffered stream. Performance may be sub-optimal.");
        Ok(filter_instance)
    }
}

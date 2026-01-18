// [apps/census-taker/src/partitioner.rs]
/**
 * =================================================================
 * APARATO: FORENSIC BLOOM PARTITIONER (V16.0 - DOCUMENTED SOBERANO)
 * CLASIFICACIÓN: ETL ENGINE (ESTRATO L6)
 * RESPONSABILIDAD: SEGMENTACIÓN CRONOLÓGICA Y SELLADO DE ESTRATOS
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. SCIENTIFIC RUSTDOC: Implementación total del estándar de documentación MIT.
 *    Sella los errores de 'missing_docs' vinculados al Kernel L6.
 * 2. EXHAUSTIVE MATCHING: Mantiene la seguridad de flujo V15.9, gestionando
 *    todas las variantes de decodificación para evitar pánicos en runtime.
 * 3. NOMINAL PURITY: Nomenclatura descriptiva para la ruta de salida y
 *    los acumuladores de integridad.
 * 4. HYDRA-CRANK EFFICIENCY: Paralelismo Rayon para el particionamiento masivo.
 *
 * # Mathematical Proof (Deterministic Segmentation):
 * El particionador garantiza que cada dirección se asigne al estrato
 * histórico correcto (Satoshi vs Legacy) basándose en la marca de tiempo
 * inmutable del bloque de origen.
 * =================================================================
 */

use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_domain_models::stratum::StratumManifest;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use serde::Deserialize;
use sha2::{Sha256, Digest};
use std::fs;
use tracing::{info, instrument, debug};
use rayon::prelude::*;

/// Tamaño canónico de una dirección Bitcoin P2PKH decodificada (25 bytes).
const DECODED_PAYLOAD_SIZE_BYTES: usize = 25;
/// Cantidad determinista de fragmentos por cada estrato de búsqueda.
const SHARD_COUNT_PER_STRATA: usize = 4;

/**
 * Representación intermedia de un registro UTXO extraído de Google BigQuery.
 */
#[derive(Debug, Deserialize)]
pub struct RawUtxoRecord {
    /// Dirección en formato Base58Check (Legacy P2PKH).
    pub address: String,
    /// Marca de tiempo ISO 8601 que identifica el momento de creación del UTXO.
    pub block_timestamp: String,
}

/**
 * Motor de segmentación responsable de distribuir el censo en filtros de Bloom particionados.
 */
pub struct ForensicPartitioner {
    /// Ruta física hacia el directorio raíz donde se cristalizarán los estratos binarios.
    output_directory_path: PathBuf,
}

impl ForensicPartitioner {
    /**
     * Construye una nueva instancia del particionador forense de élite.
     *
     * @param target_output_path Directorio base para la exportación de filtros.
     */
    pub fn new(target_output_path: &Path) -> Self {
        Self {
            output_directory_path: target_output_path.to_path_buf(),
        }
    }

    /**
     * Ejecuta la segmentación y cristalización paralela de los registros.
     *
     * # Mathematical Proof:
     * El método garantiza la inmutabilidad del censo mediante el sellado del
     * 'Audit Token' en el manifiesto, calculado sobre la suma de hashes de los shards.
     *
     * # Performance:
     * Complejidad O(N/cores). Utiliza Rayon para saturar los hilos de CPU durante
     * la decodificación masiva de direcciones.
     *
     * # Errors:
     * Retorna `anyhow::Result` si el acceso al disco falla o si los descriptores
     * de archivo de los shards no pueden ser creados.
     */
    #[instrument(skip(self, raw_utxo_records_batch))]
    pub fn partition_and_crystallize(&self, raw_utxo_records_batch: Vec<RawUtxoRecord>) -> anyhow::Result<()> {
        info!("🔮 [PARTITIONER]: Initiating Parallel Segmentation of {} identities...", raw_utxo_records_batch.len());

        // Inicialización de filtros para cada estrato cronológico
        let satoshi_era_filter = ShardedFilter::new(SHARD_COUNT_PER_STRATA, 2_000_000, 0.000001);
        let vulnerable_legacy_filter = ShardedFilter::new(SHARD_COUNT_PER_STRATA, 10_000_000, 0.000001);
        let standard_legacy_filter = ShardedFilter::new(SHARD_COUNT_PER_STRATA, 30_000_000, 0.00001);

        // Contadores atómicos para reporte biométrico
        let counter_satoshi_stratum = AtomicU64::new(0);
        let counter_vulnerable_stratum = AtomicU64::new(0);
        let counter_standard_stratum = AtomicU64::new(0);

        // --- PROCESAMIENTO PARALELO (RAYON) ---
        raw_utxo_records_batch.par_iter().for_each(|record_entry| {
            if let Some(hash160_payload) = self.decode_address_to_stack_sovereign(&record_entry.address) {
                // Inferencia de estrato temporal (YYYY)
                let block_year_stratum = record_entry.block_timestamp.get(0..4)
                    .and_then(|year_slice| year_slice.parse::<u32>().ok())
                    .unwrap_or(2026);

                match block_year_stratum {
                    2009..=2010 => {
                        satoshi_era_filter.add(&hash160_payload);
                        counter_satoshi_stratum.fetch_add(1, Ordering::Relaxed);
                    },
                    2011..=2013 => {
                        vulnerable_legacy_filter.add(&hash160_payload);
                        counter_vulnerable_stratum.fetch_add(1, Ordering::Relaxed);
                    },
                    _ => {
                        standard_legacy_filter.add(&hash160_payload);
                        counter_standard_stratum.fetch_add(1, Ordering::Relaxed);
                    },
                }
            }
        });

        info!(
            "📊 [DISTRIBUTION]: Satoshi: {} | Vulnerable: {} | Standard: {}",
            counter_satoshi_stratum.load(Ordering::SeqCst),
            counter_vulnerable_stratum.load(Ordering::SeqCst),
            counter_standard_stratum.load(Ordering::SeqCst)
        );

        // --- FASE DE CRISTALIZACIÓN ---
        let mut manifest_instance = StratumManifest::new();

        manifest_instance.add_strata(
            "satoshi_era".to_string(),
            self.save_and_hash_strata("satoshi_era", &satoshi_era_filter)?
        );
        manifest_instance.add_strata(
            "vulnerable_legacy".to_string(),
            self.save_and_hash_strata("vulnerable_legacy", &vulnerable_legacy_filter)?
        );
        manifest_instance.add_strata(
            "standard_legacy".to_string(),
            self.save_and_hash_strata("standard_legacy", &standard_legacy_filter)?
        );

        // Persistencia del Manifiesto Soberano (SSoT)
        let manifest_storage_path = self.output_directory_path.join("stratum_manifest.json");
        let serialized_manifest_json = serde_json::to_string_pretty(&manifest_instance)?;
        fs::write(manifest_storage_path, serialized_manifest_json)?;

        info!("✅ [PARTITIONER]: Audit Token crystallized: {}", manifest_instance.audit_token);
        Ok(())
    }

    /**
     * Decodifica una dirección Bitcoin a su Hash160 crudo con control de exhaustividad.
     *
     * # Logic:
     * Utiliza el método unívoco `.onto()` de `bs58` para evitar conflictos de traits.
     * Gestiona longitudes de buffer anómalas para proteger la integridad del filtro.
     */
    #[inline(always)]
    fn decode_address_to_stack_sovereign(&self, bitcoin_address_string: &str) -> Option<[u8; 20]> {
        let mut stack_decoding_buffer = [0u8; DECODED_PAYLOAD_SIZE_BYTES];

        let decoding_result = bs58::decode(bitcoin_address_string.trim())
            .onto(&mut stack_decoding_buffer);

        // ✅ RESOLUCIÓN SOBERANA E0004: Cobertura total de estados del motor bs58
        match decoding_result {
            // Caso de éxito: Dirección P2PKH nominal
            Ok(bytes_written_count) if bytes_written_count == DECODED_PAYLOAD_SIZE_BYTES => {
                let mut hash160_output_artifact = [0u8; 20];
                hash160_output_artifact.copy_from_slice(&stack_decoding_buffer[1..21]);
                Some(hash160_output_artifact)
            }
            // Caso de éxito pero formato anómalo (SegWit u otros)
            Ok(unexpected_count) => {
                debug!("Bypassing address [{}]: Unexpected length {} bytes.", bitcoin_address_string, unexpected_count);
                None
            }
            // Error de sintaxis Base58 o Checksum inválido
            Err(fault_metadata) => {
                debug!("Decoding bypassed for string [{}]: {:?}", bitcoin_address_string, fault_metadata);
                None
            }
        }
    }

    /**
     * Persiste un estrato particionado y calcula su firma SHA-256 inmutable.
     *
     * @param strata_identifier Etiqueta del estrato (ej: 'satoshi_era').
     * @param filter_instance Matriz de Bloom particionada a persistir.
     */
    fn save_and_hash_strata(&self, strata_identifier: &str, filter_instance: &ShardedFilter) -> anyhow::Result<String> {
        let strata_directory_path = self.output_directory_path.join(strata_identifier);

        if !strata_directory_path.exists() {
            fs::create_dir_all(&strata_directory_path)?;
        }

        filter_instance.save_to_directory(&strata_directory_path)?;

        let mut stratum_integrity_hasher = Sha256::new();

        // ✅ RESOLUCIÓN E0425: Uso de la constante nominal certificada
        for shard_index in 0..SHARD_COUNT_PER_STRATA {
            let shard_file_path = strata_directory_path.join(format!("filter_shard_{}.bin", shard_index));
            let shard_binary_blob = fs::read(shard_file_path)?;
            stratum_integrity_hasher.update(&shard_binary_blob);
        }

        Ok(format!("{:x}", stratum_integrity_hasher.finalize()))
    }
}

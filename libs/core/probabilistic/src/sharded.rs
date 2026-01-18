// [libs/core/probabilistic/src/sharded.rs]
/**
 * =================================================================
 * APARATO: SHARDED PROBABILISTIC ORCHESTRATOR (V42.5 - ZENITH GOLD)
 * CLASIFICACIÓN: CORE INFRASTRUCTURE (ESTRATO L1)
 * RESPONSABILIDAD: COORDINACIÓN DE FILTROS DE BLOOM CONCURRENTE
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. API NOMINAL: Implementa los métodos 'add' y 'contains' para
 *    sincronía total con los motores de ingesta L6 y minería L2.
 * 2. STYLE COMPLIANCE: Sanea parámetros genéricos a 'TargetPath'.
 * 3. POISON SHIELD: Gestión robusta de RwLocks ante pánicos de hilo.
 * 4. PERFORMANCE: Ruteo O(1) vía SipHash-1-3 con llaves estáticas.
 * =================================================================
 */

use crate::errors::FilterError;
use crate::filter_wrapper::RichListFilter;
use rayon::prelude::*;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::sync::{RwLock, atomic::{AtomicUsize, Ordering}};

// Inyección nominal de macros de observabilidad para el Proyecto Panóptico
use ::tracing::{info, instrument, error, debug, warn as tracing_warn};

/// Orquestador de alta densidad para la matriz de búsqueda distribuida.
pub struct ShardedFilter {
    /// Colección de fragmentos probabilísticos protegidos por cerrojos de lectura/escritura.
    pub strata_shard_collection: Vec<RwLock<RichListFilter>>,
    /// Cantidad total de particiones (Shards) configuradas para el estrato.
    pub total_partition_count: usize,
    /// Registro atómico del volumen acumulado de identidades indexadas.
    pub aggregated_identity_count: AtomicUsize,
}

impl ShardedFilter {
    /// Inicializa una nueva matriz de filtros particionados con balanceo de carga.
    ///
    /// # Performance:
    /// Realiza una pre-alocación del vector de shards para evitar re-alocaciones en runtime.
    #[must_use]
    pub fn new(
        partition_count: usize,
        total_expected_items: usize,
        false_positive_rate: f64,
    ) -> Self {
        let safe_partition_count = if partition_count == 0 { 1 } else { partition_count };
        debug!("⚙️ [SHARDED_GENESIS]: Partitioning {} units into {} shards.", total_expected_items, safe_partition_count);

        let items_per_partition = (total_expected_items / safe_partition_count).max(100);
        let mut strata_shard_collection = Vec::with_capacity(safe_partition_count);

        for _ in 0..safe_partition_count {
            let filter = RichListFilter::new(items_per_partition, false_positive_rate);
            strata_shard_collection.push(RwLock::new(filter));
        }

        Self {
            strata_shard_collection,
            total_partition_count: safe_partition_count,
            aggregated_identity_count: AtomicUsize::new(0),
        }
    }

    /**
     * Calcula el índice del shard de destino de forma determinista.
     * Utiliza SipHash-1-3 para garantizar una distribución uniforme en el espacio de búsqueda.
     */
    #[inline(always)]
    fn compute_deterministic_routing_index(&self, hash160_payload: &[u8; 20]) -> usize {
        let mut stable_hasher = siphasher::sip::SipHasher13::new_with_keys(0, 0);
        hash160_payload.hash(&mut stable_hasher);
        (stable_hasher.finish() as usize) % self.total_partition_count
    }

    /**
     * Inserta un identificador de forma nominal en el censo.
     *
     * # Logic:
     * 1. Determina el shard destino mediante ruteo SipHash.
     * 2. Adquiere un cerrojo de escritura exclusivo sobre el shard.
     * 3. Incrementa el contador global atómico.
     *
     * # Errors:
     * Si el cerrojo está envenenado (Poisoned), reporta el fallo al Panóptico sin detener el proceso.
     */
    #[inline(always)]
    pub fn add(&self, hash160_payload: &[u8; 20]) {
        let target_shard_index = self.compute_deterministic_routing_index(hash160_payload);

        if let Some(shard_lock) = self.strata_shard_collection.get(target_shard_index) {
            match shard_lock.write() {
                Ok(mut target_filter) => {
                    target_filter.add_identity_hash(hash160_payload);
                    self.aggregated_identity_count.fetch_add(1, Ordering::Relaxed);
                }
                Err(poison_error) => {
                    error!("💀 [LOCK_POISONED]: Shard {} write failed: {}", target_shard_index, poison_error);
                }
            }
        }
    }

    /**
     * Verifica la existencia de un hash en la matriz con latencia constante O(1).
     *
     * # Arguments:
     * * `hash160_payload` - El Hash160 (RIPEMD160 de SHA256) de la clave pública.
     */
    #[must_use]
    #[inline(always)]
    pub fn contains(&self, hash160_payload: &[u8; 20]) -> bool {
        let target_shard_index = self.compute_deterministic_routing_index(hash160_payload);

        if let Some(shard_lock) = self.strata_shard_collection.get(target_shard_index) {
            match shard_lock.read() {
                Ok(target_filter) => target_filter.contains_identity_hash(hash160_payload),
                Err(_) => {
                    tracing_warn!("⚠️ [LOCK_POISONED]: Defaulting to negative for shard {}", target_shard_index);
                    false
                }
            }
        } else {
            false
        }
    }

    /**
     * Sincroniza la matriz completa con el almacenamiento físico en paralelo.
     *
     * # Arguments:
     * * `output_directory_path` - Ruta al directorio base de exportación.
     */
    #[instrument(skip(self, output_directory_path))]
    pub fn save_to_directory<TargetPath: AsRef<Path>>(&self, output_directory_path: TargetPath) -> Result<(), FilterError> {
        let base_path = output_directory_path.as_ref();
        if !base_path.exists() {
            std::fs::create_dir_all(base_path).map_err(FilterError::IoError)?;
        }

        self.strata_shard_collection.par_iter().enumerate().try_for_each(|(shard_index, shard_lock)| {
            let filter_strata = shard_lock.read().map_err(|_| FilterError::IoError(
                std::io::Error::new(std::io::ErrorKind::Other, "RWLOCK_POISON_ON_SAVE")
            ))?;
            let shard_filename = format!("filter_shard_{shard_index}.bin");
            filter_strata.save_to_disk(&base_path.join(shard_filename))
        })?;

        Ok(())
    }

    /**
     * Carga y certifica todos los fragmentos binarios desde el disco de forma concurrente.
     * Utiliza Rayon para paralelizar las operaciones de I/O y descompresión.
     */
    #[instrument(skip(source_directory_path))]
    pub fn load_from_directory<TargetPath: AsRef<Path>>(
        source_directory_path: TargetPath,
        expected_partition_count: usize,
    ) -> Result<Self, FilterError> {
        let base_path = source_directory_path.as_ref();
        info!("🌊 [SHARDED_LOAD]: Hydrating {} probabilistic shards...", expected_partition_count);

        let loaded_shards_result: Result<Vec<RichListFilter>, FilterError> = (0..expected_partition_count)
            .into_par_iter()
            .map(|shard_index| {
                let shard_filename = format!("filter_shard_{shard_index}.bin");
                let full_artifact_path = base_path.join(shard_filename);

                RichListFilter::load_from_disk_mmap(&full_artifact_path)
                    .or_else(|error_mmap| {
                        tracing_warn!("⚠️ [MMAP_FAIL]: Shard {} requires stream fallback: {:?}", shard_index, error_mmap);
                        RichListFilter::load_from_disk_buffered(&full_artifact_path)
                    })
            })
            .collect();

        match loaded_shards_result {
            Ok(shards_collection) => {
                let total_indexed_count: usize = shards_collection.iter().map(|s| s.get_item_count()).sum();
                let synchronized_shards = shards_collection.into_iter().map(RwLock::new).collect();
                Ok(Self {
                    strata_shard_collection: synchronized_shards,
                    total_partition_count: expected_partition_count,
                    aggregated_identity_count: AtomicUsize::new(total_indexed_count),
                })
            },
            Err(fault) => Err(fault)
        }
    }

    /// Obtiene el conteo total de identidades registradas en la memoria volátil.
    #[must_use]
    pub fn get_total_indexed_count(&self) -> usize {
        self.aggregated_identity_count.load(Ordering::Relaxed)
    }

    // --- ALIASES DE COMPATIBILIDAD (PREVENCIÓN DE REGRESIONES) ---

    /// Alias para el método 'add'. Utilizado por consumidores antiguos.
    #[deprecated(note = "Use 'add' instead for better API elocution")]
    #[inline(always)]
    pub fn add_identity_hash(&self, payload: &[u8; 20]) {
        self.add(payload);
    }

    /// Alias para el método 'contains'. Utilizado por consumidores antiguos.
    #[deprecated(note = "Use 'contains' instead for better API elocution")]
    #[inline(always)]
    pub fn contains_identity_hash(&self, payload: &[u8; 20]) -> bool {
        self.contains(payload)
    }
}

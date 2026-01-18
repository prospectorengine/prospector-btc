// [apps/census-taker/src/main.rs]
/**
 * =================================================================
 * APARATO: CENSUS TAKER SHELL (V12.0 - SOBERANO)
 * CLASIFICACIÓN: APPLICATION LAYER (ENTRY POINT)
 * RESPONSABILIDAD: ORQUESTACIÓN DE ARGUMENTOS Y DISPARO DEL MOTOR L6
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. LIBRARY SYNERGY: Resuelve la redundancia de módulos eliminando 'mod'
 *    locales. Ahora consume la lógica desde 'prospector_census_lib',
 *    garantizando que el binario sea un orquestador puro y ligero.
 * 2. NOMINAL PURITY: Erradicación total de abreviaciones. 'size' transiciona
 *    a 'expected_record_volume' y 'config' a 'cli_configuration'.
 * 3. TRACING INITIALIZATION: Configura el sumidero de telemetría para
 *    monitorear el throughput de cristalización en tiempo real.
 * 4. ERROR TRIAGE: Utiliza 'anyhow' para la captura de fallos de I/O
 *    durante el acceso al CSV de BigQuery.
 * =================================================================
 */

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tracing::{info, instrument};

// ✅ SINCRO SOBERANA: Consumo de la interfaz pública de la librería local
use prospector_census_lib::pipeline::IngestionPipeline;

/// Configuración de argumentos para la cartografía criptográfica.
#[derive(Parser, Debug)]
#[command(
    author = "Raz Podesta <metaShark Tech>",
    version = "12.0",
    about = "Cartógrafo Criptográfico: Cristaliza el censo UTXO en filtros binarios particionados."
)]
struct CommandArguments {
    /// Ruta física del archivo CSV de origen (Dataset de BigQuery).
    #[arg(short, long, value_name = "FILE_PATH")]
    input_source: PathBuf,

    /// Directorio de destino para los fragmentos binarios (.bin) y el manifiesto.
    #[arg(short, long, alias = "output", default_value = "dist/filters")]
    output_directory: PathBuf,

    /// Volumen nominal de registros únicos esperados en el censo.
    #[arg(short, long, default_value_t = 1_000_000)]
    expected_record_volume: usize,

    /// Tasa de falsos positivos (False Positive Rate) para la matriz de Bloom.
    #[arg(long = "false-positive-rate", default_value_t = 0.0000001)]
    target_false_positive_rate: f64,

    /// Cantidad de particiones deterministas para el Sharding masivo.
    #[arg(short, long, default_value_t = 4)]
    shard_count: usize,
}

/**
 * Punto de ignición del binario ejecutable.
 *
 * # Performance:
 * El proceso de decodificación Base58 y hashing RIPEMD160 es intensivo.
 * Se recomienda ejecutar con el flag --release para habilitar SIMD.
 */
#[instrument]
fn main() -> Result<()> {
    // 1. INICIALIZACIÓN DEL SISTEMA NERVIOSO (LOGGING)
    tracing_subscriber::fmt::init();

    info!("🗺️ [CARTOGRAPHER]: Initializing execution shell V12.0...");

    // 2. PARSEO DE DIRECTIVAS DE MANDO
    let cli_configuration = CommandArguments::parse();

    // 3. CONSTRUCCIÓN DEL MOTOR DE INGESTA
    // El motor reside en la librería para permitir su auditoría en Proving Grounds.
    let ingestion_engine = IngestionPipeline::new(
        &cli_configuration.input_source,
        &cli_configuration.output_directory,
        cli_configuration.expected_record_volume,
        cli_configuration.shard_count,
        cli_configuration.target_false_positive_rate,
    );

    info!("🚀 [IGNITION]: Starting census crystallization sequence.");

    // 4. EJECUCIÓN DE LA MISIÓN ETL
    // Este paso bloquea hasta que el censo sea nivelado en disco.
    ingestion_engine.execute_ingestion_sequence()
}

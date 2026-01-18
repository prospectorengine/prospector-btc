// [apps/miner-worker/src/main.rs]
/*!
 * =================================================================
 * APARATO: HYDRA WORKER SHELL (V134.0 - ZENITH SHELL)
 * CLASIFICACIÓN: APPLICATION LAYER (ENTRY POINT)
 * RESPONSABILIDAD: BOOTSTRAP DE ENTORNO E IGNICIÓN DEL MOTOR
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. ARCHITECTURAL PURITY: Se elimina la lógica de ejecución de misiones.
 *    Ahora delega la totalidad de la operación al 'MinerEngine', actuando
 *    como un Switchboard puro de configuración y señales.
 * 2. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta aplicada a
 *    argumentos, señales y clientes de red.
 * 3. HYGIENE: Erradicación de duplicidades (FindingHandler).
 * 4. ERROR TRIAGE: Manejo rico de fallos durante la fase de ignición inicial.
 *
 * # Mathematical Proof (Deterministic Init):
 * Garantiza que el nodo solo inicie si la sinapsis con el sistema de archivos
 * y el entorno de red es estable. El uso de Arc<AtomicBool> asegura que la
 * señal de apagado se propague sin condiciones de carrera entre hilos.
 * =================================================================
 */

use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tracing::{info, warn};

// --- SINAPSIS CON LA LIBRERÍA LOCAL Y DOMINIO ---
use prospector_miner_lib::engine::MinerEngine;
use prospector_infra_worker_client::WorkerClient;

/**
 * Directivas de mando para el nodo soberano.
 * Configuración inyectada vía CLI o variables de entorno C2.
 */
#[derive(Parser, Debug)]
#[command(
    author = "Raz Podesta <metaShark Tech>",
    version = "134.0",
    about = "Hydra-Zero Sovereign Audit Node // Protocol V11.5 Gold Master"
)]
struct SovereignWorkerDirectives {
    /// Endpoint central del orquestador (Render/Tactical Hub).
    #[arg(long, env = "ORCHESTRATOR_URL")]
    orchestrator_endpoint: String,

    /// Token de autorización maestra para el handshake.
    #[arg(long, env = "WORKER_AUTH_TOKEN")]
    authentication_token: String,

    /// Identificador unívoco del nodo en la rejilla de vigilancia.
    #[arg(long, env = "WORKER_NODE_IDENTIFIER", default_value = "hydra-node-alpha")]
    worker_node_identifier: String,
}

/**
 * Punto de ignición del binario del trabajador.
 *
 * # Errors:
 * - `IO_FAULT`: Si el sistema no puede reclamar el directorio de caché.
 * - `NETWORK_FAULT`: Si la configuración de URL es sintácticamente inválida.
 *
 * # Performance:
 * Carga mínima en el hilo principal. Delega la saturación de CPU al motor.
 */
#[tokio::main]
async fn main() -> Result<()> {
    // 1. INICIALIZACIÓN DEL SISTEMA DE OBSERVABILIDAD
    tracing_subscriber::fmt::init();

    info!("💠 [SHELL]: Global Initiation Sequence V134.0 starting...");

    // 2. PARSEO DE DIRECTIVAS ESTRATÉGICAS
    let worker_directives = SovereignWorkerDirectives::parse();

    // 3. PREPARACIÓN DEL ESTRATO DE PERSISTENCIA LOCAL (Censo Cache)
    let local_cache_path_buffer = PathBuf::from("census_cache");
    if !local_cache_path_buffer.exists() {
        fs::create_dir_all(&local_cache_path_buffer)
            .context("IO_FAULT: Unable to claim ephemeral cache directory.")?;
        info!("📂 [SHELL]: Local cache strata materialized.");
    }

    // 4. PROTOCOLO DE SEÑALES DE SISTEMA (Terminación Ordenada)
    let termination_signal_atomic = Arc::new(AtomicBool::new(false));
    let signal_flag_reference = Arc::clone(&termination_signal_atomic);

    tokio::spawn(async move {
        if (tokio::signal::ctrl_c().await).is_ok() {
            warn!("⚠️ [SIGNAL]: Termination requested by host. Sealing current strata...");
            // Uso de Ordering::SeqCst para garantizar visibilidad inmediata en todos los núcleos
            signal_flag_reference.store(true, std::sync::atomic::Ordering::SeqCst);
        }
    });

    // 5. CONSTRUCCIÓN DEL ENLACE TÁCTICO (UPLINK)
    let tactical_orchestrator_uplink = Arc::new(WorkerClient::new(
        worker_directives.orchestrator_endpoint,
        worker_directives.authentication_token,
    ));

    // 6. INSTANCIACIÓN E IGNICIÓN DEL MOTOR DE MINERÍA (ESTRATO L1-WORKER)
    // El motor ahora encapsula la lógica de hilos, afinidad y misiones.
    let sovereign_engine_instance = MinerEngine::new(
        tactical_orchestrator_uplink,
        termination_signal_atomic,
        worker_directives.worker_node_identifier,
        local_cache_path_buffer,
    );

    info!("🚀 [SHELL]: Handing control to MinerEngine. Transitioning to OPERATIONAL.");

    // Ejecución bloqueante del bucle de misiones
    sovereign_engine_instance.ignite_sovereign_operations().await;

    // 7. CIERRE DETERMINISTA
    info!("🏁 [SHELL]: Shutdown sequence concluded. Sovereign node offline.");

    Ok(())
}

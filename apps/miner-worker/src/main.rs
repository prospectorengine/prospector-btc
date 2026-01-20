// [apps/miner-worker/src/main.rs]
/*!
 * =================================================================
 * APARATO: HYDRA WORKER SHELL (V134.1 - GOLD MASTER)
 * CLASIFICACIÓN: APPLICATION LAYER (ENTRY POINT)
 * RESPONSABILIDAD: BOOTSTRAP DE ENTORNO E IGNICIÓN DEL MOTOR
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. IMPORT SINCRO: Resuelve E0432 consumiendo el prelude de la
 *    librería soberana nivelada en el paso anterior.
 * 2. ZERO ANY POLICY: Tipado estricto en la gestión de señales y
 *    configuración de red.
 * 3. NOMINAL PURITY: Erradicación total de abreviaciones.
 * 4. ERROR TRIAGE: Manejo de fallos IO con contexto para el Panóptico.
 * =================================================================
 */

use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tracing::{info, warn};

// --- SINAPSIS CON LA LIBRERÍA SOBERANA (PRELUDE) ---
use prospector_miner_lib::prelude::*;
use prospector_infra_worker_client::WorkerClient;

/**
 * Directivas de mando táctico para el nodo del enjambre.
 * Configuración inyectada vía variables de entorno o parámetros CLI.
 */
#[derive(Parser, Debug)]
#[command(
    author = "Raz Podesta <metaShark Tech>",
    version = "134.1",
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
 * Orquesta la infraestructura local antes de disparar el motor matemático.
 */
#[tokio::main]
async fn main() -> Result<()> {
    // 1. INICIALIZACIÓN DEL SISTEMA DE OBSERVABILIDAD (HEIMDALL)
    tracing_subscriber::fmt::init();

    info!("💠 [SHELL]: Global Initiation Sequence V134.1 starting...");

    // 2. PARSEO DE DIRECTIVAS ESTRATÉGICAS
    let worker_directives = SovereignWorkerDirectives::parse();

    // 3. PREPARACIÓN DEL ESTRATO DE PERSISTENCIA (Censo Cache)
    let local_cache_path_buffer = PathBuf::from("census_cache");
    if !local_cache_path_buffer.exists() {
        fs::create_dir_all(&local_cache_path_buffer)
            .context("IO_FAULT: Unable to materialize ephemeral cache strata.")?;
        info!("📂 [SHELL]: Local cache strata materialized at {:?}", local_cache_path_buffer);
    }

    // 4. PROTOCOLO DE SEÑALES DE SISTEMA (Terminación Resiliente)
    let global_termination_signal = Arc::new(AtomicBool::new(false));
    let signal_flag_reference = Arc::clone(&global_termination_signal);

    tokio::spawn(async move {
        if (tokio::signal::ctrl_c().await).is_ok() {
            warn!("⚠️ [SIGNAL]: Termination pulse detected. Sealing audit trail...");
            // Garantiza visibilidad atómica en todos los hilos del worker
            signal_flag_reference.store(true, Ordering::SeqCst);
        }
    });

    // 5. CONSTRUCCIÓN DEL ENLACE TÁCTICO (UPLINK L4)
    let tactical_orchestrator_uplink = Arc::new(WorkerClient::new(
        worker_directives.orchestrator_endpoint,
        worker_directives.authentication_token,
    ));

    // 6. INSTANCIACIÓN E IGNICIÓN DEL MOTOR (L1-WORKER)
    // El motor ahora recibe el mando absoluto de la operación.
    let sovereign_engine_instance = MinerEngine::new(
        tactical_orchestrator_uplink,
        global_termination_signal,
        worker_directives.worker_node_identifier,
        local_cache_path_buffer,
    );

    info!("🚀 [SHELL]: Handing control to MinerEngine. Transitioning to OPERATIONAL.");

    // Ejecución del bucle infinito de misiones
    sovereign_engine_instance.ignite_sovereign_operations().await;

    // 7. CIERRE DETERMINISTA
    info!("🏁 [SHELL]: Shutdown sequence concluded. Sovereign node offline.");

    Ok(())
}

// [apps/orchestrator/src/main.rs]
/*!
 * =================================================================
 * APARATO: ORCHESTRATOR MAIN ENTRY POINT (V113.0 - SINGULARITY GOLD)
 * CLASIFICACIÓN: APPLICATION SHELL (ESTRATO L3)
 * RESPONSABILIDAD: BOOTSTRAP DE INFRAESTRUCTURA E IGNICIÓN SEGURA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. KERNEL SYNERGY: Sincroniza la ignición con el OrchestratorKernel V370.0,
 *    garantizando el acceso nominal al estado 'application_shared_state'.
 * 2. RUNTIME SOVEREIGNTY: Configura el reactor de Tokio con una pila de 4MB
 *    para proteger la ejecución de algoritmos criptográficos pesados.
 * 3. NOMINAL PURITY: Nomenclatura descriptiva absoluta nivel Tesis Doctoral MIT.
 *    'url' -> 'tactical_database_connection_url'.
 * 4. PANOPTICON IGNITION: Asegura que el rastro de trazado (Heimdall) sea el
 *    primer subsistema operativo, capturando el proceso de arranque íntegro.
 *
 * # Mathematical Proof (Deterministic Ignition):
 * El shell garantiza que la hidratación de ADN (Arqueología) sea una
 * precondición atómica para la apertura del socket de red, eliminando
 * estados de carrera en el aprovisionamiento de nodos.
 * =================================================================
 */

// --- SINAPSIS CON LA LIBRERÍA SOBERANA ---
use prospector_orchestrator::prelude::*;
use prospector_orchestrator::bootstrap_forensics::perform_automatic_forensic_ignition;

// --- UTILIDADES DE ESTRATO ---
use dotenvy::dotenv;
use prospector_shared_heimdall::init_tracing;
use tracing::{info, error, instrument};

/**
 * Punto de entrada supremo del binario del Orquestador.
 * Orquesta la transición del entorno de shell hacia el runtime asíncrono.
 *
 * # Errors:
 * Retorna un error dinámico si el sistema falla en la fase de 'Ignition'
 * o si el reactor de Tokio no puede ser inicializado.
 */
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. CARGA DE ENTORNO SOBERANO
    // Hidrata las variables del archivo .env en el espacio de nombres del proceso.
    dotenv().ok();

    // 2. INICIALIZACIÓN DE OBSERVABILIDAD NEURAL (HEIMDALL-RS)
    // Este es el primer órgano activo para garantizar el rastro de la Tesis.
    init_tracing("prospector_orchestrator_master_v17");

    // 3. CONFIGURACIÓN DEL RUNTIME SOBERANO CON PROTECCIÓN DE PILA (4MB)
    // El tamaño de pila elevado es imperativo para la recursión del oráculo GQL
    // y el manejo de grandes volúmenes de datos en el stack durante la minería.
    let runtime_orchestrator_handle = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(4 * 1024 * 1024) // 4 Megabytes de seguridad contra overflow
        .build()?;

    // 4. EJECUCIÓN DENTRO DEL REACTOR DE TOKIO
    runtime_orchestrator_handle.block_on(async {
        info!("🛰️  [COMMAND_CENTER]: Initiating global singularity sequence V17.0...");

        // 5. ADQUISICIÓN DE COORDENADAS TÁCTICAS (Persistencia)
        let tactical_database_connection_url = std::env::var("DATABASE_URL")
            .expect("CRITICAL_FAULT: DATABASE_URL undefined in runtime environment.");

        let tactical_database_access_token = std::env::var("TURSO_AUTH_TOKEN").ok();

        let orchestrator_listening_network_port: u16 = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);

        // 6. CONSTRUCCIÓN DEL KERNEL SOBERANO (ESTRATO L1-APP)
        // El Kernel realiza el handshake con el Motor A (Turso Cloud).
        let kernel_instance_artifact = OrchestratorKernel::ignite(
            &tactical_database_connection_url,
            tactical_database_access_token,
            orchestrator_listening_network_port
        ).await;

        // 7. PROTOCOLO DE ARQUEOLOGÍA (DNA AUTO-HYDRATION)
        // ✅ NIVELACIÓN V113: Sincronización con el miembro 'application_shared_state'
        info!("🧬 [FORENSIC_SHIELD]: Verifying cryptographic DNA registries in Motor A...");
        
        if let Err(hydration_fault_message) = perform_automatic_forensic_ignition(
            &kernel_instance_artifact.application_shared_state
        ).await {
            error!(
                "❌ [HYDRATION_FAILED]: Forensic DNA initialization collapsed: {}", 
                塑造_fault_message
            );
            // Bloqueo preventivo: No permitimos la ignición si los activos forenses fallan.
            std::process::exit(1);
        }

        // 8. IGNICIÓN DE OPERACIONES DEL ENJAMBRE
        // Lanza Daemons adaptativos, sincronía galvánica y levanta el transporte Axum.
        info!(
            "🚀 [PROSPECTOR_ONLINE]: System fully operational on port {}.", 
            orchestrator_listening_network_port
        );
        
        kernel_instance_artifact.launch_sovereign_operations().await;

        Ok(())
    })
}
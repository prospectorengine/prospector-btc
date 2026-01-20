// [apps/orchestrator/src/main.rs]
/*!
 * =================================================================
 * APARATO: ORCHESTRATOR MAIN ENTRY POINT (V112.0 - GOLD MASTER)
 * CLASIFICACIÓN: APPLICATION SHELL (ESTRATO L3)
 * RESPONSABILIDAD: BOOTSTRAP DE INFRAESTRUCTURA E IGNICIÓN SEGURA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. NOMINAL SYNC: Resuelve el error de campo 'application_state'
 *    mediante el acceso al miembro nivelado 'application_shared_state'.
 * 2. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta en la captura
 *    de entorno y configuración de red.
 * 3. RUNTIME REFINERY: Optimización de la pila de Tokio (4MB) para
 *    procesar el Ledger Táctico U256 sin riesgo de overflow.
 * 4. HYGIENE: Documentación técnica nivel Tesis Doctoral MIT.
 *
 * # Mathematical Proof (Deterministic Ignition):
 * El proceso garantiza que la ignición forense (DNA) ocurra antes
 * que la apertura del socket TCP, previniendo estados de carrera donde
 * un worker solicite activos aún no cristalizados en el Motor A.
 * =================================================================
 */

// Sincronización con la librería nominal del orquestador
use prospector_orchestrator::prelude::*;
use prospector_orchestrator::bootstrap_forensics::perform_automatic_forensic_ignition;

use dotenvy::dotenv;
use prospector_shared_heimdall::init_tracing;
use tracing::{info, error};

/**
 * Punto de ignición supremo del binario del Orquestador.
 */
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. CARGA DE ENTORNO SOBERANO
    // Hidrata las variables de .env en el espacio de nombres del proceso.
    dotenv().ok();

    // 2. INICIALIZACIÓN DE OBSERVABILIDAD NEURAL (HEIMDALL)
    init_tracing("prospector_orchestrator_master");

    // 3. CONFIGURACIÓN DEL RUNTIME SOBERANO CON PROTECCIÓN DE PILA (4MB)
    // El tamaño de pila elevado es imperativo para la recursión del oráculo GQL.
    let runtime_orchestrator = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(4 * 1024 * 1024)
        .build()?;

    runtime_orchestrator.block_on(async {
        info!("🛰️  [COMMAND_CENTER]: Global ignition sequence starting...");

        // 4. ADQUISICIÓN DE COORDENADAS TÁCTICAS
        let database_connection_url = std::env::var("DATABASE_URL")
            .expect("CRITICAL_FAULT: DATABASE_URL not defined in runtime environment.");

        let database_access_token = std::env::var("TURSO_AUTH_TOKEN").ok();

        let listening_network_port: u16 = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);

        // 5. CONSTRUCCIÓN DEL KERNEL SOBERANO (ESTRATO L1-APP)
        // El Kernel orquesta la sinapsis entre L3 (DB) y L4 (API).
        let kernel_instance = OrchestratorKernel::ignite(
            &database_connection_url,
            database_access_token,
            listening_network_port
        ).await;

        // 6. PROTOCOLO DE ARQUEOLOGÍA (DNA AUTO-HYDRATION)
        // ✅ RESOLUCIÓN SOBERANA: Sincronización con 'application_shared_state'
        info!("🧬 [FORENSIC_SHIELD]: Verifying system DNA registries in Motor A...");
        if let Err(hydration_error) = perform_automatic_forensic_ignition(
            &kernel_instance.application_shared_state
        ).await {
            error!("❌ [HYDRATION_FAILED]: Forensic DNA initialization collapsed: {}", hydration_error);
            std::process::exit(1);
        }

        // 7. IGNICIÓN DE OPERACIONES DEL ENJAMBRE
        // Activa Daemons de mantenimiento, telemetría y levanta el servidor Axum.
        info!("🚀 [PROSPECTOR_ONLINE]: System fully operational on port {}.", listening_network_port);
        kernel_instance.launch_sovereign_operations().await;

        Ok(())
    })
}

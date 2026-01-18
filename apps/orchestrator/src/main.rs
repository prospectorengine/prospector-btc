// [apps/orchestrator/src/main.rs]
/**
 * =================================================================
 * APARATO: ORCHESTRATOR MAIN ENTRY POINT (V111.0 - LIBRARY CONSUMER)
 * CLASIFICACIÓN: APPLICATION SHELL (ESTRATO L3)
 * RESPONSABILIDAD: BOOTSTRAP DE INFRAESTRUCTURA E IGNICIÓN SEGURA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. ARCHITECTURAL ALIGNMENT: Elimina las declaraciones 'mod' locales.
 *    Ahora consume el árbol de módulos desde la librería nominal
 *    'prospector_orchestrator', resolviendo el error E0432.
 * 2. ZERO ABBREVIATIONS: 'url' -> 'database_connection_url',
 *    'token' -> 'database_access_token'.
 * 3. PRELUDE ADOPTION: Utiliza el 'prelude' de la librería para
 *    una ignición minimalista y desacoplada.
 * 4. STACK PROTECTION: Mantiene la protección de pila de 4MB para
 *    procesar el esquema de misiones masivas.
 *
 * # Mathematical Proof (Binary/Library Separation):
 * Al delegar la jerarquía de módulos a 'lib.rs', el binario opera en
 * el espacio de nombres de la crate. Esto garantiza que 'crate::' dentro
 * de los módulos de la librería apunte correctamente a la raíz de la
 * misma, donde 'graphql' reside oficialmente.
 * =================================================================
 */

// ✅ REPARACIÓN DEFINITIVA: No se declaran módulos aquí.
// El binario consume la librería para evitar la duplicidad del árbol.
use prospector_orchestrator::prelude::*;
use prospector_orchestrator::bootstrap_forensics::perform_automatic_forensic_ignition;

use dotenvy::dotenv;
use prospector_shared_heimdall::init_tracing;
use tracing::{info, error};

/**
 * Punto de ignición del binario del Orquestador.
 */
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. CARGA DE ENTORNO SOBERANO
    dotenv().ok();

    // 2. INICIALIZACIÓN DE OBSERVABILIDAD (ESTRATO L4/L6)
    init_tracing("prospector_orchestrator");

    // 3. CONFIGURACIÓN DEL RUNTIME CON PROTECCIÓN DE STACK (4MB)
    // Requerido para la manipulación segura de misiones con rangos U256.
    let runtime_orchestrator = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(4 * 1024 * 1024)
        .build()?;

    runtime_orchestrator.block_on(async {
        info!("🛰️ [COMMAND_CENTER]: Global ignition sequence starting...");

        // 4. ADQUISICIÓN DE PARÁMETROS TÁCTICOS
        let database_connection_url = std::env::var("DATABASE_URL")
            .expect("CRITICAL_FAULT: DATABASE_URL not defined in runtime.");

        let database_access_token = std::env::var("TURSO_AUTH_TOKEN").ok();

        let listening_port: u16 = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);

        // 5. CONSTRUCCIÓN DEL KERNEL SOBERANO (L1-APP)
        // El Kernel orquesta la sinapsis entre L3 (DB) y L4 (API).
        let kernel_instance = OrchestratorKernel::ignite(
            &database_connection_url,
            database_access_token,
            listening_port
        ).await;

        // 6. PROTOCOLO DE ARQUEOLOGÍA (DNA AUTO-HYDRATION)
        info!("🧬 [FORENSIC_SHIELD]: Verifying system DNA registries...");
        if let Err(hydration_error) = perform_automatic_forensic_ignition(
            &kernel_instance.application_state
        ).await {
            error!("❌ [HYDRATION_FAILED]: Forensic initialization failed: {}", hydration_error);
            std::process::exit(1);
        }

        // 7. IGNICIÓN DE OPERACIONES SOBERANAS
        // Lanza daemons de mantenimiento, telemetría y el servidor HTTP.
        info!("🚀 [PROSPECTOR_ONLINE]: System fully operational on port {}", listening_port);
        kernel_instance.launch_sovereign_operations().await;

        Ok(())
    })
}

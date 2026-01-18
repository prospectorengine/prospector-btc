// INICIO DEL ARCHIVO [apps/orchestrator/src/bin/migrator.rs]
/**
 * =================================================================
 * APARATO: DB MIGRATOR CLI (V2.6 - GOLD MASTER)
 * CLASIFICACIÓN: OPS INFRASTRUCTURE (ESTRATO L6)
 * RESPONSABILIDAD: EJECUCIÓN DE MANTENIMIENTO Y NIVELACIÓN DE NUBE
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el disparador de migraciones soberanas para el Motor A.
 * 1. RESOLUCIÓN STACK_OVERFLOW: Configura un runtime de Tokio con
 *    soberanía de memoria (4MB stack) para procesar el esquema atómico.
 * 2. Handshake determinista con Turso Cloud vía TLS 1.3.
 * 3. Ejecución de los 3 estratos de esquema: Tablas, Evolución e Índices.
 * =================================================================
 */

use dotenvy::dotenv;
use prospector_infra_db::schema::apply_full_sovereign_schema;
use prospector_infra_db::TursoClient;
use prospector_shared_heimdall::init_tracing;
use tracing::{error, info};

fn main() -> anyhow::Result<()> {
    // 1. CARGA DE ENTORNO OPERATIVO
    // Sincroniza las credenciales del .env local con el túnel hacia la nube.
    dotenv().ok();
    init_tracing("prospector_migrator");

    // 2. CONFIGURACIÓN DEL RUNTIME SOBERANO
    // Elevamos la capacidad de la pila para procesar la máquina de estados
    // del esquema V142.5 sin riesgo de desbordamiento (Stack Overflow).
    let runtime_orchestrator = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(4 * 1024 * 1024) // 4 Megabytes de seguridad
        .build()?;

    runtime_orchestrator.block_on(async {
        info!("🚀 [MIGRATOR]: Initiating structural audit of Cloud Strata...");

        // 3. ADQUISICIÓN DE CREDENCIALES ESTRATÉGICAS
        let database_url = std::env::var("DATABASE_URL")
            .expect("CRITICAL_FAULT: DATABASE_URL undefined in terminal context.");
        let database_token = std::env::var("TURSO_AUTH_TOKEN").ok();

        // 4. HANDSHAKE CON EL MOTOR A (TURSO)
        let database_client = match TursoClient::connect(&database_url, database_token).await {
            Ok(client) => client,
            Err(connection_error) => {
                error!("❌ [UPLINK_FAULT]: Failed to establish link to Turso: {}", connection_error);
                return Err(anyhow::anyhow!(connection_error));
            }
        };

        let database_connection = database_client
            .get_connection()
            .map_err(|error| anyhow::anyhow!("POOL_EXHAUSTED: {}", error))?;

        // 5. EJECUCIÓN DEL PROTOCOLO DE REPARACIÓN (V142.5)
        match apply_full_sovereign_schema(&database_connection).await {
            Ok(_) => {
                info!("✨ [MIGRATOR_SUCCESS]: Tactical Ledger is now Gold Master level.");
                Ok(())
            }
            Err(schema_fault) => {
                error!("💀 [SCHEMA_COLLAPSE]: Fatal structural error: {}", schema_fault);
                std::process::exit(1);
            }
        }
    })
}
// FIN DEL ARCHIVO [apps/orchestrator/src/bin/migrator.rs]

// INICIO DEL ARCHIVO [apps/orchestrator/src/bin/seed.rs]
//! =================================================================
//! APARATO: GENESIS FORGE SEEDER (V140.2 - DOCS FIXED)
//! CLASIFICACIÓN: INFRASTRUCTURE UTILITY (ESTRATO L3)
//! RESPONSABILIDAD: SEMBRADO ATÓMICO Y REPLICABILIDAD DE INSTANCIA
//! =================================================================

use prospector_infra_db::TursoClient;
use tracing::{info, error};
use dotenvy::dotenv;
use uuid::Uuid;
use libsql::params;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter("info,prospector_infra_db=debug")
        .with_target(false)
        .init();

    info!("💠 [GENESIS_FORGE]: Initiating Sovereign Infrastructure Hydration...");

    let database_url = std::env::var("DATABASE_URL")
        .expect("CRITICAL_FAULT: DATABASE_URL is missing in environment.");
    let database_token = std::env::var("TURSO_AUTH_TOKEN").ok();

    let database_client = match TursoClient::connect(&database_url, database_token).await {
        Ok(client) => client,
        Err(e) => {
            error!("❌ [FORGE_FAULT]: UPLINK_COLLAPSE: {}", e);
            return Err(anyhow::anyhow!(e));
        }
    };

    let database_connection = database_client.get_connection()
        .map_err(|e| anyhow::anyhow!("POOL_FAULT: {}", e))?;

    info!("⚙️  [FORGE]: Synchronizing system_state control metadata...");
    database_connection.execute(
        "INSERT INTO system_state (key, value_text, updated_at)
         VALUES ('active_census_audit_token', 'PROSPECTOR_V10.8_MASTER', CURRENT_TIMESTAMP)
         ON CONFLICT(key) DO UPDATE SET value_text = excluded.value_text",
        ()
    ).await?;

    info!("🧪 [FORGE]: Planting Golden Tickets for forensic validation...");
    database_connection.execute(
        "INSERT INTO test_scenarios (id, name, target_address, status)
         VALUES (?1, ?2, ?3, 'idle')
         ON CONFLICT(id) DO NOTHING",
        params![
            "cert-alpha-001",
            "SATOSHI_GENESIS_BLOCK_1",
            "12cbqSREwGrvtd3LsBhymWvCX9A9Snd9E7"
        ]
    ).await?;

    info!("🚀 [FORGE]: Deploying initial sequential search segments...");
    let mission_id = Uuid::new_v4().to_string();
    database_connection.execute(
        "INSERT INTO jobs (id, range_start, range_end, strategy_type, required_strata, status)
         VALUES (?1, ?2, ?3, ?4, ?5, 'queued')
         ON CONFLICT(id) DO NOTHING",
        params![
            mission_id,
            "0000000000000000000000000000000000000000000000000000000000000001",
            "000000000000000000000000000000000000000000000000000000000000FFFF",
            "Sequential",
            "StandardLegacy"
        ]
    ).await?;

    info!("✅ [GENESIS_COMPLETE]: Strata synchronized. Node ready for mission dispatch.");
    Ok(())
}
// FIN DEL ARCHIVO [apps/orchestrator/src/bin/seed.rs]

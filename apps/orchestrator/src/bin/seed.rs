// [apps/orchestrator/src/bin/seed.rs]
/**
 * =================================================================
 * APARATO: GENESIS FORGE SEEDER (V180.5 - GOLD MASTER ALIGNED)
 * CLASIFICACIÓN: INFRASTRUCTURE UTILITY (ESTRATO L3)
 * RESPONSABILIDAD: HIDRATACIÓN ATÓMICA Y CERTIFICACIÓN DE ESTRATOS
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. PRE-FLIGHT AUDIT: Verifica la existencia de credenciales antes de
 *    la ignición del cliente para evitar pánicos ciegos en el CI/CD.
 * 2. DUAL MISSION SEEDING: Inyecta misiones para Satoshi-XP y Sequential
 *    simultáneamente, garantizando cobertura de motores L2.
 * 3. NOMINAL PURITY: Erradicación de abreviaciones. 'e' -> 'db_fault'.
 * 4. VERSION SYNC: Nivelado a la V12.0 Gold Master para paridad con el HUD.
 * =================================================================
 */

use prospector_infra_db::TursoClient;
use tracing::{info, error, warn, instrument, debug};
use dotenvy::dotenv;
use uuid::Uuid;
use libsql::params;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. CARGA DE ENTORNO SOBERANO
    dotenv().ok();

    // 2. INICIALIZACIÓN DE OBSERVABILIDAD ZENITH
    tracing_subscriber::fmt()
        .with_env_filter("info,prospector_infra_db=debug")
        .with_target(false)
        .init();

    info!("💠 [GENESIS_FORGE]: Initiating Sovereign Infrastructure Hydration V180.5...");

    // 3. AUDITORÍA DE PRE-VUELO (Environment Scrutiny)
    let database_connection_url = env::var("DATABASE_URL").map_err(|_| {
        error!("❌ [ENV_FAULT]: DATABASE_URL is undefined in the current strata.");
        anyhow::anyhow!("MISSING_DATABASE_URL")
    })?;

    let database_auth_token = env::var("TURSO_AUTH_TOKEN").ok();

    // Ofuscación de seguridad para el log de auditoría
    let masked_url = if database_connection_url.len() > 15 {
        format!("{}...", &database_connection_url[..15])
    } else {
        "PROTECTED_URL".to_string()
    };

    debug!("🔗 [UPLINK]: Target endpoint identified: {}", masked_url);

    // 4. HANDSHAKE CON EL MOTOR A (TURSO CLOUD)
    let database_client = match TursoClient::connect(&database_connection_url, database_auth_token).await {
        Ok(client) => client,
        Err(connection_fault) => {
            error!("❌ [FORGE_FAULT]: UPLINK_COLLAPSE: {}", connection_fault);
            return Err(anyhow::anyhow!("DATABASE_UPLINK_SEVERED: {}", connection_fault));
        }
    };

    let database_connection = database_client.get_connection()
        .map_err(|db_fault| {
            error!("💀 [POOL_FAULT]: Strata connection pool exhausted: {}", db_fault);
            anyhow::anyhow!("DATABASE_POOL_COLLAPSE")
        })?;

    // 5. SINCRONIZACIÓN DE METADATOS (System State)
    info!("⚙️  [FORGE]: Synchronizing system_state control metadata...");
    database_connection.execute(
        "INSERT INTO system_state (key, value_text, updated_at)
         VALUES ('active_census_audit_token', 'V12.0_GOLD_MASTER', CURRENT_TIMESTAMP)
         ON CONFLICT(key) DO UPDATE SET
            value_text = excluded.value_text,
            updated_at = CURRENT_TIMESTAMP",
        ()
    ).await?;

    // 6. INYECCIÓN DE VECTORES DORADOS (Golden Tickets)
    info!("🧪 [FORGE]: Planting Golden Tickets for forensic validation...");
    let golden_tickets = vec![
        ("cert-alpha-001", "SATOSHI_GENESIS_BLOCK_1", "12cbqSREwGrvtd3LsBhymWvCX9A9Snd9E7"),
        ("cert-beta-001", "XP_ENTROPY_VECTOR_RECOVERY", "1ADJqstUMBB5zFquWg19UqZ7Zc6ePCpzLE"),
    ];

    for (id, name, addr) in golden_tickets {
        database_connection.execute(
            "INSERT INTO test_scenarios (id, name, target_address, status)
             VALUES (?1, ?2, ?3, 'idle')
             ON CONFLICT(id) DO NOTHING",
            params![id, name, addr]
        ).await?;
    }

    // 7. DESPACHO DE MISIONES GÉNESIS (Dual Strategy)
    info!("🚀 [FORGE]: Deploying Multi-Strata search segments...");

    // Misión 01: Arqueología Satoshi-XP (Uptime Initial Window)
    let xp_mission_id = Uuid::new_v4().to_string();
    database_connection.execute(
        "INSERT INTO jobs (id, range_start, range_end, strategy_type, required_strata, status)
         VALUES (?1, 'uptime_0', 'uptime_600', 'SatoshiWindowsXpForensic', 'SatoshiEra', 'queued')",
        params![xp_mission_id]
    ).await?;

    // Misión 02: Barrido Secuencial U256 (Meloni 5M Engine)
    let seq_mission_id = Uuid::new_v4().to_string();
    database_connection.execute(
        "INSERT INTO jobs (id, range_start, range_end, strategy_type, required_strata, status)
         VALUES (?1, ?2, ?3, 'Sequential', 'StandardLegacy', 'queued')",
        params![
            seq_mission_id,
            "0000000000000000000000000000000000000000000000000000000000000001",
            "000000000000000000000000000000000000000000000000000000000000FFFF"
        ]
    ).await?;

    info!("✅ [GENESIS_COMPLETE]: Strata synchronized. Node ready for mission dispatch.");
    Ok(())
}

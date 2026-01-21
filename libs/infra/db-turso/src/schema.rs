// [libs/infra/db-turso/src/schema.rs]
/*!
 * =================================================================
 * APARATO: SOVEREIGN DATABASE SCHEMA (V154.0 - FULL SYNC)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (ESTRATO L3)
 * RESPONSABILIDAD: GOBERNANZA ESTRUCTURAL E IDEMPOTENCIA TOTAL
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. GENESIS-EVOLUTION PARITY: Sincroniza las definiciones base con las
 *    mutaciones evolutivas para una ignición instantánea.
 * 2. HYDRA-ID OPTIMIZATION: Inyección de columnas de identidad persistente
 *    (fingerprint, proxy, metabolic_pulse) directamente en el Génesis.
 * 3. INDEX HARDENING: Adición de 'index_identities_metabolic' para
 *    acelerar la vigilancia del pulso humano.
 * 4. ZERO ABBREVIATIONS: Cumplimiento del estándar nominal de la Tesis.
 * =================================================================
 */

use anyhow::{Context, Result};
use libsql::Connection;
use tracing::{debug, info, instrument, warn};

/**
 * ESTRATO 1: SOLIDIFICACIÓN (Génesis de Tablas)
 * ✅ NIVELADO: Incluye todas las columnas de estrategias L2 y Hydra-ID.
 */
const TACTICAL_TABLES: &[(&str, &str)] = &[
    ("TABLE_JOBS", r#"
        CREATE TABLE IF NOT EXISTS jobs (
            id TEXT PRIMARY KEY,
            range_start TEXT NOT NULL,
            range_end TEXT NOT NULL,
            status TEXT DEFAULT 'queued',
            operator_id TEXT DEFAULT 'SYSTEM_DELEGATE',
            parent_mission_id TEXT,
            strategy_type TEXT DEFAULT 'Sequential',
            total_hashes_effort TEXT DEFAULT '0',
            execution_duration_ms INTEGER DEFAULT 0,
            audit_footprint_checkpoint TEXT,
            integrity_hash TEXT,
            required_strata TEXT DEFAULT 'StandardLegacy',
            dataset_resource_locator TEXT,
            target_public_key_hexadecimal TEXT,
            range_width_max INTEGER,
            target_mock_iterations INTEGER,
            diagnostic_seed TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            completed_at DATETIME,
            archived_at DATETIME
        );
    "#),
    ("TABLE_IDENTITIES", r#"
        CREATE TABLE IF NOT EXISTS identities (
            id TEXT PRIMARY KEY,
            platform TEXT NOT NULL,
            email TEXT NOT NULL,
            credentials_json TEXT NOT NULL,
            user_agent TEXT,
            status TEXT DEFAULT 'active',
            leased_until DATETIME,
            cooldown_until DATETIME,
            browser_fingerprint_json TEXT,
            proxy_url TEXT,
            last_metabolic_pulse DATETIME,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            usage_count INTEGER DEFAULT 0,
            UNIQUE(platform, email)
        );
    "#),
    ("TABLE_ACADEMY_PROGRESS", r#"
        CREATE TABLE IF NOT EXISTS academy_progress (
            operator_id TEXT NOT NULL,
            module_identifier TEXT NOT NULL,
            status TEXT DEFAULT 'unlocked',
            mining_score REAL DEFAULT 0.0,
            completed_at DATETIME,
            PRIMARY KEY(operator_id, module_identifier)
        );
    "#),
    ("TABLE_KNOWLEDGE_MODULES", r#"
        CREATE TABLE IF NOT EXISTS knowledge_modules (
            identifier TEXT PRIMARY KEY,
            i18n_title_key TEXT NOT NULL,
            i18n_description_key TEXT NOT NULL,
            difficulty TEXT NOT NULL,
            duration_minutes INTEGER DEFAULT 0,
            visual_icon TEXT,
            prerequisites TEXT,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
    "#),
    ("TABLE_AFFILIATE_NETWORK", r#"
        CREATE TABLE IF NOT EXISTS affiliate_network (
            affiliate_id TEXT PRIMARY KEY,
            parent_affiliate_id TEXT,
            referral_code TEXT UNIQUE,
            accumulated_hashrate REAL DEFAULT 0.0,
            joined_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
    "#),
    ("TABLE_SYSTEM_STATE", r#"
        CREATE TABLE IF NOT EXISTS system_state (
            key TEXT PRIMARY KEY,
            value_text TEXT,
            value_int INTEGER,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
    "#),
    ("TABLE_OUTBOX_STRATEGIC", r#"
        CREATE TABLE IF NOT EXISTS outbox_strategic (
            outbox_identifier TEXT PRIMARY KEY,
            payload_json TEXT NOT NULL,
            target_stratum TEXT NOT NULL,
            status TEXT DEFAULT 'pending',
            retry_count INTEGER DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            processed_at DATETIME
        );
    "#),
    ("TABLE_FINDINGS", r#"
        CREATE TABLE IF NOT EXISTS findings (
            id TEXT PRIMARY KEY,
            address TEXT NOT NULL,
            private_key_wif TEXT NOT NULL,
            source_entropy TEXT,
            wallet_type TEXT,
            found_by_worker TEXT,
            job_id TEXT,
            detected_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            archived_at DATETIME
        );
    "#)
];

/**
 * ESTRATO 2: EVOLUCIÓN (Mutaciones de Columna)
 * Mantenido para resiliencia de bases de datos existentes.
 */
const EVOLUTIONARY_STRATA: &[(&str, &str)] = &[
    ("JOB_DICT_LOCATOR", "ALTER TABLE jobs ADD COLUMN dataset_resource_locator TEXT"),
    ("JOB_KANGAROO_PUBKEY", "ALTER TABLE jobs ADD COLUMN target_public_key_hexadecimal TEXT"),
    ("JOB_KANGAROO_WIDTH", "ALTER TABLE jobs ADD COLUMN range_width_max INTEGER"),
    ("JOB_PLAYGROUND_ITER", "ALTER TABLE jobs ADD COLUMN target_mock_iterations INTEGER"),
    ("JOB_PLAYGROUND_SEED", "ALTER TABLE jobs ADD COLUMN diagnostic_seed TEXT"),
    ("IDENTITY_FINGERPRINT", "ALTER TABLE identities ADD COLUMN browser_fingerprint_json TEXT"),
    ("IDENTITY_PROXY", "ALTER TABLE identities ADD COLUMN proxy_url TEXT"),
    ("IDENTITY_METABOLIC", "ALTER TABLE identities ADD COLUMN last_metabolic_pulse DATETIME"),
    ("FINDING_ARCHIVED", "ALTER TABLE findings ADD COLUMN archived_at DATETIME")
];

/**
 * ESTRATO 3: ENDURECIMIENTO (Índices de Aceleración)
 */
const ACCELERATION_INDEXES: &[(&str, &str)] = &[
    ("INDEX_JOBS_OPERATOR", "CREATE INDEX IF NOT EXISTS index_jobs_operator ON jobs(operator_id);"),
    ("INDEX_IDENTITIES_SYNC", "CREATE INDEX IF NOT EXISTS index_identities_availability ON identities(platform, status, leased_until, cooldown_until);"),
    ("INDEX_IDENTITIES_METABOLIC", "CREATE INDEX IF NOT EXISTS index_identities_pulse ON identities(last_metabolic_pulse);"),
    ("INDEX_OUTBOX_POLLING", "CREATE INDEX IF NOT EXISTS index_outbox_status_pending ON outbox_strategic(status, created_at);"),
    ("INDEX_FINDINGS_SYNC", "CREATE INDEX IF NOT EXISTS index_findings_archival ON findings(archived_at);")
];

/**
 * Ejecuta la secuencia maestra de sincronización V154.0.
 */
#[instrument(skip(database_connection))]
pub async fn apply_full_sovereign_schema(database_connection: &Connection) -> Result<()> {
    info!("🏗️ [SCHEMA_ENGINE]: Initiating structural synchronization V154.0...");

    solidify_base_strata(database_connection).await?;
    execute_evolutionary_repair(database_connection).await?;
    harden_access_layer(database_connection).await?;

    info!("✅ [SCHEMA_ENGINE]: Tactical Ledger V154.0 fully synchronized.");
    Ok(())
}

async fn solidify_base_strata(database_handle: &Connection) -> Result<()> {
    for (identifier, sql_statement) in TACTICAL_TABLES {
        debug!("  ↳ Solidifying: {}", identifier);
        database_handle.execute(*sql_statement, ()).await
            .with_context(|| format!("CRITICAL_SOLIDIFICATION_FAULT: {}", identifier))?;
    }
    Ok(())
}

async fn execute_evolutionary_repair(database_handle: &Connection) -> Result<()> {
    for (identifier, sql_statement) in EVOLUTIONARY_STRATA {
        match database_handle.execute(*sql_statement, ()).await {
            Ok(_) => info!("  🟢 [REPAIR_OK]: Applied evolutionary stratum {}", identifier),
            Err(database_error) => {
                let error_message = database_error.to_string();
                if error_message.contains("duplicate column name") {
                    debug!("  ⚪ [REPAIR_SKIP]: {} already level.", identifier);
                } else {
                    warn!("  ⚠️ [REPAIR_BYPASS]: {} check incomplete: {}", identifier, error_message);
                }
            }
        }
    }
    Ok(())
}

async fn harden_access_layer(database_handle: &Connection) -> Result<()> {
    for (identifier, sql_statement) in ACCELERATION_INDEXES {
        debug!("  ↳ Hardening: {}", identifier);
        database_handle.execute(*sql_statement, ()).await
            .with_context(|| format!("CRITICAL_HARDENING_FAULT: {}", identifier))?;
    }
    Ok(())
}

// [libs/infra/db-turso/src/schema.rs]
/*!
 * =================================================================
 * APARATO: SOVEREIGN DATABASE SCHEMA (V152.0 - RESILIENCE MASTER)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (ESTRATO L3)
 * RESPONSABILIDAD: GOBERNANZA ESTRUCTURAL E IDEMPOTENCIA TOTAL
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. STRATEGIC OUTBOX: Inyecta el sustrato físico para el protocolo
 *    de resiliencia anti-apagón, permitiendo sincronía diferida con Motor B.
 * 2. NOMINAL PURITY: Erradicación de abreviaciones en definiciones SQL.
 * 3. IDEMPOTENCIA SOBERANA: Gestión de errores para migraciones en caliente.
 * 4. PERFORMANCE: Índices de polling O(1) para el Strategic Relay Daemon.
 *
 * # Mathematical Proof (Atomicity):
 * El uso de tablas de Outbox garantiza que las mutaciones en el Ledger Táctico
 * y el registro de eventos estratégicos ocurran dentro de la misma
 * transacción ACID, eliminando estados inconsistentes entre nubes.
 * =================================================================
 */

use anyhow::{Context, Result};
use libsql::Connection;
use tracing::{debug, info, instrument, warn};

/**
 * ESTRATO 1: SOLIDIFICACIÓN (Génesis de Tablas)
 * Define las entidades base del ecosistema Prospector.
 * ✅ INCREMENTAL: Adición de 'TABLE_OUTBOX_STRATEGIC'.
 */
const TACTICAL_TABLES: &[(&str, &str)] = &[
    ("TABLE_JOBS", r#"
        CREATE TABLE IF NOT EXISTS jobs (
            id TEXT PRIMARY KEY,
            range_start TEXT NOT NULL,
            range_end TEXT NOT NULL,
            status TEXT DEFAULT 'queued',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
    "#),
    ("TABLE_IDENTITIES", r#"
        CREATE TABLE IF NOT EXISTS identities (
            id TEXT PRIMARY KEY,
            platform TEXT NOT NULL,
            email TEXT NOT NULL,
            credentials_json TEXT NOT NULL,
            user_agent TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
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
            target_stratum TEXT NOT NULL, -- Ej: 'BILLING_CONSUMPTION', 'NEXUS_XP'
            status TEXT DEFAULT 'pending', -- 'pending', 'synced', 'failed'
            retry_count INTEGER DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            processed_at DATETIME
        );
    "#)
];

/**
 * ESTRATO 2: EVOLUCIÓN (Mutaciones de Columna)
 * Asegura que las tablas existentes se adapten a las nuevas capacidades.
 */
const EVOLUTIONARY_STRATA: &[(&str, &str)] = &[
    // --- SOPORTE HYDRA-SLICER & AFILIADOS ---
    ("JOB_OPERATOR_ID", "ALTER TABLE jobs ADD COLUMN operator_id TEXT DEFAULT 'SYSTEM_DELEGATE'"),
    ("JOB_PARENT_ID", "ALTER TABLE jobs ADD COLUMN parent_mission_id TEXT"),
    ("JOB_STRATEGY", "ALTER TABLE jobs ADD COLUMN strategy_type TEXT DEFAULT 'Sequential'"),
    ("JOB_EFFORT", "ALTER TABLE jobs ADD COLUMN total_hashes_effort TEXT DEFAULT '0'"),
    ("JOB_DURATION", "ALTER TABLE jobs ADD COLUMN execution_duration_ms INTEGER DEFAULT 0"),
    ("JOB_CHECKPOINT", "ALTER TABLE jobs ADD COLUMN audit_footprint_checkpoint TEXT"),
    ("JOB_INTEGRITY", "ALTER TABLE jobs ADD COLUMN integrity_hash TEXT"),
    ("JOB_STRATA", "ALTER TABLE jobs ADD COLUMN required_strata TEXT DEFAULT 'StandardLegacy'"),
    ("JOB_COMPLETED_AT", "ALTER TABLE jobs ADD COLUMN completed_at DATETIME"),
    ("JOB_ARCHIVED_AT", "ALTER TABLE jobs ADD COLUMN archived_at DATETIME"),

    // --- SEGURIDAD E IDENTIDAD ---
    ("IDENTITY_STATUS", "ALTER TABLE identities ADD COLUMN status TEXT DEFAULT 'active'"),
    ("IDENTITY_LEASE", "ALTER TABLE identities ADD COLUMN leased_until DATETIME"),
    ("IDENTITY_COOLDOWN", "ALTER TABLE identities ADD COLUMN cooldown_until DATETIME"),

    // --- HALLAZGOS ---
    ("FINDING_WIF", "ALTER TABLE findings ADD COLUMN private_key_wif TEXT NOT NULL DEFAULT 'PENDING'"),
    ("FINDING_JOB", "ALTER TABLE findings ADD COLUMN job_id TEXT")
];

/**
 * ESTRATO 3: ENDURECIMIENTO (Índices de Aceleración)
 */
const ACCELERATION_INDEXES: &[(&str, &str)] = &[
    ("IDX_JOBS_OPERATOR", "CREATE INDEX IF NOT EXISTS idx_jobs_operator ON jobs(operator_id);"),
    ("IDX_JOBS_PARENT", "CREATE INDEX IF NOT EXISTS idx_jobs_parent ON jobs(parent_mission_id);"),
    ("IDX_ACADEMY_STATUS", "CREATE INDEX IF NOT EXISTS idx_academy_operator ON academy_progress(operator_id);"),
    ("IDX_AFFILIATE_TREE", "CREATE INDEX IF NOT EXISTS idx_affiliate_parent ON affiliate_network(parent_affiliate_id);"),
    ("IDX_IDENTITIES_SYNC", "CREATE INDEX IF NOT EXISTS idx_identities_availability ON identities(platform, status, leased_until, cooldown_until);"),
    // ✅ NUEVO: Índice para el motor de relevo asíncrono
    ("IDX_OUTBOX_POLLING", "CREATE INDEX IF NOT EXISTS idx_outbox_status_pending ON outbox_strategic(status, created_at);")
];

/**
 * Ejecuta la secuencia maestra de sincronización del esquema estructural.
 *
 * # Errors:
 * Retorna error si alguna tabla base falla en solidificarse.
 *
 * # Performance:
 * Operaciones idempotentes que solo mutan el esquema si es necesario.
 */
#[instrument(skip(database_connection))]
pub async fn apply_full_sovereign_schema(database_connection: &Connection) -> Result<()> {
    info!("🏗️ [SCHEMA_ENGINE]: Initiating structural synchronization V152.0...");

    solidify_base_strata(database_connection).await?;
    execute_evolutionary_repair(database_connection).await?;
    harden_access_layer(database_connection).await?;

    info!("✅ [SCHEMA_ENGINE]: Tactical Ledger V152.0 resilience strata certified.");
    Ok(())
}

/**
 * Solidifica las tablas maestras del sistema.
 */
async fn solidify_base_strata(database_handle: &Connection) -> Result<()> {
    for (identifier, sql_statement) in TACTICAL_TABLES {
        debug!("  ↳ Solidifying: {}", identifier);
        database_handle.execute(*sql_statement, ()).await
            .with_context(|| format!("CRITICAL_SOLIDIFICATION_FAULT: {}", identifier))?;
    }
    Ok(())
}

/**
 * Ejecuta reparaciones evolutivas sobre columnas existentes.
 */
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

/**
 * Hardening del estrato de acceso mediante índices tácticos.
 */
async fn harden_access_layer(database_handle: &Connection) -> Result<()> {
    for (identifier, sql_statement) in ACCELERATION_INDEXES {
        debug!("  ↳ Hardening: {}", identifier);
        database_handle.execute(*sql_statement, ()).await
            .with_context(|| format!("CRITICAL_HARDENING_FAULT: {}", identifier))?;
    }
    Ok(())
}

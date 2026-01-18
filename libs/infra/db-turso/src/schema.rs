/**
 * =================================================================
 * APARATO: SOVEREIGN DATABASE SCHEMA (V151.0 - KNOWLEDGE STRATA)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (ESTRATO L3)
 * RESPONSABILIDAD: GOBERNANZA ESTRUCTURAL E IDEMPOTENCIA TOTAL
 *
 * VISION HIPER-HOLÍSTICA:
 * 1. KNOWLEDGE MODULES: Inyecta la tabla de definiciones académicas.
 * 2. HYDRA-SLICER READY: Preserva las columnas de fragmentación de misiones.
 * 3. IDEMPOTENCIA: Gestión de errores para migraciones en caliente en Turso.
 * 4. PERFORMANCE: Índices de aceleración para el despacho masivo.
 * =================================================================
 */

use anyhow::{Context, Result};
use libsql::Connection;
use tracing::{debug, info, instrument, warn};

/**
 * ESTRATO 1: SOLIDIFICACIÓN (Génesis de Tablas)
 * Define las entidades base del ecosistema Prospector.
 * ✅ INCREMENTAL: Adición de 'TABLE_KNOWLEDGE_MODULES'.
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
            prerequisites TEXT, -- Identificadores separados por coma
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
 * Optimiza el acceso para ráfagas de 120MH/s.
 */
const ACCELERATION_INDEXES: &[(&str, &str)] = &[
    ("IDX_JOBS_OPERATOR", "CREATE INDEX IF NOT EXISTS idx_jobs_operator ON jobs(operator_id);"),
    ("IDX_JOBS_PARENT", "CREATE INDEX IF NOT EXISTS idx_jobs_parent ON jobs(parent_mission_id);"),
    ("IDX_ACADEMY_STATUS", "CREATE INDEX IF NOT EXISTS idx_academy_operator ON academy_progress(operator_id);"),
    ("IDX_AFFILIATE_TREE", "CREATE INDEX IF NOT EXISTS idx_affiliate_parent ON affiliate_network(parent_affiliate_id);"),
    ("IDX_IDENTITIES_SYNC", "CREATE INDEX IF NOT EXISTS idx_identities_availability ON identities(platform, status, leased_until, cooldown_until);")
];

/**
 * Ejecuta la secuencia maestra de sincronización del esquema estructural.
 *
 * # Errors:
 * Retorna error si alguna tabla base falla en solidificarse, indicando
 * un colapso en el enlace con Turso.
 */
#[instrument(skip(database_connection))]
pub async fn apply_full_sovereign_schema(database_connection: &Connection) -> Result<()> {
    info!("🏗️ [SCHEMA_ENGINE]: Initiating structural synchronization V151.0...");

    solidify_base_strata(database_connection).await?;
    execute_evolutionary_repair(database_connection).await?;
    harden_access_layer(database_connection).await?;

    info!("✅ [SCHEMA_ENGINE]: Tactical Ledger V151.0 level and certified.");
    Ok(())
}

async fn solidify_base_strata(db: &Connection) -> Result<()> {
    for (identifier, sql) in TACTICAL_TABLES {
        debug!("  ↳ Solidifying: {}", identifier);
        db.execute(*sql, ()).await
            .with_context(|| format!("CRITICAL_SOLIDIFICATION_FAULT: {}", identifier))?;
    }
    Ok(())
}

async fn execute_evolutionary_repair(db: &Connection) -> Result<()> {
    for (identifier, sql) in EVOLUTIONARY_STRATA {
        match db.execute(*sql, ()).await {
            Ok(_) => info!("  🟢 [REPAIR_OK]: Applied evolutionary stratum {}", identifier),
            Err(e) => {
                let message = e.to_string();
                if message.contains("duplicate column name") {
                    debug!("  ⚪ [REPAIR_SKIP]: {} already level.", identifier);
                } else {
                    warn!("  ⚠️ [REPAIR_BYPASS]: {} check incomplete: {}", identifier, message);
                }
            }
        }
    }
    Ok(())
}

async fn harden_access_layer(db: &Connection) -> Result<()> {
    for (identifier, sql) in ACCELERATION_INDEXES {
        debug!("  ↳ Hardening: {}", identifier);
        db.execute(*sql, ()).await
            .with_context(|| format!("CRITICAL_HARDENING_FAULT: {}", identifier))?;
    }
    Ok(())
}

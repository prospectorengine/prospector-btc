// [libs/infra/db-turso/src/repositories/identity/queries.rs]
/*!
 * =================================================================
 * APARATO: IDENTITY SQL QUERIES (V36.0 - OMNISCIENT)
 * CLASIFICACIÓN: INFRASTRUCTURE SQL (ESTRATO L3)
 * RESPONSABILIDAD: DEFINICIÓN DE ACCESOS ATÓMICOS AL POOL ZK
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la física de estados de la Bóveda de Identidad.
 * Garantiza la integridad del enjambre mediante bloqueos temporales
 * (Leases) y protocolos de auto-curación (Self-Healing) con
 * trazabilidad total hacia el Proyecto Panóptico.
 * =================================================================
 */

/// Arrienda una identidad con selección determinista y bloqueo atómico.
///
/// # Physics:
/// 1. Filtra por plataforma y estado 'active'.
/// 2. Excluye registros bloqueados (`leased_until`) o en enfriamiento (`cooldown_until`).
/// 3. Aplica Round-Robin implícito mediante `ORDER BY last_used_at ASC`.
/// 4. Captura la identidad del solicitante en `user_agent`.
///
/// # Contrato de Retorno:
/// Devuelve la fila completa post-actualización para mapeo inmediato en L1.
pub const LEASE_SOVEREIGN_IDENTITY: &str = r#"
    UPDATE identities
    SET
        usage_count = usage_count + 1,
        last_used_at = CURRENT_TIMESTAMP,
        leased_until = datetime(CURRENT_TIMESTAMP, '+' || ?2 || ' minutes'),
        user_agent = ?3,
        status = 'active'
    WHERE id = (
        SELECT id FROM identities
        WHERE platform = ?1
          AND status = 'active'
          AND (leased_until IS NULL OR leased_until < CURRENT_TIMESTAMP)
          AND (cooldown_until IS NULL OR cooldown_until < CURRENT_TIMESTAMP)
        ORDER BY last_used_at ASC
        LIMIT 1
    )
    RETURNING id, platform, email, credentials_json, user_agent, status, usage_count, last_used_at, created_at
"#;

/// Degrada el estado de una identidad y activa el protocolo de enfriamiento (24h).
/// Se invoca tras detectar bloqueos de red o fallos de autenticación en Colab.
pub const REPORT_IDENTITY_MALFUNCTION: &str = r#"
    UPDATE identities
    SET
        status = ?2,
        cooldown_until = datetime(CURRENT_TIMESTAMP, '+24 hours'),
        updated_at = CURRENT_TIMESTAMP
    WHERE email = ?1
    RETURNING email
"#;

/// Rompe el bloqueo de arrendamiento de forma preemtiva.
/// Utilizado para intervenciones manuales desde el Dashboard de Gobernanza.
pub const FORCE_RELEASE_IDENTITY_LOCK: &str = r#"
    UPDATE identities
    SET
        leased_until = NULL,
        updated_at = CURRENT_TIMESTAMP
    WHERE email = ?1
    RETURNING email
"#;

/// Protocolo de Incineración: Elimina físicamente el registro de la bóveda.
pub const PURGE_IDENTITY_RECORD: &str = r#"
    DELETE FROM identities
    WHERE email = ?1
    RETURNING email
"#;

/// Protocolo Phoenix: Actualización de material cifrado post-rotación.
/// Reactiva la cuenta y sella las nuevas cookies sin alterar el histórico de uso.
pub const REFRESH_IDENTITY_CREDENTIALS: &str = r#"
    UPDATE identities
    SET
        credentials_json = ?2,
        updated_at = CURRENT_TIMESTAMP,
        last_used_at = CURRENT_TIMESTAMP,
        status = 'active',
        cooldown_until = NULL
    WHERE email = ?1
    RETURNING email
"#;

/// Protocolo de Inmunología: Libera bloqueos de workers muertos (Self-Healing).
/// ✅ MEJORA V36.0: Devuelve los emails afectados para logueo en el Panóptico.
pub const PRUNE_EXPIRED_LEASES: &str = r#"
    UPDATE identities
    SET leased_until = NULL
    WHERE leased_until < CURRENT_TIMESTAMP
    RETURNING email
"#;

/// Recupera métricas de capacidad del pool en tiempo real.
/// Optimizado para el HUD del Dashboard (L5).
pub const GET_IDENTITY_CAPACITY_STATS: &str = r#"
    SELECT
        status,
        COUNT(*) as total,
        SUM(CASE WHEN leased_until > CURRENT_TIMESTAMP THEN 1 ELSE 0 END) as active_leases
    FROM identities
    GROUP BY status
"#;

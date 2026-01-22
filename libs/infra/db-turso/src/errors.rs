// [libs/infra/db-turso/src/errors.rs]
/*!
 * =================================================================
 * APARATO: DATABASE ERROR CATALOG (V180.7 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE CORE (ESTRATO L3)
 * RESPONSABILIDAD: CATALOGACIÓN SEMÁNTICA DE FALLOS DE PERSISTENCIA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. CONFIGURATION AWARENESS: Inyecta 'ConfigurationError' para distinguir
 *    entre fallos de red y variables de entorno vacías (GitHub Actions).
 * 2. PANOPTICON COMPLIANCE: Formatea los mensajes con prefijos de estrato
 *    para su renderizado cromático en el Dashboard Zenith.
 * 3. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta.
 * =================================================================
 */

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    /// Error de enlace físico o de red con el cluster de Turso.
    #[error("[L3_DB_NET_FAULT]: DATABASE_UPLINK_SEVERED -> {0}")]
    ConnectionError(String),

    /// Fallo en la configuración del entorno (Variables vacías o malformadas).
    /// ✅ NIVELACIÓN V180.7: Detecta el "Silencio de GitHub".
    #[error("[L3_DB_CONFIG_FAULT]: STRATEGIC_ENV_VOID -> {0}")]
    ConfigurationError(String),

    /// Error de sintaxis o ejecución devuelto por el motor libSQL.
    #[error("[L3_DB_QUERY_FAULT]: SQL_QUERY_REJECTED -> {0}")]
    QueryError(#[from] libsql::Error),

    /// Fallo en la transformación de tipos entre SQLite y el Dominio Rust.
    #[error("[L3_DB_MAPPING_FAULT]: DATA_MAPPING_VIOLATION -> {0}")]
    MappingError(String),

    // --- ESTRATO DE CONTROL DE MISIONES (L2/L3) ---

    /// La misión solicitada no existe en las tablas activas.
    #[error("[L3_MISSION_FAULT]: IDENTIFIER_NOT_FOUND")]
    MissionNotFound,

    /// Conflicto de propiedad: La misión ya tiene un candado de otro worker.
    #[error("[L3_MISSION_FAULT]: OWNERSHIP_VIOLATION")]
    OwnershipConflict,

    /// La misión no se encuentra en un estado apto para la operación.
    #[error("[L3_MISSION_FAULT]: INVALID_STATE_TRANSITION")]
    InvalidState,

    /// Error al comprometer cambios en una secuencia multi-tabla.
    #[error("[L3_DB_FAULT]: TRANSACTION_COLLAPSE")]
    TransactionError,

    // --- ESTRATO DE GOBERNANZA DE IDENTIDAD (IGFS) ---

    /// La cuenta solicitada no existe en la bóveda.
    #[error("[L3_GOVERNANCE_FAULT]: IDENTITY_NOT_FOUND")]
    IdentityNotFound,

    /// El tiempo de arrendamiento (lease) ha expirado antes del reporte.
    #[error("[L3_GOVERNANCE_FAULT]: IDENTITY_LEASE_EXPIRED")]
    IdentityLeaseExpired,

    // --- ESTRATO DE ARQUEOLOGÍA (DNA) ---

    /// La plantilla de ADN solicitada no está registrada.
    #[error("[L3_DNA_FAULT]: ARTIFACT_NOT_FOUND")]
    DnaArtifactNotFound,
}

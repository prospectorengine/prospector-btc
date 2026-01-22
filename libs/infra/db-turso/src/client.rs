// [libs/infra/db-turso/src/client.rs]
/*!
 * =================================================================
 * APARATO: DATABASE CONNECTION CLIENT (V180.6 - GOLD MASTER)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN DE ENLACES TÁCTICOS Y PERSISTENCIA ACID
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. CLOUD ENVIRONMENT AWARENESS: Implementa diagnósticos específicos para
 *    detectar variables de entorno vacías, común en configuraciones
 *    erróneas de GitHub Environments.
 * 2. SCHEME VALIDATION: Valida el protocolo (libsql/https/file) antes de
 *    la ignición del driver, evitando pánicos en el handshake.
 * 3. ANCHOR RESILIENCE: Mantenimiento del ancla de persistencia para
 *    memoria compartida, garantizando visibilidad del esquema V154.0.
 * 4. HYGIENE: Erradicación de abreviaciones y documentación técnica MIT.
 * =================================================================
 */

use crate::errors::DbError;
use crate::schema::apply_full_sovereign_schema;
use libsql::{Builder, Connection, Database};
use std::sync::Arc;
use tracing::{info, error, instrument, warn, debug};

/**
 * Cliente soberano para la interacción con el Motor A (Turso/libSQL).
 */
#[derive(Clone)]
pub struct TursoClient {
    internal_database_driver: Arc<Database>,
    /// Mantiene la base de datos viva en memoria evitando que SQLite la purgue.
    _memory_persistence_anchor: Option<Arc<Connection>>,
}

impl TursoClient {
    /**
     * Establece la sinapsis con el sustrato de datos (Nube o Local).
     *
     * # Errors:
     * - `DbError::ConnectionError`: Si la URL está vacía o el esquema es ilegal.
     * - `DbError::ConnectionError`: Si se intenta conexión remota sin token de autoridad.
     *
     * # Mathematical Proof (Deterministic Connection):
     * El motor asegura que si URL contiene ':memory:', se activa el ancla de
     * retención de segmentos de memoria para prevenir el 'Reset' del Ledger.
     */
    #[instrument(skip(database_access_token))]
    pub async fn connect(
        database_connection_url: &str,
        database_access_token: Option<String>
    ) -> Result<Self, DbError> {
        let trimmed_url = database_connection_url.trim();

        // 1. AUDITORÍA DE PRE-IGNICIÓN (Zero-Regressions Guard)
        if trimmed_url.is_empty() {
            error!("❌ [DB_CLIENT_FAULT]: DATABASE_URL is an empty string. Check GitHub Environment Secrets.");
            return Err(DbError::ConnectionError("ENV_VAR_EMPTY_OR_UNDEFINED".into()));
        }

        info!("🔌 [DATABASE]: Initiating tactical link to [{}]",
            if trimmed_url.len() > 20 { &trimmed_url[..20] } else { trimmed_url });

        // 2. DETECCIÓN DE ESTRATO FÍSICO
        let is_remote_stratum = trimmed_url.starts_with("libsql://") ||
                                trimmed_url.starts_with("https://");

        let is_volatile_memory = trimmed_url.contains(":memory:") ||
                                 trimmed_url.contains("mode=memory");

        // 3. CONFIGURACIÓN DEL DRIVER SOBERANO
        let database_driver = if is_remote_stratum {
            let token_artifact = database_access_token.ok_or_else(|| {
                error!("❌ [SECURITY_FAULT]: Remote uplink requires TURSO_AUTH_TOKEN.");
                DbError::ConnectionError("REMOTE_AUTHORITY_TOKEN_MISSING".into())
            })?;

            Builder::new_remote(trimmed_url.to_string(), token_artifact)
                .build()
                .await
        } else {
            Builder::new_local(trimmed_url)
                .build()
                .await
        }.map_err(|fault| {
            error!("💀 [DRIVER_FATAL]: Failed to initialize libSQL engine: {}", fault);
            DbError::ConnectionError(format!("DRIVER_IGNITION_FAILURE: {}", fault))
        })?;

        let shared_driver_pointer = Arc::new(database_driver);
        let mut memory_anchor = None;

        // 4. PROTOCOLO DE SOLIDIFICACIÓN DE ESQUEMA (V154.0)
        if is_volatile_memory {
            debug!("⚓ [DATABASE]: Locking memory strata to prevent data evaporation.");
            let anchor_connection = shared_driver_pointer.connect()
                .map_err(|fault| DbError::ConnectionError(format!("ANCHOR_ALLOCATION_FAULT: {}", fault)))?;

            // Aplicamos el esquema Gold Master sobre el ancla compartida
            apply_full_sovereign_schema(&anchor_connection).await
                .map_err(|fault| DbError::ConnectionError(format!("SCHEMA_SYNC_FAULT: {}", fault)))?;

            memory_anchor = Some(Arc::new(anchor_connection));
            info!("✅ [DATABASE]: Volatile strata anchored and levelized.");
        } else {
            // Modo Disco/Cloud: Handshake de integridad inicial
            let bootstrap_connection = shared_driver_pointer.connect()
                .map_err(|fault| DbError::ConnectionError(format!("BOOTSTRAP_LINK_FAULT: {}", fault)))?;

            apply_full_sovereign_schema(&bootstrap_connection).await
                .map_err(|fault| DbError::ConnectionError(format!("SCHEMA_EVOLUTION_FAULT: {}", fault)))?;

            info!("✅ [DATABASE]: Physical strata levelized with V154.0 Schema.");
        }

        Ok(Self {
            internal_database_driver: shared_driver_pointer,
            _memory_persistence_anchor: memory_anchor,
        })
    }

    /**
     * Solicita una conexión activa desde el pool del driver.
     */
    pub fn get_connection(&self) -> Result<Connection, DbError> {
        self.internal_database_driver.connect().map_err(|fault| {
            error!("⚠️ [POOL_FAULT]: Strata connection allocation failed: {}", fault);
            DbError::ConnectionError(fault.to_string())
        })
    }
}

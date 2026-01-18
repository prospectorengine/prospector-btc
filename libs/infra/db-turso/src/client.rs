// INICIO DEL ARCHIVO [libs/infra/db-turso/src/client.rs]
/*!
 * =================================================================
 * APARATO: DATABASE CONNECTION CLIENT (V24.4 - MEMORY FIXED)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN DE ENLACES TÁCTICOS Y PERSISTENCIA ACID
 *
 * # Mathematical Proof (In-Memory Consistency):
 * Para garantizar que el esquema sea visible entre hilos en modo RAM,
 * el cliente ahora detecta URLs de memoria y aplica el ancla de
 * persistencia ANTES de la ejecución del bootstrap, asegurando
 * que las tablas residan en un segmento de memoria compartido.
 * =================================================================
 */

use crate::errors::DbError;
use crate::schema::apply_full_sovereign_schema;
use libsql::{Builder, Connection, Database};
use std::sync::Arc;
use tracing::{info, error, instrument};

#[derive(Clone)]
pub struct TursoClient {
    internal_database_driver: Arc<Database>,
    /// Mantiene la base de datos viva en memoria evitando que SQLite la purgue.
    _memory_persistence_anchor: Option<Arc<Connection>>,
}

impl TursoClient {
    #[instrument(skip(database_access_token))]
    pub async fn connect(
        database_connection_url: &str,
        database_access_token: Option<String>
    ) -> Result<Self, DbError> {
        if database_connection_url.is_empty() {
            return Err(DbError::ConnectionError("CRITICAL_FAULT: DATABASE_URL_UNDEFINED".into()));
        }

        info!("🔌 [DATABASE]: Initiating tactical link synchronization to [{}]", database_connection_url);

        let is_remote = database_connection_url.starts_with("libsql://") ||
                        database_connection_url.starts_with("https://");

        let is_memory = database_connection_url.contains(":memory:") ||
                        database_connection_url.contains("mode=memory");

        let database_driver = if is_remote {
            let token = database_access_token.ok_or_else(|| {
                DbError::ConnectionError("SECURITY_FAULT: Remote access denied (Token missing)".into())
            })?;
            Builder::new_remote(database_connection_url.to_string(), token).build().await
        } else {
            Builder::new_local(database_connection_url).build().await
        }.map_err(|e| DbError::ConnectionError(format!("DRIVER_IGNITION_FAILURE: {}", e)))?;

        let shared_driver = Arc::new(database_driver);
        let mut anchor = None;

        // 🛡️ REPARACIÓN: En modo memoria, abrimos el ancla ANTES que cualquier otra operación
        if is_memory {
            let anchor_conn = shared_driver.connect()
                .map_err(|e| DbError::ConnectionError(format!("ANCHOR_FAULT: {}", e)))?;

            // Aplicamos el esquema directamente sobre el ancla para garantizar persistencia
            apply_full_sovereign_schema(&anchor_conn).await
                .map_err(|e| DbError::ConnectionError(format!("SCHEMA_SYNC_FAULT: {}", e)))?;

            anchor = Some(Arc::new(anchor_conn));
            info!("⚓ [DATABASE]: Memory strata solidified and anchored.");
        } else {
            // En modo Disco/Remoto, usamos una conexión temporal para el bootstrap
            let bootstrap_conn = shared_driver.connect()
                .map_err(|e| DbError::ConnectionError(format!("BOOTSTRAP_LINK_FAULT: {}", e)))?;
            apply_full_sovereign_schema(&bootstrap_conn).await
                .map_err(|e| DbError::ConnectionError(format!("SCHEMA_SYNC_FAULT: {}", e)))?;
        }

        Ok(Self {
            internal_database_driver: shared_driver,
            _memory_persistence_anchor: anchor,
        })
    }

    pub fn get_connection(&self) -> Result<Connection, DbError> {
        self.internal_database_driver.connect().map_err(|e| {
            error!("⚠️ [POOL_FAULT]: Connection allocation failed: {}", e);
            DbError::ConnectionError(e.to_string())
        })
    }
}
// FIN DEL ARCHIVO [libs/infra/db-turso/src/client.rs]

// INICIO DEL ARCHIVO [libs/infra/db-turso/src/repositories/system_repository.rs]
/*!
 * =================================================================
 * APARATO: SYSTEM STATE REPOSITORY (V110.5 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: PERSISTENCIA ACÍDICA DEL ESTADO GLOBAL
 *
 * # Mathematical Proof:
 * Implementa el sellado determinista del Audit Token del censo.
 * Garantiza que la "Fuente de Verdad" del censo sea inmutable
 * durante el ciclo de vida de la misión.
 * =================================================================
 */

use crate::errors::DbError;
use crate::TursoClient;
use libsql::params;
use tracing::{info, instrument, error};

/// Repositorio especializado en metadatos críticos de infraestructura.
pub struct SystemStateRepository {
    database_client: TursoClient,
}

impl SystemStateRepository {
    /**
     * Inicializa una nueva instancia del repositorio.
     * @param database_client Cliente Turso/libSQL activo.
     */
    pub fn new(database_client: TursoClient) -> Self {
        Self { database_client }
    }

    /**
     * Recupera el Token de Auditoría del censo activo.
     * Esencial para verificar la paridad entre el Orquestador y los Workers.
     */
    #[instrument(skip(self))]
    pub async fn retrieve_active_census_audit_token(&self) -> Result<Option<String>, DbError> {
        let connection = self.database_client.get_connection()?;

        let query_statement = "
            SELECT value_text FROM system_state
            WHERE key = 'active_census_audit_token'
            LIMIT 1
        ";

        let mut query_result = connection.query(query_statement, ()).await?;

        if let Some(data_row) = query_result.next().await? {
            let audit_token: String = data_row.get(0)?;
            Ok(Some(audit_token))
        } else {
            Ok(None)
        }
    }

    /**
     * Sella el Audit Token en el Ledger Táctico.
     * Realiza un UPSERT atómico para evitar duplicidad de llaves de configuración.
     *
     * @param new_audit_token Hash SHA-256 del manifiesto de estratos.
     */
    #[instrument(skip(self, new_audit_token))]
    pub async fn seal_system_audit_token(&self, new_audit_token: &str) -> Result<(), DbError> {
        let connection = self.database_client.get_connection()?;

        let sql_statement = "
            INSERT INTO system_state (key, value_text, updated_at)
            VALUES ('active_census_audit_token', ?1, CURRENT_TIMESTAMP)
            ON CONFLICT(key) DO UPDATE SET
                value_text = excluded.value_text,
                updated_at = CURRENT_TIMESTAMP
        ";

        connection.execute(sql_statement, params![new_audit_token]).await
            .map_err(|database_error| {
                error!("❌ [PERSISTENCE_FAULT]: Failed to seal audit token: {}", database_error);
                DbError::QueryError(database_error)
            })?;

        info!("🛡️ [SYSTEM_STATE]: Integrity Audit Token crystallized: [{}]", new_audit_token);
        Ok(())
    }

    /**
     * Verifica si el sistema requiere una re-hidratación del censo.
     */
    pub async fn is_census_outdated(&self, current_manifest_hash: &str) -> Result<bool, DbError> {
        match self.retrieve_active_census_audit_token().await? {
            Some(stored_token) => Ok(stored_token != current_manifest_hash),
            None => Ok(true), // No hay token previo, requiere hidratación
        }
    }
}
// FIN DEL ARCHIVO [libs/infra/db-turso/src/repositories/system_repository.rs]

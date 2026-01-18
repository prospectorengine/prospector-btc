// INICIO DEL ARCHIVO [libs/infra/db-turso/src/repositories/audit_repository.rs]
/*!
 * =================================================================
 * APARATO: STRATEGIC AUDIT REPOSITORY (V50.3 - NOMINAL SYNC)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: REGISTRO ACÍDICO Y CADENA DE CONTINUIDAD
 * =================================================================
 */

use crate::errors::DbError;
use crate::TursoClient;
use libsql::params;
use prospector_domain_models::work::AuditReport;
use sha2::{Sha256, Digest};
use tracing::{info, instrument};

pub struct AuditRepository {
    database_client: TursoClient,
}

impl AuditRepository {
    pub fn new(client: TursoClient) -> Self {
        Self { database_client: client }
    }

    #[instrument(skip(self, report))]
    pub async fn seal_mission_audit_trail(&self, report: &AuditReport) -> Result<String, DbError> {
        let connection = self.database_client.get_connection()?;

        let last_hash_query = "
            SELECT integrity_hash FROM jobs
            WHERE status = 'completed'
            ORDER BY completed_at DESC LIMIT 1
        ";

        let mut rows = connection.query(last_hash_query, ()).await?;
        let previous_integrity_hash: String = if let Some(row) = rows.next().await? {
            row.get(0)?
        } else {
            "PROSPECTOR_GENESIS_V10.8".to_string()
        };

        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(previous_integrity_hash.as_bytes());
        sha256_hasher.update(report.job_mission_identifier.as_bytes());
        sha256_hasher.update(report.total_wallets_audited.as_bytes());
        sha256_hasher.update(report.audit_footprint_checkpoint.as_bytes());
        let current_integrity_hash = format!("{:x}", sha256_hasher.finalize());

        let update_sql = "
            UPDATE jobs
            SET status = 'completed',
                total_hashes_effort = ?2,
                execution_duration_ms = ?3,
                audit_footprint_checkpoint = ?4,
                integrity_hash = ?5,
                completed_at = CURRENT_TIMESTAMP
            WHERE id = ?1
        ";

        connection.execute(update_sql, params![
            report.job_mission_identifier.clone(),
            report.total_wallets_audited.clone(),
            report.execution_duration_milliseconds as i64,
            report.audit_footprint_checkpoint.clone(),
            current_integrity_hash.clone()
        ]).await?;

        info!("🛡️ [AUDIT_SEALED]: Mission {} linked with hash {}",
            report.job_mission_identifier, &current_integrity_hash[0..8]);

        Ok(current_integrity_hash)
    }

    pub async fn get_certified_missions_count(&self) -> Result<u64, DbError> {
        let connection = self.database_client.get_connection()?;
        let mut rows = connection.query("SELECT COUNT(*) FROM jobs WHERE status = 'completed'", ()).await?;

        if let Some(row) = rows.next().await? {
            let count: i64 = row.get(0)?;
            Ok(count as u64)
        } else {
            Ok(0)
        }
    }
}
// FIN DEL ARCHIVO [libs/infra/db-turso/src/repositories/audit_repository.rs]

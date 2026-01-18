// [libs/infra/db-turso/src/repositories/affiliate_repository.rs]
/*!
 * =================================================================
 * APARATO: AFFILIATE RECURSIVE ENGINE (V1.0 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: CÁLCULO DE RECURSIÓN DE RED Y HASHRATE AGREGADO
 *
 * # Mathematical Proof (Recursive CTE):
 * Utiliza una Common Table Expression (CTE) recursiva para recorrer
 * el árbol de afiliados en O(log N), sumando el 'accumulated_hashrate'
 * de cada hoja hacia la raíz del sub-enjambre.
 * =================================================================
 */

use crate::errors::DbError;
use crate::TursoClient;
use libsql::params;
use prospector_domain_models::academy::AffiliateNode;
use tracing::{info, instrument};

pub struct AffiliateRepository {
    database_client: TursoClient,
}

impl AffiliateRepository {
    pub fn new(client: TursoClient) -> Self {
        Self { database_client: client }
    }

    /**
     * Calcula el hashrate total generado por un afiliado y toda su red descendente.
     *
     * # Errors:
     * - DbError::QueryError si la recursión excede el límite de profundidad de SQLite.
     */
    #[instrument(skip(self, root_affiliate_id))]
    pub async fn get_aggregated_network_power(&self, root_affiliate_id: &str) -> Result<f64, DbError> {
        let connection = self.database_client.get_connection()?;

        let sql_recursive_sum = r#"
            WITH RECURSIVE sub_network AS (
                -- Ancla: El nodo raíz
                SELECT affiliate_id, accumulated_hashrate
                FROM affiliate_network
                WHERE affiliate_id = ?1
                UNION ALL
                -- Recursión: Todos los hijos directos e indirectos
                SELECT an.affiliate_id, an.accumulated_hashrate
                FROM affiliate_network an
                JOIN sub_network sn ON an.parent_affiliate_id = sn.affiliate_id
            )
            SELECT SUM(accumulated_hashrate) FROM sub_network
        "#;

        let mut rows = connection.query(sql_recursive_sum, params![root_affiliate_id]).await?;

        if let Some(row) = rows.next().await? {
            let total_power: f64 = row.get(0).unwrap_or(0.0);
            Ok(total_power)
        } else {
            Ok(0.0)
        }
    }

    /**
     * Registra un nuevo nodo en la comunidad vinculándolo a un progenitor.
     */
    #[instrument(skip(self))]
    pub async fn onboard_affiliate(&self, node: &AffiliateNode, referral_code: &str) -> Result<(), DbError> {
        let connection = self.database_client.get_connection()?;

        let sql = r#"
            INSERT INTO affiliate_network (affiliate_id, parent_affiliate_id, referral_code, joined_at)
            VALUES (?1, ?2, ?3, CURRENT_TIMESTAMP)
        "#;

        connection.execute(sql, params![
            node.affiliate_id.clone(),
            node.parent_affiliate_id.clone(),
            referral_code
        ]).await?;

        info!("🤝 [COMMUNITY]: New node {} onboarded to the grid.", node.affiliate_id);
        Ok(())
    }
}

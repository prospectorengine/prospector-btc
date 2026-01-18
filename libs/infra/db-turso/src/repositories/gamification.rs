// [libs/infra/db-turso/src/repositories/gamification.rs]
/*!
 * =================================================================
 * APARATO: NEXUS GAMIFICATION REPOSITORY (V1.3 - OWNERSHIP SECURED)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN DE PRESTIGIO Y PERSISTENCIA DE XP TÁCTICO
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. BORROW CHECKER ALIGNMENT: Resolución definitiva del error E0382.
 *    Sincroniza el ciclo de vida de 'outbox_unique_identifier' mediante
 *    el uso estratégico de referencias en el rastro de depuración.
 * 2. HYGIENE ABSOLUTA: Erradicación de los imports 'info' y 'error' no
 *    utilizados, eliminando ruidos en el log de compilación.
 * 3. DOMAIN CONSISTENCY: Enlace bit-perfecto con 'OperatorRank' y
 *    'AchievementBadge' del estrato L2 (Gamification Domain).
 * 4. NOMINAL PURITY: Mantenimiento de nomenclatura descriptiva absoluta
 *    en cada transición del Ledger Táctico.
 *
 * # Mathematical Proof (Experience Linearity):
 * El aparato garantiza la atomicidad de la ganancia de prestigio mediante
 * el sellado previo en el Outbox Táctico. La relación Hashes -> XP es
 * inmutable hasta que el 'SovereignRelayService' certifique la entrega al HQ.
 * =================================================================
 */

use crate::errors::DbError;
use crate::TursoClient;
use libsql::params;
use prospector_domain_gamification::{OperatorRank, AchievementBadge};
use chrono::Utc;
use uuid::Uuid;
use tracing::{instrument, debug};
use serde_json::json;

/// Identificador nominal del estrato de gamificación en el Outbox.
const NEXUS_STRATUM_IDENTIFIER: &str = "NEXUS_XP_GAIN";

/// Repositorio especializado en la persistencia del estatus y reputación del operador.
pub struct GamificationRepository {
    /// Cliente táctico para el enlace con el cluster de Turso (Motor A).
    database_client: TursoClient,
}

impl GamificationRepository {
    /**
     * Construye una nueva instancia del repositorio Nexus inyectando el cliente táctico.
     */
    pub fn new(client: TursoClient) -> Self {
        Self { database_client: client }
    }

    /**
     * Transforma el esfuerzo computacional certificado en un evento de prestigio.
     *
     * # Errors:
     * - `DbError::QueryError`: Si el sustrato 'outbox_strategic' es inalcanzable.
     *
     * # Performance:
     * Operación O(1). Utiliza serialización JSONB para flexibilidad en el Motor B.
     */
    #[instrument(skip(self, operator_identifier, computational_effort_volume, mission_identifier))]
    pub async fn record_computational_prestige(
        &self,
        operator_identifier: &str,
        computational_effort_volume: u64,
        mission_identifier: &str
    ) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;

        let outbox_unique_identifier = Uuid::new_v4().to_string();

        // 1. CONSTRUCCIÓN DEL ARTEFACTO DE PRESTIGIO
        let prestige_payload_artifact = json!({
            "operator_id": operator_identifier,
            "hashes_audited": computational_effort_volume,
            "mission_id": mission_identifier,
            "conversion_ratio": 0.0001, // 1 XP por cada 10k hashes nominales
            "timestamp": Utc::now().to_rfc3339()
        });

        // ✅ RESOLUCIÓN E0382: Logueamos ANTES de entregar la propiedad a params![]
        // O utilizamos una referencia para evitar el 'move' prematuro.
        debug!("🏆 [NEXUS_OUTBOX]: prestige_gain_queued identifier=[{}] operator=[{}]",
            &outbox_unique_identifier, operator_identifier);

        let sql_statement = "
            INSERT INTO outbox_strategic (outbox_identifier, payload_json, target_stratum, status)
            VALUES (?1, ?2, ?3, 'pending')
        ";

        // 2. SELLO EN EL LEDGER TÁCTICO (OWNERSHIP MOVE)
        database_connection.execute(sql_statement, params![
            outbox_unique_identifier,
            prestige_payload_artifact.to_string(),
            NEXUS_STRATUM_IDENTIFIER
        ]).await?;

        Ok(())
    }

    /**
     * Recupera el rango y maestría del operador desde el caché local.
     *
     * # Performance:
     * Operación indexada O(1) sobre la tabla de estado del sistema.
     */
    #[instrument(skip(self, operator_identifier))]
    pub async fn get_operator_rank(&self, operator_identifier: &str) -> Result<OperatorRank, DbError> {
        let database_connection = self.database_client.get_connection()?;

        let sql_query = "
            SELECT value_text, value_int
            FROM system_state
            WHERE key = ?1
        ";

        let rank_cache_key = format!("rank_{}", operator_identifier);
        let mut query_results = database_connection.query(sql_query, params![rank_cache_key]).await?;

        if let Some(data_row) = query_results.next().await? {
            let rank_title_label: String = data_row.get(0)?;
            let total_experience_points: i64 = data_row.get(1)?;

            Ok(OperatorRank {
                level: (total_experience_points / 1000) as u32 + 1,
                title: rank_title_label,
                experience_points: total_experience_points as u64,
                next_level_threshold: (((total_experience_points / 1000) + 1) * 1000) as u64,
            })
        } else {
            // Sello de Operador Novato (Génesis)
            Ok(OperatorRank {
                level: 1,
                title: "Novice_Archaeologist".to_string(),
                experience_points: 0,
                next_level_threshold: 1000,
            })
        }
    }

    /**
     * Recupera insignias de logro vinculando la academia con la reputación.
     */
    #[instrument(skip(self, operator_identifier))]
    pub async fn fetch_unlocked_achievements(
        &self,
        operator_identifier: &str
    ) -> Result<Vec<AchievementBadge>, DbError> {
        let database_connection = self.database_client.get_connection()?;

        let sql_query = "
            SELECT module_identifier, completed_at
            FROM academy_progress
            WHERE operator_id = ?1 AND status = 'completed'
        ";

        let mut query_results = database_connection.query(sql_query, params![operator_identifier]).await?;
        let mut achievements_collection = Vec::new();

        while let Some(data_row) = query_results.next().await? {
            let module_id: String = data_row.get(0)?;
            achievements_collection.push(AchievementBadge {
                identifier: module_id.clone(),
                i18n_label_key: format!("ACHIEVEMENT_{}", module_id),
                unlocked_at: data_row.get(1)?,
            });
        }

        Ok(achievements_collection)
    }
}

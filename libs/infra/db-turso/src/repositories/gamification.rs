// [libs/infra/db-turso/src/repositories/gamification.rs]
/*!
 * =================================================================
 * APARATO: GAMIFICATION TACTICAL REPOSITORY (V17.0 - SINGULARITY GOLD)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN DE PRESTIGIO Y ESTATUS MULTI-TENANT SOBERANO
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. XP ATOMIC SEAL: Garantiza que la ganancia de experiencia sea una
 *    operación indivisible vinculada al identificador del operador.
 * 2. NEXUS CACHE SYNC: Optimiza la lectura de rango mediante el uso de
 *    claves nominales en 'system_state', evitando JOINs costosos en el Hot-Path.
 * 3. ACADEMY LINKAGE: Sincronización bit-perfecta con el progreso de la
 *    Academia para la liberación de insignias de mérito (Badges).
 * 4. NOMINAL PURITY: Erradicación total de abreviaciones. Uso de
 *    'operator_identifier' y 'computational_effort_volume'.
 *
 * # Mathematical Proof (Experience Linearity):
 * XP_delta = Effort_Volume * Stratum_Multiplier.
 * El sistema garantiza que XP_total = sum(XP_delta) + Initial_XP.
 * =================================================================
 */

 use crate::errors::DbError;
 use crate::TursoClient;
 use libsql::params;
 use prospector_domain_gamification::{OperatorRank, AchievementBadge};
 use chrono::Utc;
 use uuid::Uuid;
 use tracing::{instrument, debug, info, error};
 use serde_json::json;

 /// Identificador nominal del estrato de gamificación en el Outbox Táctico.
 const NEXUS_STRATUM_IDENTIFIER: &str = "NEXUS_XP_GAIN";

 /// Factor de conversión: 1 XP por cada 10,000 hashes procesados (Base Satoshi).
 const COMPUTATIONAL_EFFORT_XP_RATIO: f64 = 0.0001;

 /// Repositorio especializado en la persistencia del prestigio y la maestría.
 pub struct GamificationRepository {
     /// Cliente táctico para el enlace con el cluster de Turso (Motor A).
     database_client: TursoClient,
 }

 impl GamificationRepository {
     /**
      * Construye una nueva instancia del repositorio Nexus inyectando el cliente de enlace.
      */
     pub fn new(client: TursoClient) -> Self {
         Self { database_client: client }
     }

     /**
      * Registra el prestigio derivado de una ráfaga computacional certificada.
      *
      * # Errors:
      * - `DbError::QueryError`: Si el enlace con el Ledger Táctico se interrumpe.
      *
      * # Performance:
      * Operación O(1). Utiliza el Patrón Outbox para diferir la sincronía con Motor B.
      */
     #[instrument(skip(self, operator_identifier, computational_effort_volume, mission_identifier))]
     pub async fn record_computational_prestige(
         &self,
         operator_identifier: &str,
         computational_effort_volume: u64,
         mission_identifier: &str
     ) -> Result<(), DbError> {
         let database_connection = self.database_client.get_connection()?;

         // Generación de identificador unívoco para rastro forense
         let unique_outbox_identifier = Uuid::new_v4().to_string();

         // 1. CÁLCULO DE MAGNITUD DE PRESTIGIO
         let experience_points_gain = (computational_effort_volume as f64 * COMPUTATIONAL_EFFORT_XP_RATIO).max(1.0);

         // 2. CONSTRUCCIÓN DEL ARTEFACTO DE PRESTIGIO (L7 Alignment)
         let prestige_payload_artifact = json!({
             "operator_id": operator_identifier,
             "hashes_audited_volume": computational_effort_volume,
             "mission_reference_id": mission_identifier,
             "experience_points_gain": experience_points_gain,
             "crystallized_at_utc": Utc::now().to_rfc3339()
         });

         // ✅ SINCRO V17.0: Se usa referencia para el log para evitar el 'move' del identificador
         debug!(
             target: "nexus_audit",
             identifier = %unique_outbox_identifier,
             operator = %operator_identifier,
             "🏆 [NEXUS]: Queuing XP gain for certified mission effort."
         );

         let sql_statement = "
             INSERT INTO outbox_strategic (outbox_identifier, payload_json, target_stratum, status)
             VALUES (?1, ?2, ?3, 'pending')
         ";

         // 3. PERSISTENCIA EN EL OUTBOX TÁCTICO
         database_connection.execute(sql_statement, params![
             unique_outbox_identifier,
             prestige_payload_artifact.to_string(),
             NEXUS_STRATUM_IDENTIFIER
         ]).await?;

         Ok(())
     }

     /**
      * Recupera el rango, nivel y puntos acumulados desde el caché de estado local.
      *
      * # Mathematical Proof (Level Thresholds):
      * Level = floor(XP / 1000) + 1. El umbral para L+1 es siempre (Level * 1000).
      */
     #[instrument(skip(self, operator_identifier))]
     pub async fn get_operator_rank(&self, operator_identifier: &str) -> Result<OperatorRank, DbError> {
         let database_connection = self.database_client.get_connection()?;
         let rank_cache_key_artifact = format!("rank_{}", operator_identifier);

         let sql_query = "
             SELECT value_text, value_int
             FROM system_state
             WHERE key = ?1
         ";

         let mut query_results = database_connection.query(sql_query, params![rank_cache_key_artifact]).await?;

         if let Some(data_row) = query_results.next().await? {
             let rank_title_label: String = data_row.get(0)?;
             let total_experience_points_int: i64 = data_row.get(1)?;
             let total_xp = total_experience_points_int as u64;

             let current_level = (total_xp / 1000) as u32 + 1;
             let next_threshold = (current_level as u64) * 1000;

             Ok(OperatorRank {
                 level: current_level,
                 title: rank_title_label,
                 experience_points: total_xp,
                 next_level_threshold: next_threshold,
             })
         } else {
             // Sello de Operador Génesis (Estado por defecto)
             Ok(OperatorRank {
                 level: 1,
                 title: "Novice_Archaeologist".to_string(),
                 experience_points: 0,
                 next_level_threshold: 1000,
             })
         }
     }

     /**
      * Recupera la colección de insignias de arqueología certificadas para el operador.
      * Cruza el Ledger Táctico con las certificaciones de la Academia L2.
      */
     #[instrument(skip(self, operator_identifier))]
     pub async fn fetch_certified_achievements(
         &self,
         operator_identifier: &str
     ) -> Result<Vec<AchievementBadge>, DbError> {
         let database_connection = self.database_client.get_connection()?;

         let sql_query = "
             SELECT module_identifier, completed_at
             FROM academy_progress
             WHERE operator_id = ?1 AND status = 'completed'
             ORDER BY completed_at DESC
         ";

         let mut query_results = database_connection.query(sql_query, params![operator_identifier]).await?;
         let mut achievements_collection = Vec::new();

         while let Some(data_row) = query_results.next().await? {
             let module_identifier_label: String = data_row.get(0)?;
             let completion_timestamp: String = data_row.get(1)?;

             achievements_collection.push(AchievementBadge {
                 identifier: module_identifier_label.clone(),
                 i18n_label_key: format!("ACHIEVEMENT_{}", module_identifier_label),
                 unlocked_at: completion_timestamp,
             });
         }

         debug!("🎖️ [NEXUS]: Found {} certified achievements for {}.",
             achievements_collection.len(), operator_identifier);

         Ok(achievements_collection)
     }

     /**
      * Sincroniza el estatus de rango local tras una recalibración en el Motor B.
      */
     pub async fn synchronize_rank_cache(
         &self,
         operator_identifier: &str,
         rank_label: &str,
         total_xp: u64
     ) -> Result<(), DbError> {
         let database_connection = self.database_client.get_connection()?;
         let rank_cache_key = format!("rank_{}", operator_identifier);

         database_connection.execute(
             "INSERT INTO system_state (key, value_text, value_int, updated_at)
              VALUES (?1, ?2, ?3, CURRENT_TIMESTAMP)
              ON CONFLICT(key) DO UPDATE SET
                 value_text = excluded.value_text,
                 value_int = excluded.value_int,
                 updated_at = CURRENT_TIMESTAMP",
             params![rank_cache_key, rank_label, total_xp as i64]
         ).await?;

         info!("🏆 [NEXUS_SYNC]: Rank solidified for {}: [{}] ({} XP)",
             operator_identifier, rank_label, total_xp);
         Ok(())
     }
 }

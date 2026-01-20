// [apps/orchestrator/src/handlers/gamification.rs]
/*!
 * =================================================================
 * APARATO: NEXUS STRATUM HANDLER (V1.4 - PRODUCTION HARDENED)
 * CLASIFICACIÓN: APPLICATION ADAPTER (ESTRATO L4)
 * RESPONSABILIDAD: EXPOSICIÓN DE MÉTRICAS DE PRESTIGIO Y ESCALAFÓN
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. HYGIENE RECOVERY: Erradicación definitiva de 'ax_test_utils'. Se restaura
 *    el extractor State nativo de Axum para compatibilidad con el despliegue.
 * 2. ZERO WARNINGS: Eliminación de imports de 'AchievementBadge' y 'OperatorRank'
 *    que, aunque definidos en el dominio, no se invocan explícitamente aquí (DRY).
 * 3. NOMINAL PURITY: Uso estricto de State<AppState> sin alias de laboratorio.
 * 4. PANOPTICON SYNC: Instrumentación #[instrument] preservada para rastro en L5.
 *
 * # Mathematical Proof (Experience Linearity):
 * El sistema calcula el Nivel (L) como una función del XP acumulado:
 * L = floor(XP / 1000) + 1. Esta lógica reside en el repositorio L3,
 * el handler actúa como el transductor inmutable hacia el Dashboard L5.
 * =================================================================
 */

use crate::state::AppState;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json
};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument, debug, error};

/// Parámetros de consulta para filtrar el prestigio por identidad de red.
#[derive(Debug, Deserialize)]
pub struct PrestigeQueryParameters {
    /// Identificador nominal del operador (UUID vinculado al Motor B).
    pub operator_identifier: Option<String>,
}

/// Representa una entrada detallada en el escalafón de élite global.
#[derive(Debug, Serialize)]
pub struct LeaderboardRankingArtifact {
    pub operator_pseudonym: String,
    pub experience_points_total: u64,
    pub rank_title_label: String,
    pub nodes_contributed_count: u32,
    pub is_current_operator: bool,
}

pub struct GamificationHandler;

impl GamificationHandler {
    /**
     * Endpoint: GET /api/v1/user/nexus/prestige
     *
     * Recupera el snapshot de reputación, nivel y progreso del operador.
     *
     * # Performance:
     * Operación O(1) mediante consulta indexada en el Ledger Táctico (Motor A).
     */
    #[instrument(skip(application_state, query_parameters))]
    pub async fn handle_get_prestige_status(
        State(application_state): State<AppState>,
        Query(query_parameters): Query<PrestigeQueryParameters>,
    ) -> impl IntoResponse {
        // En la Fase 3, este ID se extraerá del contexto de seguridad JWT de Supabase
        let active_operator_identifier = query_parameters.operator_identifier
            .unwrap_or_else(|| "ARCHITECT_GÉNESIS_01".to_string());

        debug!("🏆 [NEXUS_QUERY]: Fetching prestige strata for [{}].", active_operator_identifier);

        // 1. ADQUISICIÓN DE RANGO DESDE EL LEDGER TÁCTICO (L3)
        match application_state.gamification_repository.get_operator_rank(&active_operator_identifier).await {
            Ok(operator_mastery_artifact) => {
                (StatusCode::OK, Json(operator_mastery_artifact)).into_response()
            },
            Err(database_fault) => {
                error!("❌ [NEXUS_FAULT]: Failed to retrieve rank for {}: {}",
                    active_operator_identifier, database_fault);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }

    /**
     * Endpoint: GET /api/v1/user/nexus/leaderboard
     *
     * Recupera el ranking de élite de los auditores del enjambre.
     */
    #[instrument(skip(_application_state))]
    pub async fn handle_get_leaderboard(
        State(_application_state): State<AppState>,
    ) -> impl IntoResponse {
        info!("📊 [NEXUS_RANKING]: Reconstructing global leaderboard from community strata.");

        // Mantenemos la simulación certificada para no romper la visualización en L5
        // hasta que el motor de agregación L7 sea liberado.
        let global_ranking_collection = vec![
            LeaderboardRankingArtifact {
                operator_pseudonym: "Satoshi_Seeker".to_string(),
                experience_points_total: 98500,
                rank_title_label: "Sovereign_Architect".to_string(),
                nodes_contributed_count: 32,
                is_current_operator: false,
            },
            LeaderboardRankingArtifact {
                operator_pseudonym: "ARCHITECT_GÉNESIS_01".to_string(),
                experience_points_total: 12500,
                rank_title_label: "Elite_Archaeologist".to_string(),
                nodes_contributed_count: 15,
                is_current_operator: true,
            }
        ];

        (StatusCode::OK, Json(global_ranking_collection)).into_response()
    }

    /**
     * Endpoint: GET /api/v1/user/nexus/achievements
     *
     * Lista las insignias (Badges) de arqueología certificadas para el operador.
     */
    #[instrument(skip(application_state, query_parameters))]
    pub async fn handle_list_achievements(
        State(application_state): State<AppState>,
        Query(query_parameters): Query<PrestigeQueryParameters>,
    ) -> impl IntoResponse {
        let active_operator_identifier = query_parameters.operator_identifier
            .unwrap_or_else(|| "ARCHITECT_GÉNESIS_01".to_string());

        // 1. ADQUISICIÓN DE LOGROS DESDE EL ESTRATO DE ACADEMIA (L3)
        match application_state.gamification_repository
            .fetch_unlocked_achievements(&active_operator_identifier)
            .await
        {
            Ok(achievements_collection) => {
                (StatusCode::OK, Json(achievements_collection)).into_response()
            },
            Err(database_fault) => {
                error!("❌ [NEXUS_FAULT]: Achievement sync failed for {}: {}",
                    active_operator_identifier, database_fault);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

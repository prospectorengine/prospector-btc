// [apps/orchestrator/src/graphql/academy.rs]
/**
 * =================================================================
 * APARATO: NEURAL ACADEMY ORACLE (V3.2 - ZENITH SINCRO)
 * CLASIFICACIÓN: GRAPHQL LOGIC (ESTRATO L4)
 * RESPONSABILIDAD: ORQUESTACIÓN DE CONOCIMIENTO Y CÁLCULO DE RED
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. CONTRACT PARITY: Implementa 'get_operator_mastery' para satisfacer
 *    la demanda visual del Dashboard L5, eliminando el warning de
 *    importación no utilizada (E0026).
 * 2. ZERO ABBREVIATIONS: 'op_id' -> 'operator_identifier', 'res' -> 'query_result'.
 * 3. NOMINAL SYMMETRY: Sincroniza los estados de módulos (LOCKED/UNLOCKED)
 *    basándose en la recursión de prerrequisitos.
 * 4. HYGIENE: Documentación técnica nivel Tesis Doctoral y rastro forense #[instrument].
 *
 * # Mathematical Proof (Inference Engine):
 * El motor de inferencia garantiza que un módulo 'M' solo alcance el
 * estado 'Unlocked' si el conjunto de sus prerrequisitos 'P' es un
 * subconjunto del conjunto de módulos completados 'C' (P ⊆ C).
 * =================================================================
 */

use async_graphql::{Object, Context, Result, Error};
use prospector_domain_models::academy::{
    KnowledgeModule, DifficultyLevel, ModuleStatus,
    AffiliateNode, OperatorAcademyProgress
};
use prospector_infra_db::TursoClient;
use prospector_infra_db::repositories::AffiliateRepository;
use tracing::{info, instrument, error, debug};
use std::collections::HashSet;

#[derive(Default)]
pub struct AcademyQuery;

#[Object]
impl AcademyQuery {
    /**
     * Punto de control de integridad de la Academia.
     */
    #[instrument(skip(self, _ctx))]
    async fn academy_status(&self, _ctx: &Context<'_>) -> String {
        "ACADEMY_DYNAMIC_ORACLE_V3.2_ZENITH_ACTIVE".to_string()
    }

    /**
     * Recupera el currículum adaptativo inyectando el estado del operador.
     *
     * # Performance:
     * Utiliza un HashSet para la resolución de prerrequisitos en O(1),
     * optimizando el renderizado de grafos de conocimiento complejos.
     */
    #[instrument(skip(self, ctx, operator_identifier), fields(op = %operator_identifier))]
    async fn get_adaptive_curriculum(
        &self,
        ctx: &Context<'_>,
        operator_identifier: String
    ) -> Result<Vec<KnowledgeModule>> {
        let database_client = ctx.data::<TursoClient>()
            .map_err(|_| Error::new("INFRA_FAULT: Strategic link to Engine A severed."))?;

        let connection = database_client.get_connection()
            .map_err(|e| Error::new(format!("POOL_FAULT: {}", e)))?;

        info!("📚 [ORACLE]: Reconstructing knowledge graph for operator [{}].", operator_identifier);

        // 1. ADQUISICIÓN DE DEFINICIONES MAESTRAS
        let mut modules_rows = connection.query(
            "SELECT identifier, i18n_title_key, i18n_description_key, difficulty,
                    duration_minutes, visual_icon, prerequisites
             FROM knowledge_modules ORDER BY identifier ASC",
            ()
        ).await.map_err(|e| Error::new(format!("DB_READ_ERROR: {}", e)))?;

        let mut master_modules_collection = Vec::new();
        while let Some(row) = modules_rows.next().await.map_err(|e| Error::new(e.to_string()))? {
            let difficulty_label: String = row.get(3)?;
            let prerequisites_raw_string: String = row.get(6)?;

            master_modules_collection.push(KnowledgeModule {
                identifier: row.get(0)?,
                i18n_title_key: row.get(1)?,
                i18n_description_key: row.get(2)?,
                difficulty: match difficulty_label.as_str() {
                    "Elite" => DifficultyLevel::Elite,
                    "Intermediate" => DifficultyLevel::Intermediate,
                    _ => DifficultyLevel::Foundation,
                },
                estimated_duration_minutes: row.get::<i64>(4)? as u32,
                current_status: ModuleStatus::Locked,
                visual_icon_signature: row.get(5)?,
                prerequisite_identifiers: prerequisites_raw_string.split(',')
                    .filter(|identifier_slice| !identifier_slice.is_empty())
                    .map(|identifier_slice| identifier_slice.to_string())
                    .collect(),
            });
        }

        // 2. ADQUISICIÓN DE PROGRESO DEL OPERADOR
        let mut progress_rows = connection.query(
            "SELECT module_identifier FROM academy_progress
             WHERE operator_id = ?1 AND status = 'completed'",
            [operator_identifier.clone()]
        ).await.map_err(|e| Error::new(e.to_string()))?;

        let mut completed_modules_set = HashSet::new();
        while let Some(row) = progress_rows.next().await.map_err(|e| Error::new(e.to_string()))? {
            completed_modules_set.insert(row.get::<String>(0)?);
        }

        // 3. INFERENCIA LÓGICA DE ESTADOS
        for module_artifact in master_modules_collection.iter_mut() {
            if completed_modules_set.contains(&module_artifact.identifier) {
                module_artifact.current_status = ModuleStatus::Completed;
            } else {
                let prerequisites_satisfied = module_artifact.prerequisite_identifiers.iter()
                    .all(|req_id| completed_modules_set.contains(req_id));

                module_artifact.current_status = if prerequisites_satisfied {
                    ModuleStatus::Unlocked
                } else {
                    ModuleStatus::Locked
                };
            }
        }

        Ok(master_modules_collection)
    }

    /**
     * Recupera las métricas de maestría consolidadas del operador.
     * ✅ RESOLUCIÓN WARNING: Uso nominal de OperatorAcademyProgress.
     */
    #[instrument(skip(self, ctx, operator_identifier), fields(op = %operator_identifier))]
    async fn get_operator_mastery(
        &self,
        ctx: &Context<'_>,
        operator_identifier: String
    ) -> Result<OperatorAcademyProgress> {
        let database_client = ctx.data::<TursoClient>()
            .map_err(|_| Error::new("INFRA_FAULT: Strategic link severed."))?;

        let connection = database_client.get_connection()
            .map_err(|e| Error::new(e.to_string()))?;

        // 1. Cálculo de módulos certificados
        let mut count_query = connection.query(
            "SELECT COUNT(*) FROM academy_progress WHERE operator_id = ?1 AND status = 'completed'",
            [operator_identifier.clone()]
        ).await.map_err(|e| Error::new(e.to_string()))?;

        let certified_count = if let Some(row) = count_query.next().await? {
            row.get::<i64>(0)? as u32
        } else { 0 };

        // 2. Cálculo de tiempo acumulado de minería (Sincronizado con L3)
        let mut time_query = connection.query(
            "SELECT SUM(duration_minutes) FROM knowledge_modules WHERE identifier IN (
                SELECT module_identifier FROM academy_progress WHERE operator_id = ?1 AND status = 'completed'
            )",
            [operator_identifier.clone()]
        ).await.map_err(|e| Error::new(e.to_string()))?;

        let total_minutes = if let Some(row) = time_query.next().await? {
            row.get::<i64>(0).unwrap_or(0) as u32
        } else { 0 };

        debug!("📊 [MASTERY]: Operator {} has certified {} modules.", operator_identifier, certified_count);

        Ok(OperatorAcademyProgress {
            operator_id: operator_identifier,
            certified_modules_count: certified_count,
            total_mining_time_minutes: total_minutes,
            master_stratum_level: (certified_count / 2).max(1), // Heurística de nivel base
        })
    }

    /**
     * Recupera el nodo de red y calcula recursivamente la potencia del sub-enjambre.
     */
    #[instrument(skip(self, ctx))]
    async fn get_affiliate_node(&self, ctx: &Context<'_>, affiliate_id: String) -> Result<AffiliateNode> {
        let database_client = ctx.data::<TursoClient>()
            .map_err(|_| Error::new("INFRA_FAULT: Strategic link severed."))?;

        let affiliate_repository_engine = AffiliateRepository::new(database_client.clone());

        let connection = database_client.get_connection()
            .map_err(|e| Error::new(e.to_string()))?;

        let mut query_result = connection.query(
            "SELECT parent_affiliate_id, joined_at FROM affiliate_network WHERE affiliate_id = ?1",
            [affiliate_id.clone()]
        ).await.map_err(|e| Error::new(e.to_string()))?;

        if let Some(data_row) = query_result.next().await? {
            // CÁLCULO RECURSIVO SOBERANO (O1 mediante CTE en L3)
            let aggregated_hashrate_magnitude = affiliate_repository_engine.get_aggregated_network_power(&affiliate_id)
                .await
                .map_err(|e| Error::new(e.to_string()))?;

            Ok(AffiliateNode {
                affiliate_id,
                parent_affiliate_id: data_row.get(0)?,
                network_depth: 0, // Placeholder para el motor de profundidad de la Fase 3
                contribution_hashrate: aggregated_hashrate_magnitude,
                joined_at_timestamp: data_row.get(1)?,
            })
        } else {
            error!("🤝 [AFFILIATE_FAULT]: Node {} not found in community strata.", affiliate_id);
            Err(Error::new("AFFILIATE_NOT_FOUND"))
        }
    }
}

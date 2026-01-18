// [apps/orchestrator/src/graphql/mod.rs]
/**
 * =================================================================
 * APARATO: NEURAL GRAPHQL GATEWAY (V2.8 - VISIBILITY HARDENED)
 * CLASIFICACIÓN: API GATEWAY (ESTRATO L4)
 * RESPONSABILIDAD: ORÁCULO DE DATOS RELACIONALES Y ACADEMIA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. ENCAPSULATION SOVEREIGNTY: Resolución de errores de importación privada.
 *    Se garantiza que NeuralSchema y build_neural_schema sean accesibles
 *    para el orquestador de estado.
 * 2. NOMINAL PARITY: Alineación bit-perfecta con el constructor de AppState.
 * 3. HYGIENE: Erradicación de redundancias y variables muertas.
 * =================================================================
 */

pub mod academy;

use async_graphql::{Context, Object, Result, MergedObject, EmptySubscription, EmptyMutation, Schema};
use std::sync::Arc;
use prospector_infra_db::TursoClient;
use crate::services::event_bus::EventBus;
use tracing::{ instrument, debug, info};

/// Definición pública del esquema neuronal. Requerido por el AppState (L1-APP).
pub type NeuralSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

/// Resolver de Infraestructura para el diagnóstico de salud del núcleo.
#[derive(Default, Debug, Clone, Copy)]
pub struct SystemHealthQuery;

#[Object]
impl SystemHealthQuery {
    /**
     * Certifica la integridad del Oráculo y el enlace con el Motor A.
     */
    #[instrument(skip(self, context))]
    async fn neural_gateway_status(&self, context: &Context<'_>) -> Result<String> {
        debug!("🧠 [ORACLE]: Pulsing tactical ledger...");

        let database_client = context.data::<TursoClient>()
            .map_err(|_| "CRITICAL_FAULT: Database context void.")?;

        let database_connection = database_client.get_connection()
            .map_err(|fault| format!("STRATA_L3_UNREACHABLE: {}", fault))?;

        database_connection.execute("SELECT 1", ())
            .await
            .map_err(|fault| format!("STRATA_L3_FROZEN: {}", fault))?;

        Ok("ZENITH_ORACLE_V2.8_ACTIVE_OPERATIONAL".to_string())
    }
}

/// Raíz Unificada del Grafo.
#[derive(MergedObject, Default)]
pub struct QueryRoot(SystemHealthQuery, academy::AcademyQuery);

/**
 * Factoría Soberana de Construcción del Esquema.
 * ✅ RESOLUCIÓN: Marcada como 'pub' para permitir la ignición desde el AppState.
 */
pub fn build_neural_schema(
    database_client: TursoClient,
    event_bus: Arc<EventBus>
) -> NeuralSchema {
    info!("🧬 [ORACLE_IGNITION]: Crystallizing Neural GraphQL Schema V2.8...");

    Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .data(database_client)
        .data(event_bus)
        .finish()
}

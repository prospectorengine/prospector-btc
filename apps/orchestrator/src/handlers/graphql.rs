// [apps/orchestrator/src/handlers/graphql.rs]
/**
 * =================================================================
 * APARATO: GRAPHQL HTTP GATEWAY (V43.1 - VERSION SHIELD)
 * CLASIFICACIÓN: API ADAPTER (ESTRATO L4)
 * RESPONSABILIDAD: TRANSPORTE RESILIENTE E INDEPENDIENTE DE CRATES
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * Implementa un puente de comunicación de "Cero Dependencia de Trait".
 * 1. VERSION SHIELD: Usa axum::Json nativo para evitar colisiones de versiones
 *    entre async-graphql-axum y el núcleo del orquestador.
 * 2. PERFORMANCE: Serialización directa del resultado del motor GQL.
 * 3. HYGIENE: Limpieza total de imports ambiguos y firmas frágiles.
 * =================================================================
 */

use crate::state::AppState;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use axum::{
    extract::{State, Json},
    response::{Html, IntoResponse},
};
use tracing::{instrument, debug};

/**
 * Procesa peticiones GraphQL mediante serialización nativa de Axum.
 *
 * # Mathematical Proof (Version Independence):
 * Al encapsular el resultado en axum::Json, el compilador solo requiere
 * que el tipo de datos sea 'Serialize', eliminando la necesidad de que
 * las versiones de Axum coincidan entre la aplicación y el adaptador.
 */
#[instrument(skip(state, request_payload), fields(op = ?request_payload.operation_name))]
pub async fn handle_graphql_query(
    State(state): State<AppState>,
    Json(request_payload): Json<async_graphql::Request>,
) -> impl IntoResponse {
    debug!("🧠 [ORACLE]: Ingesting neural query signal...");

    // 1. Ejecución soberana contra el esquema inyectado en RAM
    let query_response = state.graphql_schema.execute(request_payload).await;

    // 2. Auditoría de integridad de la consulta
    if query_response.is_err() {
        warn!("⚠️ [ORACLE_ALERT]: Response generated with internal strata errors.");
    }

    // 3. Retorno vía Json Nativo
    // ✅ RESOLUCIÓN DEFINITIVA: Evita el uso del trait IntoResponse de async-graphql-axum
    // que causaba el fallo de compilación en Render.
    Json(query_response)
}

/**
 * Renderiza el Playground del Oráculo (Interfaz Académica).
 * Provee la documentación viva del esquema para el operador.
 */
pub async fn handle_playground() -> impl IntoResponse {
    // Configuración apuntando al endpoint táctico relativo
    let config = GraphQLPlaygroundConfig::new("/api/v1/graphql");
    Html(playground_source(config))
}

// Inyección de macro de aviso para evitar warnings de tracing no usados si aplica
use ::tracing::warn;

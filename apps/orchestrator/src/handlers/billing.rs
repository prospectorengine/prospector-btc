// [apps/orchestrator/src/handlers/billing.rs]
/*!
 * =================================================================
 * APARATO: BILLING STRATUM HANDLER (V1.3 - MULTI-TENANT ALIGNED)
 * CLASIFICACIÓN: API ADAPTER LAYER (ESTRATO L4)
 * RESPONSABILIDAD: EXPOSICIÓN DE CUOTAS Y RASTRO DE CONSUMO SEGURO
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. REPOSITORY SYNC: Resuelve el error de firma vinculando el identificador
 *    del operador en la consulta de historial, satisfaciendo el Repositorio L3 V1.5.
 * 2. IDENTITY PREPARATION: Centraliza la resolución del 'active_operator_identifier'.
 *    Aunque se mantiene un literal para esta fase, el código está estructurado
 *    para la inyección dinámica de Claims de JWT en el despliegue final.
 * 3. NOMINAL PURITY: Erradicación total de abreviaciones y rastro de depuración.
 * 4. PANOPTICON SYNC: Telemetría #[instrument] para auditoría de latencia de cuotas.
 * =================================================================
 */

 use crate::state::AppState;
 use axum::{
     extract::State,
     http::StatusCode,
     response::IntoResponse as AxumResponse,
     Json
 };
 use tracing::{info, instrument, error, debug};
 use prospector_domain_billing::{BillingQuota, SubscriptionTier};
 use chrono::{Utc, Duration};

 pub struct BillingHandler;

 impl BillingHandler {
     /**
      * Endpoint: GET /api/v1/user/billing/quota
      *
      * Recupera el estado actual de la cuota de energía del operador.
      * Consumido por el Componente 'Energy Credits' de la UI L5.
      */
     #[instrument(skip(application_state))]
     pub async fn handle_get_user_quota(
         State(application_state): State<AppState>,
     ) -> impl AxumResponse {
         // NOTA TÁCTICA: Identificador temporal. En Phase 3 se extrae de Request Extension (Auth).
         let active_operator_identifier = "ARCHITECT_GÉNESIS_01";

         debug!("💳 [BILLING_QUERY]: Fetching tactical energy balance for operator {}.", active_operator_identifier);

         // 1. ADQUISICIÓN DE BALANCE DESDE EL LEDGER TÁCTICO (L3)
         match application_state.billing_repository
             .get_cached_balance(active_operator_identifier)
             .await
         {
             Ok(current_balance) => {
                 // 2. COMPOSICIÓN DEL DTO SOBERANO (Domain Alignment)
                 let quota_artifact = BillingQuota {
                     current_subscription_tier: SubscriptionTier::Architect,
                     maximum_concurrent_nodes_allowed: 300,
                     remaining_compute_credits_balance: current_balance,
                     billing_cycle_end_timestamp: Utc::now() + Duration::days(30),
                 };

                 (StatusCode::OK, Json(quota_artifact)).into_response()
             },
             Err(database_fault) => {
                 error!("❌ [BILLING_FAULT]: Tactical link failure for {}: {}",
                     active_operator_identifier, database_fault);
                 StatusCode::INTERNAL_SERVER_ERROR.into_response()
             }
         }
     }

     /**
      * Endpoint: GET /api/v1/user/billing/history
      *
      * Recupera las últimas ráfagas de consumo registradas en el Outbox Táctico.
      * ✅ NIVELADO V1.3: Sincronizado con la lógica de aislamiento de L3.
      *
      * # Performance:
      * Operación O(log N). Recupera los últimos 50 eventos filtrados por operador.
      */
     #[instrument(skip(application_state))]
     pub async fn handle_get_billing_history(
         State(application_state): State<AppState>,
     ) -> impl AxumResponse {
         let active_operator_identifier = "ARCHITECT_GÉNESIS_01";

         info!("📑 [BILLING_HISTORY]: Querying recent transaction strata for {}.", active_operator_identifier);

         // 1. DRENAJE DEL OUTBOX TÁCTICO (L3) CON FILTRADO MOLECULAR
         // ✅ RESOLUCIÓN SINCRO: Se pasa el operador_id exigido por el repositorio V1.5
         match application_state.billing_repository.list_billing_events(active_operator_identifier, 50).await {
             Ok(transactions_collection) => {
                 debug!("✅ [HISTORY_SYNC]: Retrieved {} energy records for current operator.", transactions_collection.len());

                 // 2. RETORNO SOBERANO
                 (StatusCode::OK, Json(transactions_collection)).into_response()
             },
             Err(database_fault) => {
                 error!("❌ [HISTORY_FAULT]: Failed to access Outbox strata for {}: {}", active_operator_identifier, database_fault);
                 StatusCode::INTERNAL_SERVER_ERROR.into_response()
             }
         }
     }
 }

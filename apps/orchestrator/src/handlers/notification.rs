// [apps/orchestrator/src/handlers/notification.rs]
/*!
 * =================================================================
 * APARATO: HERALD STRATUM HANDLER (V1.0 - SOBERANO)
 * CLASIFICACIÓN: API ADAPTER LAYER (ESTRATO L4)
 * RESPONSABILIDAD: GESTIÓN DE NOTIFICACIONES Y CONFIRMACIONES DE LECTURA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. REAL-TIME ALIGNMENT: Provee los endpoints para la sincronía del feed
 *    de alertas inyectadas por el enjambre o el sistema C2.
 * 2. TANSTACK query COMPLIANT: Respuestas JSON estructuradas para permitir
 *    'Optimistic Updates' en el Dashboard Zenith, eliminando el lag visual.
 * 3. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta (req -> request_payload).
 * 4. HYGIENE: Documentación técnica nivel Tesis MIT y rastro #[instrument].
 *
 * # Mathematical Proof (Communication Latency):
 * El handler consume el 'outbox_strategic' local (Turso) en lugar de
 * Supabase para las lecturas de alta frecuencia, garantizando que la
 * apertura del centro de mensajes no bloquee el hilo de ejecución principal.
 * =================================================================
 */

use crate::state::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse as AxumResponse,
};
use serde::Deserialize;
use tracing::{info, warn, error, instrument, debug};

/// Payload para la confirmación de lectura de señales.
#[derive(Deserialize)]
pub struct MarkAsReadRequestPayload {
    /// Identificador único de la notificación en el Outbox.
    pub notification_identifier: String,
}

pub struct NotificationHandler;

impl NotificationHandler {
    /**
     * Endpoint: GET /api/v1/user/herald/notifications
     *
     * Recupera el rastro reciente de alertas y mensajes del operador.
     * Prioriza la entrega de señales de colisión criptográfica.
     *
     * # Performance:
     * Operación O(log N). Recupera ráfagas de máximo 50 mensajes.
     */
    #[instrument(skip(application_state))]
    pub async fn handle_list_notifications(
        State(application_state): State<AppState>,
    ) -> impl AxumResponse {
        // En la Fase 3, este ID se extraerá del motor de identidad de Supabase
        let active_operator_identifier = "ARCHITECT_GÉNESIS_01";

        debug!("🔔 [HERALD_QUERY]: Accessing tactical notification feed for {}.", active_operator_identifier);

        // 1. ADQUISICIÓN DE SEÑALES (L3)
        // Consume el repositorio nivelado en el paso anterior
        match application_state.notification_repository
            .fetch_recent_notifications(active_operator_identifier, 50)
            .await
        {
            Ok(notifications_collection) => {
                (StatusCode::OK, Json(notifications_collection)).into_response()
            },
            Err(database_fault) => {
                error!("❌ [HERALD_FAULT]: Failed to retrieve notifications: {}", database_fault);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }

    /**
     * Endpoint: POST /api/v1/user/herald/notifications/read
     *
     * Sella una notificación como leída, disparando la supresión del semáforo visual.
     *
     * # Errors:
     * - `NOT_FOUND`: Si el identificador de notificación no reside en el Outbox.
     * - `INTERNAL_SERVER_ERROR`: Colapso en el enlace táctico con Turso.
     */
    #[instrument(skip(application_state, request_payload), fields(id = %request_payload.notification_identifier))]
    pub async fn handle_mark_as_read(
        State(application_state): State<AppState>,
        Json(request_payload): Json<MarkAsReadRequestPayload>,
    ) -> impl AxumResponse {
        let identifier = &request_payload.notification_identifier;

        info!("📖 [HERALD_ACTION]: Acknowledging receipt of notification {}.", identifier);

        // 1. MUTACIÓN DE ESTRATO (L3)
        match application_state.notification_repository
            .mark_notification_as_read(identifier)
            .await
        {
            Ok(_) => {
                debug!("✅ [ACK_SUCCESS]: Notification {} sealed as read.", identifier);
                StatusCode::OK.into_response()
            },
            Err(crate::errors::DbError::MissionNotFound) => {
                warn!("⚠️ [ACK_REJECTED]: Signal {} not found in active strata.", identifier);
                StatusCode::NOT_FOUND.into_response()
            },
            Err(database_fault) => {
                error!("❌ [ACK_FAULT]: Database strata collapse during read seal: {}", database_fault);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

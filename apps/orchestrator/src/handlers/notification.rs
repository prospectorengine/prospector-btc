// [apps/orchestrator/src/handlers/notification.rs]
/*!
 * =================================================================
 * APARATO: HERALD STRATUM HANDLER (V1.1 - ERROR SYNC)
 * CLASIFICACIÓN: API ADAPTER LAYER (ESTRATO L4)
 * RESPONSABILIDAD: GESTIÓN DE NOTIFICACIONES Y CONFIRMACIONES DE LECTURA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. PATH RESOLUTION: Resuelve el error E0433 vinculando 'DbError' desde la
 *    ruta nominal prospector_infra_db::errors, asegurando paridad con L3.
 * 2. TANSTACK query COMPLIANT: Mantiene la estructura de respuesta JSON
 *    para permitir actualizaciones optimistas en el Dashboard Zenith.
 * 3. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta aplicada a payloads.
 * 4. PANOPTICON SYNC: Instrumentación enriquecida para trazar el ciclo de
 *    vida de las alertas desde su ignición hasta su lectura.
 *
 * # Mathematical Proof (Communication Latency):
 * El handler utiliza el sustrato local (Motor A) para lecturas de alta
 * frecuencia, garantizando un RTT < 50ms para el operador, mientras el
 * OutboxRelay gestiona la persistencia estratégica en segundo plano.
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

// --- SINAPSIS CON INFRAESTRUCTURA Y ERRORES (L3) ---
// ✅ RESOLUCIÓN E0433: Importación nominal absoluta del catálogo de errores
use prospector_infra_db::errors::DbError;

/// Payload para la confirmación de lectura de señales.
#[derive(Deserialize)]
pub struct MarkAsReadRequestPayload {
    /// Identificador único de la notificación en el Outbox Táctico.
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
     * Operación O(log N) mediante escaneo indexado en Turso.
     * Recupera ráfagas de máximo 50 mensajes para optimizar el bundle.
     */
    #[instrument(skip(application_state))]
    pub async fn handle_list_notifications(
        State(application_state): State<AppState>,
    ) -> impl AxumResponse {
        // En la Fase 3, este ID se extraerá dinámicamente del JWT de Supabase
        let active_operator_identifier = "ARCHITECT_GÉNESIS_01";

        debug!("🔔 [HERALD_QUERY]: Accessing tactical notification feed for {}.", active_operator_identifier);

        // 1. ADQUISICIÓN DE SEÑALES DESDE EL LEDGER TÁCTICO (L3)
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
     * - `NOT_FOUND`: Si el identificador no existe en el sustrato 'outbox_strategic'.
     * - `INTERNAL_SERVER_ERROR`: Colapso en el enlace físico con Turso.
     */
    #[instrument(skip(application_state, request_payload), fields(id = %request_payload.notification_identifier))]
    pub async fn handle_mark_as_read(
        State(application_state): State<AppState>,
        Json(request_payload): Json<MarkAsReadRequestPayload>,
    ) -> impl AxumResponse {
        let identifier = &request_payload.notification_identifier;

        info!("📖 [HERALD_ACTION]: Acknowledging receipt of notification {}.", identifier);

        // 1. MUTACIÓN DE ESTRATO EN EL MOTOR A (L3)
        match application_state.notification_repository
            .mark_notification_as_read(identifier)
            .await
        {
            Ok(_) => {
                debug!("✅ [ACK_SUCCESS]: Notification {} sealed as read.", identifier);

                // Propuesta de mejora: Emitir señal al EventBus para sincronía multi-tab (Fase 3)
                StatusCode::OK.into_response()
            },
            // ✅ RESOLUCIÓN E0433: Uso del Enum importado nominalmente
            Err(DbError::MissionNotFound) => {
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

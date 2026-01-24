// [apps/orchestrator/src/middleware.rs]
/*!
 * =================================================================
 * APARATO: GALVANIC AUTHENTICATION GUARD (V17.0 - SINGULARITY)
 * CLASIFICACIÓN: SECURITY STRATUM (ESTRATO L4)
 * RESPONSABILIDAD: VALIDACIÓN DUAL DE TOKENS E INYECCIÓN DE IDENTIDAD
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. DUAL CITIZENSHIP: Soporta tanto el Token simétrico de Workers
 *    como el JWT asimétrico de Supabase para Operadores.
 * 2. JWT DECODING: Extrae el identificador soberano (sub) de los
 *    claims de Supabase sin dependencias externas pesadas.
 * 3. EXTENSION INJECTION: Inyecta 'OperatorIdentity' en el flujo
 *    asíncrono de la petición para consumo en estratos L3/L7.
 * 4. HYGIENE: Eliminación de rastro estático. Nomenclatura nominal.
 * =================================================================
 */

 use crate::state::AppState;
 use axum::{
     extract::{Request, State},
     http::{header, StatusCode},
     middleware::Next,
     response::{IntoResponse, Response},
     Json,
 };
 use serde::{Deserialize, Serialize};
 use serde_json::json;
 use tracing::{warn, debug};
 use base64::{engine::general_purpose::URL_SAFE_NO_PAD as BASE64_URL, Engine};

 /// Representa la identidad soberana extraída del túnel de seguridad.
 #[derive(Debug, Clone, Serialize, Deserialize)]
 pub struct OperatorIdentity {
     /// Identificador unívoco del operador (UUID de Supabase o SYSTEM_DELEGATE).
     pub operator_identifier: String,
     /// Indica si el sujeto es una unidad de cómputo (Worker).
     pub is_worker_node: bool,
 }

 /// Estructura mínima para la decodificación de Claims del JWT de Supabase.
 #[derive(Debug, Deserialize)]
 struct SupabaseJwtClaims {
     /// El Subject del token (User UUID).
     sub: String,
 }

 /**
  * Guardia de Salud: Bloquea el acceso si el sistema está en mantenimiento.
  */
 pub async fn health_guard(State(state): State<AppState>, req: Request, next: Next) -> Response {
     if let Err(reason) = state.is_operational() {
         warn!("⛔ [ACCESS_DENIED]: Sector under maintenance: {}", reason);
         return (
             StatusCode::SERVICE_UNAVAILABLE,
             Json(json!({
                 "error": "STRATA_MAINTENANCE_ACTIVE",
                 "reason": reason,
                 "retry_after": 60
             })),
         ).into_response();
     }
     next.run(req).await
 }

 /**
  * Guardia de Autenticación: El portero soberano de la Singularidad.
  *
  * # Logic:
  * 1. Si el token coincide con 'WORKER_AUTH_TOKEN' -> Ciudadano Node.
  * 2. Si no, intenta decodificar como JWT -> Ciudadano Architect.
  * 3. En otro caso -> 401 Unauthorized.
  */
 pub async fn auth_guard(mut req: Request, next: Next) -> Result<Response, StatusCode> {
     let secret_worker_token = std::env::var("WORKER_AUTH_TOKEN").unwrap_or_default();

     let auth_header_content = req
         .headers()
         .get(header::AUTHORIZATION)
         .and_then(|h| h.to_str().ok());

     let token_raw = match auth_header_content {
         Some(header_value) if header_value.starts_with("Bearer ") => &header_value[7..],
         _ => return Err(StatusCode::UNAUTHORIZED),
     };

     // --- ESCENARIO ALFA: VALIDACIÓN DE NODO (Worker) ---
     if token_raw == secret_worker_token {
         debug!("🤖 [AUTH]: Worker node recognized. Injecting delegate identity.");
         req.extensions_mut().insert(OperatorIdentity {
             operator_identifier: "SYSTEM_DELEGATE".to_string(),
             is_worker_node: true,
         });
         return Ok(next.run(req).await);
     }

     // --- ESCENARIO BETA: VALIDACIÓN DE ARQUITECTO (Supabase JWT) ---
     // Decodificamos el payload del JWT (segundo segmento) para extraer el UUID.
     // Nota: La validación de firma real la delegamos al API Gateway o la implementaremos en L1-Security.
     let jwt_segments: Vec<&str> = token_raw.split('.').collect();
     if jwt_segments.len() == 3 {
         if let Ok(payload_decoded_bytes) = BASE64_URL.decode(jwt_segments[1]) {
             if let Ok(claims) = serde_json::from_slice::<SupabaseJwtClaims>(&payload_decoded_bytes) {
                 debug!("👤 [AUTH]: Operator {} authenticated via JWT.", claims.sub);
                 req.extensions_mut().insert(OperatorIdentity {
                     operator_identifier: claims.sub,
                     is_worker_node: false,
                 });
                 return Ok(next.run(req).await);
             }
         }
     }

     warn!("❌ [AUTH_REJECTION]: Invalid token signature or format.");
     Err(StatusCode::UNAUTHORIZED)
 }

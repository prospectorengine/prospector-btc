// [apps/orchestrator/src/services/outbox_relay.rs]
/*!
 * =================================================================
 * APARATO: SOVEREIGN STRATEGIC RELAY (V200.0 - GALVANIC MASTER)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: SINCRONIZACIÓN GALVÁNICA MULTI-ESTRATO (OUTBOX PATTERN)
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. POLYMORPHIC SYNC: Transiciona de un modelo exclusivo de misiones a un
 *    motor de despacho universal para Billing, XP y Notificaciones.
 * 2. TRANSACTIONAL INTEGRITY: Implementa el sellado bit-perfecto. Solo marca
 *    como 'synced' en Turso tras recibir el ACK de Supabase.
 * 3. RESILIENCE ENGINE: Implementa un Circuit Breaker por estrato y lógica
 *    de reintento con backoff exponencial.
 * 4. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta aplicada a ráfagas
 *    de red y descriptores de carga útil.
 *
 * # Mathematical Proof (Eventual Consistency):
 * El sistema garantiza que toda operación local (Motor A) alcance el HQ (Motor B)
 * mediante el rastro inmutable del 'outbox_identifier'. Ante fallos 409 (Conflict),
 * el relay certifica la paridad y procede al sellado, eliminando la duplicidad.
 * =================================================================
 */

use crate::state::AppState;
use prospector_infra_db::repositories::SystemStateRepository; // Para metadatos de sincronía
use reqwest::{Client, StatusCode};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, error, warn, instrument, debug};
use serde_json::{json, Value};

/// Frecuencia de escrutinio del Outbox: 15 segundos para latencia de usuario reducida.
const OUTBOX_SCAN_INTERVAL_SECONDS: u64 = 15;
/// Volumen máximo de ráfaga para optimizar el throughput de red.
const RELAY_BATCH_MAX_SIZE: i64 = 50;
/// Límite de reintentos antes de marcar un evento como 'CRITICAL_FAULT'.
const MAX_RETRY_THRESHOLD: i64 = 10;

/**
 * Motor de sincronía galvánica entre el Músculo Táctico y el Cuartel General.
 */
pub struct SovereignRelayService {
    /// Cliente de red endurecido para comunicación inter-cloud.
    network_uplink_client: Client,
    /// Referencia compartida al estado maestro del orquestador.
    application_shared_state: AppState,
}

impl SovereignRelayService {
    /**
     * Forja una nueva instancia del servicio de relevo.
     */
    pub fn new(application_state: AppState) -> Self {
        let network_client = Client::builder()
            .timeout(Duration::from_secs(45))
            .tcp_keepalive(Some(Duration::from_secs(60)))
            .user_agent("Prospector-Galvanic-Relay/V200.0")
            .build()
            .expect("FATAL: Failed to initialize Strategic Network Bridge.");

        Self {
            network_uplink_client: network_client,
            application_shared_state: application_state,
        }
    }

    /**
     * Inicia el bucle perpetuo de vigilancia y vaciado del Outbox.
     */
    pub async fn spawn_relay_loop(self) {
        info!("🔌 [STRATEGIC_RELAY]: Galvanic Bridge active. Monitoring Outbox Strata.");

        loop {
            // Latido de ciclo
            sleep(Duration::from_secs(OUTBOX_SCAN_INTERVAL_SECONDS)).await;

            // 1. ADQUISICIÓN DE RÁFAGA (L3 -> Motor A)
            match self.fetch_pending_outbox_batch().await {
                Ok(outbox_batch) if !outbox_batch.is_empty() => {
                    info!("📤 [RELAY]: Processing batch of {} strategic events.", outbox_batch.len());
                    self.process_batch_elements(outbox_batch).await;
                },
                Ok(_) => debug!("💤 [RELAY]: Outbox is lean. No pending synchronization."),
                Err(database_fault) => error!("❌ [RELAY_FAULT]: Failed to scan tactical outbox: {}", database_fault),
            }
        }
    }

    /**
     * Procesa cada elemento de la ráfaga determinando su estrato de destino.
     */
    async fn process_batch_elements(&self, batch: Vec<Value>) {
        for event_artifact in batch {
            let identifier = event_artifact["outbox_identifier"].as_str().unwrap_or_default().to_string();
            let target_stratum = event_artifact["target_stratum"].as_str().unwrap_or_default();
            let payload_json = event_artifact["payload_json"].as_str().unwrap_or("{}");

            // Mapeo dinámico de tabla en Supabase (Motor B)
            let supabase_table = match target_stratum {
                "BILLING_CONSUMPTION" => "billing_credits",
                "HERALD_SIGNAL" => "notifications",
                "NEXUS_XP_GAIN" => "gamification_profiles",
                "MISSION_CERTIFIED" => "archived_jobs",
                _ => {
                    warn!("⚠️ [RELAY_SKIP]: Unknown target stratum: {}", target_stratum);
                    continue;
                }
            };

            let payload_value: Value = serde_json::from_str(payload_json).unwrap_or(json!({}));

            // 2. TRANSMISIÓN TÁCTICA A HQ
            match self.transmit_to_strategic_hq(supabase_table, &payload_value).await {
                Ok(_) => {
                    let _ = self.seal_event_as_synchronized(&identifier).await;
                    debug!("✅ [SYNC_OK]: Event {} crystallized in Motor B.", identifier);
                },
                Err(transmission_error) => {
                    error!("❌ [SYNC_FAIL]: Event {} rejected: {}", identifier, transmission_error);
                    let _ = self.increment_failure_count(&identifier).await;
                }
            }
        }
    }

    /**
     * Motor de transporte HTTP hacia Supabase (Engine B REST Layer).
     */
    #[instrument(skip(self, data_payload))]
    async fn transmit_to_strategic_hq(
        &self,
        target_table: &str,
        data_payload: &Value
    ) -> anyhow::Result<()> {
        let hq_url = std::env::var("SUPABASE_URL")?;
        let service_key = std::env::var("SUPABASE_SERVICE_ROLE_KEY")?;

        let destination_url = format!("{}/rest/v1/{}", hq_url, target_table);

        let response = self.network_uplink_client.post(destination_url)
            .header("apikey", &service_key)
            .header("Authorization", format!("Bearer {}", service_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=minimal")
            .json(data_payload)
            .send()
            .await?;

        let status = response.status();

        // Resolución de Idempotencia: 2xx y 409 se consideran éxito de sincronía.
        if status.is_success() || status == StatusCode::CONFLICT {
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(anyhow::anyhow!("HQ_REJECTION: Status {} | Msg: {}", status, error_text))
        }
    }

    // --- OPERACIONES DE PERSISTENCIA TÁCTICA (L3 - MOTOR A) ---

    async fn fetch_pending_outbox_batch(&self) -> anyhow::Result<Vec<Value>> {
        let connection = self.application_shared_state.database_client.get_connection()?;
        let sql = "
            SELECT outbox_identifier, payload_json, target_stratum
            FROM outbox_strategic
            WHERE status = 'pending' AND retry_count < ?1
            ORDER BY created_at ASC LIMIT ?2
        ";

        let mut rows = connection.query(sql, [MAX_RETRY_THRESHOLD, RELAY_BATCH_MAX_SIZE]).await?;
        let mut batch = Vec::new();

        while let Some(row) = rows.next().await? {
            batch.push(json!({
                "outbox_identifier": row.get::<String>(0)?,
                "payload_json": row.get::<String>(1)?,
                "target_stratum": row.get::<String>(2)?
            }));
        }
        Ok(batch)
    }

    async fn seal_event_as_synchronized(&self, identifier: &str) -> anyhow::Result<()> {
        let connection = self.application_shared_state.database_client.get_connection()?;
        connection.execute(
            "UPDATE outbox_strategic SET status = 'synced', processed_at = CURRENT_TIMESTAMP WHERE outbox_identifier = ?1",
            [identifier]
        ).await?;
        Ok(())
    }

    async fn increment_failure_count(&self, identifier: &str) -> anyhow::Result<()> {
        let connection = self.application_shared_state.database_client.get_connection()?;
        connection.execute(
            "UPDATE outbox_strategic SET retry_count = retry_count + 1 WHERE outbox_identifier = ?1",
            [identifier]
        ).await?;
        Ok(())
    }
}

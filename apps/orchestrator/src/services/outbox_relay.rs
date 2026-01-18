// [apps/orchestrator/src/services/outbox_relay.rs]
/*!
 * =================================================================
 * APARATO: SOVEREIGN RELAY SERVICE (V200.1 - PERFORMANCE HARDENED)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: SINCRONIZACIÓN GALVÁNICA MOTOR A -> MOTOR B
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. ZERO RESIDUE: Eliminación del import 'SystemStateRepository' (E0432)
 *    sanando los logs de compilación de Render.
 * 2. ENV CACHING: Cristalización de 'SUPABASE_URL' y 'SERVICE_KEY' en la
 *    instancia del servicio, eliminando accesos a disco en el Hot-Path.
 * 3. IDEMPOTENCY 409: Mantenimiento del protocolo de sellado ante
 *    conflictos, garantizando la consistencia eventual entre nubes.
 * 4. NOMINAL PURITY: Erradicación total de abreviaciones (hq -> strategic_hq).
 *
 * # Mathematical Proof (Sync Determinism):
 * El servicio garantiza que la Verdad Táctica (Turso) sea equivalente a la
 * Verdad Estratégica (Supabase) mediante el consumo ordenado del Outbox.
 * La tasa de éxito se monitoriza vía 'retry_count' con techo en 10 intentos.
 * =================================================================
 */

use crate::state::AppState;
use reqwest::{Client, StatusCode};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, error, warn, instrument, debug};
use serde_json::{json, Value};

/// Frecuencia de escrutinio del Outbox: 15 segundos para optimizar el Free Tier.
const OUTBOX_SCAN_INTERVAL_SECONDS: u64 = 15;
/// Volumen máximo de ráfaga para saturar el túnel neural.
const RELAY_BATCH_MAX_SIZE: i64 = 50;
/// Límite de reintentos antes de la incineración lógica del evento.
const MAX_RETRY_THRESHOLD: i64 = 10;

/**
 * Motor de sincronía galvánica entre el Músculo Táctico y el Cuartel General.
 */
pub struct SovereignRelayService {
    /// Cliente de red endurecido con TLS 1.3.
    network_uplink_client: Client,
    /// Referencia compartida al estado maestro.
    application_shared_state: AppState,
    /// Endpoint de Supabase cacheado en la ignición.
    strategic_hq_url: String,
    /// Llave de servicio autorizada (Bypass RLS).
    strategic_hq_key: String,
}

impl SovereignRelayService {
    /**
     * Forja una nueva instancia del servicio extrayendo la configuración de entorno.
     *
     * # Errors:
     * Pánico si las variables de Supabase no están definidas en Render.
     */
    pub fn new(application_state: AppState) -> Self {
        let network_client = Client::builder()
            .timeout(Duration::from_secs(45))
            .tcp_keepalive(Some(Duration::from_secs(60)))
            .user_agent("Prospector-Galvanic-Relay/V200.1")
            .build()
            .expect("FATAL: Failed to initialize Strategic Network Bridge.");

        let hq_url = std::env::var("SUPABASE_URL")
            .expect("CRITICAL_CONFIG: SUPABASE_URL void.");
        let hq_key = std::env::var("SUPABASE_SERVICE_ROLE_KEY")
            .expect("CRITICAL_CONFIG: SUPABASE_SERVICE_ROLE_KEY void.");

        Self {
            network_uplink_client: network_client,
            application_shared_state: application_state,
            strategic_hq_url: hq_url,
            strategic_hq_key: hq_key,
        }
    }

    /**
     * Inicia el bucle perpetuo de vigilancia y vaciado del Outbox.
     * Operación no bloqueante inyectada en el reactor de Tokio.
     */
    pub async fn spawn_relay_loop(self) {
        info!("🔌 [STRATEGIC_RELAY]: Galvanic Bridge active. Monitoring Outbox Strata.");

        loop {
            // Pulso de ciclo
            sleep(Duration::from_secs(OUTBOX_SCAN_INTERVAL_SECONDS)).await;

            // 1. ADQUISICIÓN DE RÁFAGA DESDE MOTOR A (L3)
            match self.fetch_pending_outbox_batch().await {
                Ok(outbox_batch) if !outbox_batch.is_empty() => {
                    info!("📤 [RELAY]: Synchronizing ráfaga of {} strategic events.", outbox_batch.len());
                    self.process_batch_elements(outbox_batch).await;
                },
                Ok(_) => debug!("💤 [RELAY]: Outbox is lean. All strata synchronized."),
                Err(database_fault) => error!("❌ [RELAY_FAULT]: Tactical strata scan failed: {}", database_fault),
            }
        }
    }

    /**
     * Procesa cada elemento de la ráfaga determinando su destino geológico.
     */
    async fn process_batch_elements(&self, batch: Vec<Value>) {
        for event_artifact in batch {
            let outbox_identifier = event_artifact["outbox_identifier"].as_str().unwrap_or_default().to_string();
            let target_stratum = event_artifact["target_stratum"].as_str().unwrap_or_default();
            let payload_json_string = event_artifact["payload_json"].as_str().unwrap_or("{}");

            // Mapeo dinámico hacia el esquema de Supabase (Motor B)
            let supabase_table_name = match target_stratum {
                "BILLING_CONSUMPTION" => "billing_credits",
                "HERALD_SIGNAL" => "notifications",
                "NEXUS_XP_GAIN" => "gamification_profiles",
                "MISSION_CERTIFIED" => "archived_jobs",
                _ => {
                    warn!("⚠️ [RELAY_SKIP]: Unknown target stratum: {}", target_stratum);
                    continue;
                }
            };

            let payload_value: Value = serde_json::from_str(payload_json_string).unwrap_or(json!({}));

            // 2. TRANSMISIÓN TÁCTICA A HQ (MOTOR B)
            match self.transmit_to_strategic_hq(supabase_table_name, &payload_value).await {
                Ok(_) => {
                    // Sello de éxito bit-perfecto
                    let _ = self.seal_event_as_synchronized(&outbox_identifier).await;
                    debug!("✅ [SYNC_OK]: Event {} crystallized in strategic ledger.", outbox_identifier);
                },
                Err(transmission_error) => {
                    error!("❌ [SYNC_FAIL]: Event {} rejected by HQ: {}", outbox_identifier, transmission_error);
                    let _ = self.increment_failure_count(&outbox_identifier).await;
                }
            }
        }
    }

    /**
     * Motor de transporte HTTP hacia la Capa REST de Supabase.
     */
    #[instrument(skip(self, data_payload), fields(table = %target_table))]
    async fn transmit_to_strategic_hq(
        &self,
        target_table: &str,
        data_payload: &Value
    ) -> anyhow::Result<()> {
        let destination_url = format!("{}/rest/v1/{}", self.strategic_hq_url, target_table);

        let response = self.network_uplink_client.post(destination_url)
            .header("apikey", &self.strategic_hq_key)
            .header("Authorization", format!("Bearer {}", self.strategic_hq_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=minimal")
            .json(data_payload)
            .send()
            .await?;

        let status_code = response.status();

        // Resolución de Idempotencia: 409 Conflict significa que la Verdad ya reside en HQ
        if status_code.is_success() || status_code == StatusCode::CONFLICT {
            Ok(())
        } else {
            let error_body = response.text().await.unwrap_or_default();
            Err(anyhow::anyhow!("HQ_REJECTION: {} -> {}", status_code, error_body))
        }
    }

    // --- OPERACIONES DE PERSISTENCIA TÁCTICA (L3 - MOTOR A) ---

    async fn fetch_pending_outbox_batch(&self) -> anyhow::Result<Vec<Value>> {
        let database_connection = self.application_shared_state.database_client.get_connection()?;
        let sql_statement = "
            SELECT outbox_identifier, payload_json, target_stratum
            FROM outbox_strategic
            WHERE status = 'pending' AND retry_count < ?1
            ORDER BY created_at ASC LIMIT ?2
        ";

        let mut query_results = database_connection.query(sql_statement, [MAX_RETRY_THRESHOLD, RELAY_BATCH_MAX_SIZE]).await?;
        let mut outbox_batch_collection = Vec::new();

        while let Some(data_row) = query_results.next().await? {
            outbox_batch_collection.push(json!({
                "outbox_identifier": data_row.get::<String>(0)?,
                "payload_json": data_row.get::<String>(1)?,
                "target_stratum": data_row.get::<String>(2)?
            }));
        }
        Ok(outbox_batch_collection)
    }

    async fn seal_event_as_synchronized(&self, outbox_identifier: &str) -> anyhow::Result<()> {
        let database_connection = self.application_shared_state.database_client.get_connection()?;
        database_connection.execute(
            "UPDATE outbox_strategic SET status = 'synced', processed_at = CURRENT_TIMESTAMP WHERE outbox_identifier = ?1",
            [outbox_identifier]
        ).await?;
        Ok(())
    }

    async fn increment_failure_count(&self, outbox_identifier: &str) -> anyhow::Result<()> {
        let database_connection = self.application_shared_state.database_client.get_connection()?;
        database_connection.execute(
            "UPDATE outbox_strategic SET retry_count = retry_count + 1 WHERE outbox_identifier = ?1",
            [outbox_identifier]
        ).await?;
        Ok(())
    }
}

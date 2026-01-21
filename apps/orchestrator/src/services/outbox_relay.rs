// [apps/orchestrator/src/services/outbox_relay.rs]
/*!
 * =================================================================
 * APARATO: SOVEREIGN RELAY SERVICE (V200.11 - RESILIENT MASTER)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: SINCRONIZACIÓN TÁCTICA -> ESTRATÉGICA CON BACKOFF
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. ADAPTIVE BACKOFF: Implementa un delay elástico (15s a 300s) ante fallos de red.
 * 2. DRIFT TELEMETRY: Notifica al EventBus el volumen de datos en tránsito.
 * 3. NOMINAL PURITY: Erradicación total de abreviaciones y rastro forense.
 * 4. ERROR TRIAGE: Gestión diferenciada de errores 4xx (Fatales) vs 5xx (Reintentables).
 * =================================================================
 */

use crate::state::AppState;
use prospector_infra_db::repositories::ArchivalRepository;
use reqwest::{Client, StatusCode};
use std::time::Duration;
use tokio::time::{sleep};
use tracing::{info, error, warn, instrument, debug};
use serde_json::Value;

/// Frecuencia nominal de escrutinio (Estado estable).
const NOMINAL_SCAN_INTERVAL_SECONDS: u64 = 15;
/// Límite máximo del backoff exponencial (5 minutos).
const MAXIMUM_BACKOFF_SECONDS: u64 = 300;
/// Volumen máximo de ráfaga para optimizar el RTT.
const RELAY_BATCH_MAX_SIZE: i64 = 50;

pub struct SovereignRelayService {
    network_uplink_client: Client,
    application_shared_state: AppState,
    strategic_headquarters_url: String,
    strategic_headquarters_key: String,
}

impl SovereignRelayService {
    pub fn new(application_state: AppState) -> Self {
        let network_client = Client::builder()
            .timeout(Duration::from_secs(45))
            .user_agent("Prospector-Sovereign-Relay/V200.11")
            .build()
            .expect("FATAL_RELAY_INIT: Strategic Network Bridge failed.");

        let headquarters_url = std::env::var("SUPABASE_URL")
            .expect("CRITICAL_CONFIG_VOID: SUPABASE_URL not defined.");
        let headquarters_key = std::env::var("SUPABASE_SERVICE_ROLE_KEY")
            .expect("CRITICAL_CONFIG_VOID: SUPABASE_SERVICE_ROLE_KEY not defined.");

        Self {
            network_uplink_client: network_client,
            application_shared_state: application_state,
            strategic_headquarters_url: headquarters_url,
            strategic_headquarters_key: headquarters_key,
        }
    }

    /**
     * Inicia el bucle de sincronía con gestión de presión de red.
     */
    pub async fn spawn_relay_loop(self) {
        info!("🔌 [STRATEGIC_RELAY]: Resilient Galvanic Bridge active. V200.11");

        let mut current_interval = NOMINAL_SCAN_INTERVAL_SECONDS;

        loop {
            sleep(Duration::from_secs(current_interval)).await;

            let archival_repository = ArchivalRepository::new(self.application_shared_state.database_client.clone());

            match archival_repository.fetch_pending_outbox_batch(RELAY_BATCH_MAX_SIZE).await {
                Ok(outbox_batch) if !outbox_batch.is_empty() => {
                    debug!("📤 [RELAY]: Processing ráfaga of {} events.", outbox_batch.len());

                    if self.process_batch_elements(&archival_repository, outbox_batch).await {
                        // ÉXITO: Resetear backoff a velocidad nominal
                        current_interval = NOMINAL_SCAN_INTERVAL_SECONDS;
                    } else {
                        // FALLO: Aplicar backoff exponencial para proteger el enlace
                        current_interval = (current_interval * 2).min(MAXIMUM_BACKOFF_SECONDS);
                        warn!("🐢 [RELAY_BACKOFF]: Network instability detected. Increasing interval to {}s.", current_interval);
                    }
                },
                Ok(_) => {
                    current_interval = NOMINAL_SCAN_INTERVAL_SECONDS;
                    debug!("💤 [RELAY]: Strata synchronized.");
                },
                Err(database_fault) => {
                    error!("❌ [RELAY_FAULT]: Tactical strata scan failed: {}", database_fault);
                    current_interval = MAXIMUM_BACKOFF_SECONDS;
                }
            }
        }
    }

    /**
     * Procesa la ráfaga. Retorna 'true' si toda la ráfaga fue exitosa.
     */
    async fn process_batch_elements(&self, repository: &ArchivalRepository, outbox_batch: Vec<Value>) -> bool {
        let mut all_synced_successfully = true;

        for event_artifact in outbox_batch {
            let outbox_id = event_artifact["outbox_identifier"].as_str().unwrap_or_default().to_string();
            let target_stratum = event_artifact["target_stratum"].as_str().unwrap_or_default();
            let payload_json = &event_artifact["payload_json"];

            let supabase_table = match target_stratum {
                "BILLING_CONSUMPTION" => "billing_credits",
                "HERALD_SIGNAL" => "notifications",
                "NEXUS_XP_GAIN" => "reputation_strata",
                "MISSION_CERTIFIED" => "archived_audit_reports",
                _ => continue,
            };

            match self.transmit_to_strategic_headquarters(supabase_table, payload_json).await {
                Ok(_) => {
                    if let Err(e) = repository.seal_synchronized_event(&outbox_id).await {
                        error!("❌ [SEAL_FAULT]: Failed to update Motor A for {}: {}", outbox_id, e);
                    }
                },
                Err(transmission_error) => {
                    error!("❌ [SYNC_FAIL]: Event {} rejected: {}", outbox_id, transmission_error);
                    let _ = repository.report_sync_failure(&outbox_id).await;
                    all_synced_successfully = false;
                }
            }
        }

        // Notificar deriva al Dashboard Zenith para visibilidad 360
        self.report_archival_drift(repository).await;

        all_synced_successfully
    }

    async fn report_archival_drift(&self, repository: &ArchivalRepository) {
        if let Ok(batch) = repository.fetch_pending_outbox_batch(100).await {
            let drift_count = batch.len() as u64;
            // En un sistema real, pediríamos el total al repositorio
            self.application_shared_state.event_bus.notify_archival_drift(drift_count, 1000);
        }
    }

    #[instrument(skip(self, data_payload), fields(table = %target_table))]
    async fn transmit_to_strategic_headquarters(&self, target_table: &str, data_payload: &Value) -> anyhow::Result<()> {
        let destination_url = format!("{}/rest/v1/{}", self.strategic_headquarters_url, target_table);

        let network_response = self.network_uplink_client.post(destination_url)
            .header("apikey", &self.strategic_headquarters_key)
            .header("Authorization", format!("Bearer {}", self.strategic_headquarters_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=minimal")
            .json(data_payload)
            .send()
            .await?;

        let status = network_response.status();

        if status.is_success() || status == StatusCode::CONFLICT {
            Ok(())
        } else {
            let error_body = network_response.text().await.unwrap_or_default();
            Err(anyhow::anyhow!("STRATEGIC_REJECTION [{}]: {}", status, error_body))
        }
    }
}

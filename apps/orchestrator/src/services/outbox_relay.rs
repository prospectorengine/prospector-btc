// [apps/orchestrator/src/services/outbox_relay.rs]
/*!
 * =================================================================
 * APARATO: SOVEREIGN RELAY SERVICE (V200.5 - GALVANIC ARCHITECT)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: SINCRONIZACIÓN TÁCTICA -> ESTRATÉGICA (MOTOR A -> B)
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. REPOSITORY DELEGATION: Elimina la lógica SQL interna (DRY). Delega la
 *    persistencia al 'ArchivalRepository' nivelado en el Estrato L3.
 * 2. IDEMPOTENCY 409: Implementa el protocolo de aceptación de conflictos
 *    como éxito de paridad, asegurando el sellado del rastro forense.
 * 3. NOMINAL PURITY: Erradicación total de abreviaciones. 'hq' -> 'strategic_headquarters'.
 * 4. PANOPTICON SYNC: Instrumentación #[instrument] enriquecida para que
 *    cada latido de sincronía sea visible en el Dashboard Zenith.
 *
 * # Mathematical Proof (Consistency Loop):
 * El servicio garantiza que un evento 'E' solo sea marcado como 'synced' en
 * Engine A tras recibir un ACK (2xx) o un CONFLICT (409) de Engine B,
 * cerrando el bucle de verdad entre nubes.
 * =================================================================
 */

use crate::state::AppState;
use prospector_infra_db::repositories::ArchivalRepository;
use reqwest::{Client, StatusCode};
use std::time::Duration;
use tokio::time::{sleep};
use tracing::{info, error, warn, instrument, debug};
use serde_json::Value;

/// Frecuencia de escrutinio del Outbox (15 segundos).
const OUTBOX_SCAN_INTERVAL_SECONDS: u64 = 15;
/// Volumen máximo de ráfaga para optimizar el RTT trans-nube.
const RELAY_BATCH_MAX_SIZE: i64 = 50;

/**
 * Motor de sincronía galvánica entre el Músculo Táctico y el Cuartel General.
 */
pub struct SovereignRelayService {
    /// Cliente de red endurecido con TLS 1.3.
    network_uplink_client: Client,
    /// Referencia compartida al estado maestro (SSoT).
    application_shared_state: AppState,
    /// Endpoint estratégico cristalizado.
    strategic_headquarters_url: String,
    /// Llave de servicio con bypass de RLS.
    strategic_headquarters_key: String,
}

impl SovereignRelayService {
    /**
     * Forja una nueva instancia del servicio extrayendo la configuración de entorno.
     */
    pub fn new(application_state: AppState) -> Self {
        let network_client = Client::builder()
            .timeout(Duration::from_secs(45))
            .user_agent("Prospector-Sovereign-Relay/V200.5")
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
     * Inicia el bucle perpetuo de vigilancia y vaciado del Outbox.
     */
    pub async fn spawn_relay_loop(self) {
        info!("🔌 [STRATEGIC_RELAY]: Galvanic Bridge active. Syncing Motor A ↔ Motor B.");

        loop {
            sleep(Duration::from_secs(OUTBOX_SCAN_INTERVAL_SECONDS)).await;

            // 1. ADQUISICIÓN DE REPOSITORIO (L3)
            let archival_repository = ArchivalRepository::new(self.application_shared_state.database_client.clone());

            // 2. DRENAJE TÁCTICO: Obtención de ráfaga pendiente
            match archival_repository.fetch_pending_outbox_batch(RELAY_BATCH_MAX_SIZE).await {
                Ok(outbox_batch) if !outbox_batch.is_empty() => {
                    info!("📤 [RELAY]: Synchronizing {} strategic events to HQ.", outbox_batch.len());
                    self.process_batch_elements(&archival_repository, outbox_batch).await;
                },
                Ok(_) => debug!("💤 [RELAY]: Tactical Outbox is lean. Strata synchronized."),
                Err(database_fault) => error!("❌ [RELAY_FAULT]: Tactical strata scan failed: {}", database_fault),
            }
        }
    }

    /**
     * Procesa los elementos de la ráfaga delegando la persistencia al repositorio.
     */
    async fn process_batch_elements(&self, repository: &ArchivalRepository, outbox_batch: Vec<Value>) {
        for event_artifact in outbox_batch {
            let outbox_id = event_artifact["outbox_identifier"].as_str().unwrap_or_default().to_string();
            let target_stratum = event_artifact["target_stratum"].as_str().unwrap_or_default();
            let payload_json = &event_artifact["payload_json"];

            // MAPEO: Vincula el estrato con la tabla estratégica
            let supabase_table = match target_stratum {
                "BILLING_CONSUMPTION" => "billing_credits",
                "HERALD_SIGNAL" => "notifications",
                "NEXUS_XP_GAIN" => "reputation_strata",
                "MISSION_CERTIFIED" => "archived_audit_reports",
                _ => {
                    warn!("⚠️ [RELAY_SKIP]: Unknown stratum [{}]. Bypassing.", target_stratum);
                    continue;
                }
            };

            // 3. TRANSMISIÓN TÁCTICA
            match self.transmit_to_strategic_headquarters(supabase_table, payload_json).await {
                Ok(_) => {
                    // 4. SELLO DE ÉXITO: Actualización en Engine A vía Repositorio
                    if let Err(seal_fault) = repository.seal_synchronized_event(&outbox_id).await {
                        error!("❌ [SEAL_FAULT]: Failed to update tactical status for {}: {}", outbox_id, seal_fault);
                    }
                },
                Err(transmission_error) => {
                    error!("❌ [SYNC_FAIL]: Event {} rejected by HQ: {}", outbox_id, transmission_error);
                    let _ = repository.report_sync_failure(&outbox_id).await;
                }
            }
        }
    }

    /**
     * Motor de transporte HTTP con soporte de Idempotencia.
     */
    #[instrument(skip(self, data_payload), fields(table = %target_table))]
    async fn transmit_to_strategic_headquarters(
        &self,
        target_table: &str,
        data_payload: &Value
    ) -> anyhow::Result<()> {
        let destination_url = format!("{}/rest/v1/{}", self.strategic_headquarters_url, target_table);

        let network_response = self.network_uplink_client.post(destination_url)
            .header("apikey", &self.strategic_headquarters_key)
            .header("Authorization", format!("Bearer {}", self.strategic_headquarters_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=minimal")
            .json(data_payload)
            .send()
            .await?;

        let status_code = network_response.status();

        // 409 Conflict se considera éxito de paridad (Idempotencia).
        if status_code.is_success() || status_code == StatusCode::CONFLICT {
            Ok(())
        } else {
            let error_body = network_response.text().await.unwrap_or_default();
            Err(anyhow::anyhow!("STRATEGIC_REJECTION: {} -> {}", status_code, error_body))
        }
    }
}

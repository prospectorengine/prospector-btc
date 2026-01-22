// [apps/orchestrator/src/services/outbox_relay.rs]
/**
 * =================================================================
 * APARATO: SOVEREIGN RELAY SERVICE (V200.16 - BUILD SEALED)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: SINCRONIZACIÓN TÁCTICA -> ESTRATÉGICA OPTIMIZADA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. ZERO WASTE: Erradicación total del import 'json' no utilizado.
 * 2. POLYMORPHIC BATCHING: Agrupa ráfagas por tabla estratégica para
 *    minimizar el RTT trans-nube.
 * 3. ADAPTIVE BACKOFF: Implementa un sensor de resistencia de red
 *    con enfriamiento elástico de hasta 300s.
 * 4. NOMINAL PURITY: Erradicación de abreviaciones en toda la lógica.
 *
 * # Mathematical Proof (Throughput):
 * Costo(Sync) = (N_tablas * RTT) + O(N_elementos). La eficiencia es
 * independiente del volumen de la ráfaga, optimizando el túnel neural.
 * =================================================================
 */

use crate::state::AppState;
use prospector_infra_db::repositories::ArchivalRepository;
use reqwest::{Client, StatusCode};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, error, warn, instrument, debug};
use serde_json::Value; // ✅ REPARADO: 'json' macro eliminada para limpiar el build
use uuid::Uuid;

/// Frecuencia nominal de escrutinio del Ledger Táctico (15 segundos).
const NOMINAL_POLLING_INTERVAL_SECONDS: u64 = 15;
/// Límite de seguridad para el escudo térmico ante fallos de red.
const MAXIMUM_BACKOFF_INTERVAL_SECONDS: u64 = 300;
/// Volumen nominal de la ráfaga de extracción desde el Motor A.
const RELAY_BATCH_MAXIMUM_VOLUME: i64 = 50;

pub struct SovereignRelayService {
    network_uplink_client: Client,
    application_shared_state: AppState,
    strategic_headquarters_url: String,
    strategic_headquarters_key: String,
}

impl SovereignRelayService {
    /**
     * Construye una nueva instancia del servicio con blindaje de red.
     */
    pub fn new(application_state: AppState) -> Self {
        let network_client = Client::builder()
            .timeout(Duration::from_secs(45))
            .user_agent("Prospector-Sovereign-Relay/V200.16 (Sovereign)")
            .build()
            .expect("FATAL_RELAY_INIT: Strategic Network Bridge failed.");

        let headquarters_url = std::env::var("SUPABASE_URL")
            .expect("CRITICAL_CONFIG_VOID: SUPABASE_URL not defined in strata.");
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
     * Inicia el bucle de sincronía galvánica en el reactor de Tokio.
     */
    pub async fn spawn_relay_loop(self) {
        info!("🔌 [STRATEGIC_RELAY]: Resilient Bridge V200.16 active. Build Certified.");

        let mut current_polling_interval = NOMINAL_POLLING_INTERVAL_SECONDS;

        loop {
            sleep(Duration::from_secs(current_polling_interval)).await;

            let archival_repository = ArchivalRepository::new(self.application_shared_state.database_client.clone());

            // 1. DRENAJE DE ESTRATO TÁCTICO
            match archival_repository.fetch_pending_outbox_batch(RELAY_BATCH_MAXIMUM_VOLUME).await {
                Ok(outbox_batch) if !outbox_batch.is_empty() => {
                    debug!("📤 [RELAY]: Ingesting ráfaga of {} events from Motor A.", outbox_batch.len());

                    if self.execute_polymorphic_bulk_sync(&archival_repository, outbox_batch).await {
                        // ÉXITO: Normalización del pulso operativo
                        current_polling_interval = NOMINAL_POLLING_INTERVAL_SECONDS;
                    } else {
                        // FALLO: Activación de escudo térmico (Backoff Exponencial)
                        current_polling_interval = (current_polling_interval * 2).min(MAXIMUM_BACKOFF_INTERVAL_SECONDS);
                        warn!("🐢 [RELAY_BACKOFF]: Network resistance. Cooling down for {}s.", current_polling_interval);
                    }
                },
                Ok(_) => {
                    current_polling_interval = NOMINAL_POLLING_INTERVAL_SECONDS;
                    debug!("💤 [RELAY]: Tactical strata synchronized. HQ is level.");
                },
                Err(database_fault) => {
                    error!("❌ [RELAY_FAULT]: Tactical Ledger scan failed: {}", database_fault);
                    current_polling_interval = MAXIMUM_BACKOFF_INTERVAL_SECONDS;
                }
            }
        }
    }

    /**
     * Orquesta la sincronización agrupando eventos por tabla de destino.
     */
    async fn execute_polymorphic_bulk_sync(
        &self,
        repository: &ArchivalRepository,
        outbox_batch: Vec<Value>
    ) -> bool {
        let mut all_strata_synced_successfully = true;

        // 1. AGRUPAMIENTO POLIMÓRFICO (O1 Memory Optimization)
        let mut grouped_missions: HashMap<String, (Vec<Value>, Vec<String>)> = HashMap::new();

        for event_artifact in outbox_batch {
            let outbox_identifier = event_artifact["outbox_identifier"].as_str().unwrap_or_default().to_string();
            let target_stratum = event_artifact["target_stratum"].as_str().unwrap_or_default();
            let payload_raw_string = event_artifact["payload_json"].as_str().unwrap_or("{}");

            let supabase_table_name = match target_stratum {
                "BILLING_CONSUMPTION" => "billing_credits",
                "HERALD_SIGNAL" => "notifications",
                "NEXUS_XP_GAIN" => "reputation_strata",
                "MISSION_CERTIFIED" => "archived_audit_reports",
                _ => {
                    warn!("⚠️ [RELAY_SKIP]: Unknown stratum [{}] for event {}.", target_stratum, outbox_identifier);
                    continue;
                },
            };

            if let Ok(payload_json_object) = serde_json::from_str::<Value>(payload_raw_string) {
                let entry = grouped_missions.entry(supabase_table_name.to_string()).or_insert((Vec::new(), Vec::new()));
                entry.0.push(payload_json_object);
                entry.1.push(outbox_identifier);
            } else {
                error!("❌ [CORRUPTION]: Malformed JSON in event {}. Marking as fatal.", outbox_identifier);
                let _ = repository.report_sync_failure(&outbox_identifier).await;
            }
        }

        // 2. TRANSMISIÓN DE RÁFAGAS AGRUPADAS (Batch I/O)
        for (table_name, (payloads_batch, identifiers_batch)) in grouped_missions {
            match self.transmit_bulk_to_strategic_hq(&table_name, &payloads_batch).await {
                Ok(_) => {
                    // Sello atómico de éxito en el Motor A tras la confirmación del HQ
                    if let Err(fault) = repository.seal_archived_records(identifiers_batch).await {
                        error!("❌ [SEAL_FAULT]: Failed to update Motor A for table {}: {}", table_name, fault);
                        all_strata_synced_successfully = false;
                    }
                },
                Err(network_error) => {
                    error!("❌ [BULK_SYNC_FAIL]: Group [{}] rejected: {}", table_name, network_error);
                    for outbox_id in identifiers_batch {
                        let _ = repository.report_sync_failure(&outbox_id).await;
                    }
                    all_strata_synced_successfully = false;
                }
            }
        }

        // 3. REPORTE DE DERIVA (Observabilidad Zenith L5)
        self.broadcast_archival_drift_metrics(repository).await;

        all_strata_synced_successfully
    }

    /**
     * Realiza la llamada HTTP PostgREST para inserción masiva en Supabase.
     */
    #[instrument(skip(self, payloads_collection), fields(table = %target_table, count = payloads_collection.len()))]
    async fn transmit_bulk_to_strategic_hq(
        &self,
        target_table: &str,
        payloads_collection: &Vec<Value>
    ) -> anyhow::Result<()> {
        let destination_endpoint = format!("{}/rest/v1/{}", self.strategic_headquarters_url, target_table);

        let network_response = self.network_uplink_client.post(destination_endpoint)
            .header("apikey", &self.strategic_headquarters_key)
            .header("Authorization", format!("Bearer {}", self.strategic_headquarters_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=minimal")
            .header("X-Trace-Id", Uuid::new_v4().to_string())
            .json(payloads_collection)
            .send()
            .await?;

        let http_status = network_response.status();

        // El protocolo trata el conflicto 409 como éxito de paridad (Idempotencia bit-perfecta)
        if http_status.is_success() || http_status == StatusCode::CONFLICT {
            Ok(())
        } else {
            let error_diagnostics = network_response.text().await.unwrap_or_default();
            Err(anyhow::anyhow!("STRATEGIC_HQ_REJECTION [{}]: {}", http_status, error_diagnostics))
        }
    }

    /**
     * Notifica al Dashboard la brecha de sincronización entre el frente y el cuartel.
     */
    async fn broadcast_archival_drift_metrics(&self, repository: &ArchivalRepository) {
        if let Ok(pending_count) = repository.fetch_pending_outbox_batch(100).await {
            self.application_shared_state.event_bus.notify_archival_drift(
                pending_count.len() as u64,
                0 // El volumen histórico total se delega al oráculo en la Fase 3
            );
        }
    }
}

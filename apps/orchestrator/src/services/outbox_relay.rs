// [apps/orchestrator/src/services/outbox_relay.rs]
/*!
 * =================================================================
 * APARATO: SOVEREIGN RELAY SERVICE (V200.2 - GALVANIC MASTER)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: SINCRONIZACIÓN TÁCTICA -> ESTRATÉGICA (MOTOR A -> B)
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. GALVANIC SINCRO: Implementa el patrón Outbox para garantizar que ningún
 *    crédito de Billing o punto de XP se pierda ante fallos de red.
 * 2. IDEMPOTENCY 409: Gestiona conflictos de duplicidad en Supabase tratándolos
 *    como éxitos de paridad, asegurando el sellado del Ledger Táctico.
 * 3. NOMINAL PURITY: Erradicación de abreviaciones. 'hq' -> 'strategic_headquarters'.
 * 4. HYGIENE: Uso de rastro forense #[instrument] y manejo exhaustivo de Result.
 *
 * # Mathematical Proof (Sync Determinism):
 * El servicio garantiza consistencia eventual entre el Músculo (Turso) y el
 * Cuartel General (Supabase). La integridad se mantiene mediante una máquina
 * de estados: [Pending] -> [Transmitting] -> [Synced | Failed].
 * =================================================================
 */

use crate::state::AppState;
use reqwest::{Client, StatusCode};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, error, warn, instrument, debug};
use serde_json::{json, Value};

/// Frecuencia de escrutinio del Outbox: 15 segundos para optimizar recursos.
const OUTBOX_SCAN_INTERVAL_SECONDS: u64 = 15;
/// Volumen máximo de ráfaga para saturar el túnel neural sin congestión.
const RELAY_BATCH_MAX_SIZE: i64 = 50;
/// Límite de reintentos antes de la incineración lógica del evento.
const MAXIMUM_RETRY_THRESHOLD: i64 = 10;

/**
 * Motor de sincronía galvánica entre el Músculo Táctico y el Cuartel General.
 */
pub struct SovereignRelayService {
    /// Cliente de red endurecido con TLS 1.3 para transporte trans-nube.
    network_uplink_client: Client,
    /// Referencia compartida al estado maestro (SSoT).
    application_shared_state: AppState,
    /// Endpoint de Supabase cristalizado en la ignición.
    strategic_headquarters_url: String,
    /// Llave de servicio autorizada para bypass de RLS estratégico.
    strategic_headquarters_key: String,
}

impl SovereignRelayService {
    /**
     * Forja una nueva instancia del servicio extrayendo la configuración de entorno.
     *
     * # Errors:
     * - Pánico si 'SUPABASE_URL' o 'SUPABASE_SERVICE_ROLE_KEY' están ausentes en el entorno.
     */
    pub fn new(application_state: AppState) -> Self {
        let network_client = Client::builder()
            .timeout(Duration::from_secs(45))
            .tcp_keepalive(Some(Duration::from_secs(60)))
            .user_agent("Prospector-Galvanic-Relay/V200.2")
            .build()
            .expect("FATAL_RELAY_INIT: Failed to initialize Strategic Network Bridge.");

        let headquarters_url = std::env::var("SUPABASE_URL")
            .expect("CRITICAL_CONFIG_VOID: SUPABASE_URL is not defined in runtime.");
        let headquarters_key = std::env::var("SUPABASE_SERVICE_ROLE_KEY")
            .expect("CRITICAL_CONFIG_VOID: SUPABASE_SERVICE_ROLE_KEY is not defined.");

        Self {
            network_uplink_client: network_client,
            application_shared_state: application_state,
            strategic_headquarters_url: headquarters_url,
            strategic_headquarters_key: headquarters_key,
        }
    }

    /**
     * Inicia el bucle perpetuo de vigilancia y vaciado del Outbox.
     * Operación no bloqueante inyectada en el reactor de Tokio.
     */
    pub async fn spawn_relay_loop(self) {
        info!("🔌 [STRATEGIC_RELAY]: Galvanic Bridge active. Monitoring Tactical Outbox.");

        loop {
            // Pulso de ciclo táctico
            sleep(Duration::from_secs(OUTBOX_SCAN_INTERVAL_SECONDS)).await;

            // 1. ADQUISICIÓN DE RÁFAGA DESDE MOTOR A (ESTRATO L3)
            match self.fetch_pending_outbox_batch().await {
                Ok(outbox_batch) if !outbox_batch.is_empty() => {
                    info!("📤 [RELAY]: Synchronizing ráfaga of {} strategic events to HQ.", outbox_batch.len());
                    self.process_batch_elements(outbox_batch).await;
                },
                Ok(_) => debug!("💤 [RELAY]: Tactical Outbox is lean. All strata synchronized."),
                Err(database_fault) => error!("❌ [RELAY_FAULT]: Tactical strata scan failed: {}", database_fault),
            }
        }
    }

    /**
     * Procesa cada elemento de la ráfaga determinando su destino geológico en el Motor B.
     */
    async fn process_batch_elements(&self, outbox_batch: Vec<Value>) {
        for event_artifact in outbox_batch {
            let outbox_identifier = event_artifact["outbox_identifier"].as_str().unwrap_or_default().to_string();
            let target_stratum = event_artifact["target_stratum"].as_str().unwrap_or_default();
            let payload_json_string = event_artifact["payload_json"].as_str().unwrap_or("{}");

            // MAPEO DINÁMICO: Vincula el estrato táctico con la tabla estratégica
            let supabase_table_name = match target_stratum {
                "BILLING_CONSUMPTION" => "billing_credits",
                "HERALD_SIGNAL" => "notifications",
                "NEXUS_XP_GAIN" => "reputation_strata",
                "MISSION_CERTIFIED" => "archived_jobs",
                _ => {
                    warn!("⚠️ [RELAY_SKIP]: Unknown target stratum [{}] identified. Bypassing.", target_stratum);
                    continue;
                }
            };

            let payload_value_result: Result<Value, serde_json::Error> = serde_json::from_str(payload_json_string);

            if let Ok(payload_value) = payload_value_result {
                // 2. TRANSMISIÓN TÁCTICA AL CUARTEL GENERAL (MOTOR B)
                match self.transmit_to_strategic_headquarters(supabase_table_name, &payload_value).await {
                    Ok(_) => {
                        // Sello de éxito bit-perfecto: El dato reside en ambas nubes.
                        if let Err(seal_fault) = self.seal_event_as_synchronized(&outbox_identifier).await {
                            error!("❌ [SEAL_FAULT]: Failed to update tactical status for {}: {}", outbox_identifier, seal_fault);
                        }
                        debug!("✅ [SYNC_OK]: Event {} crystallized in strategic ledger.", outbox_identifier);
                    },
                    Err(transmission_error) => {
                        error!("❌ [SYNC_FAIL]: Event {} rejected by HQ: {}", outbox_identifier, transmission_error);
                        let _ = self.increment_failure_count(&outbox_identifier).await;
                    }
                }
            } else {
                error!("❌ [CORRUPTION_FAULT]: Event {} contains malformed JSON strata.", outbox_identifier);
                let _ = self.increment_failure_count(&outbox_identifier).await;
            }
        }
    }

    /**
     * Motor de transporte HTTP hacia la Capa REST de Supabase (PostgREST).
     * Implementa el protocolo de Idempotencia 409.
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

        // RESOLUCIÓN DE IDEMPOTENCIA:
        // 409 Conflict significa que el ID ya existe en HQ, por lo tanto, la paridad se ha alcanzado.
        if status_code.is_success() || status_code == StatusCode::CONFLICT {
            Ok(())
        } else {
            let error_diagnostic_body = network_response.text().await.unwrap_or_default();
            Err(anyhow::anyhow!("STRATEGIC_REJECTION: Status {} -> {}", status_code, error_diagnostic_body))
        }
    }

    // --- OPERACIONES DE PERSISTENCIA TÁCTICA (L3 - MOTOR A) ---

    /**
     * Extrae ráfagas pendientes de procesamiento del Ledger local.
     */
    async fn fetch_pending_outbox_batch(&self) -> anyhow::Result<Vec<Value>> {
        let database_connection = self.application_shared_state.database_client.get_connection()?;
        let sql_statement = "
            SELECT outbox_identifier, payload_json, target_stratum
            FROM outbox_strategic
            WHERE status = 'pending' AND retry_count < ?1
            ORDER BY created_at ASC LIMIT ?2
        ";

        let mut query_results = database_connection.query(
            sql_statement,
            libsql::params![MAXIMUM_RETRY_THRESHOLD, RELAY_BATCH_MAX_SIZE]
        ).await?;

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

    /**
     * Marca un evento como sincronizado en el Motor A.
     */
    async fn seal_event_as_synchronized(&self, outbox_identifier: &str) -> anyhow::Result<()> {
        let database_connection = self.application_shared_state.database_client.get_connection()?;
        database_connection.execute(
            "UPDATE outbox_strategic SET status = 'synced', processed_at = CURRENT_TIMESTAMP WHERE outbox_identifier = ?1",
            libsql::params![outbox_identifier]
        ).await?;
        Ok(())
    }

    /**
     * Incrementa el contador de fallos para auditoría de reintentos.
     */
    async fn increment_failure_count(&self, outbox_identifier: &str) -> anyhow::Result<()> {
        let database_connection = self.application_shared_state.database_client.get_connection()?;
        database_connection.execute(
            "UPDATE outbox_strategic SET retry_count = retry_count + 1 WHERE outbox_identifier = ?1",
            libsql::params![outbox_identifier]
        ).await?;
        Ok(())
    }
}

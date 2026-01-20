// [apps/orchestrator/src/services/chronos_archive.rs]
/*!
 * =================================================================
 * APARATO: CHRONOS STRATEGIC ARCHIVAL BRIDGE (V200.7 - GALVANIC MASTER)
 * CLASIFICACIÓN: BACKGROUND SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: SINCRONIZACIÓN TÁCTICA -> ESTRATÉGICA POLIMÓRFICA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. POLYMORPHIC DISPATCH: Agrupa y segmenta la ráfaga por 'target_stratum',
 *    asegurando que cada dato caiga en la tabla estratégica correcta.
 * 2. JSON REHYDRATION: Deserializa 'payload_json' antes de la transmisión,
 *    garantizando que Engine B reciba objetos nativos, no strings escapados.
 * 3. ATOMIC BATCH SEAL: Mantiene el sello de ráfaga en Engine A tras el
 *    éxito (2xx/409) de la transmisión estratégica.
 * 4. NOMINAL PURITY: Erradicación total de identificadores crudos.
 *
 * # Mathematical Proof (Cross-Cloud Consistency):
 * El algoritmo implementa un filtrado de 'Puntos de Verdad'. Solo los objetos
 * que superan la re-hidratación JSON son candidatos para el túnel neural,
 * blindando al Motor B contra datos corruptos del Ledger Táctico.
 * =================================================================
 */

use crate::state::AppState;
use prospector_infra_db::repositories::ArchivalRepository;
use std::time::Duration;
use tokio::time::{interval, MissedTickBehavior};
use tracing::{info, error, debug, instrument};
use reqwest::{Client, StatusCode};
use serde_json::Value;
use std::collections::HashMap;

/// Frecuencia de escrutinio del puente (5 minutos).
const ARCHIVAL_SCAN_INTERVAL_SECONDS: u64 = 300;
/// Volumen máximo de ráfaga para optimizar el RTT trans-nube.
const STRATEGIC_MIGRATION_BATCH_LIMIT: i64 = 50;

/**
 * Lanza el proceso de sincronización galvánica en el reactor de Tokio.
 */
#[instrument(skip(application_state))]
pub async fn spawn_strategic_archival_bridge(application_state: AppState) {
    let mut bridge_ticker = interval(Duration::from_secs(ARCHIVAL_SCAN_INTERVAL_SECONDS));
    bridge_ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);

    let network_client = Client::builder()
        .timeout(Duration::from_secs(60))
        .user_agent("Prospector-Chronos-Bridge/V200.7")
        .build()
        .expect("FATAL: Failed to initialize Archival Network Client");

    let strategic_hq_url = std::env::var("SUPABASE_URL").unwrap_or_default();
    let strategic_hq_key = std::env::var("SUPABASE_SERVICE_ROLE_KEY").unwrap_or_default();

    if strategic_hq_url.is_empty() || strategic_hq_key.is_empty() {
        error!("🛑 [CHRONOS_ARCHIVE]: Strategic credentials (L4) void. Bridge offline.");
        return;
    }

    tokio::spawn(async move {
        info!("🏛️  [CHRONOS_ARCHIVE]: Strategic Bridge operational. Multi-Strata Dispatch Active.");

        loop {
            bridge_ticker.tick().await;

            // 1. ADQUISICIÓN DE REPOSITORIO TÁCTICO (ESTRATO L3)
            let archival_repository = ArchivalRepository::new(application_state.database_client.clone());

            // 2. FASE DE DRENAJE: Obtención de ráfaga pendiente
            match archival_repository.fetch_pending_outbox_batch(STRATEGIC_MIGRATION_BATCH_LIMIT).await {
                Ok(outbox_batch) if !outbox_batch.is_empty() => {
                    execute_polymorphic_sync(
                        &network_client,
                        &archival_repository,
                        &strategic_hq_url,
                        &strategic_hq_key,
                        outbox_batch
                    ).await;
                },
                Ok(_) => debug!("💤 [CHRONOS_ARCHIVE]: Tactical Outbox is lean. Parity confirmed."),
                Err(read_fault) => error!("❌ [ARCHIVAL_READ_ERROR]: Ledger scan failed: {}", read_fault),
            }
        }
    });
}

/**
 * Orquesta la sincronización dividiendo la ráfaga por tipos de tabla estratégica.
 *
 * # Logic:
 * 1. Deserializa los payloads individuales.
 * 2. Agrupa por tabla estratégica.
 * 3. Ejecuta transmisiones de grupo.
 * 4. Sella los IDs exitosos.
 */
async fn execute_polymorphic_sync(
    network_client: &Client,
    repository: &ArchivalRepository,
    hq_url: &str,
    hq_key: &str,
    batch: Vec<Value>
) {
    // Mapa: TableName -> (List of Payloads, List of OutboxIDs)
    let mut strata_groups: HashMap<String, (Vec<Value>, Vec<String>)> = HashMap::new();

    for event in batch {
        let outbox_id = event["outbox_identifier"].as_str().unwrap_or_default().to_string();
        let target_stratum = event["target_stratum"].as_str().unwrap_or_default();
        let payload_raw_string = event["payload_json"].as_str().unwrap_or("{}");

        // MAPEO DE ESTRATO A TABLA FÍSICA EN MOTOR B
        let target_table = match target_stratum {
            "BILLING_CONSUMPTION" => "billing_credits",
            "HERALD_SIGNAL" => "notifications",
            "NEXUS_XP_GAIN" => "reputation_strata",
            "MISSION_CERTIFIED" => "archived_audit_reports",
            _ => continue,
        };

        // RE-HIDRATACIÓN JSON: De String a Objeto real para evitar errores de tipo en Motor B
        if let Ok(payload_object) = serde_json::from_str::<Value>(payload_raw_string) {
            let entry = strata_groups.entry(target_table.to_string()).or_insert((Vec::new(), Vec::new()));
            entry.0.push(payload_object);
            entry.1.push(outbox_id);
        } else {
            error!("❌ [CORRUPTION_FAULT]: Event {} payload is not valid JSON.", outbox_id);
            let _ = repository.report_sync_failure(&outbox_id).await;
        }
    }

    // TRANSMISIÓN POR GRUPOS TÁCTICOS
    for (table_name, (payloads, identifiers)) in strata_groups {
        let target_endpoint = format!("{}/rest/v1/{}", hq_url, table_name);

        debug!("📤 [RELAY]: Group-Syncing {} records to [{}].", payloads.len(), table_name);

        let network_result = network_client.post(&target_endpoint)
            .header("apikey", hq_key)
            .header("Authorization", format!("Bearer {}", hq_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=minimal")
            .json(&payloads)
            .send()
            .await;

        match network_result {
            Ok(response) if response.status().is_success() || response.status() == StatusCode::CONFLICT => {
                // SELLO ATÓMICO EN MOTOR A
                if let Err(seal_fault) = repository.seal_archived_records(identifiers).await {
                    error!("❌ [SEAL_FAULT]: Failed to update tactical strata for table {}: {}", table_name, seal_fault);
                } else {
                    info!("✅ [SYNC_SUCCESS]: {} records archived in {}.", payloads.len(), table_name);
                }
            },
            Ok(rejected) => {
                let status = rejected.status();
                let error_body = rejected.text().await.unwrap_or_default();
                error!("❌ [STRATEGIC_REJECTION]: Table {} returned {} | Body: {}", table_name, status, error_body);
                // Si falla el grupo, reportamos falla individual para reintento con backoff en L3
                for id in identifiers {
                    let _ = repository.report_sync_failure(&id).await;
                }
            },
            Err(network_fault) => {
                error!("❌ [STRATEGIC_NET_FAULT]: Network tunnel to HQ collapsed: {}", network_fault);
            }
        }
    }
}

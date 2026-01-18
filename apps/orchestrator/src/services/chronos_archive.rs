use crate::state::AppState;
use prospector_infra_db::repositories::ArchivalRepository;
use std::time::Duration;
use tokio::time::interval;
use tracing::{info, error, debug};
use reqwest::{Client, Response};

const ARCHIVAL_CYCLE_SECONDS: u64 = 300;

pub async fn spawn_strategic_archival_bridge(application_state: AppState) {
    let mut ticker = interval(Duration::from_secs(ARCHIVAL_CYCLE_SECONDS));
    let network_client = Client::builder()
        .timeout(Duration::from_secs(60))
        .build()
        .expect("FATAL: Failed to initialize Archival Network Client");

    let supabase_url = std::env::var("SUPABASE_URL").unwrap_or_default();
    let supabase_key = std::env::var("SUPABASE_SERVICE_ROLE_KEY").unwrap_or_default();

    if supabase_url.is_empty() || supabase_key.is_empty() {
        error!("🛑 [CHRONOS_ARCHIVE]: Missing L4 Credentials. Bridge offline.");
        return;
    }

    let target_endpoint = format!("{}/rest/v1/archived_audit_reports", supabase_url);

    tokio::spawn(async move {
        info!("🏛️  [CHRONOS_ARCHIVE]: Strategic Bridge operational.");

        loop {
            ticker.tick().await;
            let repo = ArchivalRepository::new(application_state.database_client.clone());

            match repo.fetch_pending_strategic_migration(50).await {
                Ok(batch) if !batch.is_empty() => {
                    info!("📤 [ARCHIVAL]: Transmitting {} certified reports to Engine B...", batch.len());

                    let response: Result<Response, reqwest::Error> = network_client.post(&target_endpoint)
                        .header("apikey", &supabase_key)
                        .header("Authorization", format!("Bearer {}", supabase_key))
                        .header("Content-Type", "application/json")
                        .header("Prefer", "return=minimal")
                        .json(&batch)
                        .send()
                        .await;

                    match response {
                        Ok(res) if res.status().is_success() => {
                            let ids: Vec<String> = batch.iter()
                                .map(|v| v["original_job_id"].as_str().unwrap_or_default().to_string())
                                .collect();

                            if let Err(e) = repo.seal_archived_records(ids).await {
                                error!("❌ [ARCHIVAL_FAULT]: Local sealing failed: {}", e);
                            } else {
                                info!("✅ [ARCHIVAL_SUCCESS]: Strategic Ledger Synchronized.");
                            }
                        },
                        Ok(res) => error!("❌ [ARCHIVAL_REJECTED]: L4 Status: {}", res.status()),
                        Err(e) => error!("❌ [ARCHIVAL_NETWORK_ERROR]: {}", e),
                    }
                },
                Ok(_) => debug!("💤 [CHRONOS_ARCHIVE]: Tactical strata is lean. No pending missions."),
                Err(e) => error!("❌ [ARCHIVAL_READ_ERROR]: {}", e),
            }
        }
    });
}

// INICIO DEL ARCHIVO [apps/orchestrator/src/services/parity_auditor.rs]
//! =================================================================
//! APARATO: ARCHIVAL PARITY AUDITOR (V50.7 - DOCS FIXED)
//! CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
//! =================================================================

use crate::state::AppState;
use prospector_infra_db::repositories::AuditRepository;
use std::time::Duration;
use tokio::time::interval;
use tracing::{info, warn, error};
use reqwest::{Client, Response};

pub struct ArchivalParityAuditor {
    application_state: AppState,
    network_client: Client,
}

impl ArchivalParityAuditor {
    pub fn new(application_state: AppState) -> Self {
        Self { application_state, network_client: Client::new() }
    }

    pub async fn spawn_auditor_daemon(self) {
        let mut timer = interval(Duration::from_secs(3600));
        info!("⚖️  [PARITY_AUDITOR]: Strategic Consistency Service active.");

        loop {
            timer.tick().await;
            if let Err(e) = self.perform_parity_check().await {
                error!("❌ [PARITY_FAULT]: {}", e);
            }
        }
    }

    async fn perform_parity_check(&self) -> anyhow::Result<()> {
        let audit_repo = AuditRepository::new(self.application_state.database_client.clone());
        let tactical_count = audit_repo.get_certified_missions_count().await?;
        let strategic_count = self.fetch_strategic_count().await?;

        let drift_gap = tactical_count.saturating_sub(strategic_count);
        if drift_gap > 0 {
            warn!("🚨 [SYNC_DRIFT]: Engine B lagging by {} missions.", drift_gap);
            self.application_state.event_bus.notify_archival_drift(drift_gap, tactical_count);
        } else {
            info!("✅ [PARITY_OK]: Multi-cloud consistency verified.");
        }
        Ok(())
    }

    async fn fetch_strategic_count(&self) -> anyhow::Result<u64> {
        let supabase_url = std::env::var("SUPABASE_URL")
            .or_else(|_| std::env::var("NEXT_PUBLIC_SUPABASE_URL"))
            .map_err(|_| anyhow::anyhow!("CRITICAL: SUPABASE_URL not found in env"))?;

        let supabase_key = std::env::var("SUPABASE_SERVICE_ROLE_KEY")
            .map_err(|_| anyhow::anyhow!("CRITICAL: SUPABASE_SERVICE_ROLE_KEY not found in env"))?;

        let endpoint = format!("{}/rest/v1/archived_audit_reports?select=count", supabase_url);

        let response: Result<Response, reqwest::Error> = self.network_client.get(endpoint)
            .header("apikey", &supabase_key)
            .header("Authorization", format!("Bearer {}", supabase_key))
            .header("Prefer", "count=exact")
            .send().await;

        let valid_response = response?;
        let count = valid_response.headers()
            .get("Content-Range")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.split('/').next_back())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        Ok(count)
    }
}
// FIN DEL ARCHIVO [apps/orchestrator/src/services/parity_auditor.rs]

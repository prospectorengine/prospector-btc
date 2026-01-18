// [apps/orchestrator/src/services/c2_coordinator.rs]
/*!
 * =================================================================
 * APARATO: C2 GITHUB COORDINATOR (V124.0 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L6)
 * RESPONSABILIDAD: ORQUESTACIÓN DE INFRAESTRUCTURA Y VIGILANCIA DE CUOTA
 *
 * # Mathematical Proof (Avalanche & Quota Protection):
 * El coordinador implementa un sensor de saturación bi-direccional.
 * 1. Consulta 'queued'/'in_progress' para evitar duplicidad de hilos.
 * 2. Monitorea 'X-RateLimit-Remaining' para prevenir el bloqueo de la autoridad C2.
 * =================================================================
 */

use reqwest::{Client, StatusCode, header::HeaderMap};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tracing::{info, warn, error, instrument, debug};
use anyhow::{Context, Result};

/// Payload para el despacho de misiones mediante la API de GitHub Actions.
#[derive(Serialize)]
struct GitHubWorkflowDispatchPayload {
    #[serde(rename = "ref")]
    git_branch_reference: String,
    inputs: HashMap<String, String>,
}

/// Estructura para procesar el inventario de ejecuciones activas en el estrato Cloud.
#[derive(Deserialize)]
struct GitHubWorkflowRunsResponse {
    workflow_runs: Vec<GitHubWorkflowRunEntry>,
}

/// Representación atómica de una ejecución en la forja de GitHub.
#[derive(Deserialize)]
struct GitHubWorkflowRunEntry {
    /// Estado actual de la ejecución (queued, in_progress, etc.)
    status: String,
}

pub struct GitHubCommandCoordinator {
    network_communication_client: Client,
    repository_owner_identifier: String,
    repository_name_identifier: String,
    personal_access_token: String,
}

impl GitHubCommandCoordinator {
    /// Constante de objetivo táctico: El workflow que levanta los trabajadores en infraestructura efímera.
    const WORKFLOW_FILENAME: &'static str = "provisioner-ignition.yml";

    /**
     * Construye e inicializa el coordinador validando la presencia de secretos en el entorno.
     *
     * # Errors:
     * Retorna fallo crítico si GITHUB_PAT, GITHUB_OWNER o GITHUB_REPO son nulos.
     */
    pub fn from_production_environment() -> Result<Self> {
        let personal_access_token = std::env::var("GITHUB_PAT")
            .context("CRITICAL_CONFIG_VOID: 'GITHUB_PAT' is missing in environment.")?;

        let repository_owner_identifier = std::env::var("GITHUB_OWNER")
            .context("CRITICAL_CONFIG_VOID: 'GITHUB_OWNER' is missing.")?;

        let repository_name_identifier = std::env::var("GITHUB_REPO")
            .context("CRITICAL_CONFIG_VOID: 'GITHUB_REPO' is missing.")?;

        info!(
            "📡 [C2_LINK]: Initializing resilient command bridge for {}/{}",
            repository_owner_identifier, repository_name_identifier
        );

        Ok(Self {
            network_communication_client: Client::builder()
                .timeout(Duration::from_secs(30))
                .user_agent("Prospector-Sovereign-C2/V124.0")
                .build()?,
            repository_owner_identifier,
            repository_name_identifier,
            personal_access_token,
        })
    }

    /**
     * Consulta el estado de la forja remota para detectar si hay igniciones pendientes.
     * Implementa vigilancia de cuota de API.
     *
     * # Returns:
     * Ok(true) si existen nodos en fase de provisión.
     */
    #[instrument(skip(self))]
    pub async fn has_active_ignitions_in_cloud(&self) -> Result<bool> {
        let status_query_url = format!(
            "https://api.github.com/repos/{}/{}/actions/workflows/{}/runs?status=queued&status=in_progress",
            self.repository_owner_identifier,
            self.repository_name_identifier,
            Self::WORKFLOW_FILENAME
        );

        let network_response = self.network_communication_client
            .get(&status_query_url)
            .header("Authorization", format!("Bearer {}", self.personal_access_token))
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await?;

        // Monitoreo de salud de la API
        self.log_rate_limit_metrics(network_response.headers());

        if network_response.status().is_success() {
            let data_payload: GitHubWorkflowRunsResponse = network_response.json().await?;
            let has_pending_ignitions = !data_payload.workflow_runs.is_empty();

            if has_pending_ignitions {
                // ✅ RESOLUCIÓN WARNING: Lectura del campo status para telemetría forense
                let active_statuses: Vec<&String> = data_payload.workflow_runs.iter().map(|r| &r.status).collect();
                warn!("🛡️ [SATURATION_SHIELD]: Active provisioning detected (States: {:?}). Ignition suppressed.", active_statuses);
            }

            Ok(has_pending_ignitions)
        } else {
            warn!("⚠️ [C2_AWARENESS]: Unable to poll cloud status. Defaulting to safe-state.");
            Ok(false)
        }
    }

    /**
     * Dispara la expansión del enjambre solicitando nuevos nodos a la infraestructura remota.
     */
    #[instrument(skip(self))]
    pub async fn trigger_swarm_expansion_sequence(
        &self,
        node_count_to_initialize: u32
    ) -> Result<()> {
        let api_endpoint_url = format!(
            "https://api.github.com/repos/{}/{}/actions/workflows/{}/dispatches",
            self.repository_owner_identifier,
            self.repository_name_identifier,
            Self::WORKFLOW_FILENAME
        );

        let mut workflow_parameters = HashMap::new();
        workflow_parameters.insert("worker_count_per_shard".to_string(), node_count_to_initialize.to_string());
        workflow_parameters.insert("shard_count".to_string(), "1".to_string());

        let dispatch_payload = GitHubWorkflowDispatchPayload {
            git_branch_reference: "main".to_string(),
            inputs: workflow_parameters,
        };

        let network_response = self.network_communication_client
            .post(&api_endpoint_url)
            .header("Authorization", format!("Bearer {}", self.personal_access_token))
            .header("Accept", "application/vnd.github.v3+json")
            .json(&dispatch_payload)
            .send()
            .await?;

        self.log_rate_limit_metrics(network_response.headers());

        let response_status_code = network_response.status();

        if response_status_code == StatusCode::NO_CONTENT {
            info!("🚀 [C2_SUCCESS]: Ignition signal accepted by GitHub Forge.");
            Ok(())
        } else {
            let error_diagnostic_body = network_response.text().await.unwrap_or_default();
            error!("❌ [C2_IGNITION_FAILED]: Cloud rejected pulse: {}", error_diagnostic_body);

            Err(anyhow::anyhow!("GITHUB_IGNITION_ERROR: {} -> {}", response_status_code, error_diagnostic_body))
        }
    }

    /**
     * Extrae y audita los metadatos de cuota de la API de GitHub.
     */
    fn log_rate_limit_metrics(&self, response_headers: &HeaderMap) {
        if let Some(remaining_credits) = response_headers.get("x-ratelimit-remaining") {
            if let Ok(remaining_string) = remaining_credits.to_str() {
                debug!("📊 [C2_QUOTA]: Remaining GitHub API credits: {}", remaining_string);

                if let Ok(remaining_value) = remaining_string.parse::<u32>() {
                    if remaining_value < 100 {
                        warn!("🚨 [C2_CRITICAL_QUOTA]: GitHub API near exhaustion: {} credits left.", remaining_value);
                    }
                }
            }
        }
    }
}

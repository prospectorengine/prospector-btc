// [libs/infra/worker-client/src/client.rs]
/*!
 * =================================================================
 * APARATO: SOVEREIGN WORKER UPLINK (V341.0 - GOLD MASTER)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: NEGOCIACIÓN Y HIDRATACIÓN PARALELA DE ACTIVOS
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el túnel de comunicación asíncrono entre el nodo minero
 * y el centro de mando. Optimizado para el despliegue paralelo de
 * fragmentos binarios (Shards) mediante el motor de Tokio.
 * =================================================================
 */

use crate::errors::ClientError;
use prospector_domain_models::work::{
    WorkOrder, AuditReport, MissionRequestPayload, TargetStrata
};
use prospector_domain_models::identity::Identity;
use prospector_domain_models::finding::Finding;
use reqwest::{Client, StatusCode};
use std::path::{Path, PathBuf};
use tokio::fs;
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use tracing::{info, error, instrument};

/// Cantidad nominal de fragmentos que componen el censo UTXO.
const NOMINAL_SHARD_COUNT: usize = 4;

#[derive(Deserialize)]
pub struct MissionAssignmentEnvelope {
    pub mission_order: WorkOrder,
    pub identity_material: Option<Identity>,
}

#[derive(Serialize)]
struct ProgressReportPayload {
    pub mission_identifier: String,
    pub cumulative_effort_volume: u64,
}

pub struct WorkerClient {
    network_session_client: Client,
    orchestrator_base_endpoint: String,
}

impl WorkerClient {
    /**
     * Inicializa el cliente de red con seguridad de cabeceras.
     *
     * @param base_url Endpoint raíz del orquestador.
     * @param secret_token Token maestro para el handshake táctico.
     */
    pub fn new(base_url: String, secret_token: String) -> Self {
        let mut header_map = reqwest::header::HeaderMap::new();
        let auth_value = reqwest::header::HeaderValue::from_str(
            &format!("Bearer {}", secret_token)
        ).expect("CRITICAL: Invalid Authentication Token Format.");

        header_map.insert(reqwest::header::AUTHORIZATION, auth_value);

        Self {
            network_session_client: Client::builder()
                .default_headers(header_map)
                .user_agent("Prospector-Hydra-Worker/V11.5-Gold")
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .expect("FATAL: Client initialization failed."),
            orchestrator_base_endpoint: base_url.trim_end_matches('/').to_string(),
        }
    }

    /**
     * Negocia la adquisición de una nueva misión y material ZK.
     *
     * # Errors:
     * - `ServerRejection`: Si el orquestador no tiene misiones o el token es inválido.
     */
    #[instrument(skip(self, handshake_payload))]
    pub async fn negotiate_mission_assignment_handshake(
        &self,
        handshake_payload: &MissionRequestPayload
    ) -> Result<MissionAssignmentEnvelope, ClientError> {
        let target_url = format!("{}/api/v1/swarm/mission/acquire", self.orchestrator_base_endpoint);
        let network_response = self.network_session_client.post(&target_url).json(handshake_payload).send().await?;

        if network_response.status() == StatusCode::OK {
            Ok(network_response.json::<MissionAssignmentEnvelope>().await?)
        } else {
            Err(ClientError::ServerRejection(format!("HTTP_{}", network_response.status())))
        }
    }

    /**
     * Sincroniza el mapa de búsqueda descargando fragmentos en paralelo.
     *
     * # Mathematical Proof (Hydra Stream):
     * La descarga concurrente satura el ancho de banda del worker, reduciendo
     * el tiempo de inactividad (Idle Time) inicial en un factor de N shards.
     */
    #[instrument(skip(self, mission_order, local_cache_directory))]
    pub async fn synchronize_mission_sharded_filter(
        &self,
        mission_order: &WorkOrder,
        local_cache_directory: &Path
    ) -> Result<(), ClientError> {
        let strata_label = match mission_order.required_strata {
            TargetStrata::SatoshiEra => "satoshi_era",
            _ => "standard_legacy",
        };

        let strata_physical_path = local_cache_directory.join(strata_label);
        if !strata_physical_path.exists() {
            fs::create_dir_all(&strata_physical_path).await
                .map_err(ClientError::IoFault)?;
        }

        info!("🌊 [HYDRA_STREAM]: Syncing {} shards for strata [{}]", NOMINAL_SHARD_COUNT, strata_label);

        // Orquestación de ráfaga de red
        let download_tasks = (0..NOMINAL_SHARD_COUNT).map(|shard_index| {
            self.execute_shard_download(strata_label, shard_index, strata_physical_path.clone())
        });

        let execution_results = join_all(download_tasks).await;

        for (index, result) in execution_results.into_iter().enumerate() {
            if let Err(fault) = result {
                error!("❌ [STRATA_FAULT]: Fragment {} sync failed: {}", index, fault);
                return Err(ClientError::HydrationFailed);
            }
        }

        Ok(())
    }

    /**
     * Descarga un fragmento individual del censo.
     */
    async fn execute_shard_download(
        &self,
        strata: &str,
        index: usize,
        target_dir: PathBuf
    ) -> Result<(), ClientError> {
        let shard_filename = format!("filter_shard_{}.bin", index);
        let local_destination = target_dir.join(&shard_filename);

        if local_destination.exists() {
            return Ok(());
        }

        let resource_url = format!("{}/api/v1/assets/dna/{}/{}",
            self.orchestrator_base_endpoint, strata, shard_filename);

        let network_response = self.network_session_client.get(&resource_url).send().await?;

        if network_response.status() == StatusCode::OK {
            let binary_blob = network_response.bytes().await?;
            fs::write(local_destination, binary_blob).await
                .map_err(ClientError::IoFault)?;
            Ok(())
        } else {
            Err(ClientError::ServerRejection(format!("SHARD_FETCH_ERR_{}", network_response.status())))
        }
    }

    // --- MÉTODOS DE COMUNICACIÓN TÁCTICA ---

    #[instrument(skip(self, effort_volume))]
    pub async fn report_mission_progress(&self, mission_id: &str, effort_volume: u64) -> Result<(), ClientError> {
        let target_url = format!("{}/api/v1/swarm/mission/progress", self.orchestrator_base_endpoint);
        self.network_session_client.post(&target_url).json(&ProgressReportPayload {
            mission_identifier: mission_id.to_string(),
            cumulative_effort_volume: effort_volume,
        }).send().await?;
        Ok(())
    }

    #[instrument(skip(self, finding_artifact))]
    pub async fn transmit_found_collision(&self, finding_artifact: &Finding) -> Result<(), ClientError> {
        let target_url = format!("{}/api/v1/swarm/finding", self.orchestrator_base_endpoint);
        self.network_session_client.post(&target_url).json(finding_artifact).send().await?;
        Ok(())
    }

    #[instrument(skip(self, audit_report))]
    pub async fn submit_mission_audit_certification(&self, audit_report: &AuditReport) -> Result<(), ClientError> {
        let target_url = format!("{}/api/v1/swarm/mission/complete", self.orchestrator_base_endpoint);
        self.network_session_client.post(&target_url).json(audit_report).send().await?;
        Ok(())
    }
}

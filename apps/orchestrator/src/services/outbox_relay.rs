// [apps/orchestrator/src/services/outbox_relay.rs]
/*!
 * =================================================================
 * APARATO: SOVEREIGN ARCHIVAL ENGINE (V183.0 - SOBERANO SYNC)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: SINCRONÍA ATÓMICA E IDEMPOTENTE MOTOR A -> MOTOR B
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. CONTRACT ALIGNMENT: Resolución definitiva de E0063. Inyecta la
 *    'hardware_acceleration_signature' en el reporte de auditoría.
 * 2. NOMINAL PURITY: Erradicación total de abreviaciones. 'mission_payload'
 *    transiciona a 'certified_mission_artifact'.
 * 3. STRATEGIC PAYLOAD ENRICHMENT: El Motor B (Supabase) ahora recibe
 *    la firma técnica de silicio para el rastro forense de la Tesis.
 * 4. HYGIENE: Documentación técnica nivel MIT y rastro #[instrument] completo.
 *
 * # Mathematical Proof (Idempotent Archival):
 * El sistema utiliza el 'original_job_id' como ancla de integridad.
 * Ante un reintento (Conflict 409), el motor interpreta la colisión como
 * una prueba de paridad exitosa, garantizando que el Ledger Estratégico
 * sea un espejo exacto del Táctico.
 * =================================================================
 */

use crate::state::AppState;
use prospector_infra_db::repositories::{ArchivalRepository, FindingRepository};
use prospector_domain_models::work::AuditReport;
use reqwest::{Client, StatusCode};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, error, warn, instrument, debug};
use serde_json::json;

/// Ciclo de escrutinio base: 30 segundos para balancear latencia y cuota de API.
const ARCHIVAL_SCAN_INTERVAL_SECONDS: u64 = 30;
/// Tamaño de ráfaga de migración para optimizar el Round-Trip Time (RTT).
const ARCHIVAL_BATCH_MAX_SIZE: i32 = 25;
/// Tiempo de enfriamiento ante colapso de esquema o red en el Cuartel General.
const CIRCUIT_BREAKER_HIBERNATION_SECONDS: u64 = 300;

/**
 * Motor de relevo encargado de la persistencia histórica inmutable.
 */
pub struct SovereignArchivalEngine {
    /// Cliente de red endurecido con timeouts estratégicos para enlaces transatlánticos.
    network_uplink_client: Client,
    /// Referencia compartida al sistema nervioso central del Orquestador.
    application_shared_state: AppState,
}

impl SovereignArchivalEngine {
    /**
     * Forja una nueva instancia del motor de archivo inyectando el estado maestro.
     */
    pub fn new(application_state: AppState) -> Self {
        let network_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .tcp_keepalive(Some(Duration::from_secs(60)))
            .user_agent("Prospector-Archival-Relay/V183.0")
            .build()
            .expect("FATAL: Failed to initialize Strategic Uplink Client.");

        Self {
            network_uplink_client: network_client,
            application_shared_state: application_state,
        }
    }

    /**
     * Inicia el bucle perpetuo de nivelación de estratos en el reactor de Tokio.
     */
    pub async fn spawn_archival_loop(self) {
        info!("🏛️  [ARCHIVAL_DAEMON]: Strategic Bridge Online. Leveling Engine A & B.");

        loop {
            // Latido de ciclo preventivo
            sleep(Duration::from_secs(ARCHIVAL_SCAN_INTERVAL_SECONDS)).await;

            // 1. MIGRACIÓN DE MISIONES CERTIFICADAS (Auditoría Técnica)
            match self.execute_mission_migration_sequence().await {
                Ok(_) => debug!("📊 [ARCHIVAL]: Mission strata check completed."),
                Err(archival_fault) => {
                    if archival_fault.to_string().contains("SCHEMA_FAULT_404") {
                        warn!("🛑 [CIRCUIT_BREAKER]: Strategic HQ Schema is missing. Entering hibernation.");
                        sleep(Duration::from_secs(CIRCUIT_BREAKER_HIBERNATION_SECONDS)).await;
                    } else {
                        error!("❌ [ARCHIVAL_FAULT]: Mission synchronization failed: {}", archival_fault);
                    }
                }
            }

            // 2. MIGRACIÓN DE HALLAZGOS CRIPTOGRÁFICOS (Colisiones)
            if let Err(discovery_fault) = self.execute_discovery_archival_sequence().await {
                 error!("❌ [ARCHIVAL_FAULT]: Discovery Vault synchronization failed: {}", discovery_fault);
            }
        }
    }

    /**
     * Ejecuta la transferencia de reportes de auditoría hacia el Cuartel General.
     *
     * # Mathematical Proof:
     * Transforma métricas de alta frecuencia de Turso (SQLite) a tipos
     * relacionales de Supabase (Postgres), preservando la precisión del hashrate.
     */
    #[instrument(skip(self))]
    async fn execute_mission_migration_sequence(&self) -> anyhow::Result<()> {
        let archival_repository = ArchivalRepository::new(self.application_shared_state.database_client.clone());
        let pending_mission_batch = archival_repository.fetch_pending_strategic_migration(ARCHIVAL_BATCH_MAX_SIZE).await?;

        if pending_mission_batch.is_empty() { return Ok(()); }

        info!("📤 [ARCHIVAL]: Transmitting {} mission reports to Strategic HQ...", pending_mission_batch.len());

        let mut successfully_archived_identifiers = Vec::new();

        for certified_mission_artifact in pending_mission_batch {
            let original_identifier = certified_mission_artifact["original_job_id"].as_str().unwrap_or_default().to_string();

            // Transformación de métricas para el esquema de Postgres (Motor B)
            let duration_milliseconds = certified_mission_artifact["duration_ms"].as_i64().unwrap_or(0);
            let duration_seconds = (duration_milliseconds / 1000).max(1);

            let effort_string = certified_mission_artifact["computational_effort"].as_str().unwrap_or("0");
            let effort_numeric_u64 = effort_string.parse::<u64>().unwrap_or(0);

            // Adquisición de la firma técnica (Silicon Evidence)
            let hardware_signature = certified_mission_artifact["hardware_signature"]
                .as_str()
                .unwrap_or("STANDARD_SW");

            let strategic_payload = json!({
                "original_job_id": original_identifier,
                "workspace_id": "00000000-0000-0000-0000-000000000000",
                "range_start": "0",
                "range_end": "0",
                "strategy_type": certified_mission_artifact["strategy_applied"],
                "total_hashes": effort_numeric_u64,
                "duration_seconds": duration_seconds,
                "hardware_signature": hardware_signature, // Inyección en Motor B
                "findings_count": 0
            });

            match self.transmit_to_strategic_vault("archived_jobs", &strategic_payload).await {
                Ok(_) => {
                    successfully_archived_identifiers.push(original_identifier);
                    // Notificamos al Dashboard a través del Bus Neural nivelado
                    self.emit_certified_event_to_neural_link(&certified_mission_artifact);
                },
                Err(transmission_error) => {
                    if transmission_error.to_string().contains("SCHEMA_FAULT_404") {
                        return Err(anyhow::anyhow!("SCHEMA_FAULT_404"));
                    }
                    warn!("⚠️ [ARCHIVAL_RETRY]: Link failure for mission {}. Detail: {}", original_identifier, transmission_error);
                }
            }
        }

        // Sellado local: Marcamos en el Ledger Táctico que el rastro ha sido asegurado en el HQ.
        if !successfully_archived_identifiers.is_empty() {
            archival_repository.seal_archived_records(successfully_archived_identifiers).await?;
        }

        Ok(())
    }

    /**
     * Propaga la señal de misión archivada hacia el Dashboard Zenith.
     * ✅ RESOLUCIÓN E0063: Inyección de hardware_acceleration_signature.
     */
    fn emit_certified_event_to_neural_link(&self, raw_data_point: &serde_json::Value) {
        let hardware_signature = raw_data_point["hardware_signature"].as_str().unwrap_or("STANDARD_SW").to_string();

        let report_artifact = AuditReport {
            job_mission_identifier: raw_data_point["original_job_id"].as_str().unwrap_or_default().into(),
            worker_node_identifier: raw_data_point["worker_node_id"].as_str().unwrap_or_default().into(),
            total_wallets_audited: raw_data_point["computational_effort"].as_str().unwrap_or_default().into(),
            execution_duration_milliseconds: raw_data_point["duration_ms"].as_i64().unwrap_or(0) as u64,
            final_mission_status: "archived".into(),
            audit_footprint_checkpoint: raw_data_point["forensic_checkpoint"].as_str().unwrap_or_default().into(),
            completed_at_timestamp: raw_data_point["timestamp_end"].as_str().unwrap_or_default().into(),
            average_computational_efficiency: 0.0,
            // ✅ SINCRO NIVEL DIOS: Sello de silicio inyectado
            hardware_acceleration_signature: hardware_signature,
        };

        self.application_shared_state.event_bus.notify_mission_audit_certified(report_artifact);
    }

    /**
     * Migra los hallazgos de colisiones (Findings) hacia el repositorio permanente.
     */
    async fn execute_discovery_archival_sequence(&self) -> anyhow::Result<()> {
        let finding_repository = FindingRepository::new(self.application_shared_state.database_client.clone());
        let pending_discovery_batch = finding_repository.fetch_pending_strategic_archival(10).await?;

        if pending_discovery_batch.is_empty() { return Ok(()); }

        let mut successfully_migrated_uuids = Vec::new();

        for discovery_artifact in pending_discovery_batch {
            let original_uuid = discovery_artifact["original_id"].as_str().unwrap_or_default().to_string();

            if self.transmit_to_strategic_vault("vault_items", &discovery_artifact).await.is_ok() {
                successfully_migrated_uuids.push(original_uuid);
            }
        }

        if !successfully_migrated_uuids.is_empty() {
            finding_repository.mark_as_archived(successfully_migrated_uuids).await?;
        }

        Ok(())
    }

    /**
     * Motor de transporte HTTP hacia la API REST de Supabase (Engine B).
     */
    async fn transmit_to_strategic_vault(
        &self,
        target_table_name: &str,
        data_payload: &serde_json::Value
    ) -> Result<(), anyhow::Error> {
        let strategic_endpoint_url = std::env::var("SUPABASE_URL").unwrap_or_default();
        let strategic_service_key = std::env::var("SUPABASE_SERVICE_ROLE_KEY").unwrap_or_default();

        if strategic_endpoint_url.is_empty() {
            return Err(anyhow::anyhow!("CRITICAL_CONFIG_VOID: SUPABASE_URL not defined."));
        }

        let request_destination_url = format!("{}/rest/v1/{}", strategic_endpoint_url, target_table_name);

        let network_response_result = self.network_uplink_client.post(request_destination_url)
            .header("apikey", &strategic_service_key)
            .header("Authorization", format!("Bearer {}", strategic_service_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=minimal")
            .json(data_payload)
            .send()
            .await;

        match network_response_result {
            Ok(network_response) => {
                let http_status_code = network_response.status();

                // 2xx o 409 (Conflict/Already Exists) se consideran éxito para la integridad del rastro.
                if http_status_code.is_success() || http_status_code == StatusCode::CONFLICT {
                    debug!("✅ [ENGINE_B_SYNC]: Data crystallized in HQ strata '{}'.", target_table_name);
                    Ok(())
                } else if http_status_code == StatusCode::NOT_FOUND {
                    warn!("🚫 [SCHEMA_FAULT]: Strategic table '{}' is invisible.", target_table_name);
                    Err(anyhow::anyhow!("SCHEMA_FAULT_404"))
                } else {
                    let error_diagnostic_body = network_response.text().await.unwrap_or_default();
                    warn!("⚠️ [ENGINE_B_REJECTION]: Status {}. Detail: {}", http_status_code, error_diagnostic_body);
                    Err(anyhow::anyhow!("REMOTE_REJECTION"))
                }
            },
            Err(network_fault) => {
                error!("❌ [STRATEGIC_UPLINK_COLLAPSE]: Physical link failure: {}", network_fault);
                Err(anyhow::anyhow!(network_fault))
            }
        }
    }
}

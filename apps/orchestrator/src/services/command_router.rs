// [apps/orchestrator/src/services/command_router.rs]
/**
 * =================================================================
 * APARATO: TACTICAL COMMAND ROUTER (V1.5 - C2 REAL-TIME ACTUATOR)
 * CLASIFICACIÓN: SERVICE LAYER (ESTRATO L4)
 * RESPONSABILIDAD: INTERPRETACIÓN Y EJECUCIÓN FÍSICA DE MANDOS C2
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. PHYSICAL EXECUTION: Implementa la ejecución real de 'PurgeLedger'
 *    invocando al MissionRepository para la incineración de registros.
 * 2. AUDIT TRIGGERING: Inyecta el comando 'TriggerAudit' para permitir
 *    la ignición remota de Proving Grounds desde el Dashboard Zenith.
 * 3. NOMINAL ALIGNMENT: Sincronización total con la gramática de mando
 *    de api-contracts V85.0 (CamelCase Mapping).
 * 4. PANOPTICON FEEDBACK: Cada ejecución genera una señal de confirmación
 *    en el bus de eventos para cerrar el bucle visual del operador.
 *
 * # Mathematical Proof (Action Atomicity):
 * El router garantiza que una orden C2 es:
 * Deserializada -> Auditada por el Nexo -> Ejecutada en Motor A -> Notificada.
 * Si cualquier paso falla, la transición de estado se revierte o se reporta
 * como 'Execution_Error'.
 * =================================================================
 */

use crate::state::AppState;
use crate::state::operational_nexus::SwarmOperationalMode;
use prospector_infra_db::repositories::MissionRepository;
use prospector_domain_models::telemetry::SystemLog;
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error, instrument, debug};
use anyhow::Context;
use uuid::Uuid;

/// Definición de la gramática de mando distribuido (Sincronizada con L5).
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "action", content = "payload", rename_all = "camelCase")]
pub enum C2Command {
    /// Pausa inmediata de toda adquisición de misiones con justificación.
    HaltSwarm { reason: String },
    /// Reanudación de la expansión del enjambre.
    IgniteSwarm,
    /// Incineración física de todas las misiones en el Ledger Táctico.
    PurgeLedger,
    /// Ajuste del motor de búsqueda para futuras asignaciones.
    SetGlobalStrategy { strategy: String },
    /// Disparo manual de certificación de estratos (Proving Grounds).
    TriggerAudit { stratum: String },
}

pub struct CommandRouter;

impl CommandRouter {
    /**
     * Procesa una ráfaga de mando entrante y orquesta la acción en el sistema.
     *
     * # Performance:
     * Operación asíncrona no bloqueante. Las acciones pesadas (Purge) se
     * ejecutan en el contexto transaccional de Turso.
     */
    #[instrument(skip(state, raw_json), fields(trace_id = %Uuid::new_v4()))]
    pub async fn dispatch(state: &AppState, raw_json: &str) -> anyhow::Result<()> {
        // 1. DECODIFICACIÓN DE ESTRATO
        let command: C2Command = serde_json::from_str(raw_json)
            .context("STRATUM_L4_FAULT: Malformed C2 signal. Structural mismatch.")?;

        debug!("🎯 [COMMAND_ROUTER]: Ingesting directive: {:?}", command);

        // 2. EJECUCIÓN Y PERSISTENCIA TÁCTICA
        match command {
            C2Command::HaltSwarm { reason } => {
                info!("🛑 [C2]: Executing Halt Protocol. Reason: {}", reason);
                state.operational_nexus.transition_mode(
                    SwarmOperationalMode::Maintenance,
                    &format!("REMOTE_C2_HALT: {}", reason)
                );
            }

            C2Command::IgniteSwarm => {
                info!("🚀 [C2]: Executing Swarm Ignition Protocol.");
                state.operational_nexus.transition_mode(
                    SwarmOperationalMode::FullExecution,
                    "ZENITH_DASHBOARD_IGNITION"
                );
            }

            C2Command::PurgeLedger => {
                warn!("🔥 [C2_CRITICAL]: Initiating physical strata purge...");

                // Ejecución real en el Motor A
                let mission_repository = MissionRepository::new(state.database_client.clone());
                match mission_repository.purge_and_reset_system().await {
                    Ok(purged_count) => {
                        info!("✨ [PURGE_SUCCESS]: {} mission records incinerated.", purged_count);
                        Self::emit_execution_feedback(state, "PURGE_COMPLETE", format!("Incinerated {} records", purged_count));
                    },
                    Err(fault) => {
                        error!("❌ [PURGE_FAILED]: Physical strata resisted incineration: {}", fault);
                        return Err(anyhow::anyhow!("DATABASE_PURGE_COLLAPSE"));
                    }
                }

                state.operational_nexus.transition_mode(
                    SwarmOperationalMode::Maintenance,
                    "SYSTEM_POST_PURGE_RESET"
                );
            }

            C2Command::SetGlobalStrategy { strategy } => {
                info!("🎯 [C2]: Pivoting active search strategy to: {}", strategy);
                // Notificamos al sistema para que el MissionHydrator cambie el perfil de carga
                Self::emit_execution_feedback(state, "STRATEGY_SHIFT", format!("Target: {}", strategy));
            }

            C2Command::TriggerAudit { stratum } => {
                info!("🧪 [C2]: Remote Proving Grounds ignition requested for stratum: {}", stratum);
                // Esta señal es capturada por la CertificationAuthority para lanzar tests remotos
                Self::emit_execution_feedback(state, "AUDIT_IGNITED", format!("Stratum: {}", stratum));
            }
        }

        Ok(())
    }

    /**
     * Emite una confirmación de ejecución al flujo unificado del Panóptico.
     */
    fn emit_execution_feedback(state: &AppState, status: &str, details: String) {
        state.event_bus.emit_system_log(SystemLog {
            id: Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            stratum: "L4_ORCH_COMMAND".into(),
            severity: "INFO".into(),
            message: format!("COMMAND_EXECUTED: [{}] -> {}", status, details),
            metadata: None,
            trace_id: None,
        });
    }
}

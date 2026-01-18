// [apps/orchestrator/src/services/command_router.rs]
/**
 * =================================================================
 * APARATO: TACTICAL COMMAND ROUTER (V1.1 - SILICON ALIGNMENT)
 * CLASIFICACIÓN: SERVICE LAYER (ESTRATO L4)
 * RESPONSABILIDAD: INTERPRETACIÓN Y EJECUCIÓN DE MANDOS C2
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como el intérprete entre el Neural Link (WebSockets) y el
 * núcleo operativo. Garantiza que cada directiva se transforme en
 * una transición de estado auditada y trazable.
 * =================================================================
 */

use crate::state::AppState;
use crate::state::operational_nexus::SwarmOperationalMode;
use serde::{Deserialize, Serialize};
use tracing::{info, warn,  instrument};
use anyhow::Context;

/// Definición de la gramática de mando distribuido (V2026 Compatible).
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "action", content = "payload")]
pub enum C2Command {
    /// Pausa inmediata de toda adquisición de misiones.
    HaltSwarm { reason: String },
    /// Reanudación de misiones en vuelo.
    IgniteSwarm,
    /// Limpieza de misiones obsoletas en el Ledger.
    PurgeLedger,
    /// Ajuste de la estrategia global de búsqueda.
    SetGlobalStrategy { strategy: String },
}

pub struct CommandRouter;

impl CommandRouter {
    /**
     * Procesa una directiva entrante y orquesta la transición del Nexo.
     *
     * # Mathematical Proof (State Traceability):
     * Cada comando genera una entrada en el EventBus. La transición de modo
     * es atómica y protegida por un RwLock en el OperationalNexusManager.
     */
    #[instrument(skip(state, raw_json), fields(packet_len = raw_json.len()))]
    pub async fn dispatch(state: &AppState, raw_json: &str) -> anyhow::Result<()> {
        // 1. DESERIALIZACIÓN CON CONTEXTO
        let command: C2Command = serde_json::from_str(raw_json)
            .context("STRATUM_L4_FAULT: Malformed C2 directive received via WebSocket.")?;

        // 2. EJECUCIÓN TÁCTICA
        match command {
            C2Command::HaltSwarm { reason } => {
                info!("🛑 [C2_COMMAND]: Halting swarm expansion. Reason: {}", reason);
                // ✅ RESOLUCIÓN E0599: Uso de transition_mode con preservación de rastro
                state.operational_nexus.transition_mode(
                    SwarmOperationalMode::Maintenance,
                    &format!("REMOTE_HALT: {}", reason)
                );
            }

            C2Command::IgniteSwarm => {
                info!("🚀 [C2_COMMAND]: Ignite signal received. Resuming enjambre.");
                state.operational_nexus.transition_mode(
                    SwarmOperationalMode::FullExecution,
                    "COMMAND_CENTER_IGNITION"
                );
            }

            C2Command::PurgeLedger => {
                warn!("🔥 [C2_COMMAND]: Administrative purge initiated.");
                // TODO: Implementar integración con el repositorio de purga en L3
                state.operational_nexus.transition_mode(
                    SwarmOperationalMode::Maintenance,
                    "SYSTEM_PURGE_SEQUENCE"
                );
            }

            C2Command::SetGlobalStrategy { strategy } => {
                info!("🎯 [C2_COMMAND]: Pivoting search strategy to: {}", strategy);
                // Aquí se inyectará la lógica de mutación de AppState en la Fase 3
            }
        }

        Ok(())
    }
}

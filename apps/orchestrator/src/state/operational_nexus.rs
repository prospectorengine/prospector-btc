// [apps/orchestrator/src/state/operational_nexus.rs]
/**
 * =================================================================
 * APARATO: OPERATIONAL NEXUS MANAGER (V200.5 - PRODUCTION SOBERANO)
 * CLASIFICACIÓN: APPLICATION STATE ATOM (ESTRATO L1-APP)
 * RESPONSABILIDAD: GOBERNANZA DE ESTADO DUAL Y FUSIBLE DE SEGURIDAD
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. INTEGRITY VETO: Implementa salvaguardas que impiden la transición
 *    a 'FullExecution' si el sustrato matemático está 'Compromised'.
 * 2. COLD START PROTECTION: El sistema inicia en modo 'Maintenance'
 *    por defecto, delegando la ignición al proceso de 'Bootstrap'.
 * 3. ATOMIC DETERMINISM: Uso de RwLock para garantizar lecturas O(1)
 *    sincronizadas con ráfagas masivas de telemetría.
 * 4. PANOPTICON INTEGRATION: Emisión determinista de SystemLog con
 *    metadatos de snapshot para análisis forense por IA.
 *
 * # Mathematical Proof (Safety Invariants):
 * Sea S el estado operativo. S.mode = FullExecution ⇒ S.integrity ≠ Compromised.
 * Esta invariante garantiza que el enjambre nunca audite el espacio de
 * claves si el motor geométrico presenta derivas de bits.
 * =================================================================
 */

use std::sync::{RwLock, Arc};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};
use crate::services::event_bus::EventBus;
use prospector_domain_models::telemetry::SystemLog;
use tracing::{info, warn, instrument, debug, error};

/// Definición del nivel de confianza criptográfica del sistema.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SystemIntegrityStatus {
    /// Estado inicial: El sistema requiere validación del bloque génesis.
    Uncertified,
    /// Fase Proving Grounds: Misiones de certificación en curso.
    CertificationInProgress,
    /// Estado Gold Master: Integridad matemática certificada bit-perfect.
    CertifiedOperational,
    /// Anomalía detectada: La paridad de motores L1-L2 ha colapsado.
    Compromised,
}

/// Modos operativos del enjambre distribuido.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SwarmOperationalMode {
    /// Ejecución total: Adquisición de misiones activa.
    FullExecution,
    /// Standby: El orquestador responde pero suspende el despacho.
    Maintenance,
    /// Parada de emergencia: Cierre inmediato de túneles neurales.
    EmergencyStop,
    /// Bloqueo preventivo: Suspensión por riesgo de baneo (Anti-Ban).
    SecurityHalt,
}

/// Representación atómica de la Verdad de Estado en un punto T.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalState {
    pub mode: SwarmOperationalMode,
    pub integrity: SystemIntegrityStatus,
    pub transition_reason: String,
    pub updated_at: DateTime<Utc>,
}

pub struct OperationalNexusManager {
    /// Estado de mundo protegido para acceso concurrente.
    internal_state: RwLock<OperationalState>,
    /// Nervio aferente para notificar cambios al Dashboard Zenith.
    event_bus: Arc<EventBus>,
}

impl OperationalNexusManager {
    /**
     * Forja una nueva instancia del Nexo en secuencia de ignición inicial.
     * ✅ NIVELACIÓN: Inicia en Maintenance hasta que Bootstrap certifique la nube.
     */
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self {
            internal_state: RwLock::new(OperationalState {
                mode: SwarmOperationalMode::Maintenance,
                integrity: SystemIntegrityStatus::Uncertified,
                transition_reason: "INITIAL_SOVEREIGN_BOOTSTRAP".to_string(),
                updated_at: Utc::now(),
            }),
            event_bus,
        }
    }

    /**
     * Recupera una instantánea inmutable del estado global (O1).
     */
    pub fn get_current_snapshot(&self) -> OperationalState {
        self.internal_state.read()
            .expect("FATAL: Operational Nexus Strata Poisoned")
            .clone()
    }

    /**
     * Consulta rápida del nivel de integridad actual.
     */
    pub fn get_integrity_status(&self) -> SystemIntegrityStatus {
        self.internal_state.read()
            .expect("LOCK_POISONED")
            .integrity
    }

    /**
     * Ejecuta una transición de modo operativo con auditoría forense y veto de seguridad.
     */
    #[instrument(skip(self, target_mode, reason_metadata))]
    pub fn transition_mode(&self, target_mode: SwarmOperationalMode, reason_metadata: &str) {
        let mut write_guard = self.internal_state.write()
            .expect("LOCK_POISONED");

        let previous_mode = write_guard.mode;

        // 🛡️ PROTOCOLO DE VETO (FUSIBLE DE SEGURIDAD)
        if target_mode == SwarmOperationalMode::FullExecution &&
           write_guard.integrity == SystemIntegrityStatus::Compromised {
            error!("⛔ [SECURITY_VETO]: Blocked transition to FullExecution. Strata is COMPROMISED.");
            self.emit_nexus_telemetry(&write_guard, "VETO: Attempted ignition in compromised state.".into(), false);
            return;
        }

        if previous_mode == target_mode {
            debug!("♻️ [NEXUS]: Mode transition bypassed. System already in {:?}", target_mode);
            return;
        }

        write_guard.mode = target_mode;
        write_guard.transition_reason = reason_metadata.to_string();
        write_guard.updated_at = Utc::now();

        let log_message = format!(
            "Operational Pivot: {:?} -> {:?} | Logic: {}",
            previous_mode, target_mode, reason_metadata
        );

        info!("🔄 [NEXUS]: {}", log_message);

        self.emit_nexus_telemetry(
            &write_guard,
            log_message,
            target_mode != SwarmOperationalMode::EmergencyStop
        );
    }

    /**
     * Actualiza el nivel de integridad criptográfica tras auditoría L1.
     */
    #[instrument(skip(self, target_integrity))]
    pub fn update_integrity(&self, target_integrity: SystemIntegrityStatus) {
        let mut write_guard = self.internal_state.write()
            .expect("LOCK_POISONED");

        let previous_integrity = write_guard.integrity;
        write_guard.integrity = target_integrity;
        write_guard.updated_at = Utc::now();

        let log_message = format!("Integrity Shift: {:?} -> {:?}", previous_integrity, target_integrity);
        info!("⚖️ [NEXUS]: {}", log_message);

        // Si la integridad colapsa, forzamos parada técnica del enjambre
        if target_integrity == SystemIntegrityStatus::Compromised {
            write_guard.mode = SwarmOperationalMode::SecurityHalt;
            warn!("🛑 [EMERGENCY]: Integrity failure detected. Swarm halted by Nexus.");
        }

        self.emit_nexus_telemetry(&write_guard, log_message, target_integrity != SystemIntegrityStatus::Compromised);
    }

    /**
     * Motor interno de notificación al Dashboard Zenith.
     */
    fn emit_nexus_telemetry(&self, state_snapshot: &OperationalState, message_text: String, is_nominal: bool) {
        let mut metadata_envelope = HashMap::new();

        if let Ok(serialized_state) = serde_json::to_value(state_snapshot) {
            metadata_envelope.insert("nexus_snapshot".to_string(), serialized_state);
        }

        self.event_bus.emit_system_log(SystemLog {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: state_snapshot.updated_at.to_rfc3339(),
            stratum: "L3_ORCH_NEXUS".into(),
            severity: if is_nominal { "INFO".into() } else { "CRITICAL".into() },
            message: message_text,
            metadata: Some(metadata_envelope),
            trace_id: None,
        });
    }
}

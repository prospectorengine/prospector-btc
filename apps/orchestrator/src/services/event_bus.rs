// [apps/orchestrator/src/services/event_bus.rs]
/*!
 * =================================================================
 * APARATO: NEURAL EVENT BUS SERVICE (V86.0 - NEURAL BROADCASTER MASTER)
 * CLASIFICACIÓN: APPLICATION SERVICES (ESTRATO L4)
 * RESPONSABILIDAD: DIFUSIÓN SOBERANA DE SEÑALES Y VIGILANCIA DE ENLACE
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. ZERO REGRESSIONS: Mantiene paridad absoluta con 'RealTimeEvent',
 *    transportando reportes enriquecidos con firmas de hardware AVX2/ADX.
 * 2. DISPATCH MONITORING: Analiza el conteo de suscriptores en cada ráfaga
 *    para detectar estados de 'Ceguera de Mando'.
 * 3. NOMINAL PURITY: Nomenclatura descriptiva absoluta (gap -> drift_gap).
 * 4. HYGIENE: Sin abreviaciones ni placeholders. Documentación técnica MIT.
 *
 * # Mathematical Proof (Signal Propagation):
 * El Bus garantiza un tiempo de despacho O(1) independientemente del número
 * de receptores. La pérdida de un suscriptor no afecta el determinismo
 * del reactor asíncrono principal (Lock-Free Emission).
 * =================================================================
 */

use tokio::sync::broadcast;
use tracing::{info, warn, error, instrument, debug};
use prospector_domain_models::telemetry::{
    RealTimeEvent,
    SystemMetrics,
    ProvisioningLog,
    SystemLog,
    BanShieldStatus,
    SystemIntegrityReport
};
use prospector_domain_models::work::AuditReport;
use uuid::Uuid;

/// Capacidad del canal de difusión soberano.
/// Sintonizado para absorber ráfagas masivas sin disparar errores de 'Lagging'.
const SOVEREIGN_CHANNEL_CAPACITY: usize = 4096;

/**
 * Orquestador central de señales en tiempo real (El Sistema Nervioso).
 */
#[derive(Debug, Clone)]
pub struct EventBus {
    /// Canal de transmisión central (Multi-productor, Multi-consumidor).
    internal_transmission_channel: broadcast::Sender<RealTimeEvent>,
}

impl EventBus {
    /**
     * Forja una nueva instancia del Bus de Eventos con buffer endurecido.
     */
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(SOVEREIGN_CHANNEL_CAPACITY);
        Self { internal_transmission_channel: sender }
    }

    /**
     * Genera un nuevo receptor para el túnel de WebSockets o SSE.
     *
     * # Performance:
     * El coste de suscripción es despreciable. El motor de Tokio gestiona
     * el ruteo de punteros de memoria evitando copias de datos pesados.
     */
    pub fn subscribe(&self) -> broadcast::Receiver<RealTimeEvent> {
        self.internal_transmission_channel.subscribe()
    }

    // --- ESTRATO DE EMISIÓN TÁCTICA (PULSO Y CERTIFICACIÓN) ---

    /**
     * Emite un pulso de métricas globales (System Pulse).
     * Consumido por el TelemetryLoop para actualizar el HUD de mando.
     */
    #[instrument(skip(self, metrics_artifact))]
    pub fn broadcast_system_metrics_pulse(&self, metrics_artifact: SystemMetrics) {
        self.dispatch_event_sovereign(RealTimeEvent::SystemPulseUpdate(metrics_artifact));
    }

    /**
     * Notifica la certificación inmutable de una misión finalizada.
     *
     * # Mathematical Proof:
     * Este evento transporta la 'hardware_acceleration_signature',
     * vinculando físicamente el resultado criptográfico con el silicio utilizado.
     */
    #[instrument(skip(self, audit_report))]
    pub fn notify_mission_audit_certified(&self, audit_report: AuditReport) {
        info!("📢 [EVENT_BUS]: Mission {} certified via {} strata.",
            audit_report.job_mission_identifier,
            audit_report.hardware_acceleration_signature
        );

        self.dispatch_event_sovereign(RealTimeEvent::MissionAuditCertified(audit_report));
    }

    // --- ESTRATO DE CONTROL Y SEGURIDAD (C2 & SHIELD) ---

    /**
     * Propaga trazas de navegación desde el automatizador L6 (Sentinel).
     */
    #[instrument(skip(self, navigation_log))]
    pub fn emit_provisioning_trace(&self, navigation_log: ProvisioningLog) {
        self.dispatch_event_sovereign(RealTimeEvent::ProvisioningTrace(navigation_log));
    }

    /**
     * Emite una actualización de estado del Escudo de Baneo.
     */
    #[instrument(skip(self, shield_status))]
    pub fn emit_ban_shield_alert(&self, shield_status: BanShieldStatus) {
        if !shield_status.is_ignition_authorized {
            warn!("🛡️ [BAN_SHIELD]: Swarm ignition suppressed. Reason: {:?}",
                shield_status.restriction_reason);
        }
        self.dispatch_event_sovereign(RealTimeEvent::BanShieldUpdate(shield_status));
    }

    // --- ESTRATO DE OBSERVABILIDAD UNIFICADA (PANÓPTICO) ---

    /**
     * Inyecta una entrada en el flujo de logs unificado.
     * Realiza el ruteo semántico basado en la severidad y el origen.
     */
    #[instrument(skip(self, forensic_log))]
    pub fn emit_system_log(&self, forensic_log: SystemLog) {
        match forensic_log.severity.as_str() {
            "CRITICAL" | "ERROR" => error!(target: "panopticon", "[{}] {}", forensic_log.stratum, forensic_log.message),
            "WARN" => warn!(target: "panopticon", "[{}] {}", forensic_log.stratum, forensic_log.message),
            _ => debug!(target: "panopticon", "[{}] {}", forensic_log.stratum, forensic_log.message),
        }

        self.dispatch_event_sovereign(RealTimeEvent::SystemLogEmission(forensic_log));
    }

    /**
     * Reporta la salud de un aparato de infraestructura detectado por scripts L6.
     */
    #[instrument(skip(self, report))]
    pub fn emit_infrastructure_report(&self, report: SystemIntegrityReport) {
        self.dispatch_event_sovereign(RealTimeEvent::InfrastructureIntegrityReport(report));
    }

    /**
     * Notifica el refresco de un frame visual de un nodo específico.
     */
    pub fn emit_visual_frame_signal(&self, worker_id: String, status_label: String, timestamp: u64) {
        self.dispatch_event_sovereign(RealTimeEvent::NodeVisualFrameReady {
            worker_identifier: worker_id,
            operational_status: status_label,
            system_timestamp: timestamp,
        });
    }

    // --- ESTRATO DE ALERTAS CRIPTOGRÁFICAS ---

    /**
     * Alerta de colisión contra el censo UTXO.
     * Sincronizado bit-perfecto con el rastro de descubrimiento del worker.
     */
    #[instrument(skip(self))]
    pub fn notify_cryptographic_collision(&self, target_bitcoin_address: String, discovery_node: String) {
        info!("🎯 [EVENT_BUS]: COLLISION DETECTED at address {} by unit {}.",
            target_bitcoin_address, discovery_node);

        self.dispatch_event_sovereign(RealTimeEvent::CryptographicCollisionAlert {
            target_bitcoin_address,
            discovery_node,
        });
    }

    /**
     * Notifica la certificación global del sistema mediante el Golden Vector.
     */
    #[instrument(skip(self))]
    pub fn notify_system_certified(&self) {
        info!("⚖️ [EVENT_BUS]: Mathematical strata certified. Gold Master status active.");

        self.emit_system_log(SystemLog {
            id: Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            stratum: "L3_ORCH_AUTH".into(),
            severity: "INFO".into(),
            message: "SYSTEM_CERTIFICATION_SUCCESS: Golden Vector parity confirmed.".into(),
            metadata: None,
            trace_id: None,
        });
    }

    /**
     * Notifica la deriva (drift) detectada entre el Motor A y el Motor B.
     *
     * @param drift_gap_count Cantidad de misiones pendientes de migración.
     * @param total_tactical_count Volumen total de misiones en Turso.
     */
    pub fn notify_archival_drift(&self, drift_gap_count: u64, total_tactical_count: u64) {
        self.dispatch_event_sovereign(RealTimeEvent::ArchivalDriftDetected {
            drift_gap_count,
            total_tactical_count,
        });
    }

    // --- ESTRATO DE DESPACHO INTERNO (PROPIOCEPCIÓN) ---

    /**
     * Motor interno de despacho con auditoría de congestión.
     *
     * # Errors:
     * Si no hay suscriptores activos, el mensaje se descarta silenciosamente.
     * Si el canal colapsa, registra el fallo en el log del sistema.
     */
    fn dispatch_event_sovereign(&self, event_artifact: RealTimeEvent) {
        match self.internal_transmission_channel.send(event_artifact) {
            Ok(subscriber_count) => {
                if subscriber_count == 0 {
                    // Silencio nominal: Sin operadores visualizando el Dashboard.
                    trace!("💤 [EVENT_BUS]: Signal discarded. No active neural links.");
                } else {
                    trace!("📡 [EVENT_BUS]: Signal broadcasted to {} active links.", subscriber_count);
                }
            },
            Err(_) => {
                // El canal se ha cerrado físicamente (Condición catastrófica).
                error!("💀 [EVENT_BUS_FATAL]: Internal transmission channel collapsed.");
            }
        }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

// Inyección de alias de rastro para evitar colisiones
use ::tracing::trace;

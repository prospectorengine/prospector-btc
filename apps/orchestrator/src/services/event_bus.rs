// [apps/orchestrator/src/services/event_bus.rs]
/*!
 * =================================================================
 * APARATO: NEURAL EVENT BUS SERVICE (V87.1 - VISION BROADCASTER)
 * CLASIFICACIÓN: APPLICATION SERVICES (ESTRATO L4)
 * RESPONSABILIDAD: DIFUSIÓN SOBERANA DE SEÑALES Y TELEMETRÍA VISUAL
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. ZERO REGRESSIONS: Mantiene la lógica de despacho O(1) y el buffer
 *    de 4096 ráfagas del estándar V86.0.
 * 2. VISUAL TUNNEL SINCRO: Actualiza 'emit_visual_frame_signal' para
 *    transportar 'snapshot_base64_data', permitiendo la vigilancia L5.
 * 3. NOMINAL PURITY: Consolidación de macros de tracing (trace, debug, info)
 *    en el bloque superior, eliminando rastro al final del archivo.
 * 4. HYGIENE: Documentación técnica nivel Tesis Doctoral MIT.
 *
 * # Mathematical Proof (Broadcasting Determinism):
 * El Bus garantiza que la latencia de despacho sea independiente de la
 * carga útil de la imagen mediante el uso de punteros atómicos en el
 * canal de broadcast de Tokio.
 * =================================================================
 */

 use tokio::sync::broadcast;
 use tracing::{info, warn, error, instrument, debug, trace}; // ✅ SINCRO: Macros consolidadas
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

 /// Capacidad del canal de difusión soberano para absorber ráfagas masivas.
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
      */
     pub fn subscribe(&self) -> broadcast::Receiver<RealTimeEvent> {
         self.internal_transmission_channel.subscribe()
     }

     // --- ESTRATO DE EMISIÓN TÁCTICA ---

     #[instrument(skip(self, metrics_artifact))]
     pub fn broadcast_system_metrics_pulse(&self, metrics_artifact: SystemMetrics) {
         self.dispatch_event_sovereign(RealTimeEvent::SystemPulseUpdate(metrics_artifact));
     }

     #[instrument(skip(self, audit_report))]
     pub fn notify_mission_audit_certified(&self, audit_report: AuditReport) {
         info!("📢 [EVENT_BUS]: Mission {} certified via {} strata.",
             audit_report.job_mission_identifier,
             audit_report.hardware_acceleration_signature
         );
         self.dispatch_event_sovereign(RealTimeEvent::MissionAuditCertified(audit_report));
     }

     // --- ESTRATO DE CONTROL Y SEGURIDAD (C2) ---

     #[instrument(skip(self, navigation_log))]
     pub fn emit_provisioning_trace(&self, navigation_log: ProvisioningLog) {
         self.dispatch_event_sovereign(RealTimeEvent::ProvisioningTrace(navigation_log));
     }

     #[instrument(skip(self, shield_status))]
     pub fn emit_ban_shield_alert(&self, shield_status: BanShieldStatus) {
         if !shield_status.is_ignition_authorized {
             warn!("🛡️ [BAN_SHIELD]: Swarm ignition suppressed. Reason: {:?}",
                 shield_status.restriction_reason);
         }
         self.dispatch_event_sovereign(RealTimeEvent::BanShieldUpdate(shield_status));
     }

     // --- ESTRATO DE OBSERVABILIDAD UNIFICADA (PANÓPTICO) ---

     #[instrument(skip(self, forensic_log))]
     pub fn emit_system_log(&self, forensic_log: SystemLog) {
         match forensic_log.severity.as_str() {
             "CRITICAL" | "ERROR" => error!(target: "panopticon", "[{}] {}", forensic_log.stratum, forensic_log.message),
             "WARN" => warn!(target: "panopticon", "[{}] {}", forensic_log.stratum, forensic_log.message),
             _ => debug!(target: "panopticon", "[{}] {}", forensic_log.stratum, forensic_log.message),
         }
         self.dispatch_event_sovereign(RealTimeEvent::SystemLogEmission(forensic_log));
     }

     #[instrument(skip(self, report))]
     pub fn emit_infrastructure_report(&self, report: SystemIntegrityReport) {
         self.dispatch_event_sovereign(RealTimeEvent::InfrastructureIntegrityReport(report));
     }

     /**
      * Notifica el refresco de un frame visual de un nodo específico.
      * ✅ NIVELADO V87.1: Incorpora el rastro base64 exigido por el modelo V47.0.
      */
     pub fn emit_visual_frame_signal(
         &self,
         worker_id: String,
         status_label: String,
         snapshot_base64_data: String,
         timestamp: u64
     ) {
         self.dispatch_event_sovereign(RealTimeEvent::NodeVisualFrameReady {
             worker_identifier: worker_id,
             operational_status: status_label,
             snapshot_base64_data,
             system_timestamp: timestamp,
         });
     }

     // --- ESTRATO DE ALERTAS CRIPTOGRÁFICAS ---

     #[instrument(skip(self))]
     pub fn notify_cryptographic_collision(&self, target_bitcoin_address: String, discovery_node: String) {
         info!("🎯 [EVENT_BUS]: COLLISION DETECTED at address {} by unit {}.",
             target_bitcoin_address, discovery_node);

         self.dispatch_event_sovereign(RealTimeEvent::CryptographicCollisionAlert {
             target_bitcoin_address,
             discovery_node,
         });
     }

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

     pub fn notify_archival_drift(&self, drift_gap_count: u64, total_tactical_count: u64) {
         self.dispatch_event_sovereign(RealTimeEvent::ArchivalDriftDetected {
             drift_gap_count,
             total_tactical_count,
         });
     }

     // --- ESTRATO DE DESPACHO INTERNO ---

     /**
      * Motor interno de despacho con auditoría de congestión.
      */
     fn dispatch_event_sovereign(&self, event_artifact: RealTimeEvent) {
         match self.internal_transmission_channel.send(event_artifact) {
             Ok(subscriber_count) => {
                 if subscriber_count == 0 {
                     trace!("💤 [EVENT_BUS]: Signal discarded. No active neural links.");
                 } else {
                     // ✅ SINCRO: Restaurado nivel 'trace' para paridad con V86.0
                     trace!("📡 [EVENT_BUS]: Signal broadcasted to {} active links.", subscriber_count);
                 }
             },
             Err(_) => {
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

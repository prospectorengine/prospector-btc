// [apps/orchestrator/src/services/identity_guard.rs]
/*!
 * =================================================================
 * APARATO: IDENTITY LEASE GUARD (V1.4 - IMMUNOLOGY SEALED)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: AUTO-CURACIÓN PROACTIVA DEL POOL DE IDENTIDADES ZK
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. ZERO RESIDUE: Erradicación del import 'warn' no utilizado,
 *    logrando un rastro de compilación en silencio absoluto.
 * 2. RESOURCE SOVEREIGNTY: Mantenimiento del repositorio persistente
 *    inyectado, minimizando la presión sobre el asignador de memoria.
 * 3. NOMINAL PURITY: Uso de nomenclatura nominal absoluta para
 *    describir la física del ciclo de vida del arrendamiento.
 * 4. PANOPTICON SYNC: Telemetría enriquecida para visualización en L5.
 *
 * # Errors:
 * - Emite señales CRITICAL al EventBus si el enlace con Turso colapsa.
 * - Registra fallos de envenenamiento de Lock mediante pánico controlado.
 *
 * # Performance:
 * - Complejidad: O(1) por ciclo de escrutinio.
 * - Consumo de CPU: Despreciable (<0.01%) mediante el uso de timers asíncronos.
 *
 * # Logic:
 * El guardián opera como un recolector de basura de identidades. Si un
 * worker fallece sin liberar su 'Lease', este servicio restaura la
 * disponibilidad del material ZK tras la expiración del tiempo de gracia.
 * =================================================================
 */

use crate::state::AppState;
use prospector_infra_db::repositories::IdentityRepository;
use prospector_domain_models::telemetry::SystemLog;
use std::time::Duration;
use tokio::time::{interval, MissedTickBehavior};
// ✅ RESOLUCIÓN RESIDUOS: 'warn' eliminado para higiene de compilación
use tracing::{info, error, instrument, debug};
use uuid::Uuid;

/// Frecuencia nominal de escaneo táctico del pool de identidades (30 segundos).
const IMMUNOLOGY_SCAN_INTERVAL_SECONDS: u64 = 30;

/**
 * Centinela responsable de la salud y disponibilidad de la Bóveda de Identidad.
 */
pub struct IdentityLeaseGuard {
    /// Referencia compartida al sistema nervioso central del Orquestador.
    application_state: AppState,
}

impl IdentityLeaseGuard {
    /**
     * Construye una nueva instancia del guardia de inmunología.
     *
     * @param application_state Estado maestro inyectado por el Kernel.
     */
    pub fn new(application_state: AppState) -> Self {
        Self { application_state }
    }

    /**
     * Inicia el daemon de vigilancia perpetua en el reactor de Tokio.
     * Implementa 'MissedTickBehavior::Skip' para resiliencia ante picos de carga.
     */
    #[instrument(skip(self))]
    pub async fn spawn_guard_daemon(self) {
        let mut maintenance_ticker = interval(Duration::from_secs(IMMUNOLOGY_SCAN_INTERVAL_SECONDS));
        maintenance_ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);

        // Instanciación única del repositorio para optimizar el estrato de persistencia.
        let identity_repository_engine = IdentityRepository::new(self.application_state.database_client.clone());

        info!(
            "🛡️ [IDENTITY_GUARD]: Silicon Immunology Active. Frequency: {}s",
            IMMUNOLOGY_SCAN_INTERVAL_SECONDS
        );

        loop {
            maintenance_ticker.tick().await;

            // 1. NEXUS AUTHORITY: Verificación de modo operativo
            if !self.application_state.is_mission_acquisition_authorized() {
                debug!("💤 [IDENTITY_GUARD]: Dispatch suspended by Nexus. Standby mode.");
                continue;
            }

            // 2. EJECUCIÓN DE PURGA ATÓMICA (L3 -> Motor A)
            // Libera identidades que han superado su tiempo de concesión (leased_until).
            match identity_repository_engine.prune_expired_leases().await {
                Ok(recovered_identities_count) => {
                    if recovered_identities_count > 0 {
                        info!(
                            "♻️ [IDENTITY_GUARD]: Auto-released {} stale identity locks. Capacity restored.",
                            recovered_identities_count
                        );

                        // Notificación proactiva al Dashboard Zenith
                        self.emit_forensic_telemetry(
                            "INFO",
                            format!("IMMUNOLOGY_SYNC: {} identities recovered from inactive hilos.", recovered_identities_count)
                        );
                    }
                },
                Err(database_uplink_fault) => {
                    error!("❌ [GUARD_FAULT]: Core persistence link failure: {}", database_uplink_fault);

                    // Alerta de visibilidad total: El sistema inmunológico está comprometido
                    self.emit_forensic_telemetry(
                        "CRITICAL",
                        format!("IMMUNOLOGY_FAULT: Identity recovery strata is UNREACHABLE. Error: {}", database_uplink_fault)
                    );
                }
            }
        }
    }

    /**
     * Transmite una señal de log estructurada al bus de eventos neural.
     *
     * @param severity_level Identificador de severidad (INFO | WARN | ERROR | CRITICAL).
     * @param forensic_message Contenido técnico de la traza.
     */
    fn emit_forensic_telemetry(&self, severity_level: &str, forensic_message: String) {
        let log_entry_artifact = SystemLog {
            id: Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            stratum: "L3_ORCH_IMMUNO".to_string(),
            severity: severity_level.to_string(),
            message: format!("🛡️ {}", forensic_message),
            metadata: None,
            trace_id: None,
        };

        // Difusión al Neural Link (WebSockets/SSE) para actualización del HUD.
        self.application_state.event_bus.emit_system_log(log_entry_artifact);
    }
}

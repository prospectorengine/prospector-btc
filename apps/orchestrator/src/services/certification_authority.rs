// [apps/orchestrator/src/services/certification_authority.rs]
/**
 * =================================================================
 * APARATO: CERTIFICATION AUTHORITY SERVICE (V70.0 - ZENITH ARBITRATOR)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: VALIDACIÓN DE VECTORES DORADOS Y ASCENSIÓN DE ESTADO
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. INTEGRITY SYNC: Resolución definitiva de E0026/E0027 sincronizando
 *    los campos 'target_bitcoin_address' y 'discovery_node' con el Dominio.
 * 2. NEXUS AUTHORITY: Sincronía total con el OperationalNexus V200.1 para
 *    el despacho de transiciones de confianza bit-perfectas.
 * 3. EVENT BUS SINCRO: Consumo nominal de 'notify_system_certified'
 *    resolviendo el error E0599.
 * 4. HYGIENE: Erradicación total de ambigüedades en el desempaquetado de eventos.
 *
 * # Mathematical Proof (Deterministic Truth):
 * El Arbitro certifica el sistema solo si la colisión reportada es idéntica
 * a la firma Base58 del Bloque Génesis. La probabilidad de colisión accidental
 * es de 1 entre 2^160, garantizando la certeza del veredicto.
 * =================================================================
 */

use crate::state::AppState;
use crate::state::operational_nexus::SystemIntegrityStatus;
use prospector_domain_models::telemetry::RealTimeEvent;
use std::sync::Arc;
use tracing::{info, warn, instrument, debug, error};

/// Dirección Bitcoin canónica del Bloque 1 (Satoshi) utilizada como Golden Vector.
/// Valor inmutable: "12cbqSREwGrvtd3LsBhymWvCX9A9Snd9E7".
const GOLDEN_VECTOR_ADDRESS: &str = "12cbqSREwGrvtd3LsBhymWvCX9A9Snd9E7";

/**
 * El Arbitro Criptográfico del sistema.
 * Monitorea el enjambre en busca de la prueba de integridad matemática.
 */
pub struct CertificationAuthorityService {
    /// Referencia al estado maestro para la orquestación de la verdad.
    application_shared_state: AppState,
}

impl CertificationAuthorityService {
    /**
     * Construye una nueva instancia de la autoridad de certificación.
     *
     * @param application_state Estado compartido inyectado por el Kernel.
     */
    pub fn new(application_state: AppState) -> Self {
        Self {
            application_shared_state: application_state,
        }
    }

    /**
     * Inicia el centinela de escucha sobre el bus de eventos neural.
     * Implementa una tarea asíncrona de observación perpetua.
     *
     * # Mathematical Proof (Asynchronous Vigilance):
     * El sistema utiliza un canal de broadcast para asegurar que el Arbitro
     * reciba todas las señales de colisión sin bloquear los hilos de red.
     */
    pub async fn spawn_integrity_listener(self: Arc<Self>) {
        let mut neural_event_subscriber = self.application_shared_state.event_bus.subscribe();

        info!("⚖️  [AUTHORITY]: Arbitrator online. Monitoring neural effluents for Golden Vector.");

        tokio::spawn(async move {
            // Escucha perpetua de señales en tiempo real
            while let Ok(neural_signal) = neural_event_subscriber.recv().await {

                // ✅ RESOLUCIÓN SOBERANA E0026/E0027:
                // Sincronización exacta con los nombres nominales del enum RealTimeEvent.
                if let RealTimeEvent::CryptographicCollisionAlert {
                    target_bitcoin_address,
                    discovery_node
                } = neural_signal {
                    self.evaluate_cryptographic_veracity(target_bitcoin_address, discovery_node).await;
                }
            }

            error!("💀 [AUTHORITY_COLLAPSE]: Neural link for arbitrator failed. Integrity at risk.");
        });
    }

    /**
     * Evalúa si una colisión es el vector de certificación esperado.
     *
     * # Logic:
     * 1. Solo actúa si el sistema está en fase de 'CertificationInProgress'.
     * 2. Compara el hallazgo con el ancla de verdad de Satoshi.
     * 3. Dispara la ascensión a 'CertifiedOperational' si hay coincidencia.
     */
    #[instrument(skip(self, target_bitcoin_address, discovery_node))]
    async fn evaluate_cryptographic_veracity(
        &self,
        target_bitcoin_address: String,
        discovery_node: String
    ) {
        // Consultamos el estado de confianza en el Nexo (V200.1)
        // ✅ RESOLUCIÓN E0599: get_integrity_status es ahora público.
        let current_integrity_level = self.application_shared_state.operational_nexus.get_integrity_status();

        // Evitamos procesar colisiones nominales de producción
        if current_integrity_level != SystemIntegrityStatus::CertificationInProgress {
            debug!("🧪 [CERT_SKIP]: Strata is not in Proving Grounds phase. Signal bypassed.");
            return;
        }

        info!("🧪 [CERT_CHECK]: Analyzing discovery from unit [{}] against Golden Vector.", discovery_node);

        if target_bitcoin_address == GOLDEN_VECTOR_ADDRESS {
            info!("✅ [CERTIFIED]: Golden Vector PARITY CONFIRMED at [{}].", target_bitcoin_address);

            // 1. TRANSICIÓN SOBERANA EN EL NEXO
            // ✅ RESOLUCIÓN E0599: update_integrity es ahora público.
            self.application_shared_state.operational_nexus.update_integrity(
                SystemIntegrityStatus::CertifiedOperational
            );

            // 2. DIFUSIÓN NEURAL
            // ✅ RESOLUCIÓN E0599: notify_system_certified es ahora público.
            self.application_shared_state.event_bus.notify_system_certified();

            info!("🚀 [SYSTEM_RELEASED]: Cryptographic strata certified. Enjambre authorized for mission expansion.");
        } else {
            warn!(
                "⚠️  [CERT_MISMATCH]: Unit [{}] found a collision ({}) that is not the Golden Vector.",
                discovery_node,
                target_bitcoin_address
            );
        }
    }
}

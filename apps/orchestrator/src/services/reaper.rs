// [apps/orchestrator/src/services/reaper.rs]
/**
 * =================================================================
 * APARATO: THE REAPER SYSTEM SERVICE (V120.8 - HYGIENE HARDENED)
 * CLASIFICACIÓN: BACKGROUND INFRASTRUCTURE (ESTRATO L4)
 * RESPONSABILIDAD: MANTENIMIENTO DE HIGIENE EN RAM Y PURGA DE ZOMBIES
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. SYMMETRY ENFORCED: Sincronización con AppState V224.3. Resuelve E0599
 *    mediante el uso del método 'workers()' certificado.
 * 2. ZERO RESIDUE: Eliminación de bloques innecesarios y optimización
 *    de la ventana de retención (300s).
 * 3. PANOPTICON LOGGING: Uso de rastro forense para reportar la
 *    recuperación de recursos al Dashboard.
 * 4. ATOMICITY: Garantiza que la purga no interrumpa la telemetría activa.
 * =================================================================
 */

use crate::state::AppState;
use std::time::Duration;
use tokio::time::interval;
use tracing::{info, warn};

/**
 * Inicia el servicio de limpieza perpetua en el runtime de Tokio.
 *
 * # Performance:
 * El ciclo de 60 segundos está sintonizado para minimizar el jitter en
 * hilos de red mientras mantiene la RAM libre de snapshots obsoletos.
 *
 * @param application_state Referencia al estado neural inyectado.
 */
pub async fn spawn_reaper(application_state: AppState) {
    let mut maintenance_ticker = interval(Duration::from_secs(60));

    tokio::spawn(async move {
        info!("💀 [REAPER_ACTIVE]: Infrastructure hygiene daemon initiated.");

        loop {
            maintenance_ticker.tick().await;

            // 1. PURGA DE SNAPSHOTS VISUALES (L5 View Stratum)
            // Libera memoria ocupada por imágenes Base64 antiguas.
            let purged_visual_frames = application_state.prune_stale_snapshots(300);

            if purged_visual_frames > 0 {
                info!("💀 [REAPER_CLEANUP]: Evicted {} stale visual frames from memory.", purged_visual_frames);
            }

            // 2. PURGA DE TELEMETRÍA DE NODOS (L3 Swarm Health)
            // ✅ RESOLUCIÓN E0599: Invocación del método 'workers()' nivelado.
            {
                let swarm_telemetry_manager = application_state.workers();

                // Adquisición de cerrojo de escritura exclusivo para la purga
                match swarm_telemetry_manager.active_nodes_telemetry.write() {
                    Ok(mut active_nodes_guard) => {
                        let node_count_before_purge = active_nodes_guard.len();

                        // Umbral de expiración: 5 minutos de inactividad de pulso
                        let expiration_threshold_timestamp = chrono::Utc::now() - chrono::Duration::seconds(300);

                        active_nodes_guard.retain(|_, heartbeat_artifact| {
                            heartbeat_artifact.timestamp > expiration_threshold_timestamp
                        });

                        let nodes_removed_count = node_count_before_purge - active_nodes_guard.len();

                        if nodes_removed_count > 0 {
                            warn!("💀 [REAPER_SWARM]: Purged {} inactive units from the grid radar.", nodes_removed_count);
                        }
                    }
                    Err(lock_poison_fault) => {
                        // Protocolo de pánico controlado ante fallo de concurrencia
                        tracing::error!("💀 [REAPER_FATAL]: Swarm telemetry lock poisoned: {}", lock_poison_fault);
                        break; // Terminamos el daemon para proteger la integridad del proceso
                    }
                }
            }
        }
    });
}

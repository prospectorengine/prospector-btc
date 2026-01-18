// INICIO DEL ARCHIVO [libs/infra/worker-client/src/lib.rs]
/*!
 * =================================================================
 * APARATO: WORKER CLIENT LIBRARY BARREL (V10.5 - EXPORTS FIXED)
 * CLASIFICACIÓN: INFRASTRUCTURE LIB (ESTRATO L3)
 * RESPONSABILIDAD: EXPOSICIÓN PÚBLICA DE MÓDULOS DE UPLINK
 *
 * VISION HIPER-HOLÍSTICA:
 * Centraliza la exportación de clientes, errores e hidratadores.
 * Reparación crítica E0432: Se hace público el módulo 'hydrator'.
 * =================================================================
 */

pub mod client;
pub mod errors;
pub mod hydrator; // ✅ REPARACIÓN: Módulo registrado y expuesto

// Re-exportaciones para facilitar el consumo en apps/miner-worker
pub use client::WorkerClient;
pub use errors::ClientError;
pub use hydrator::ForensicDnaHydrator; // ✅ REPARACIÓN: Exportación nominal
// FIN DEL ARCHIVO [libs/infra/worker-client/src/lib.rs]

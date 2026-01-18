// [apps/orchestrator/src/services/mod.rs]
/**
 * =================================================================
 * APARATO: SERVICE ORCHESTRATION HUB (V24.0 - COMMAND ENABLED)
 * CLASIFICACIÓN: APPLICATION SERVICES (ESTRATO L4)
 * RESPONSABILIDAD: EXPOSICIÓN NOMINAL DE DAEMONS Y MOTORES
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como la central de registro de la inteligencia del sistema.
 * 1. MANDO C2: Se activa el módulo command_router para orquestación WebSocket.
 * 2. MODULARIDAD: Cada servicio opera como una unidad atómica aislada.
 * 3. TYPE SAFETY: Garantiza la visibilidad de los tipos de servicio en apps/.
 * =================================================================
 */

pub mod binary_packer;
pub mod c2_coordinator;
pub mod certification_authority;
pub mod chronos;
pub mod chronos_archive;
// ✅ REPARACIÓN BUILD: Activación del motor de enrutamiento de mando C2
pub mod command_router;
pub mod event_bus;
pub mod finding_flusher;
pub mod flush;
pub mod identity_guard;
pub mod mission_hydrator;
pub mod outbox_relay;
pub mod parity_auditor;
pub mod reaper;
pub mod swarm_resurrection;

// --- RE-EXPORTACIONES SOBERANAS (NOMINAL ACCESS) ---

pub use chronos::ChronosPacemaker;
pub use outbox_relay::SovereignArchivalEngine as OutboxRelayService;
pub use flush::spawn_flush_service;
pub use reaper::spawn_reaper;
pub use identity_guard::IdentityLeaseGuard;
// Nota: CommandRouter se consume vía ruta completa para preservar el contexto de servicio

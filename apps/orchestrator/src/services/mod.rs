// [apps/orchestrator/src/services/mod.rs]
/**
 * =================================================================
 * APARATO: SERVICE ORCHESTRATION HUB (V24.2 - GALVANIC ALIGNMENT)
 * CLASIFICACIÓN: APPLICATION SERVICES (ESTRATO L4)
 * RESPONSABILIDAD: EXPOSICIÓN NOMINAL DE DAEMONS Y MOTORES SOBERANOS
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. NOMINAL ALIGNMENT: Resolución definitiva del error E0432. Vincula
 *    'OutboxRelayService' al struct real 'SovereignRelayService' (V200.7).
 * 2. BRIDGE STANDARDIZATION: Exportación nominal de 'spawn_strategic_archival_bridge'
 *    para unificar el protocolo de ignición en el Kernel L1-APP.
 * 3. ZERO RESIDUE: Eliminación física de toda referencia a 'SovereignArchivalEngine',
 *    alcanzando la pureza de build exigida por el Protocolo Hydra-Zero.
 * 4. HYGIENE: Documentación técnica nivel Tesis Doctoral y rastro bit-perfecto.
 *
 * # Mathematical Proof (Modular Interface):
 * El Hub actúa como un 'Facade' estático. Los consumidores externos (Kernel/Main)
 * interactúan con alias inmutables, permitiendo la evolución interna de los
 * servicios sin romper los contratos de integración.
 * =================================================================
 */

pub mod binary_packer;
pub mod c2_coordinator;
pub mod certification_authority;
pub mod chronos;
pub mod chronos_archive;
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

// --- RE-EXPORTACIONES SOBERANAS (NOMINAL ACCESS API) ---

/// Marcapasos vital para la preservación de la instancia en entornos efímeros.
pub use chronos::ChronosPacemaker;

/// Servicio de sincronización estratégica (Motor A <-> Motor B).
/// ✅ RESOLUCIÓN SOBERANA: Sello del error de importación E0432.
pub use outbox_relay::SovereignRelayService as OutboxRelayService;

/// Puente de archivo histórico entre nubes.
pub use chronos_archive::spawn_strategic_archival_bridge;

/// Daemon de cristalización de telemetría (Write-Behind).
pub use flush::spawn_flush_service;

/// Segador de rastro volátil y mantenimiento de higiene en RAM.
pub use reaper::spawn_reaper;

/// Guardián de inmunidad y gestión atómica de arrendamientos ZK.
pub use identity_guard::IdentityLeaseGuard;

// Nota: 'command_router' y 'event_bus' se consumen vía ruta completa
// para enfatizar su rol como orquestadores de señal y mando.

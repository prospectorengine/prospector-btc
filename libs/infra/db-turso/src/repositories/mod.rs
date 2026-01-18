// [libs/infra/db-turso/src/repositories/mod.rs]
/*!
 * =================================================================
 * APARATO: REPOSITORY ACCESS MATRIX (V22.0 - TOPOLOGY MASTER)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (ESTRATO L3)
 * RESPONSABILIDAD: ORQUESTACIÓN DE SUBSISTEMAS DE PERSISTENCIA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. TOPOLOGY ALIGNMENT: Resolución definitiva de E0583. Sincroniza la
 *    jerarquía de módulos con la estructura física de carpetas (job/).
 * 2. ACCESS SOVEREIGNTY: Centraliza la visibilidad de los repositorios
 *    tácticos para permitir la inyección de dependencias en el Orquestador.
 * 3. ZERO RESIDUE: Eliminación de imports redundantes (TursoClient) y
 *    módulos mal ubicados (math, queries).
 * 4. NOMINAL PARITY: Asegura que 'JobRepository' sea el ancla del Ledger.
 *
 * # Mathematical Proof (Modular Encapsulation):
 * El barrel file actúa como una interfaz galvánica. Los sub-módulos
 * permanecen aislados, y solo las estructuras de autoridad (Repositories)
 * son expuestas al exterior, reduciendo el acoplamiento sistémico.
 * =================================================================
 */

// --- ESTRATO 1: OPERACIONES DEL ENJAMBRE (TACTICAL) ---

/// Gestión de misiones, rangos escalares y estados de búsqueda.
pub mod job;
/// Registro de vitalidad y telemetría de hilos de CPU.
pub mod worker;
/// Bóveda de tránsito para hallazgos criptográficos.
pub mod finding;

// --- ESTRATO 2: GOBERNANZA E IDENTIDAD (IGFS) ---

/// Gestión de identidades ZK y arrendamientos (Leases).
pub mod identity;
/// Auditoría de integridad y rastro forense del Ledger.
pub mod audit_repository;
/// Sincronía estratégica Motor A -> Motor B.
pub mod archival;

// --- ESTRATO 3: ACADEMIA Y RED SOCIAL (COMMUNITY) ---

/// Cálculo recursivo de potencia y red de afiliados.
pub mod affiliate_repository;

// --- ESTRATO 4: ARQUEOLOGÍA Y SISTEMA (CORE) ---

/// Registro de Golden Tickets y misiones de control.
pub mod scenarios;
/// Gestión de plantillas de memoria (DNA Snapshots).
pub mod scenario_repository;
/// Acceso a activos binarios y fragmentos de RAM.
pub mod scenario_assets;
/// Metadatos de salud de la infraestructura global.
pub mod system_repository;

// --- RE-EXPORTACIONES SOBERANAS (NOMINAL ACCESS API) ---

pub use archival::ArchivalRepository;
pub use audit_repository::AuditRepository;
pub use finding::FindingRepository;
pub use identity::IdentityRepository;
pub use scenarios::ScenarioRepository;
pub use worker::WorkerRepository;
pub use system_repository::SystemStateRepository;
pub use scenario_repository::ScenarioRegistryRepository;
pub use affiliate_repository::AffiliateRepository;

/**
 * RE-EXPORTACIÓN DE AUTORIDAD TÁCTICA
 * ✅ RESOLUCIÓN E0432: Expone el repositorio nominal alineado con la carpeta job/
 */
pub use job::JobRepository;

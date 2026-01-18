// [libs/infra/db-turso/src/repositories/mod.rs]
/*!
 * =================================================================
 * APARATO: REPOSITORY ACCESS MATRIX (V24.0 - GOLD MASTER)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (ESTRATO L3)
 * RESPONSABILIDAD: ORQUESTACIÓN SOBERANA DE SUBSISTEMAS DE PERSISTENCIA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. NOMINAL UNIFICATION: Sella la transición definitiva de 'job' a
 *    'mission_repository' para coherencia total con el Dominio L2.
 * 2. L7 INTEGRATION: Expone los repositorios de Billing, Notification
 *    y Gamification bajo el estándar de 'Outbox Táctico'.
 * 3. ACCESS SOVEREIGNTY: Centraliza la visibilidad de los repositorios
 *    como la Fuente Única de Verdad (SSoT) para la inyección de dependencias.
 * 4. ZERO RESIDUE: Eliminación total de advertencias de visibilidad y
 *    comentarios de deuda técnica.
 *
 * # Mathematical Proof (Topological Consistency):
 * Este aparato actúa como un nodo raíz en el grafo de módulos de Rust.
 * Garantiza que la resolución de símbolos sea determinista, permitiendo
 * que el Orquestador acceda a cualquier estrato de datos mediante una
 * interfaz galvánica única.
 * =================================================================
 */

// --- ESTRATO 1: OPERACIONES DEL ENJAMBRE (TACTICAL LEDGER) ---

/// Gestión soberana de misiones y estados de búsqueda.
pub mod mission_repository;
/// Registro de vitalidad y telemetría de silicio de los trabajadores.
pub mod worker;
/// Bóveda de tránsito para hallazgos criptográficos confirmados.
pub mod finding;

// --- ESTRATO 2: GOBERNANZA E IDENTIDAD (IGFS) ---

/// Gestión de identidades de conocimiento cero y arrendamientos.
pub mod identity;
/// Auditoría de integridad y rastro forense de la cadena de mando.
pub mod audit_repository;
/// Sincronía estratégica inmutable (Motor A -> Motor B).
pub mod archival;

// --- ESTRATO 3: SERVICIOS AL USUARIO Y MONETIZACIÓN (L7 - OUTBOX) ---

/// Gestión táctica de suscripciones y cuotas de energía.
pub mod billing;
/// Sistema Herald: Mensajería reactiva y alertas de colisión.
pub mod notification;
/// Motor Nexus: Experiencia, rangos y red de afiliados.
pub mod gamification;
/// Cálculo recursivo de potencia y jerarquía de red social.
pub mod affiliate_repository;

// --- ESTRATO 4: ARQUEOLOGÍA Y SISTEMA (CORE INFRA) ---

/// Registro de Golden Tickets y misiones de validación.
pub mod scenarios;
/// Gestión de plantillas de memoria (DNA Snapshots).
pub mod scenario_repository;
/// Acceso a activos binarios y fragmentos de memoria RAM.
pub mod scenario_assets;
/// Metadatos de salud de la infraestructura global del sistema.
pub mod system_repository;

// --- RE-EXPORTACIONES SOBERANAS (NOMINAL ACCESS API) ---

// Tactical Stratum
pub use mission_repository::MissionRepository;
pub use worker::WorkerRepository;
pub use finding::FindingRepository;

// Governance Stratum
pub use identity::IdentityRepository;
pub use audit_repository::AuditRepository;
pub use archival::ArchivalRepository;

// User Services Stratum (L7)
pub use billing::BillingRepository;
pub use notification::NotificationRepository;
pub use gamification::GamificationRepository;
pub use affiliate_repository::AffiliateRepository;

// Core Stratum
pub use system_repository::SystemStateRepository;
pub use scenario_repository::ScenarioRegistryRepository;
pub use scenarios::ScenarioRepository;

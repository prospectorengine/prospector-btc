// [apps/orchestrator/src/handlers/mod.rs]
/*!
 * =================================================================
 * APARATO: SERVICE ORCHESTRATION HUB (V26.0 - STRATA EXPANSION)
 * CLASIFICACIÓN: APPLICATION ADAPTERS (ESTRATO L4)
 * RESPONSABILIDAD: EXPOSICIÓN NOMINAL DE PUNTOS DE ENTRADA (HANDLERS)
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. L7 REGISTRATION: Incorpora los módulos de Billing, Notification y
 *    Gamification para habilitar los túneles de servicios al usuario.
 * 2. TOPOLOGICAL ALIGNMENT: Sincroniza la visibilidad de los handlers con
 *    el 'Sovereign Routing Matrix' (routes.rs), resolviendo fallos de resolución.
 * 3. NOMINAL PURITY: Mantenimiento de descriptores técnicos precisos para
 *    cada estrato de comunicación.
 * 4. HYGIENE: Erradicación total de ambigüedades y organización jerárquica.
 *
 * # Mathematical Proof (Modular Integrity):
 * Este "Barrel File" garantiza que el compilador de Rust genere un grafo
 * de visibilidad completo, permitiendo que el Orquestador actúe como
 * una interfaz unificada para protocolos REST, WebSockets y GraphQL.
 * =================================================================
 */

// --- ESTRATO 1: GOBERNANZA Y MANDO (ADMINISTRATIVE) ---

/// Gobernanza de identidad ZK, mando C2 y diagnósticos del Kernel.
pub mod admin;
/// Oráculo de datos relacionales y currículum académico (Fase 2).
pub mod graphql;
/// Certificación forense de vectores y misiones de validación.
pub mod lab;

// --- ESTRATO 2: OPERACIONES DEL ENJAMBRE (TACTICAL) ---

/// Centro de despacho y negociación de misiones para los workers.
pub mod swarm;
/// Ingesta de señales del Panóptico y agregación de métricas globales.
pub mod telemetry;
/// Túneles neurales de tiempo real (WebSockets Full-Duplex).
pub mod stream;

// --- ESTRATO 3: SERVICIOS AL USUARIO (L7 - NEW) ---

/// Gestión de cuotas de energía computacional y facturación asíncrona.
pub mod billing;
/// Sistema Herald: Inbox de notificaciones y alertas de colisión.
pub mod notification;
/// Motor Nexus: Gestión de prestigio, XP y estatus del operador.
pub mod gamification;

// --- ESTRATO 4: RECURSOS Y VISUALIZACIÓN (INFRA) ---

/// Servido de fragmentos binarios (Shards) con protección de ruta.
pub mod assets;
/// Terminal visual de diagnóstico estático (Landing Hub).
pub mod visual;

// INICIO DEL ARCHIVO [apps/orchestrator/src/handlers/mod.rs]
/*!
 * =================================================================
 * APARATO: SERVICE ORCHESTRATION HUB (V25.0 - COMPLETE MATRIX)
 * CLASIFICACIÓN: APPLICATION ADAPTERS (ESTRATO L4)
 * RESPONSABILIDAD: EXPOSICIÓN NOMINAL DE PUNTOS DE ENTRADA (HANDLERS)
 *
 * VISION HIPER-HOLÍSTICA:
 * Centraliza la definición de los adaptadores de entrada del sistema.
 * Actúa como el índice maestro para el enrutador soberano.
 *
 * # Topología de Módulos:
 * - admin:     Gobernanza de Identidad y Logs de C2.
 * - assets:    Servido de alto rendimiento de Shards binarios.
 * - graphql:   Gateway Neural para consultas complejas (Academia).
 * - lab:       Certificación forense y pruebas de humo.
 * - stream:    Túneles de tiempo real (WebSockets / SSE).
 * - swarm:     Operaciones tácticas de los workers (Heartbeats/Misiones).
 * - telemetry: Ingesta de logs del Panóptico Unificado.
 * - visual:    Landing page de diagnóstico del sistema (HTML).
 * =================================================================
 */

pub mod admin;
pub mod assets;
pub mod graphql;
pub mod lab;
pub mod stream;
pub mod swarm;
pub mod telemetry;
pub mod visual;

// FIN DEL ARCHIVO [apps/orchestrator/src/handlers/mod.rs]

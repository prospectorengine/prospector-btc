// [apps/orchestrator/src/lib.rs]
/*!
 * =================================================================
 * APARATO: ORCHESTRATOR LIBRARY ROOT (V12.1 - SOVEREIGN AUTHORITY)
 * CLASIFICACIÓN: CRATE ROOT (ESTRATO L3)
 * RESPONSABILIDAD: DEFINICIÓN SUPREMA DEL ÁRBOL DE MÓDULOS
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. MODULE HIERARCHY HARDENING: Establece la autoridad única de
 *    módulos, permitiendo que 'crate::' sea resuelto de forma
 *    determinista tanto en la librería como en el binario.
 * 2. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta en la
 *    exposición de rutas y servicios.
 * 3. PUBLIC INTERFACE: Expone 'prelude' para facilitar la ignición
 *    del Kernel desde el binario 'main.rs'.
 * 4. HYGIENE: Documentación técnica nivel MIT. Erradica el error E0432.
 *
 * # Mathematical Proof (Modular Encapsulation):
 * Al declarar 'pub mod' aquí, Rust genera un grafo de visibilidad
 * donde 'state' y 'graphql' son hermanos bajo el mismo tronco 'crate'.
 * Esto permite el intercambio de tipos (DI) en tiempo de compilación.
 * =================================================================
 */

// --- ESTRATO DE INFRAESTRUCTURA Y ARRANQUE ---
/// Motor de validación de integridad post-despliegue.
pub mod bootstrap;
/// Hidratación automática de registros forenses (DNA).
pub mod bootstrap_forensics;

// --- ESTRATO DE DATOS Y ESTADO (L1-APP) ---
/// Oráculo de datos relacionales y conocimiento académico.
pub mod graphql;
/// Gestor del sistema nervioso central del orquestador.
pub mod state;

// --- ESTRATO DE TRANSPORTE Y ACCIÓN ---
/// Adaptadores de entrada para ráfagas HTTP y WebSockets.
pub mod handlers;
/// Núcleo de mando y control para la ignición de servicios.
pub mod kernel;
/// El túnel de mando: Definición de rutas y topología de red.
pub mod routes;

// --- ESTRATO DE SEGURIDAD Y SOPORTE ---
/// Guardianes perimetrales de salud y autenticación.
pub mod middleware;
/// Daemons de fondo y lógica de resurrección del enjambre.
pub mod services;

/**
 * PRELUDIO DEL ORQUESTADOR
 *
 * Re-exportación estratégica de los componentes necesarios para
 * la ignición mínima del sistema. Reduce el acoplamiento en 'main.rs'.
 */
pub mod prelude {
    pub use crate::kernel::OrchestratorKernel;
    pub use crate::state::AppState;
    pub use crate::state::SystemMode;
}

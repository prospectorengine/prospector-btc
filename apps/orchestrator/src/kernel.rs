// [apps/orchestrator/src/kernel.rs]
/*!
 * =================================================================
 * APARATO: ORCHESTRATOR SOVEREIGN KERNEL (V368.0 - GALVANIC MASTER)
 * CLASIFICACIÓN: COMPOSITION ROOT (ESTRATO L1-APP)
 * RESPONSABILIDAD: BOOTSTRAP DE INFRAESTRUCTURA E IGNICIÓN DE SERVICIOS
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. GALVANIC ALIGNMENT: Resuelve el error E0432 sincronizando la ingesta
 *    de 'OutboxRelayService' con la exportación nominal de services/mod.rs.
 * 2. ZERO ABBREVIATIONS: Sustitución de 'env' por 'std::env' y de
 *    identificadores cortos por descriptores nominales de la física del sistema.
 * 3. ASYNC ORCHESTRATION: Garantiza que cada daemon de fondo (Reaper, Relay,
 *    Hydrator) sea inyectado en el reactor de Tokio con su propia traza.
 * 4. HYGIENE: Documentación técnica nivel Tesis Doctoral y rastro #[instrument].
 *
 * # Mathematical Proof (Deterministic Composition):
 * El Kernel asegura que la topología de red (Axum) solo se active tras
 * la inicialización exitosa del enlace táctico (Turso), impidiendo que
 * el sistema acepte misiones en un estado de inconsistencia de datos.
 * =================================================================
 */

use crate::state::AppState;
use crate::routes::create_sovereign_router;
use crate::bootstrap::Bootstrap;
use crate::services::{
    mission_hydrator::MissionHydratorService,
    finding_flusher::FindingFlusherService,
    swarm_resurrection::SwarmResurrectionService,
    certification_authority::CertificationAuthorityService,
    parity_auditor::ArchivalParityAuditor,
    // ✅ SINCRO E0432: Uso del nombre nominal exportado en mod.rs
    OutboxRelayService,
    ChronosPacemaker,
    spawn_flush_service,
    spawn_reaper,
    IdentityLeaseGuard,
};
use crate::handlers::telemetry::spawn_telemetry_loop;
use prospector_infra_db::TursoClient;
use std::net::{SocketAddr, IpAddr};
use std::sync::Arc;
use tracing::{info, error, instrument};

/// Representa el núcleo operativo del orquestador.
pub struct OrchestratorKernel {
    /// Puerto de red asignado para el despacho Zenith.
    pub server_network_port: u16,
    /// Instancia compartida del estado neural del sistema (SSoT).
    pub application_state: AppState,
}

impl OrchestratorKernel {
    /**
     * Realiza la ignición del cliente táctico y la cristalización del estado neural.
     *
     * # Errors:
     * - Pánico determinista si el enlace físico con Turso (Motor A) es inalcanzable.
     *
     * # Performance:
     * Operación O(1) que establece el pool de conexiones asíncronas.
     */
    #[instrument(skip(database_access_token))]
    pub async fn ignite(
        database_connection_url: &str,
        database_access_token: Option<String>,
        listening_port: u16
    ) -> Self {
        info!("🧬 [KERNEL_IGNITION]: Establishing primary tactical link to Motor A...");

        let database_client = TursoClient::connect(database_connection_url, database_access_token)
            .await
            .expect("FATAL_BOOT: Database link collapse. Strata unreachable.");

        Self {
            server_network_port: listening_port,
            application_state: AppState::new(database_client),
        }
    }

    /**
     * Lanza la ejecución coordinada de todas las operaciones soberanas.
     * Orquesta la sinapsis entre hilos de fondo y la interfaz de red Axum.
     *
     * # Logic:
     * 1. Activa marcapasos Chronos.
     * 2. Certifica fragmentos (Shards) vía Bootstrap.
     * 3. Despliega Daemons de mantenimiento táctico.
     * 4. Inicia el motor galvánico de sincronía L7 (Outbox).
     */
    pub async fn launch_sovereign_operations(self) {
        let shared_application_state = self.application_state.clone();

        // --- 1. CONFIGURACIÓN DEL MARCAPASOS (CHRONOS) ---
        // Previene la suspensión del proceso en infraestructuras Cloud efímeras.
        let public_deployment_url = std::env::var("RENDER_EXTERNAL_URL")
            .unwrap_or_else(|_| format!("http://localhost:{}", self.server_network_port));

        let service_instance_origin = std::env::var("RENDER_SERVICE_NAME")
            .unwrap_or_else(|_| "local_node_development".to_string());

        ChronosPacemaker::ignite_pacemaker_loop(
            public_deployment_url,
            service_instance_origin
        ).await;

        // --- 2. PROTOCOLO DE AUTO-HIDRATACIÓN (DIAGNÓSTICO) ---
        // Certifica la integridad de los Shards del Censo antes de autorizar misiones.
        Bootstrap::spawn_diagnostics(shared_application_state.clone());

        // --- 3. SERVICIOS DE MANDO Y CERTIFICACIÓN ---
        // Arbitro que valida colisiones contra el Golden Vector de Satoshi.
        let integrity_arbitrator = Arc::new(CertificationAuthorityService::new(shared_application_state.clone()));
        integrity_arbitrator.spawn_integrity_listener().await;

        // --- 4. DESPLIEGUE DE DAEMONS DE MANTENIMIENTO TÁCTICO (MOTOR A) ---

        // A. Hidratador de Suministro (DB -> RAM Buffer)
        let state_for_hydrator = shared_application_state.clone();
        tokio::spawn(async move {
            let hydrator_engine = MissionHydratorService::new(state_for_hydrator);
            hydrator_engine.spawn_hydrator_daemon().await;
        });

        // B. Volcado de Hallazgos (RAM Buffer -> DB)
        let state_for_flusher = shared_application_state.clone();
        tokio::spawn(async move {
            let finding_flusher_engine = FindingFlusherService::new(state_for_flusher);
            finding_flusher_engine.spawn_flusher_daemon().await;
        });

        // C. Guardián de Resurrección (C2 Swarm Recovery)
        let state_for_resurrection = shared_application_state.clone();
        tokio::spawn(async move {
            let resurrection_engine = SwarmResurrectionService::new(state_for_resurrection);
            resurrection_engine.spawn_resurrection_daemon().await;
        });

        // D. Inmunología de Identidad (Lease Purge)
        let state_for_identity_guard = shared_application_state.clone();
        tokio::spawn(async move {
            let guard_engine = IdentityLeaseGuard::new(state_for_identity_guard);
            guard_engine.spawn_guard_daemon().await;
        });

        // --- 5. ESTRATO DE SINCRONÍA GALVÁNICA (OUTBOX RELAY - L7) ---
        // ✅ MEJORA V368.0: Gestión unificada de Billing, XP y Notificaciones.
        // Utiliza el patrón Outbox para garantizar 'Zero Data Loss'.
        let state_for_relay = shared_application_state.clone();
        tokio::spawn(async move {
            // Instanciación del servicio con caché de entorno inyectado (Sovereign Master)
            let relay_engine = OutboxRelayService::new(state_for_relay);
            relay_engine.spawn_relay_loop().await;
        });

        // Auditor de Paridad: Detecta deriva (Drift) entre el Motor A y el Motor B.
        let state_for_auditor = shared_application_state.clone();
        tokio::spawn(async move {
            let parity_auditor_engine = ArchivalParityAuditor::new(state_for_auditor);
            parity_auditor_engine.spawn_auditor_daemon().await;
        });

        // --- 6. HIGIENE TÉRMICA Y TELEMETRÍA ZENITH ---

        // Sincronización de latidos (Write-Behind) para el Ledger de workers.
        spawn_flush_service(shared_application_state.clone()).await;

        // El segador: Limpieza de snapshots visuales obsoletos en RAM.
        spawn_reaper(shared_application_state.clone()).await;

        // Bucle de agregación de métricas globales para el HUD Panóptico.
        spawn_telemetry_loop(shared_application_state.clone()).await;

        // --- 7. IGNICIÓN DEL TRANSPORTE HTTP/WS (AXUM) ---
        let sovereign_router = create_sovereign_router(shared_application_state);

        let bind_address = SocketAddr::new(
            "0.0.0.0".parse::<IpAddr>().expect("FAULT: Invalid network IP binding."),
            self.server_network_port
        );

        info!("🚀 [KERNEL_ONLINE]: Zenith Control Center active at {}", bind_address);

        let tcp_listener = tokio::net::TcpListener::bind(bind_address)
            .await
            .expect("CRITICAL_FAULT: Failed to bind composition port.");

        if let Err(runtime_fault) = axum::serve(tcp_listener, sovereign_router).await {
            error!("💀 [KERNEL_COLLAPSE]: Runtime failure: {}", runtime_fault);
            std::process::exit(1);
        }
    }
}

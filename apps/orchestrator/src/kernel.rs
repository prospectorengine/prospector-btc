// [apps/orchestrator/src/kernel.rs]
/**
 * =================================================================
 * APARATO: ORCHESTRATOR SOVEREIGN KERNEL (V367.0 - GALVANIC IGNITION)
 * CLASIFICACIÓN: COMPOSITION ROOT (ESTRATO L1-APP)
 * RESPONSABILIDAD: BOOTSTRAP DE INFRAESTRUCTURA E IGNICIÓN DE SERVICIOS
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. GALVANIC ALIGNMENT: Integra el 'SovereignRelayService' V200.0 para la
 *    sincronía local-first (Outbox Pattern) de los estratos L7.
 * 2. SERVICE UNIFICATION: Elimina el puente de archivo legacy por un motor
 *    polimórfico que gestiona Billing, Prestigio y Alertas simultáneamente.
 * 3. NOMINAL PURITY: Erradicación total de abreviaciones (env -> environment).
 * 4. HYGIENE: Documentación RustDoc MIT y rastro #[instrument] completo.
 *
 * # Mathematical Proof (Deterministic Startup):
 * El Kernel garantiza que el sistema solo acepte tráfico una vez que los
 * daemons de mantenimiento (Reaper, Relay, Guard) han sido inyectados en
 * el reactor asíncrono, asegurando la consistencia del estado desde T=0.
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
    // El nuevo servicio unificado que sustituye al Bridge Legacy
    outbox_relay::SovereignRelayService,
    ChronosPacemaker,
    spawn_flush_service,
    spawn_reaper,
    IdentityLeaseGuard,
};
use crate::handlers::telemetry::spawn_telemetry_loop;
use prospector_infra_db::TursoClient;
use std::net::{SocketAddr, IpAddr};
use std::sync::Arc;
use std::env;
use tracing::{info, error, instrument};

/// Representa el núcleo operativo del orquestador.
pub struct OrchestratorKernel {
    /// Puerto de red asignado para el despacho Zenith.
    pub server_network_port: u16,
    /// Instancia compartida del estado neural del sistema.
    pub application_state: AppState,
}

impl OrchestratorKernel {
    /**
     * Realiza la ignición del cliente táctico y la cristalización del estado neural.
     *
     * # Errors:
     * Pánico si el enlace físico con Turso (Motor A) es inalcanzable.
     */
    #[instrument(skip(database_access_token))]
    pub async fn ignite(
        database_connection_url: &str,
        database_access_token: Option<String>,
        listening_port: u16
    ) -> Self {
        info!("🧬 [KERNEL_IGNITION]: Establishing primary tactical link...");

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
     * Orquesta la sinapsis entre hilos de fondo y la interfaz de red.
     */
    pub async fn launch_sovereign_operations(self) {
        let shared_application_state = self.application_state.clone();

        // --- 1. CONFIGURACIÓN DEL MARCAPASOS (CHRONOS) ---
        // Previene la suspensión de la instancia en infraestructuras efímeras.
        let public_deployment_url = env::var("RENDER_EXTERNAL_URL")
            .unwrap_or_else(|_| format!("http://localhost:{}", self.server_network_port));

        let service_instance_origin = env::var("RENDER_SERVICE_NAME")
            .unwrap_or_else(|_| "local_node".to_string());

        ChronosPacemaker::ignite_pacemaker_loop(
            public_deployment_url,
            service_instance_origin
        ).await;

        // --- 2. PROTOCOLO DE AUTO-HIDRATACIÓN (DIAGNÓSTICO) ---
        // Certifica la integridad de los Shards del Censo antes de la operación.
        Bootstrap::spawn_diagnostics(shared_application_state.clone());

        // --- 3. SERVICIOS DE MANDO Y CERTIFICACIÓN ---
        // Valida colisiones contra el Golden Vector de Satoshi.
        let integrity_arbitrator = Arc::new(CertificationAuthorityService::new(shared_application_state.clone()));
        integrity_arbitrator.spawn_integrity_listener().await;

        // --- 4. DESPLIEGUE DE DAEMONS DE MANTENIMIENTO TÁCTICO (MOTOR A) ---

        // A. Hidratador de Suministro (DB -> RAM Buffer)
        let state_for_hydrator = shared_application_state.clone();
        tokio::spawn(async move {
            let hydrator = MissionHydratorService::new(state_for_hydrator);
            hydrator.spawn_hydrator_daemon().await;
        });

        // B. Volcado de Hallazgos (RAM Buffer -> DB)
        let state_for_flusher = shared_application_state.clone();
        tokio::spawn(async move {
            let flusher = FindingFlusherService::new(state_for_flusher);
            flusher.spawn_flusher_daemon().await;
        });

        // C. Guardián de Resurrección (C2 Swarm Recovery)
        let state_for_resurrection = shared_application_state.clone();
        tokio::spawn(async move {
            let resurrection = SwarmResurrectionService::new(state_for_resurrection);
            resurrection.spawn_resurrection_daemon().await;
        });

        // D. Inmunología de Identidad (Lease Purge)
        let state_for_identity_guard = shared_application_state.clone();
        tokio::spawn(async move {
            let guard = IdentityLeaseGuard::new(state_for_identity_guard);
            guard.spawn_guard_daemon().await;
        });

        // --- 5. ESTRATO DE SINCRONÍA GALVÁNICA (OUTBOX RELAY - L7) ---
        // ✅ MEJORA V367.0: Unificación del puente táctico-estratégico.
        // Gestiona Billing, XP y Notificaciones mediante el patrón Outbox.
        let state_for_relay = shared_application_state.clone();
        tokio::spawn(async move {
            let relay = SovereignRelayService::new(state_for_relay);
            relay.spawn_relay_loop().await;
        });

        // Auditor de Paridad (Turso vs Supabase) para detección de deriva (Drift).
        let state_for_auditor = shared_application_state.clone();
        tokio::spawn(async move {
            let parity_auditor = ArchivalParityAuditor::new(state_for_auditor);
            parity_auditor.spawn_auditor_daemon().await;
        });

        // --- 6. HIGIENE TÉRMICA Y TELEMETRÍA ZENITH ---

        // Persistencia de latidos de workers (Write-Behind).
        spawn_flush_service(shared_application_state.clone()).await;

        // El segador de recursos (Snapshot Cleanup).
        spawn_reaper(shared_application_state.clone()).await;

        // Bucle principal de métricas para el HUD Panóptico.
        spawn_telemetry_loop(shared_application_state.clone()).await;

        // --- 7. IGNICIÓN DEL TRANSPORTE HTTP/WS (AXUM) ---
        let sovereign_router = create_sovereign_router(shared_application_state);

        let bind_address = SocketAddr::new(
            "0.0.0.0".parse::<IpAddr>().expect("FAULT: Invalid IP binding."),
            self.server_network_port
        );

        info!("🚀 [KERNEL_ONLINE]: Zenith Control Center listening at {}", bind_address);

        let tcp_listener = tokio::net::TcpListener::bind(bind_address)
            .await
            .expect("CRITICAL_FAULT: Failed to bind network port.");

        if let Err(server_error) = axum::serve(tcp_listener, sovereign_router).await {
            error!("💀 [KERNEL_COLLAPSE]: Runtime failure: {}", server_error);
            std::process::exit(1);
        }
    }
}

// INICIO DEL ARCHIVO [apps/orchestrator/src/kernel.rs]
/**
 * =================================================================
 * APARATO: ORCHESTRATOR SOVEREIGN KERNEL (V366.0 - GUARD ACTIVE)
 * CLASIFICACIÓN: COMPOSITION ROOT (ESTRATO L1-APP)
 * RESPONSABILIDAD: BOOTSTRAP DE INFRAESTRUCTURA E IGNICIÓN SEGURA
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como la placa base donde se conectan todos los servicios.
 * Se ha inyectado el 'IdentityLeaseGuard' para completar el circuito
 * de auto-recuperación de identidades.
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
    chronos_archive::spawn_strategic_archival_bridge,
    ChronosPacemaker,
    OutboxRelayService,
    spawn_flush_service,
    spawn_reaper,
    // ✅ NUEVO: Importación del Guardia de Ciclo de Vida de Identidad
    IdentityLeaseGuard,
};
use crate::handlers::telemetry::spawn_telemetry_loop;
use prospector_infra_db::TursoClient;
use std::net::{SocketAddr, IpAddr};
use std::sync::Arc;
use std::env;
use tracing::{info, error, instrument};

pub struct OrchestratorKernel {
    pub server_network_port: u16,
    pub application_state: AppState,
}

impl OrchestratorKernel {
    /**
     * Realiza la ignición del cliente táctico y el estado neural.
     * Establece la conexión a la base de datos antes de levantar servicios.
     */
    #[instrument(skip(database_access_token))]
    pub async fn ignite(
        database_connection_url: &str,
        database_access_token: Option<String>,
        listening_port: u16
    ) -> Self {
        let database_client = TursoClient::connect(database_connection_url, database_access_token)
            .await
            .expect("FATAL: Database link collapse. Ignition aborted.");

        Self {
            server_network_port: listening_port,
            application_state: AppState::new(database_client),
        }
    }

    /**
     * Lanza la ejecución de todas las operaciones autónomas del enjambre.
     * Configura los hilos de fondo (Daemons) y el servidor HTTP principal.
     */
    pub async fn launch_sovereign_operations(self) {
        let shared_application_state = self.application_state.clone();

        // --- 1. CONFIGURACIÓN E IGNICIÓN DEL MARCAPASOS (CHRONOS) ---
        // Mantiene vivo el servicio en entornos PaaS (Render Free Tier)
        let public_deployment_url = env::var("RENDER_EXTERNAL_URL")
            .unwrap_or_else(|_| format!("http://localhost:{}", self.server_network_port));

        let service_instance_origin = env::var("RENDER_SERVICE_NAME")
            .unwrap_or_else(|_| "local_node".to_string());

        ChronosPacemaker::ignite_pacemaker_loop(
            public_deployment_url,
            service_instance_origin
        ).await;

        // --- 2. PROTOCOLO DE AUTO-HIDRATACIÓN (DIAGNÓSTICO) ---
        // Verifica la integridad de los Shards del Censo al arrancar
        Bootstrap::spawn_diagnostics(shared_application_state.clone());

        // --- 3. SERVICIOS DE MANDO Y CERTIFICACIÓN ---
        // Escucha colisiones para validar vectores dorados
        let auth_service = Arc::new(CertificationAuthorityService::new(shared_application_state.clone()));
        auth_service.spawn_integrity_listener().await;

        // --- 4. DESPLIEGUE DE DAEMONS DE MANTENIMIENTO TÁCTICO ---

        // A. Hidratador de Misiones (DB -> RAM Buffer)
        let state_ref = shared_application_state.clone();
        tokio::spawn(async move {
            let hydrator = MissionHydratorService::new(state_ref);
            hydrator.spawn_hydrator_daemon().await;
        });

        // B. Volcado de Hallazgos (RAM Buffer -> DB)
        let state_ref = shared_application_state.clone();
        tokio::spawn(async move {
            let flusher = FindingFlusherService::new(state_ref);
            flusher.spawn_flusher_daemon().await;
        });

        // C. Resurrección de Enjambre (Detección de Zombies y C2 Call)
        let state_ref = shared_application_state.clone();
        tokio::spawn(async move {
            let resurrection = SwarmResurrectionService::new(state_ref);
            resurrection.spawn_resurrection_daemon().await;
        });

        // D. ✅ NUEVO: GUARDIA DE IDENTIDAD (Limpieza de Leases Expirados)
        let state_ref = shared_application_state.clone();
        tokio::spawn(async move {
            let guard = IdentityLeaseGuard::new(state_ref);
            guard.spawn_guard_daemon().await;
        });

        // --- 5. ESTRATO DE ARCHIVO ESTRATÉGICO (MOTOR B) ---
        // Puente hacia Supabase para archivo histórico
        let archival_relay = OutboxRelayService::new(shared_application_state.clone());
        tokio::spawn(async move { archival_relay.spawn_archival_loop().await; });

        // Auditor de Paridad (Turso vs Supabase)
        let parity_auditor = ArchivalParityAuditor::new(shared_application_state.clone());
        tokio::spawn(async move { parity_auditor.spawn_auditor_daemon().await; });

        // Migración de reportes certificados
        spawn_strategic_archival_bridge(shared_application_state.clone()).await;

        // --- 6. HIGIENE Y TELEMETRÍA ---
        // Persistencia de latidos de workers
        spawn_flush_service(shared_application_state.clone()).await;
        // Limpieza de memoria (Snapshots viejos)
        spawn_reaper(shared_application_state.clone()).await;
        // Bucle principal de métricas para el Dashboard (Panóptico)
        spawn_telemetry_loop(shared_application_state.clone()).await;

        // --- 7. IGNICIÓN DEL TRANSPORTE HTTP (AXUM) ---
        let sovereign_router = create_sovereign_router(shared_application_state);

        let bind_address = SocketAddr::new(
            "0.0.0.0".parse::<IpAddr>().unwrap(),
            self.server_network_port
        );

        info!("🚀 [KERNEL_ONLINE]: Sovereign Control Center listening at {}", bind_address);

        let tcp_listener = tokio::net::TcpListener::bind(bind_address)
            .await
            .expect("CRITICAL_FAULT: Failed to bind network port.");

        if let Err(server_error) = axum::serve(tcp_listener, sovereign_router).await {
            error!("💀 [KERNEL_COLLAPSE]: Runtime failure: {}", server_error);
            std::process::exit(1);
        }
    }
}
// FIN DEL ARCHIVO [apps/orchestrator/src/kernel.rs]

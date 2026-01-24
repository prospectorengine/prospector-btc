// [apps/orchestrator/src/kernel.rs]
/*!
 * =================================================================
 * APARATO: ORCHESTRATOR SOVEREIGN KERNEL (V370.0 - NEURAL INTEGRATION)
 * CLASIFICACIÓN: COMPOSITION ROOT (ESTRATO L1-APP)
 * RESPONSABILIDAD: BOOTSTRAP DE INFRAESTRUCTURA E IGNICIÓN DE SERVICIOS
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. CLOSED-LOOP IGNITION: Asegura que el MissionHydrator (V225) nazca
 *    vinculado al AI Cortex (L9) para la gestión autónoma del enjambre.
 * 2. STRATA HARMONIZATION: Sincronización bit-perfecta entre el despacho
 *    REST/WebSocket y los Daemons de fondo mediante el AppState soberano.
 * 3. NOMINAL PURITY: Nomenclatura descriptiva absoluta nivel Tesis Doctoral.
 * 4. PANOPTICON BROADCASTING: Activa el rastro forense unificado permitiendo
 *    la visibilidad 360° desde el Dashboard Zenith (L5).
 *
 * # Mathematical Proof (Operational Determinism):
 * El Kernel garantiza la invariante de estado: ∀ Daemon ∈ {Hydrator, Relay, Reaper},
 * Contexto(Daemon) ≡ AppState(Master). Esto elimina derivas de datos
 * entre el Ledger Táctico (Turso) y el archivo estratégico (Supabase).
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
     OutboxRelayService,
     ChronosPacemaker,
     spawn_strategic_archival_bridge,
     spawn_flush_service,
     spawn_reaper,
     IdentityLeaseGuard,
 };
 use crate::handlers::telemetry::spawn_telemetry_loop;
 use prospector_infra_db::TursoClient;
 use std::net::{SocketAddr, IpAddr};
 use std::sync::Arc;
 use tracing::{info, error, instrument, debug};
 
 /**
  * Núcleo supremo del Orquestador.
  * Encargado de la materialización del universo operativo del sistema.
  */
 pub struct OrchestratorKernel {
     /// Puerto de red físico asignado para la escucha de ráfagas (Capa 4).
     pub server_network_port: u16,
     /// Instancia única del estado neural compartido (Single Source of Truth).
     pub application_shared_state: AppState,
 }
 
 impl OrchestratorKernel {
     /**
      * Realiza la ignición del cliente táctico y la cristalización del estado.
      *
      * # Errors:
      * Dispara un pánico determinista si el enlace físico con Turso (Motor A)
      * es inalcanzable, bloqueando una ignición en vacío.
      *
      * # Performance: O(1).
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
             .expect("FATAL_BOOT_COLLAPSE: Tactical strata unreachable. Audit credentials.");
 
         Self {
             server_network_port: listening_port,
             application_shared_state: AppState::new(database_client),
         }
     }
 
     /**
      * Lanza la ejecución coordinada de la flota de Daemons y el Router Axum.
      * Implementa la orquestación multihilo del reactor de Tokio.
      *
      * # Sequence:
      * 1. Pacemaker (Uptime guard).
      * 2. Bootstrap (Physical integrity scan).
      * 3. AI Effector (Mission supply).
      * 4. Galvanic Sync (Strategic Archival).
      * 5. Network Router (Neural Link).
      */
     pub async fn launch_sovereign_operations(self) {
         let application_state = self.application_shared_state.clone();
 
         // --- 1. CONFIGURACIÓN DEL MARCAPASOS (CHRONOS) ---
         // Previene la hibernación de la instancia en infraestructuras Cloud (Render Free Tier).
         let public_deployment_url = std::env::var("RENDER_EXTERNAL_URL")
             .unwrap_or_else(|_| format!("http://localhost:{}", self.server_network_port));
 
         let service_instance_origin = std::env::var("RENDER_SERVICE_NAME")
             .unwrap_or_else(|_| "local_development_node".to_string());
 
         ChronosPacemaker::ignite_pacemaker_loop(
             public_deployment_url,
             service_instance_origin
         ).await;
 
         // --- 2. PROTOCOLO DE AUTO-HIDRATACIÓN Y DIAGNÓSTICO ---
         // Certifica la validez de los fragmentos del censo (L1) antes del despacho.
         Bootstrap::spawn_diagnostics(application_state.clone());
 
         // --- 3. AUTORIDAD DE CERTIFICACIÓN (INTEGRITY ARBITRATOR) ---
         let integrity_arbitrator = Arc::new(CertificationAuthorityService::new(application_state.clone()));
         integrity_arbitrator.spawn_integrity_listener().await;
 
         // --- 4. FLOTA DE DAEMONS TÁCTICOS (MANTENIMIENTO EN TIEMPO REAL) ---
 
         // A. Adaptive Mission Hydrator: El brazo ejecutor del AI Cortex.
         let state_for_hydrator = application_state.clone();
         tokio::spawn(async move {
             let hydrator_service = MissionHydratorService::new(state_for_hydrator);
             hydrator_service.spawn_hydrator_daemon().await;
         });
 
         // B. Finding Flusher: Persistencia asíncrona de colisiones confirmadas.
         let state_for_flusher = application_state.clone();
         tokio::spawn(async move {
             let flusher_service = FindingFlusherService::new(state_for_flusher);
             flusher_service.spawn_flusher_daemon().await;
         });
 
         // C. Swarm Resurrection: Recuperación de misiones huérfanas y mando C2.
         let state_for_resurrection = application_state.clone();
         tokio::spawn(async move {
             let resurrection_service = SwarmResurrectionService::new(state_for_resurrection);
             resurrection_service.spawn_resurrection_daemon().await;
         });
 
         // D. Identity Guard: Gestión atómica de arrendamientos ZK-Vault.
         let state_for_identity_guard = application_state.clone();
         tokio::spawn(async move {
             let guard_service = IdentityLeaseGuard::new(state_for_identity_guard);
             guard_service.spawn_guard_daemon().await;
         });
 
         // --- 5. ESTRATO DE SINCRONÍA GALVÁNICA (STRATEGIC UPLINK) ---
 
         // E. Outbox Relay: Sincronización L7 (Billing, Reputation, Notifications).
         let state_for_relay = application_state.clone();
         tokio::spawn(async move {
             let relay_service = OutboxRelayService::new(state_for_relay);
             relay_service.spawn_relay_loop().await;
         });
 
         // F. Strategic Archival Bridge: Archivo permanente bit-perfecto (V200.7).
         spawn_strategic_archival_bridge(application_state.clone()).await;
 
         // G. Parity Auditor: Sensor de deriva (Drift) entre Motores A y B.
         let state_for_auditor = application_state.clone();
         tokio::spawn(async move {
             let auditor_service = ArchivalParityAuditor::new(state_for_auditor);
             auditor_service.spawn_auditor_daemon().await;
         });
 
         // --- 6. TELEMETRÍA ZENITH Y HIGIENE TÉRMICA ---
 
         // Cristalización de latidos (Write-Behind Protocol)
         spawn_flush_service(application_state.clone()).await;
 
         // Limpieza de rastro volátil y frames de video obsoletos
         spawn_reaper(application_state.clone()).await;
 
         // Bucle de inferencia cognitiva AI Cortex y agregación HUD
         spawn_telemetry_loop(application_state.clone()).await;
 
         // --- 7. IGNICIÓN DEL TRANSPORTE DE RED (AXUM ENGINE) ---
         let sovereign_router = create_sovereign_router(application_state);
 
         let network_socket_address = SocketAddr::new(
             "0.0.0.0".parse::<IpAddr>().expect("CRITICAL: Invalid binding address."),
             self.server_network_port
         );
 
         info!("🚀 [KERNEL_OPERATIONAL]: Zenith Command Center online at {}", network_socket_address);
 
         let tcp_listener_socket = tokio::net::TcpListener::bind(network_socket_address)
             .await
             .expect("CRITICAL_FAULT: Failed to bind composition port. Port occupied?");
 
         if let Err(runtime_fault) = axum::serve(tcp_listener_socket, sovereign_router).await {
             error!("💀 [KERNEL_COLLAPSE]: Runtime failure in network strata: {}", runtime_fault);
             std::process::exit(1);
         }
     }
 }
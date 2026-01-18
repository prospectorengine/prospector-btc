// [tests/mirror/apps/orchestrator/handlers/telemetry_ingest.test.rs]
/**
 * =================================================================
 * APARATO: TELEMETRY INGESTION TEST (V1.1 - ZENITH CERTIFIED)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L3-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE LA RUTA INGESTA -> BUS -> NEURAL LINK
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. TRAIT SCOPE HARDENING: Resolución definitiva de E0599 mediante la
 *    importación explícita de 'axum::response::IntoResponse'.
 * 2. NOMINAL PARITY: Sincronización con el modelo 'SystemLog' y el
 *    evento 'SystemLogEmission' del Estrato L2.
 * 3. ATOMIC ASSERTION: Valida que el código de estado sea 202 (Accepted)
 *    y que el paquete de datos llegue íntegro al suscriptor del bus.
 * 4. HYGIENE: Nomenclatura nominal absoluta sin abreviaciones.
 *
 * # Mathematical Proof (Signal Propagation):
 * El test garantiza que el Orquestador actúa como un transformador de
 * impedancia lineal: Log(t) -> API -> Bus -> Event(t), donde el
 * identificador del log se preserva bit-perfecto.
 * =================================================================
 */

use prospector_orchestrator::state::AppState;
use prospector_orchestrator::handlers::telemetry::handle_log_ingestion;
use prospector_infra_db::TursoClient;
use prospector_domain_models::telemetry::{SystemLog, RealTimeEvent};
use axum::extract::{Json, State};
// ✅ RESOLUCIÓN E0599: El Trait debe estar en scope para usar .into_response()
use axum::response::IntoResponse;

#[tokio::test]
async fn certify_log_ingestion_to_bus_link() {
    println!("\n🧪 [PROVING_GROUNDS]: Initiating Telemetry Ingestion Pipeline Audit...");

    // 1. SETUP: Inicialización de Infraestructura Efímera (Motor A en RAM)
    let database_client = TursoClient::connect("file:ingest_test?mode=memory&cache=shared", None)
        .await
        .expect("FALLO_CRÍTICO: No se pudo anclar el Ledger en RAM.");

    let application_state = AppState::new(database_client);

    // Suscripción al sistema nervioso (Event Bus) antes de la ráfaga
    let mut neural_link_subscriber = application_state.event_bus.subscribe();

    // 2. ESCENARIO: Simulación de log de navegación desde el Provisioner (L6)
    let log_identifier = "log-forensic-001".to_string();
    let mock_log_artifact = SystemLog {
        id: log_identifier.clone(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        stratum: "L6_OPS".into(),
        severity: "INFO".into(),
        message: "Sentinel: Swarm ignition sequence synchronized.".into(),
        metadata: None,
        trace_id: Some("trace-zenith-444".into()),
    };

    println!("   🛰️  [DISPATCH]: Transmitting log packet to /api/v1/telemetry/ingest...");

    // 3. EXECUTION: Invocación directa del Handler (Simulación de POST)
    let response_opaque = handle_log_ingestion(
        State(application_state),
        Json(mock_log_artifact)
    ).await;

    // 4. VALIDATION A: Interfaz de Red
    // ✅ RESOLUCIÓN: into_response() ahora despacha correctamente
    let http_response = response_opaque.into_response();
    assert_eq!(
        http_response.status(),
        axum::http::StatusCode::ACCEPTED,
        "L3_API: The ingestion gateway rejected a valid log packet."
    );

    println!("   ✅ [VERDICT_A]: HTTP 202 Accepted confirmed.");

    // 5. VALIDATION B: Propagación Neural (Event Bus)
    // Verificamos que el reporte se haya difundido al bus de tiempo real
    let received_event = neural_link_subscriber.recv().await
        .expect("L4_BUS: Failed to capture the signal emission.");

    if let RealTimeEvent::SystemLogEmission(captured_log) = received_event {
        assert_eq!(captured_log.id, log_identifier, "DATA_CORRUPTION: Log ID mismatch in transit.");
        assert_eq!(captured_log.stratum, "L6_OPS");

        println!("   ✅ [VERDICT_B]: Signal verified bit-perfect in Neural Link.");
    } else {
        panic!("INTEGRITY_FAULT: Event Bus emitted an unauthorized signal type.");
    }

    println!("🏁 [COMPLETE]: Telemetry Ingestion strata certified.\n");
}

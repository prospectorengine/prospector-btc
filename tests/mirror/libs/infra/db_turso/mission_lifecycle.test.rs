// [tests/mirror/libs/infra/db_turso/mission_lifecycle.test.rs]
/*!
 * =================================================================
 * APARATO: CERTIFICADOR DE CICLO DE VIDA DE MISIONES (V2.3 - ZENITH)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L3-MIRROR
 * RESPONSABILIDAD: AUDITORÍA DE TRANSACCIONES ACID Y GESTIÓN DE ESTADOS
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. CONTRACT PARITY: Sincroniza la instanciación de 'AuditReport' con el campo
 *    'hardware_acceleration_signature' introducido en el Estrato L2 (V151.0).
 * 2. REPOSITORY ALIGNMENT: Nivelación de la llamada 'assign_mission_to_worker'
 *    para satisfacer la firma V300.7 con soporte de 'operator_id'.
 * 3. NOMINAL PURITY: Erradicación total de abreviaciones y variables muertas.
 * 4. PANOPTICON SYNC: Reporte técnico enriquecido para visualización en el HUD.
 *
 * # Mathematical Proof (ACID Mission Lifecycle):
 * El test certifica que la base de datos Turso actúa como un semáforo atómico,
 * garantizando que una misión solo pueda ser reclamada por una única unidad
 * de ejecución, eliminando condiciones de carrera en el enjambre.
 * =================================================================
 */

use prospector_infra_db::repositories::MissionRepository;
use prospector_infra_db::TursoClient;
use prospector_domain_models::work::AuditReport;
use std::time::{Instant, Duration};
use serde_json::json;
use reqwest::blocking::Client;

// --- MOTOR DE REPORTE SOBERANO ---

/**
 * Transmite el veredicto técnico de la auditoría de persistencia al Orquestador.
 */
fn dispatch_persistence_integrity_report(
    final_verdict_label: &str,
    database_latency_milliseconds: f64,
    technical_forensic_log: String,
    total_anomalies_detected: u32
) {
    let orchestrator_gateway_url = std::env::var("ORCHESTRATOR_URL")
        .unwrap_or_else(|_| "http://localhost:3000".into());
    let worker_authentication_token = std::env::var("WORKER_AUTH_TOKEN")
        .unwrap_or_else(|_| "observer".into());

    let payload_artifact = json!({
        "testName": "TACTICAL_LEDGER_LIFECYCLE_V2_3",
        "stratum": "L3_INFRA",
        "verdict": final_verdict_label,
        "metrics": {
            "throughput": 1.0 / (database_latency_milliseconds / 1000.0),
            "latency_ms": database_latency_milliseconds,
            "error_rate": total_anomalies_detected as f64
        },
        "forensicLog": technical_forensic_log,
        "environment": "Local_VAIO_Infrastructure_Audit_Chamber",
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    let network_client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("INFRA_FAULT: Reporting engine failed to initialize.");

    let _ = network_client.post(format!("{}/api/v1/admin/qa/report", orchestrator_gateway_url))
        .header("Authorization", format!("Bearer {}", worker_authentication_token))
        .json(&payload_artifact)
        .send();
}

// --- SUITE DE AUDITORÍA DE PERSISTENCIA ---

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Ejecuta la validación de integridad del ciclo de vida: [Queued -> Active -> Completed].
     *
     * # Errors:
     * El test colapsará si el Ledger Táctico permite la duplicidad de asignación
     * o rechaza el sellado de rastro forense enriquecido.
     */
    #[tokio::test]
    async fn certify_tactical_ledger_integrity_v2_3() {
        println!("\n🗄️  [INICIO]: Iniciando Auditoría del Ciclo de Vida de Misiones V2.3...");
        let suite_execution_start = Instant::now();
        let mut technical_forensic_log = String::new();
        let mut accumulated_anomalies_count = 0;

        // 1. SETUP: Inicialización de Infraestructura Volátil (RAM con Caché compartido)
        let database_client = TursoClient::connect("file:mem_lifecycle_v23?mode=memory&cache=shared", None)
            .await
            .expect("CRITICAL_FAULT: Failed to anchor in-memory tactical ledger.");

        let mission_repository = MissionRepository::new(database_client.clone());
        let database_connection = database_client.get_connection()
            .expect("POOL_FAULT: Unable to allocate database connection.");

        // 2. FASE DE PROVISIÓN (Génesis)
        println!("   🧪 Fase 1: Inyectando misión de prueba en cola...");
        database_connection.execute(
            "INSERT INTO jobs (id, range_start, range_end, status) VALUES ('M_V23_CERT', '0', 'FFFF', 'queued')",
            ()
        ).await.expect("DB_INSERT_FAULT: Genesis injection failed.");
        technical_forensic_log.push_str("✅ SETUP: Misión M_V23_CERT inyectada en estrato 'queued'.\n");

        // 3. FASE DE ASIGNACIÓN (Theft Protection Guard)
        println!("   🧪 Fase 2: Validando protección contra colisión de propiedad...");

        // El Worker ALPHA reclama la misión con un Operador designado
        // ✅ RESOLUCIÓN SINCRO: Uso de la firma V300.7 con 'operator_id'
        mission_repository.assign_mission_to_worker("M_V23_CERT", "WORKER_ALPHA", Some("OPERATOR_MASTER"))
            .await
            .expect("DISPATCH_FAULT: Worker ALPHA failed to acquire mission.");

        // El Worker BETA intenta una asignación concurrente (Debe ser rechazada)
        let unauthorized_assignment_attempt = mission_repository
            .assign_mission_to_worker("M_V23_CERT", "WORKER_BETA", None)
            .await;

        if unauthorized_assignment_attempt.is_err() {
            println!("      ✅ Exclusividad de Propiedad: Certificada.");
            technical_forensic_log.push_str("✅ SECURITY: El Ledger bloqueó exitosamente el intento de robo de misión.\n");
        } else {
            println!("      ❌ FALLO: El sistema permitió la doble asignación de una misión activa.");
            accumulated_anomalies_count += 1;
            technical_forensic_log.push_str("❌ SECURITY: Colapso de integridad en el bloqueo de propiedad.\n");
        }

        // 4. FASE DE CERTIFICACIÓN (Seal Protocol)
        println!("   🧪 Fase 3: Validando sellado de reporte con evidencia de silicio...");

        // ✅ RESOLUCIÓN SINCRO: Inyección de hardware_acceleration_signature para cumplir el contrato V151.0
        let audit_report_artifact = AuditReport {
            job_mission_identifier: "M_V23_CERT".into(),
            worker_node_identifier: "WORKER_ALPHA".into(),
            total_wallets_audited: "65535".into(),
            execution_duration_milliseconds: 420,
            final_mission_status: "completed".into(),
            audit_footprint_checkpoint: "0xDEADBEEF_V23".into(),
            completed_at_timestamp: chrono::Utc::now().to_rfc3339(),
            average_computational_efficiency: 156.03,
            hardware_acceleration_signature: "ELITE_SIMD_ADX_VIRTUAL".into(), // Firma requerida
        };

        let database_io_start = Instant::now();
        let certification_result = mission_repository.certify_mission_completion(&audit_report_artifact).await;
        let database_io_latency_ms = database_io_start.elapsed().as_secs_f64() * 1000.0;

        if certification_result.is_ok() {
            println!("      ✅ Certificación Sellada. Latencia I/O: {:.2}ms.", database_io_latency_ms);
            technical_forensic_log.push_str(&format!("✅ PERSISTENCE: Misión certificada con rastro forense completo. Latencia: {}ms.\n", database_io_latency_ms));
        } else {
            println!("      ❌ FALLO: El Ledger rechazó el reporte de auditoría nivelado.");
            accumulated_anomalies_count += 1;
            technical_forensic_log.push_str("❌ PERSISTENCE: Error al comprometer el reporte en Motor A.\n");
        }

        // 5. SENTENCIA Y REPORTE AL DASHBOARD
        let final_verdict_label = if accumulated_anomalies_count == 0 { "GOLD_MASTER" } else { "DEGRADED" };
        technical_forensic_log.push_str(&format!("\nVEREDICTO_FINAL: {}\n", final_verdict_label));

        dispatch_persistence_integrity_report(
            final_verdict_label,
            database_io_latency_ms,
            technical_forensic_log,
            accumulated_anomalies_count
        );

        println!("\n🏁 [INFORME]: Auditoría finalizada en {:?}. Veredicto: {}", suite_execution_start.elapsed(), final_verdict_label);

        assert_eq!(accumulated_anomalies_count, 0, "La integridad del Ledger Táctico ha sido comprometida.");
    }
}

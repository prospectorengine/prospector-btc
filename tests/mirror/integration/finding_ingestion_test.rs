// [tests/mirror/integration/finding_ingestion_test.rs]
/**
 * =================================================================
 * APARATO: FINDING INGESTION INTEGRITY TEST (V12.0 - SOBERANO)
 * CLASIFICACIÓN: E2E INTEGRATION / TRINITY EVIDENCE
 * RESPONSABILIDAD: CERTIFICACIÓN DEL CICLO DE VIDA DEL HALLAZGO
 *
 * # Logic:
 * Valida la "Tríada de Notificación" ante una colisión:
 * 1. HTTP Interface: El endpoint /finding acepta la ráfaga.
 * 2. Neural Bus: El evento se difunde a los suscriptores (Dashboard).
 * 3. Tactical Vault: El hallazgo se encola para persistencia en Motor A.
 *
 * # Performance:
 * Ejecución en runtime de Tokio con aislamiento de base de datos en memoria.
 * =================================================================
 */

#[cfg(test)]
mod tests {
    use prospector_orchestrator::state::AppState;
    use prospector_orchestrator::routes::create_sovereign_router;
    use prospector_domain_models::finding::Finding;
    use prospector_domain_models::telemetry::RealTimeEvent;
    use prospector_infra_db::TursoClient;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    use chrono::Utc;

    /// Configura un entorno de pruebas aislado con Ledger en RAM.
    async fn setup_sovereign_test_environment() -> AppState {
        // Inicialización de Turso en modo memoria compartida para visibilidad de hilos
        let database_client = TursoClient::connect("file:ingestion_test?mode=memory&cache=shared", None)
            .await
            .expect("CRITICAL_FAULT: Failed to anchor in-memory ledger.");

        AppState::new(database_client)
    }

    /**
     * CERTIFICACIÓN: Ciclo completo de Ingesta y Difusión.
     *
     * # Mathematical Proof (Integrity Path):
     * Si el estatus es 201 AND el evento está en el bus AND el conteo en bóveda es 1,
     * se garantiza que la cadena de custodia del hallazgo es ininterrumpida.
     */
    #[tokio::test]
    async fn certify_finding_ingestion_and_broadcast_lifecycle() {
        println!("\n🔍 [AUDIT]: Initiating E2E Finding Lifecycle Certification...");

        // 1. IGNICIÓN DEL ESTRATO DE CONTROL
        let application_state = setup_sovereign_test_environment().await;
        let sovereign_router = create_sovereign_router(application_state.clone());

        // Suscripción al sistema nervioso (Event Bus) antes del disparo
        let mut neural_link_subscriber = application_state.event_bus.subscribe();

        // 2. PREPARACIÓN DEL MATERIAL CRIPTOGRÁFICO (GOLDEN TICKET)
        let target_address = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(); // Satoshi Genesis
        let finding_payload = Finding {
            address: target_address.clone(),
            private_key_wif: "5HvPBms4...PRIVATE_MATERIAL_REDACTED...".to_string(),
            source_entropy: "forensic_audit:sequential_scan:v12".to_string(),
            wallet_type: "p2pkh_legacy_uncompressed".to_string(),
            found_by_worker: "hydra-node-test-alpha".to_string(),
            job_id: Some("00000000-0000-0000-0000-000000000000".to_string()),
            detected_at: Utc::now().to_rfc3339(),
        };

        let serialized_json_payload = serde_json::to_string(&finding_payload)
            .expect("FAULT: Payload serialization collapsed.");

        // 3. CONFIGURACIÓN DE SEGURIDAD DE ENTORNO
        let test_auth_token = "sovereign_test_secret_2026";
        std::env::set_var("WORKER_AUTH_TOKEN", test_auth_token);

        // 4. DESPACHO DE RÁFAGA TÁCTICA (Simulación de Inyector Worker)
        println!("   🛰️  [DISPATCH]: Transmitting collision artifact to Orchestrator...");
        let network_request = Request::builder()
            .method("POST")
            .uri("/api/v1/swarm/finding")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", test_auth_token))
            .body(Body::from(serialized_json_payload))
            .unwrap();

        let network_response = sovereign_router.oneshot(network_request).await
            .expect("CRITICAL_FAULT: Router failed to process signal.");

        // --- FASE DE VERDICTO ---

        // Veredicto A: Respuesta de Interfaz
        assert_eq!(network_response.status(), StatusCode::CREATED, "L3: API rejected the discovery artifact.");
        println!("   ✅ [VERDICT_A]: HTTP 201 Created confirmed.");

        // Veredicto B: Difusión Neural (Event Bus)
        // Verificamos que el bus emitió la señal 'cc' (Cryptographic Collision)
        let neural_signal = neural_link_subscriber.try_recv()
            .expect("L4: Neural Bus failed to broadcast the collision event.");

        if let RealTimeEvent::CryptographicCollisionAlert { target_bitcoin_address, discovery_node } = neural_signal {
            assert_eq!(target_bitcoin_address, target_address, "DATA_CORRUPTION: Bus address mismatch.");
            assert_eq!(discovery_node, "hydra-node-test-alpha");
            println!("   ✅ [VERDICT_B]: Neural Link broadcast verified bit-perfect.");
        } else {
            panic!("INTEGRITY_FAULT: Event Bus emitted an incorrect signal type.");
        }

        // Veredicto C: Persistencia en RAM (Tactical Vault)
        let vault_pending_count = application_state.finding_vault.get_pending_count();
        assert_eq!(vault_pending_count, 1, "L3: Finding Vault failed to secure the artifact.");

        let secured_artifact = &application_state.finding_vault.drain_vault_for_flush()[0];
        assert_eq!(secured_artifact.address, target_address);
        println!("   ✅ [VERDICT_C]: Tactical Vault residency confirmed.");

        println!("\n🏁 [CERTIFIED]: Finding Ingestion Pipeline is Gold Master level.\n");
    }
}

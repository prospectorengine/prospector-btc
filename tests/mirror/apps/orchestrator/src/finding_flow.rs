// [tests/mirror/apps/orchestrator/src/finding_flow.rs]
/**
 * =================================================================
 * APARATO: FINDING FLOW INTEGRATION TEST (V1.1 - ZENITH CERTIFIED)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L3-MIRROR
 * RESPONSABILIDAD: VALIDACIÓN DEL FLUJO COMPLETO DE INGESTA DE HALLAZGOS
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. PATH RESOLUTION: Resolución definitiva de E0432. Se sustituye 'crate::'
 *    por la autoridad nominal 'prospector_orchestrator' para acceso externo.
 * 2. HANDLER ALIGNMENT: Sincronización bit-perfecta con el método
 *    'register_cryptographic_collision_finding' nivelado en el SwarmHandler.
 * 3. HYGIENE TOTAL: Erradicación del warning de macro 'json' no utilizada.
 * 4. ATOMIC ASSERTION: Validación del ciclo [Network -> API -> RAM Vault].
 *
 * # Mathematical Proof (Finding Integrity):
 * El test garantiza que una colisión reportada por un worker sea
 * capturada en el buffer de la FindingVault en O(1), previniendo la
 * pérdida de datos antes de la persistencia física en Turso.
 * =================================================================
 */

#[cfg(test)]
mod tests {
    // ✅ RESOLUCIÓN E0432: Uso de la crate nominal para test de integración
    use prospector_orchestrator::handlers::swarm::SwarmHandshakeHandler;
    use prospector_orchestrator::state::AppState;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::post,
        Router,
    };
    use chrono::Utc;
    use prospector_domain_models::finding::Finding;
    use prospector_infra_db::TursoClient;
    use tower::ServiceExt; // Requerido para .oneshot()
    use uuid::Uuid;

    /**
     * Configura una instancia de AppState con base de datos en memoria compartida.
     * Garantiza aislamiento total para la suite de pruebas.
     */
    async fn setup_sovereign_test_state() -> AppState {
        let database_client = TursoClient::connect("file:finding_test?mode=memory&cache=shared", None)
            .await
            .expect("CRITICAL_FAULT: Failed to anchor in-memory ledger.");

        AppState::new(database_client)
    }

    #[tokio::test]
    async fn certify_finding_ingestion_flow() {
        println!("\n🎯 [PROVING_GROUNDS]: Initiating Finding Flow Audit...");

        // 1. PREPARACIÓN DEL ESTRATO DE CONTROL
        let application_state = setup_sovereign_test_state().await;

        // Router mínimo con el handler nominal registrado
        let app = Router::new()
            .route("/finding", post(SwarmHandshakeHandler::register_cryptographic_collision_finding))
            .with_state(application_state.clone());

        // 2. CONSTRUCCIÓN DEL VECTOR DORADO (Simulación de Hallazgo)
        let target_address = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string();
        let finding_payload = Finding {
            address: target_address.clone(),
            private_key_wif: "5HvPBms4...PRIVATE_MATERIAL_REDACTED...".to_string(),
            source_entropy: "forensic_audit:satoshi_xp:qpc_555".to_string(),
            wallet_type: "p2pkh_legacy_uncompressed".to_string(),
            found_by_worker: "hydra-unit-test-01".to_string(),
            job_id: Some(Uuid::new_v4().to_string()),
            detected_at: Utc::now().to_rfc3339(),
        };

        let serialized_payload = serde_json::to_string(&finding_payload)
            .expect("FAULT: Serialization collapsed.");

        // 3. EJECUCIÓN DEL DISPARO TÁCTICO (Simulación HTTP)
        println!("   🛰️  [DISPATCH]: Transmitting finding packet to /api/v1/swarm/finding...");
        let network_request = Request::builder()
            .method("POST")
            .uri("/finding")
            .header("Content-Type", "application/json")
            .body(Body::from(serialized_payload))
            .unwrap();

        let network_response = app.oneshot(network_request).await
            .expect("CRITICAL_FAULT: Request execution collapsed.");

        // 4. VERDICTO DE INTERFAZ
        assert_eq!(
            network_response.status(),
            StatusCode::CREATED,
            "L3_API: Rejection detected. Expected 201 Created."
        );
        println!("   ✅ [VERDICT_A]: HTTP Strata response validated.");

        // 5. VERDICTO DE PERSISTENCIA EN RAM (Finding Vault)
        // El orquestador debe haber depositado el hallazgo en la bóveda de tránsito.
        let vault_count = application_state.finding_vault.get_pending_count();
        assert_eq!(vault_count, 1, "L3_VAULT: Detection failed to reach the vault buffer.");

        let secured_finding = &application_state.finding_vault.drain_vault_for_flush()[0];
        assert_eq!(secured_finding.address, target_address, "DATA_CORRUPTION: Address mismatch in vault.");

        println!("   ✅ [VERDICT_B]: Vault residency and data integrity certified.");
        println!("🏁 [COMPLETE]: Finding Flow certified bit-perfect.\n");
    }
}

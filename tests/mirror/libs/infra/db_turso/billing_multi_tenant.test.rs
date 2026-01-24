// [tests/mirror/libs/infra/db_turso/billing_multi_tenant.test.rs]
/**
 * =================================================================
 * APARATO: BILLING MULTI-TENANT TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L3-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE AISLAMIENTO DE HISTORIAL
 * =================================================================
 */

 use prospector_infra_db::repositories::BillingRepository;
 use prospector_infra_db::TursoClient;

 #[tokio::test]
 async fn certify_billing_history_isolation_between_operators() {
     println!("\n💳 [PROVING_GROUNDS]: Auditing Billing Multi-Tenancy Strata...");

     // 1. SETUP: Ledger en memoria compartida
     let client = TursoClient::connect("file:billing_iso_test?mode=memory&cache=shared", None).await.unwrap();
     let repository = BillingRepository::new(client.clone());

     // 2. ESCENARIO: Dos operadores con actividad energética
     println!("   🧪 Phase 1: Injecting activity for OPERATOR_A and OPERATOR_B...");
     repository.queue_credit_deduction("OPERATOR_A", 1.0, "MISSION_A1").await.unwrap();
     repository.queue_credit_deduction("OPERATOR_B", 5.0, "MISSION_B1").await.unwrap();

     // 3. EXECUTION: Recuperar historial exclusivo para OPERATOR_A
     println!("   🧪 Phase 2: Querying exclusive history for ALPHA...");
     let history_alpha = repository.list_billing_events("OPERATOR_A", 10).await.expect("Query failed");

     // 4. VALIDATION: El historial de A no debe ver a B
     assert_eq!(history_alpha.len(), 1, "L3_SECURITY_FAULT: History count leaked or incorrect.");
     assert!(
         history_alpha[0].audit_description_label.contains("MISSION_A1"),
         "DATA_MISMATCH: Captured wrong mission data."
     );

     println!("   ✅ [SUCCESS]: Billing history isolation certified bit-perfect.");
 }

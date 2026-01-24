// [tests/mirror/libs/infra/db_turso/billing_history.test.rs]
/**
 * =================================================================
 * APARATO: BILLING HISTORY INTEGRITY TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L3-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE RASTRO EN OUTBOX
 * =================================================================
 */

 use prospector_infra_db::repositories::BillingRepository;
 use prospector_infra_db::TursoClient;

 #[tokio::test]
 async fn certify_billing_history_retrieval_v1_4() {
     println!("\n💳 [PROVING_GROUNDS]: Auditing Billing History Strata...");

     // 1. SETUP: Ledger en RAM con caché compartido
     let client = TursoClient::connect("file:history_test?mode=memory&cache=shared", None).await.unwrap();
     let repository = BillingRepository::new(client.clone());

     // 2. EXECUTION: Generar 3 eventos de consumo
     println!("   🧪 Phase 1: Injecting tactical consumption events...");
     for i in 1..=3 {
         repository.queue_credit_deduction(
             "OPERATOR_ALPHA",
             0.5,
             &format!("MISSION_00{}", i)
         ).await.expect("Deduction failed");
     }

     // 3. VALIDATION: Recuperar historial
     println!("   🧪 Phase 2: Retrieving history from Outbox strata...");
     let history = repository.list_billing_events(10).await.expect("History query failed");

     assert_eq!(history.len(), 3, "L3_BILLING_FAULT: History count mismatch.");

     // Verificamos el primer elemento (más reciente por ORDER BY created_at DESC)
     assert!(history[0].associated_mission_identifier.contains("MISSION_003"));
     assert_eq!(history[0].credit_delta_magnitude, -0.5);

     println!("   ✅ [SUCCESS]: Billing history certified bit-perfect.");
 }

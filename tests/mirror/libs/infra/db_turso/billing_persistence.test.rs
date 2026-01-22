// [tests/mirror/libs/infra/db_turso/billing_persistence.test.rs]
/**
 * APARATO: BILLING PERSISTENCE TEST (V1.3)
 * OBJETIVO: Certificar el nuevo método nominal 'queue_credit_deduction'.
 */

use prospector_infra_db::repositories::BillingRepository;
use prospector_infra_db::TursoClient;

#[tokio::test]
async fn certify_billing_nominal_sync() {
    println!("\n💳 [PROVING_GROUNDS]: Auditing Billing Nominal Strata...");

    let client = TursoClient::connect("file:billing_test?mode=memory&cache=shared", None).await.unwrap();
    let repo = BillingRepository::new(client.clone());

    // 1. EXECUTION: Usando el método nivelado
    let result = repo.queue_credit_deduction("ARCHITECT_ALPHA", 1.0, "M_001").await;

    // 2. VALIDATION
    assert!(result.is_ok(), "L3_BILLING_FAULT: Nominal method failed.");

    let balance = repo.get_cached_balance("ARCHITECT_ALPHA").await.unwrap();
    // 100 (default) - 1.0 = 99.0
    assert_eq!(balance, 99.0);

    println!("   ✅ [SUCCESS]: Method 'queue_credit_deduction' is operational and bit-perfect.");
}

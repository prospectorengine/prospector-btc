// [tests/mirror/libs/infra/db_turso/billing_persistence.test.rs]
/**
 * =================================================================
 * APARATO: BILLING PERSISTENCE TEST (V1.1 - RECOVERY READY)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L3-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE ATOMICIDAD DE ENERGÍA
 * =================================================================
 */

use prospector_infra_db::repositories::BillingRepository;
use prospector_infra_db::TursoClient;

#[tokio::test]
async fn certify_billing_transaction_integrity_and_outbox() {
    println!("\n💳 [PROVING_GROUNDS]: Auditing Billing Strata ACIDity V1.1...");

    // 1. SETUP: Ledger Táctico en memoria con caché compartido
    let client = TursoClient::connect("file:billing_test?mode=memory&cache=shared", None).await.unwrap();
    let repo = BillingRepository::new(client.clone());
    let conn = client.get_connection().unwrap();

    let operator_id = "ARCHITECT_ALPHA";

    // 2. INITIALIZATION: Inyectar balance génesis
    repo.sync_local_balance(operator_id, 100.0).await.expect("Fallo al inicializar balance");

    // 3. EXECUTION: Deducir créditos por misión
    println!("   🚀 [EXECUTION]: Firing atomic deduction sequence...");
    repo.execute_credit_deduction_sequence(operator_id, 5.25, "M_STRESS_001")
        .await
        .expect("Deducción fallida");

    // 4. VALIDATION A: Verificación de balance en caché local
    let current_balance = repo.get_cached_balance(operator_id).await.unwrap();
    assert_eq!(current_balance, 94.75, "L3_BILLING_FAULT: Balance drift detected in Tactical Cache.");
    println!("      ✅ Local Cache updated: 94.75 units.");

    // 5. VALIDATION B: Verificación de sellado en Outbox Estratégico
    let mut outbox_query = conn.query(
        "SELECT count(*) FROM outbox_strategic WHERE target_stratum = 'BILLING_CONSUMPTION'",
        ()
    ).await.unwrap();

    let outbox_count: i64 = outbox_query.next().await.unwrap().unwrap().get(0).unwrap();
    assert_eq!(outbox_count, 1, "L3_BILLING_FAULT: Outbox event was not crystallized in Turso.");
    println!("      ✅ Outbox Strata sealed for Relay processing.");

    println!("   ✅ [SUCCESS]: Atomic energy lifecycle certified bit-perfect.");
}

// [tests/mirror/apps/orchestrator/graphql/affiliate_recursion.test.rs]
/**
 * =================================================================
 * APARATO: AFFILIATE RECURSION INTEGRATION TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar que el Oracle reporta potencia agregada.
 * =================================================================
 */

use prospector_orchestrator::graphql::build_neural_schema;
use prospector_infra_db::TursoClient;

#[tokio::test]
async fn certify_recursive_hashrate_in_graphql_oracle() {
    println!("\n🤝 [PROVING_GROUNDS]: GQL Affiliate Aggregation Audit...");

    // 1. SETUP: In-Memory con topología de 2 niveles
    let client = TursoClient::connect("file::memory:", None).await.unwrap();
    let conn = client.get_connection().unwrap();

    // ARCHITECT -> OPERATOR_A (50 MH/s)
    conn.execute(
        "INSERT INTO affiliate_network (affiliate_id, accumulated_hashrate) VALUES ('ARCHITECT', 100.0)",
        ()
    ).await.unwrap();
    conn.execute(
        "INSERT INTO affiliate_network (affiliate_id, parent_affiliate_id, accumulated_hashrate) VALUES ('OP_A', 'ARCHITECT', 50.0)",
        ()
    ).await.unwrap();

    let schema = build_neural_schema(client);

    // 2. QUERY: Solicitar el nodo del Architect
    let query = r#"
        query {
            getAffiliateNode(affiliateId: "ARCHITECT") {
                affiliateId
                contributionHashrate
            }
        }
    "#;

    let response = schema.execute(query).await;
    let data = response.data.into_json().unwrap();
    let contribution = data["getAffiliateNode"]["contributionHashrate"].as_f64().unwrap();

    // 3. VALIDATION: 100 (propio) + 50 (hijo) = 150
    assert_eq!(contribution, 150.0, "La agregación recursiva en GQL falló.");

    println!("   ✅ GQL Oracle: Recursive hashrate for 'ARCHITECT' is 150.0 MH/s. Certified.");
}

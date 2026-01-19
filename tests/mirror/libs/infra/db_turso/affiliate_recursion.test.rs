// [tests/mirror/libs/infra/db_turso/affiliate_recursion.test.rs]
/**
 * =================================================================
 * APARATO: AFFILIATE RECURSION TEST
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L3-MIRROR
 * OBJETIVO: Certificar el cálculo de potencia agregada en 3 niveles.
 * =================================================================
 */

use prospector_infra_db::TursoClient;
use prospector_infra_db::repositories::affiliate_repository::AffiliateRepository;


#[tokio::test]
async fn certify_recursive_hashrate_aggregation() {
    println!("\n🤝 [PROVING_GROUNDS]: Affiliate Network Recursion Audit...");

    // 1. SETUP: DB en memoria compartida
    let client = TursoClient::connect("file::mem_aff?mode=memory&cache=shared", None).await.unwrap();
    let conn = client.get_connection().unwrap();
    let repo = AffiliateRepository::new(client.clone());

    // 2. ESCENARIO: Árbol de 3 niveles
    // Nivel 0: ARCHITECT (Root) -> 100 MH/s
    // Nivel 1: OPERATOR_A (Hijo de Root) -> 50 MH/s
    // Nivel 2: NODE_B (Hijo de OPERATOR_A) -> 25 MH/s
    // Total esperado para ARCHITECT: 175 MH/s

    conn.execute("INSERT INTO affiliate_network (affiliate_id, accumulated_hashrate) VALUES ('ARCHITECT', 100.0)", ()).await.unwrap();
    conn.execute("INSERT INTO affiliate_network (affiliate_id, parent_affiliate_id, accumulated_hashrate) VALUES ('OP_A', 'ARCHITECT', 50.0)", ()).await.unwrap();
    conn.execute("INSERT INTO affiliate_network (affiliate_id, parent_affiliate_id, accumulated_hashrate) VALUES ('NODE_B', 'OP_A', 25.0)", ()).await.unwrap();

    // 3. EXECUTION
    let total_power = repo.get_aggregated_network_power("ARCHITECT").await.unwrap();

    // 4. VALIDATION
    assert_eq!(total_power, 175.0, "La recursión falló al agregar el hashrate descendente.");

    println!("   ✅ Aggregated Power: {} MH/s certified.", total_power);
    println!("   ✅ Recursive CTE logic: BIT-PERFECT.");
}

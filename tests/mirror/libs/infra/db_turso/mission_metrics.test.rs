// [tests/mirror/libs/infra/db_turso/mission_metrics.test.rs]
/**
 * =================================================================
 * APARATO: MISSION METRICS INTEGRITY TEST (V1.0 - SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L3-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE CÁLCULO DE COBERTURA REAL
 * =================================================================
 */

 use prospector_infra_db::repositories::MissionRepository;
 use prospector_infra_db::TursoClient;
 use prospector_domain_models::work::TargetStrata;

 #[tokio::test]
 async fn certify_real_coverage_calculation_strata() {
     println!("\n📊 [PROVING_GROUNDS]: Auditing Coverage Computation Strata...");

     // 1. SETUP: Ledger en memoria
     let client = TursoClient::connect("file:coverage_test?mode=memory&cache=shared", None).await.unwrap();
     let repository = MissionRepository::new(client.clone());
     let conn = client.get_connection().unwrap();

     // 2. INJECT: 4 misiones, 1 completada (25% cobertura)
     println!("   🧪 Phase 1: Injecting mission ratio [1:4]...");
     for i in 1..=4 {
         let status = if i == 1 { "completed" } else { "queued" };
         conn.execute(
             "INSERT INTO jobs (id, range_start, range_end, status, required_strata)
              VALUES (?1, '0', '1', ?2, 'StandardLegacy')",
             params![format!("M-{}", i), status]
         ).await.unwrap();
     }

     // 3. EXECUTION: Calcular cobertura
     let coverage = repository.calculate_strata_coverage(TargetStrata::StandardLegacy).await.unwrap();

     // 4. VALIDATION: Paridad matemática
     assert_eq!(coverage, 25.0, "L3_METRICS_FAULT: Coverage percentage drift detected.");

     println!("   ✅ [SUCCESS]: Coverage calculation is bit-perfect (25.00%).");
 }

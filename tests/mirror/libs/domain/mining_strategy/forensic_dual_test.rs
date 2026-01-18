/**
 * =================================================================
 * APARATO: FORENSIC DUAL DERIVATION TEST (V11.2 - SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar la paridad entre Compressed y Uncompressed.
 * =================================================================
 */

use prospector_domain_strategy::forensic_auditor::ForensicVectorAuditor;

#[tokio::test]
async fn certify_dual_path_cryptographic_parity() {
    // 1. SETUP: Vector 'satoshi' (Históricamente uncompressed, hoy dual)
    let input_dataset = vec![
        (1, "Brainwallet".to_string(), "satoshi".to_string(), "".into(), "".into())
    ];

    // 2. EXECUTION: Auditoría de espectro completo
    let audit_results = ForensicVectorAuditor::execute_dataset_certification(input_dataset).await;
    let primary_report = &audit_results[0];

    // 3. VALIDATION
    println!("🔍 DUAL_PATH ANALYSIS ['satoshi']:");
    println!("   ↳ Uncompressed: {}", primary_report.address_uncompressed);
    println!("   ↳ Compressed:   {}", primary_report.address_compressed);

    // Las direcciones deben empezar con '1' (Legacy) y ser distintas
    assert!(primary_report.address_uncompressed.starts_with('1'));
    assert!(primary_report.address_compressed.starts_with('1'));
    assert_ne!(primary_report.address_uncompressed, primary_report.address_compressed);

    println!("✅ FORENSIC: Dual derivation strata certified.");
}

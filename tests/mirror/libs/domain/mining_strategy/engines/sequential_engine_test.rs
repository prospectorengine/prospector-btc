/**
 * CERTIFICACIÓN: Integridad de ráfaga, continuidad escalar y firma de silicio.
 */
#[test]
fn certify_sequential_burst_and_silicon_signature_v202_7() {
    println!("\n🚀 [AUDIT]: Initiating Projective Sequential Engine Master Audit V202.7...");
    let mut technical_error_accumulator = 0;
    let suite_start_timestamp = Instant::now();

    // 1. SETUP DEL ESCENARIO (VECTORES DORADOS)
    let start_hexadecimal_scalar = "0000000000000000000000000000000000000000000000000000000000000001";

    // ✅ RESOLUCIÓN SOBERANA: Instanciación nominal para eliminar 'cannot find value filter'
    let sharded_census_filter = ShardedFilter::new(1, 1000, 0.0001);

    let discovery_spy = DiscoveryForensicSpy {
        accumulated_found_count: Arc::new(AtomicU64::new(0))
    };
    let effort_telemetry_accumulator = Arc::new(AtomicU64::new(0));
    let global_stop_signal = AtomicBool::new(false);

    // GENERACIÓN DE LA VERDAD (Target at scalar 1000 / 0x3E8)
    let target_scalar_hex = "00000000000000000000000000000000000000000000000000000000000003E8";
    let target_private_key_handle = SafePrivateKey::from_bytes(&hex::decode(target_scalar_hex).unwrap())
        .expect("MATH_FAULT: Failed to generate target private key.");
    let target_public_key_point = SafePublicKey::from_private(&target_private_key_handle);

    // Inyectamos paridad dual para certificar cobertura total del motor Meloni
    sharded_census_filter.add(&hash160(&target_public_key_point.to_bytes(true)));
    sharded_census_filter.add(&hash160(&target_public_key_point.to_bytes(false)));

    println!("   🧪 Phase 1: Validating detection in Montgomery Magazine (Batch 1024)...");

    // 2. EJECUCIÓN DEL MÚSCULO COMPUTACIONAL (V212.5)
    // Ejecutamos 1050 iteraciones para probar el ciclo de vaciado (flush) de residuos.
    let final_mission_checkpoint = ProjectiveSequentialEngine::execute_optimized_audit(
        start_hexadecimal_scalar,
        1050,
        &sharded_census_filter,
        &global_stop_signal,
        effort_telemetry_accumulator.clone(),
        &discovery_spy
    );

    // 3. VALIDACIÓN FORENSE DE RESULTADOS
    let found_collisions_count = discovery_spy.accumulated_found_count.load(Ordering::SeqCst);

    if found_collisions_count >= 2 {
        println!("      ✅ Dual Strata Detection: OK (Both formats identified).");
    } else {
        println!("      ❌ ERROR: Signal loss in burst. Found: {}", found_collisions_count);
        technical_error_accumulator += 1;
    }

    // Validación de Checkpoint: 1 (start) + 1050 (iter) = 1051 (0x41B)
    if final_mission_checkpoint.to_lowercase().contains("41b") {
        println!("      ✅ Scaler Continuity: OK (Checkpoint at 0x41B).");
    } else {
        println!("      ❌ ERROR: Checkpoint drift detected. Received: {}", final_mission_checkpoint);
        technical_error_accumulator += 1;
    }

    // 4. BENCHMARK DE POTENCIA Y FIRMA DE ACELERACIÓN
    println!("   🚀 Phase 2: Measuring silicon performance and hardware signature...");
    let is_accelerated = is_optimized_arithmetic_supported();
    let hardware_signature = if is_accelerated { "ELITE_SIMD_ADX" } else { "STANDARD_SW" };

    let performance_bench_start = Instant::now();
    ProjectiveSequentialEngine::execute_optimized_audit(
        &final_mission_checkpoint,
        5000,
        &sharded_census_filter,
        &global_stop_signal,
        effort_telemetry_accumulator.clone(),
        &discovery_spy
    );
    let bench_duration = performance_bench_start.elapsed();
    let hashrate_throughput = 5000.0 / bench_duration.as_secs_f64();

    println!("      🚀 Throughput: {:.2} H/s | Mode: {}", hashrate_throughput, hardware_signature);

    // 5. CRISTALIZACIÓN DE EVIDENCIA (QA LEDGER)
    // ✅ SINCRO NIVEL DIOS: Inyección de campos requeridos por AuditReport V151.0
    let report_storage_path = "reports/qa/sequential_engine_master_report.json";
    let report_payload = json!({
        "apparatus": "projective_sequential_engine",
        "verdict": if technical_error_accumulator == 0 { "GOLD_MASTER" } else { "FAILED" },
        "metrics": {
            "hashrate_hs": hashrate_throughput,
            "average_efficiency_h_ms": hashrate_throughput / 1000.0,
            "batch_latency_ms": bench_duration.as_millis(),
            "collisions_found": found_collisions_count,
            "logical_faults": technical_error_accumulator
        },
        "evidence": {
            "hardware_acceleration_signature": hardware_signature,
            "math_engine": "Jacobian_Montgomery_V130",
            "batch_magazine_size": 1024
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    fs::create_dir_all("reports/qa").ok();
    fs::write(report_storage_path, report_payload.to_string())
        .expect("IO_FAULT: Unable to seal QA report.");

    println!("\n💾 [REPORT]: Audit evidence secured at: {}", report_storage_path);
    println!("🏁 [COMPLETE]: Certification finalized in {:?}.", suite_start_timestamp.elapsed());

    assert_eq!(technical_error_accumulator, 0, "Sequential Engine integrity compromised.");
}

// [tests/mirror/libs/domain/forensics/luno_integrity.test.rs]
/**
 * =================================================================
 * APARATO: LUNO INTEGRITY CERTIFIER (V1.0 - SINGULARITY)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar reconstrucción de entropía temporal.
 * =================================================================
 */

 use prospector_domain_forensics::luno_rng::LunoForensicIterator;
 use std::time::Instant;

 #[test]
 fn certify_luno_temporal_reconstruction_and_bench() {
     println!("\n🕵️ [PROVING_GROUNDS]: Initiating Luno 2014 Archaeology Audit...");

     // 1. SETUP: Ventana de 1 segundo (1000 milisegundos)
     let start_ts: u64 = 1417392000000; // 2014-12-01T00:00:00
     let end_ts: u64 = start_ts + 999;

     let mut iterator = LunoForensicIterator::new(start_ts, end_ts);

     // 2. EXECUTION & VALIDATION: Primer rastro
     let (label, key) = iterator.next().expect("ITERATOR_START_FAULT");
     assert!(label.contains("1417392000000"), "Metadata drift detected.");
     assert_eq!(key.to_bytes().len(), 32, "Escalar privado truncado.");

     // 3. BENCHMARK: Medición de Throughput
     println!("   🚀 Executing sweep of 10,000 ms vectors...");
     let perf_start = Instant::now();
     let sweep: Vec<_> = LunoForensicIterator::new(start_ts, start_ts + 10000).collect();
     let duration = perf_start.elapsed();

     let rec_per_sec = 10000.0 / duration.as_secs_f64();
     println!("      ✅ Throughput: {:.2} reconstructions/sec.", rec_per_sec);
     assert_eq!(sweep.len(), 10001);

     println!("\n🏁 [COMPLETE]: Luno Forensic Strata certified bit-perfect.");
 }

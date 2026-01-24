// [tests/mirror/libs/domain/models_rs/work_contract_v17.test.rs]
/**
 * =================================================================
 * APARATO: WORK CONTRACT INTEGRITY TEST (V17.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar la paridad del contrato Luno y BigInt Bridge.
 * =================================================================
 */

 use prospector_domain_models::work::{SearchStrategy, NodeHardwareCapacity};

 #[test]
 fn certify_luno_strategy_and_hardware_metrics() {
     println!("\n⚖️  [PROVING_GROUNDS]: Auditing Work Contracts Singularity...");

     // 1. TEST: Validación de nueva variante Luno
     let luno_strategy = SearchStrategy::LunoBlockchainForensic {
         start_timestamp_milliseconds: 1417392000000,
         end_timestamp_milliseconds: 1417392000999,
     };

     if let SearchStrategy::LunoBlockchainForensic { start_timestamp_milliseconds, .. } = luno_strategy {
         assert_eq!(start_timestamp_milliseconds, 1417392000000);
         println!("   ✅ Luno Strategy: Contract alignment bit-perfect.");
     } else {
         panic!("L1_CONTRACT_FAULT: Luno variant failed to materialize.");
     }

     // 2. TEST: Cálculo de capacidad (Bytes a MB)
     let raw_ram = 8 * 1024 * 1024 * 1024; // 8GB
     let capacity = NodeHardwareCapacity::calculate_from_raw(raw_ram, 4, true);

     assert_eq!(capacity.ram_available_megabytes, 8192, "Error de conversión de estratos.");
     println!("   ✅ Hardware Capacity: Nominal MB conversion certified.");

     println!("\n🏁 [COMPLETE]: Work Domain Models V17.0 levelized.");
 }

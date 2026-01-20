use prospector_domain_models::work::NodeHardwareCapacity;

#[test]
fn certify_hardware_capacity_calculation_sovereignty() {
    println!("\n⚖️  [PROVING_GROUNDS]: Auditing Node Hardware Capacity Logic...");

    // Escenario: Nodo con 8GB de RAM
    let raw_bytes = 8 * 1024 * 1024 * 1024;
    let cpu_threads = 4;
    let avx_active = true;

    let capacity = NodeHardwareCapacity::calculate_from_raw(raw_bytes, cpu_threads, avx_active);

    assert_eq!(capacity.ram_available_megabytes, 8192);
    assert_eq!(capacity.cpu_cores, 4);
    assert!(capacity.supports_avx2);

    println!("   ✅ [SUCCESS]: Byte-to-Megabyte isomorphism bit-perfect.");
}

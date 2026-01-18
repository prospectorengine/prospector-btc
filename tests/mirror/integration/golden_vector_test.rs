// INICIO DEL ARCHIVO [tests/mirror/integration/golden_vector_test.rs]
#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicU64};
    use prospector_core_probabilistic::sharded::ShardedFilter;
    use prospector_domain_strategy::{EntropyDictionaryEngine, FindingHandler};
    use prospector_core_math::private_key::SafePrivateKey;

    struct FindingSpy {
        pub found_count: Arc<std::sync::Mutex<usize>>,
    }

    impl FindingHandler for FindingSpy {
        fn on_finding(&self, address: String, _: SafePrivateKey, source: String) {
            let mut count = self.found_count.lock().unwrap();
            *count += 1;
            println!("🎯 [MATCH]: {} from source {}", address, source);
        }
    }

    #[test]
    fn certify_detection_of_golden_33() {
        // 1. Setup del Filtro (Simulamos un filtro con los 33 objetivos)
        let mut filter = ShardedFilter::new(1, 100, 0.0001);
        let seeds = vec!["power", "satoshi", "bitcoin", "zero"];

        for seed in &seeds {
            let pk = prospector_domain_strategy::phrase_to_private_key(seed);
            let pubk = prospector_core_math::public_key::SafePublicKey::from_private(&pk);
            // Inyectamos el hash160 de la dirección no comprimida (Era Satoshi)
            let hash = prospector_core_math::hashing::hash160(&pubk.to_bytes(false));
            filter.add(&hash);
        }

        // 2. Ejecución del motor con las mismas semillas
        let spy = FindingSpy { found_count: Arc::new(std::sync::Mutex::new(0)) };
        let effort = Arc::new(AtomicU64::new(0));
        let stop = AtomicBool::new(false);

        let dataset: Vec<String> = seeds.iter().map(|s| s.to_string()).collect();

        EntropyDictionaryEngine::execute_dictionary_audit(
            &dataset,
            &filter,
            &stop,
            effort,
            &spy
        );

        // 3. Validación: Debió encontrar los 4 vectores inyectados
        assert_eq!(*spy.found_count.lock().unwrap(), 4);
        println!("✅ [SUCCESS]: Golden vectors detected bit-perfectly.");
    }
}

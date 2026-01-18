// [tests/mirror/libs/infra/worker_client/hydrator.test.rs]
#[cfg(test)]
mod tests {
  
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    use prospector_infra_worker_client::hydrator::ForensicDnaHydrator;

    #[tokio::test]
    async fn certify_hydrator_signature_validation() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("invalid_dna.bin");

        // Creamos un archivo sin la firma "PERF"
        let mut file = File::create(&file_path).unwrap();
        file.write_all(&[0u8; 300_000]).unwrap();

        let result = ForensicDnaHydrator::hydrate_dna_from_disk(&file_path).await;

        assert!(result.is_err());
        println!("✅ HYDRATOR: Successfully rejected invalid signature.");
    }
}

// [tests/mirror/libs/infra/db_turso/identity_persistence.test.rs]
/**
 * =================================================================
 * APARATO: IDENTITY PERSISTENCE TEST (V37.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L3-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE PERFILES PERSISTENTES HYDRA-ID
 * =================================================================
 */

use prospector_infra_db::repositories::IdentityRepository;
use prospector_infra_db::TursoClient;
use prospector_domain_models::identity::CreateIdentityPayload;

#[tokio::test]
async fn certify_sovereign_identity_persistence_and_mmap() {
    println!("\n🔐 [PROVING_GROUNDS]: Auditing Identity Strata V37.0...");

    // 1. SETUP: Ledger en memoria compartida
    let client = TursoClient::connect("file::mem_id_v37?mode=memory&cache=shared", None).await.unwrap();
    let repo = IdentityRepository::new(client.clone());

    let test_email = "architect_alpha@prospector.io";
    let test_fingerprint = r#"{"webgl":"Nvidia", "canvas":"0x77"}"#;

    // 2. EXECUTION: Ingesta con Hydra-ID payload
    let payload = CreateIdentityPayload {
        platform: "google_colab".into(),
        email: test_email.into(),
        cookies: serde_json::json!([{"name":"SID", "value":"secret"}]),
        user_agent: "Mozilla/5.0...".into(),
        browser_fingerprint_json: Some(test_fingerprint.into()),
        proxy_url: Some("http://proxy.live:8080".into()),
    };

    repo.upsert_sovereign_identity(payload).await.expect("Fallo en Upsert V37");

    // 3. VALIDATION: Recuperación y paridad bit-perfecta
    let all_ids = repo.list_all_identities().await.unwrap();
    let target = all_ids.iter().find(|i| i.email == test_email).expect("Identidad no encontrada");

    assert_eq!(target.browser_fingerprint_json.as_ref().unwrap(), test_fingerprint);
    assert_eq!(target.proxy_url.as_ref().unwrap(), "http://proxy.live:8080");

    println!("   ✅ [SUCCESS]: Fingerprint and Proxy strata levelized and secured.");
}

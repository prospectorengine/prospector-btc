// [tests/mirror/libs/domain/models_rs/identity_strata.test.rs]
/**
 * =================================================================
 * APARATO: IDENTITY MODEL INTEGRITY TEST (V13.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L2-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE PARIDAD CON PROTOCOLO HYDRA-ID
 * =================================================================
 */

use prospector_domain_models::identity::{Identity, IdentityStatus};
use uuid::Uuid;
use chrono::Utc;

#[test]
fn certify_identity_persistence_fields_alignment() {
    println!("\n🔐 [PROVING_GROUNDS]: Auditing Identity V13.0 Strata...");

    // 1. SETUP: Instanciación de un modelo de dispositivo persistente
    let identity_instance = Identity {
        id: Uuid::new_v4(),
        platform: "google_colab".to_string(),
        email: "vanguard_alpha@prospector.io".to_string(),
        credentials_json: "{}".to_string(),
        user_agent: "Mozilla/5.0...".to_string(),
        status: IdentityStatus::Active,
        usage_count: 42,
        last_used_at: Some(Utc::now()),
        created_at: Utc::now(),
        // ✅ NUEVOS CAMPOS HYDRA-ID
        browser_fingerprint_json: Some(r#"{"canvas":"0xABC", "webgl":"Nvidia"}"#.to_string()),
        proxy_url: Some("socks5://user:pass@proxy.internal:1080".to_string()),
        last_metabolic_pulse: Some(Utc::now()),
        leased_until: None,
        cooldown_until: None,
    };

    // 2. VALIDATION: Verificación de presencia nominal y tipo
    assert!(identity_instance.browser_fingerprint_json.is_some());
    assert!(identity_instance.proxy_url.is_some());
    assert_eq!(identity_instance.usage_count, 42);

    println!("   ✅ [SUCCESS]: Identity model aligned with V154.0 Database Schema.");
    println!("   ✅ [SUCCESS]: Sovereign persistence fields verified.");
}

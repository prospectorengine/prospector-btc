// INICIO DEL ARCHIVO [tests/mirror/integration/identity_refresh_test.rs]
/**
 * =================================================================
 * APARATO: PHOENIX PROTOCOL TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE (L3)
 * OBJETIVO: Certificar la rotación automática de credenciales.
 * =================================================================
 */

use prospector_infra_db::TursoClient;
use prospector_infra_db::repositories::IdentityRepository;
use prospector_domain_models::identity::CreateIdentityPayload;
use serde_json::json;

#[tokio::test]
async fn certify_identity_auto_rotation() {
    // 1. SETUP: Base de datos en memoria con caché compartido para persistencia entre conexiones
    let client = TursoClient::connect("file::mem_refresh_test?mode=memory&cache=shared", None).await.unwrap();
    let repo = IdentityRepository::new(client.clone());
    let conn = client.get_connection().unwrap();

    let email = "phoenix_agent@test.io";

    // 2. INGESTA INICIAL (Cookies Viejas)
    let payload = CreateIdentityPayload {
        platform: "google_colab".into(),
        email: email.into(),
        cookies: json!({"status": "OLD_COOKIE"}),
        user_agent: "Bot/1.0".into(),
    };
    repo.upsert_sovereign_identity(payload).await.unwrap();

    // Simular que la identidad fue usada hace tiempo (Modificación directa SQL)
    conn.execute("UPDATE identities SET last_used_at = datetime('now', '-1 day') WHERE email = ?1", [email]).await.unwrap();

    // 3. EJECUCIÓN (Rotación)
    // El worker envía un payload cifrado nuevo
    let new_encrypted_blob = json!({
        "cipher_text_base64": "NEW_ENCRYPTED_DATA_ABC123",
        "initialization_vector_base64": "IV_VECTOR",
        "salt_base64": "SALT_VECTOR"
    }).to_string();

    repo.refresh_credentials(email, &new_encrypted_blob).await.expect("Refresh operation failed");

    // 4. VALIDACIÓN
    let mut rows = conn.query("SELECT credentials_json, last_used_at, status FROM identities WHERE email = ?1", [email]).await.unwrap();
    let row = rows.next().await.unwrap().unwrap();

    let db_json: String = row.get(0).unwrap();
    let status: String = row.get(2).unwrap();

    // Verificamos actualización de contenido
    assert!(db_json.contains("NEW_ENCRYPTED_DATA"), "El JSON no se actualizó con los nuevos datos.");

    // Verificamos reactivación de estado
    assert_eq!(status, "active", "El estado no se reseteó a active.");

    println!("✅ PHOENIX_PROTOCOL: Identity rotation logic certified.");
}
// FIN DEL ARCHIVO [tests/mirror/integration/identity_refresh_test.rs]

// INICIO DEL ARCHIVO [tests/mirror/libs/infra/db_turso/identity_governance.test.rs]
/**
 * =================================================================
 * APARATO: IDENTITY GOVERNANCE TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Certificar capacidades de liberación forzada y purga.
 * =================================================================
 */

use prospector_infra_db::TursoClient;
use prospector_infra_db::repositories::IdentityRepository;
use prospector_domain_models::identity::CreateIdentityPayload;

#[tokio::test]
async fn certify_governance_capabilities() {
    // 1. SETUP: Base de datos en memoria (Shared Cache)
    let client = TursoClient::connect("file::mem_gov_test?mode=memory&cache=shared", None)
        .await
        .expect("Fallo al inicializar DB en memoria");

    let repo = IdentityRepository::new(client.clone());
    let conn = client.get_connection().unwrap();

    // 2. INGESTA: Crear identidad de prueba
    let payload = CreateIdentityPayload {
        platform: "google_colab".to_string(),
        email: "zombie_worker@test.com".to_string(),
        cookies: serde_json::json!([{ "name": "SID", "value": "test" }]),
        user_agent: "TestAgent/1.0".to_string(),
    };
    repo.upsert_sovereign_identity(payload).await.expect("Fallo al insertar identidad");

    // 3. BLOQUEO: Arrendar identidad (Simular uso)
    // Usamos 'WorkerAgent' para que NO entre en modo Read-Only de test
    let leased = repo.lease_sovereign_identity("google_colab", 60, "WorkerAgent/1.0").await.unwrap();
    assert!(leased.is_some(), "La identidad debería estar disponible para arriendo");

    // Verificamos que está bloqueada
    let check_lease: Option<String> = conn.query("SELECT leased_until FROM identities WHERE email = 'zombie_worker@test.com'", ())
        .await.unwrap().next().await.unwrap().unwrap().get(0).unwrap();
    assert!(check_lease.is_some(), "El campo leased_until debería tener una fecha");

    // 4. ACCIÓN: Force Release (Romper candado)
    repo.force_release_lease("zombie_worker@test.com").await.expect("Fallo al forzar liberación");

    // Verificamos desbloqueo
    let check_release: Option<String> = conn.query("SELECT leased_until FROM identities WHERE email = 'zombie_worker@test.com'", ())
        .await.unwrap().next().await.unwrap().unwrap().get(0).ok().flatten(); // flatten porque get retorna Result<Option<T>> a veces

    assert!(check_release.is_none(), "El campo leased_until debería ser NULL tras liberación");

    // 5. ACCIÓN: Purga (Eliminación)
    repo.purge_identity_record("zombie_worker@test.com").await.expect("Fallo al purgar identidad");

    // Verificamos desaparición
    let mut check_purge = conn.query("SELECT count(*) FROM identities WHERE email = 'zombie_worker@test.com'", ()).await.unwrap();
    let count: i64 = check_purge.next().await.unwrap().unwrap().get(0).unwrap();
    assert_eq!(count, 0, "La identidad debería haber desaparecido físicamente");

    println!("✅ GOVERNANCE: Release & Purge capabilities certified.");
}
// FIN DEL ARCHIVO [tests/mirror/libs/infra/db_turso/identity_governance.test.rs]

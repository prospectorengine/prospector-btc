use prospector_infra_db::repositories::MissionRepository;
use prospector_infra_db::TursoClient;

#[tokio::test]
async fn certify_hydra_slicer_logic() {
    let client = TursoClient::connect("file::memory:", None).await.unwrap();
    let repo = MissionRepository::new(client.clone());
    let conn = client.get_connection().unwrap();

    // 1. Inyectar misión gigante
    conn.execute(
        "INSERT INTO jobs (id, range_start, range_end, status, strategy_type, required_strata)
         VALUES ('M_GIANT', '0', '1000', 'active', 'Sequential', 'StandardLegacy')",
        ()
    ).await.unwrap();

    // 2. Ejecutar Slicing en el checkpoint 500
    let child_id = repo.slice_mission_range("M_GIANT", "500").await.unwrap();

    // 3. Validar Verdad en el Ledger
    let mut row_parent = conn.query("SELECT range_end FROM jobs WHERE id = 'M_GIANT'", ()).await.unwrap();
    assert_eq!(row_parent.next().await.unwrap().unwrap().get::<String>(0).unwrap(), "500");

    let mut row_child = conn.query("SELECT range_start, range_end, status FROM jobs WHERE id = ?1", [child_id]).await.unwrap();
    let data_child = row_child.next().await.unwrap().unwrap();
    assert_eq!(data_child.get::<String>(0).unwrap(), "500");
    assert_eq!(data_child.get::<String>(1).unwrap(), "1000");
    assert_eq!(data_child.get::<String>(2).unwrap(), "queued");

    println!("✅ MISSION_V300: Automatic slicing certified.");
}

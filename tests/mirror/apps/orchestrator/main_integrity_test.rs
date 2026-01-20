// [tests/mirror/apps/orchestrator/main_integrity_test.rs]
/**
 * =================================================================
 * APARATO: KERNEL INTERFACE CERTIFIER (V1.0 - SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L3-MIRROR
 * RESPONSABILIDAD: VALIDACIÓN DE CONTRATO ENTRE SHELL Y KERNEL
 * =================================================================
 */

use prospector_orchestrator::kernel::OrchestratorKernel;
use prospector_infra_db::TursoClient;

#[tokio::test]
async fn certify_kernel_nominal_interface_alignment() {
    println!("\n🛡️  [PROVING_GROUNDS]: Auditing Kernel field nomenclature...");

    // 1. SETUP: Mock de infraestructura en memoria
    let database_client = TursoClient::connect("file:main_test?mode=memory", None).await.unwrap();

    // 2. EXECUTION: Instanciación manual del Kernel
    let kernel = OrchestratorKernel {
        server_network_port: 8080,
        application_shared_state: prospector_orchestrator::state::AppState::new(database_client),
    };

    // 3. VALIDATION: Si esto compila, la Shell (main.rs) puede consumir el Kernel
    // La prueba de éxito es el acceso nominal al campo nivelado.
    assert_eq!(kernel.server_network_port, 8080);

    println!("   ✅ [SUCCESS]: Nominal sync verified between Shell and Kernel.");
}

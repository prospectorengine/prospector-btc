// INICIO DEL ARCHIVO [tests/mirror/apps/orchestrator/lib_structure.test.rs]
/**
 * =================================================================
 * APARATO: MODULE VISIBILITY CERTIFIER (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE
 * OBJETIVO: Verificar la exposición pública del Gateway Neural.
 * =================================================================
 */

#[test]
fn certify_graphql_module_is_public() {
    // Esta prueba falla en tiempo de compilación si el módulo no es 'pub'.
    // Actúa como un guardián de regresión de visibilidad.

    use prospector_orchestrator::graphql::QueryRoot;

    // Verificación de tipo nominal
    let _type_name = std::any::type_name::<QueryRoot>();

    println!("✅ LIB_STRUCTURE: Module 'graphql' and 'QueryRoot' are accessible.");
}
// FIN DEL ARCHIVO [tests/mirror/apps/orchestrator/lib_structure.test.rs]

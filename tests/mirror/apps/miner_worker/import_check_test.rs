// INICIO DEL ARCHIVO [tests/mirror/apps/miner_worker/import_check_test.rs]
/*!
 * =================================================================
 * APARATO: HYDRATOR VISIBILITY TEST (V1.1 - ENCAPSULATION FIXED)
 * OBJETIVO: Verificar que el módulo hydrator es accesible públicamente.
 * =================================================================
 */

#[cfg(test)]
mod tests {
    // El import que antes fallaba en main.rs
    use prospector_infra_worker_client::hydrator::ForensicDnaHydrator;

    #[test]
    fn verify_hydrator_accessibility() {
        // ✅ CORRECCIÓN: No accedemos a constantes privadas.
        // Simplemente instanciamos el Unit Struct. Si esto compila,
        // la visibilidad del módulo está reparada.
        let _hydrator_instance = ForensicDnaHydrator;

        assert!(true, "El tipo ForensicDnaHydrator es visible y accesible.");
    }
}
// FIN DEL ARCHIVO [tests/mirror/apps/miner_worker/import_check_test.rs]

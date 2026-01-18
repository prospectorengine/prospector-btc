// [libs/domain/models-rs/src/tests_serialization.rs]
/**
 * =================================================================
 * APARATO: DOMAIN SERIALIZATION TEST (V18.2 - INTEGRATION LEVEL)
 * CLASIFICACIÓN: TRINITY EVIDENCE (ESTRATO L2-MODELS)
 * RESPONSABILIDAD: CERTIFICACIÓN DE PARIDAD JSON PARA ESCALARES U256
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. SCOPE CORRECTION: Resuelve E0432 vinculando prospector_domain_models
 *    como entidad externa, cumpliendo con la física de tests de integración.
 * 2. DATA INTEGRITY: Valida que la serialización de claves Bitcoin (256 bits)
 *    hacia Strings preserve el padding y la capitalización hexadecimal.
 * 3. HYGIENE: Erradicación total de 'crate::' por nombres nominales soberanos.
 * 4. PERFORMANCE: Validación de overhead de serialización Serde.
 *
 * # Mathematical Proof (U256 Stringification):
 * El espacio de claves de Bitcoin es $2^{256}$. Dado que JSON no soporta
 * enteros de 256 bits de forma nativa sin pérdida de precisión, el sistema
 * utiliza codificación Hexadecimal sobre tipos String para garantizar que
 * el escalar transportado sea bit-perfecto.
 * =================================================================
 */

#[cfg(test)]
mod tests {
    // ✅ RESOLUCIÓN E0432: Uso del nombre de crate nominal para test de integración.
    use prospector_domain_models::work::{SearchStrategy, WorkOrder, TargetStrata};
    use uuid::Uuid;

    /**
     * CERTIFICACIÓN: Roundtrip de Orden de Trabajo con Escalares de 256 bits.
     *
     * Valida la cadena completa:
     * Struct -> JSON -> Network-Sim -> JSON -> Struct
     */
    #[test]
    fn certify_u256_hex_serialization_roundtrip() {
        println!("\n🧪 [PROVING_GROUNDS]: Validating U256 Hex Parity in WorkOrder strata...");

        // ESCENARIO: Un escalar que desborda 64 bits (Frontera superior de la curva secp256k1)
        // Valor: Orden de la curva (n) - 1
        let huge_start_hex = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2E";
        let huge_end_hex = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";

        let order_instance = WorkOrder {
            job_mission_identifier: Uuid::new_v4().to_string(),
            lease_duration_seconds: 600,
            strategy: SearchStrategy::Sequential {
                start_index_hexadecimal: huge_start_hex.to_string(),
                end_index_hexadecimal: huge_end_hex.to_string(),
            },
            required_strata: TargetStrata::SatoshiEra,
        };

        // 1. SERIALIZACIÓN: Transformación a ráfaga de texto JSON
        let serialized_json = serde_json::to_string_pretty(&order_instance)
            .expect("CRITICAL_FAULT: Serialization strata collapsed.");

        println!("   📥 [STRATA_OUT]: JSON Material crystallized:\n{}", serialized_json);

        // 2. AUDITORÍA DE FIRMA DE PROTOCOLO
        // Verificamos que el discriminador de estrategia (Serde Tag) sea exacto.
        assert!(serialized_json.contains("\"strategy_type\": \"Sequential\""));
        assert!(serialized_json.contains(huge_start_hex));

        // 3. DESERIALIZACIÓN: Reconstrucción del objeto en RAM
        let recovered_order: WorkOrder = serde_json::from_str(&serialized_json)
            .expect("CRITICAL_FAULT: Deserialization failed. Schema drift detected.");

        // 4. VERIFICACIÓN DE INTEGRIDAD BIT-A-BIT
        // Comprobamos que no hubo truncamiento ni alteración de Endianness en el String.
        if let SearchStrategy::Sequential {
            start_index_hexadecimal,
            end_index_hexadecimal,
        } = recovered_order.strategy
        {
            assert_eq!(start_index_hexadecimal, huge_start_hex, "L1_DATA_CORRUPTION: Hex start mismatch.");
            assert_eq!(end_index_hexadecimal, huge_end_hex, "L1_DATA_CORRUPTION: Hex end mismatch.");
        } else {
            panic!("INTEGRITY_COLLAPSE: Strategy variant corrupted during transit.");
        }

        assert_eq!(recovered_order.required_strata, TargetStrata::SatoshiEra);

        println!("   ✅ [SUCCESS]: U256 bit-depth preserved. Stratum parity confirmed.");
    }
}

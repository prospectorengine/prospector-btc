// [tests/mirror/libs/core/math_engine/field_integrity_v150.test.rs]
/**
 * =================================================================
 * APARATO: FIELD ELEMENT INTEGRITY CERTIFIER (V1.1 - ZENITH SYNC)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L1-MATH
 * RESPONSABILIDAD: CERTIFICACIÓN DE INVERSIÓN POR LOTES Y REDUCCIÓN
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. FIXED SYNERGY: Resuelve el error E0599 vinculando la función
 *    asociada 'batch_invert_into' restaurada en field.rs V160.3.
 * 2. ZERO ABBREVIATIONS: 'one' -> 'identity_element', 'prod' -> 'multiplication_result'.
 * 3. PROVING GROUNDS: Inyección de bitácora técnica detallada en Español
 *    para auditoría de Tesis Doctoral.
 * 4. PERFORMANCE: Validación de latencia en ráfagas de 1024 unidades.
 * =================================================================
 */

use prospector_core_math::prelude::*;
use std::time::Instant;

#[test]
fn certify_montgomery_strata_and_batch_inversion_v150() {
    println!("\n⚖️  [PROVING_GROUNDS]: Iniciando Certificación de Estrato de Campo Finito...");

    // --- FASE 1: VALIDACIÓN DE IDENTIDAD MULTIPLICATIVA ---
    println!("   🧪 Fase 1: Verificando Identidad en Dominio Montgomery...");
    let identity_element = FieldElement::from_u64(1);

    // (1 * 1) mod p debe ser 1
    let multiplication_result = identity_element.multiply_modular(&identity_element);

    assert_eq!(
        multiplication_result,
        identity_element,
        "CRITICAL_FAULT: La identidad multiplicativa ha colapsado tras la transición Montgomery."
    );
    println!("      ✅ Identidad Multiplicativa: OK.");

    // --- FASE 2: AUDITORÍA DE INVERSIÓN POR LOTES (TRUCO DE MONTGOMERY) ---
    println!("   🧪 Fase 2: Validando Inversión por Lote (1024 unidades)...");

    // Preparación de buffers sobre el Stack (Zero-Alloc approach para el test)
    let mut batch_input_buffer = vec![FieldElement::default(); 1024];
    let mut batch_results_buffer = vec![FieldElement::default(); 1024];
    let mut arithmetic_scratch_pad = vec![FieldElement::default(); 1024];

    // Hidratación de datos de prueba: Escalares incrementales
    for index in 0..1024 {
        batch_input_buffer[index] = FieldElement::from_u64(index as u64 + 1);
    }

    let performance_start_instant = Instant::now();

    // Invocación al método nominal soberano de L1
    let execution_result = FieldElement::batch_invert_into(
        &batch_input_buffer,
        &mut batch_results_buffer,
        &mut arithmetic_scratch_pad
    );

    let execution_duration = performance_start_instant.elapsed();

    // Verificación de éxito de la ráfaga
    assert!(
        execution_result.is_ok(),
        "STRATA_COLLAPSE: El motor de inversión por lotes rechazó la ráfaga."
    );

    // --- FASE 3: AUDITORÍA DE INTEGRIDAD BIT-PERFECT ---
    println!("   🧪 Fase 3: Verificando paridad matemática final (a * a^-1 == 1)...");

    // Validamos el último elemento de la ráfaga (El de mayor peso escalar)
    let last_index = 1023;
    let target_element = batch_input_buffer[last_index];
    let inverted_element = batch_results_buffer[last_index];

    let verification_product = target_element.multiply_modular(&inverted_element);

    assert_eq!(
        verification_product,
        identity_element,
        "MATH_DRIFT: El producto del inverso no converge en la frontera del lote."
    );

    println!("      ✅ Verificación de Inverso: BIT-PERFECT.");
    println!("   📊 MÉTRICAS: 1024 inversiones ejecutadas en {:?}.", execution_duration);
    println!("🏁 [COMPLETE]: Estrato L1-FIELD certificado para misiones V160.\n");
}

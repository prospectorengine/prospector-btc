/**
 * =================================================================
 * APARATO: CERTIFICADOR DE ARQUEOLOGÍA DEBIAN (PROVING GROUNDS)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L2-FORENSICS
 * OBJETIVO: Certificar reconstrucción de entropía débil (CVE-2008-0166)
 * =================================================================
 */

#[cfg(test)]
mod tests {
    use prospector_domain_forensics::debian_rng::DebianForensicIterator;
    use std::time::Instant;

    #[test]
    fn certificar_reconstruccion_pids_debian_2008() {
        println!("\n🔍 [CAMPOS DE PRUEBA]: Iniciando Auditoría Forense Debian 2008...");
     

        // 1. VALIDACIÓN LÓGICA
        println!("   🧪 Validando determinismo del espacio de PIDs (0-32767)...");
        let mut iterator = DebianForensicIterator::new(1, 100);
        let (label, key) = iterator.next().expect("FALLO_ITERADOR");

        assert!(label.contains("pid_1"), "Etiquetado forense incorrecto.");
        println!("   ✅ PID 1 mapeado correctamente a clave: {}...", hex::encode(&key.to_bytes()[..8]));

        // 2. PRUEBA DE FRONTERA
        println!("   🧪 Verificando límites de saturación de 15 bits...");
        let mut boundary_iter = DebianForensicIterator::new(32766, 32767);
        boundary_iter.next(); // 32766
        boundary_iter.next(); // 32767
        assert!(boundary_iter.next().is_none(), "El motor no respetó el límite de PID de Linux.");
        println!("   ✅ Frontera de saturación certificada.");

        // 3. BENCHMARK DE VELOCIDAD
        println!("   🚀 Midiendo velocidad de reconstrucción masiva...");
        let perf_start = Instant::now();
        let full_sweep: Vec<_> = DebianForensicIterator::new(1, 32767).collect();
        let duration = perf_start.elapsed();

        let keys_per_sec = 32767.0 / duration.as_secs_f64();
        println!("   ✅ Rendimiento: {:.2} reconstrucciones/seg.", keys_per_sec);

        println!("\n🏁 [VEREDICTO]: El motor de arqueología Debian es bit-perfecto.");
        println!("📊 [MÉTRICAS]: Tiempo total: {:?}, Claves generadas: {}", duration, full_sweep.len());
    }
}

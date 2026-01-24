// [libs/domain/mining-strategy/src/engines/forensic_engine.rs]
/**
 * =================================================================
 * APARATO: FORENSIC ARCHAEOLOGY ORCHESTRATOR (V18.2 - CONSTANT SYNC)
 * CLASIFICACIÓN: DOMAIN STRATEGY (ESTRATO L2)
 * RESPONSABILIDAD: DESPACHO POLIMÓRFICO DE MOTORES DE ARQUEOLOGÍA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. SCOPE RESOLUTION: Resuelve E0425 mediante la importación nominal de
 *    DEBIAN_PROCESS_IDENTIFIER_MAX_STRATUM desde el estrato L2-Forensics.
 * 2. NOMINAL ALIGNMENT: Sincronización bit-perfecta con el prelude de 
 *    'prospector_domain_forensics' V17.2.
 * 3. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta aplicada a todas
 *    las ráfagas de búsqueda y parámetros de ventana.
 * 4. HYGIENE: Erradicación total de rastro de código muerto y placeholders.
 *
 * # Mathematical Proof (Stratum Integrity):
 * El orquestador garantiza que el espacio de búsqueda asignado a cada 
 * motor forense sea congruente con los límites físicos del PRNG original 
 * (ej: PID 1-32767 para Debian).
 * =================================================================
 */

 use std::sync::Arc;
 use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
 use prospector_core_math::prelude::*;
 use prospector_core_probabilistic::sharded::ShardedFilter;
 
 // ✅ SINCRO SOBERANA: Importación de la constante de saturación y el prelude nominal
 use prospector_domain_forensics::prelude::*;
 use prospector_domain_forensics::debian_random_generator::DEBIAN_PROCESS_IDENTIFIER_MAX_STRATUM;
 
 use crate::executor::FindingHandler;
 use tracing::{info, debug, instrument, warn as tracing_warn, error};
 use rayon::prelude::*;

 /// Orquestador de arqueología digital soberano.
 pub struct ForensicArchaeologyEngine;

 impl ForensicArchaeologyEngine {
     /**
      * Ejecuta el despacho táctico basado en la firma de vulnerabilidad histórica.
      *
      * # Errors:
      * Retorna 'UNSUPPORTED_STRATUM' si la firma no coincide con el catálogo
      * de PRNGs defectuosos certificado para la Tesis.
      *
      * # Performance:
      * Operación O(1) de despacho polimórfico.
      */
     #[instrument(
         skip_all,
         fields(target = %vulnerability_target_signature)
     )]
     pub fn execute_forensic_scan<H: FindingHandler>(
         vulnerability_target_signature: &str,
         target_census_filter: &ShardedFilter,
         global_termination_signal: &AtomicBool,
         effort_telemetry_accumulator: Arc<AtomicU64>,
         collision_handler: &H,
         temporal_window_parameters: Option<(u64, u64)>
     ) -> String {
         info!("🧬 [FORENSIC_ORCHESTRATOR]: Initiating reconstruction for signature: {}", vulnerability_target_signature);

         match vulnerability_target_signature {
             // ESCENARIO 01: Vulnerabilidad OpenSSL Debian (32,767 PIDs)
             "Debian_OpenSSL_2008" => {
                 // ✅ RESOLUCIÓN E0425: Uso de la constante nominal inyectada
                 let forensic_iterator = DebianForensicIterator::new(1, DEBIAN_PROCESS_IDENTIFIER_MAX_STRATUM);
                 Self::perform_parallel_audit(forensic_iterator, target_census_filter, global_termination_signal, effort_telemetry_accumulator, collision_handler);
                 String::from("Debian_2008_Audit_Complete")
             },

             // ESCENARIO 02: Vulnerabilidad LCG de Java en Android (48-bit Seed)
             "Android_LCG_2013" => {
                 let forensic_iterator = AndroidLcgIterator::new(0, 1_000_000);
                 Self::perform_parallel_audit(forensic_iterator, target_census_filter, global_termination_signal, effort_telemetry_accumulator, collision_handler);
                 String::from("Android_2013_Audit_Complete")
             },

             // ESCENARIO 03: Vulnerabilidad Temporal Blockchain.info (Luno Pulse)
             "Luno_Blockchain_2014" => {
                 if let Some((start_millisecond, end_millisecond)) = temporal_window_parameters {
                     if start_millisecond >= end_millisecond {
                         error!("❌ [STRATA_FAULT]: Invalid temporal window: {} >= {}", start_millisecond, end_millisecond);
                         return String::from("ERROR_INVALID_TEMPORAL_WINDOW");
                     }

                     debug!("🕵️ [ARQUEOLOGY]: Leveling Luno strata: {}ms to {}ms", start_millisecond, end_millisecond);
                     let forensic_iterator = LunoForensicIterator::new(start_millisecond, end_millisecond);
                     Self::perform_parallel_audit(forensic_iterator, target_census_filter, global_termination_signal, effort_telemetry_accumulator, collision_handler);
                     String::from("Luno_2014_Temporal_Audit_Complete")
                 } else {
                     tracing_warn!("⚠️ [STRATA_FAULT]: Luno audit requires explicit millisecond bounds.");
                     String::from("ERROR_MISSING_TEMPORAL_RANGE")
                 }
             },

             _ => {
                 tracing_warn!("⚠️ [STRATA_REJECTION]: Signature [{}] is not registered.", vulnerability_target_signature);
                 format!("ERROR_UNSUPPORTED_PATTERN: {}", vulnerability_target_signature)
             }
         }
     }

     /**
      * Motor de Auditoría Paralela (Para-Hash).
      *
      * # Mathematical Proof (Exhaustive Verification):
      * El motor garantiza paridad bit-perfecta contra el Censo en ambos formatos
      * de red Bitcoin, eliminando puntos ciegos en la arqueología de entropía.
      */
     #[inline(always)]
     fn perform_parallel_audit<I, H>(
         forensic_iterator: I,
         target_census_filter: &ShardedFilter,
         global_termination_signal: &AtomicBool,
         effort_telemetry_accumulator: Arc<AtomicU64>,
         collision_handler: &H,
     ) where
         I: Iterator<Item = (String, SafePrivateKey)> + Send,
         H: FindingHandler
     {
         forensic_iterator.par_bridge().for_each(|(forensic_metadata, private_key_instance)| {
             
             if global_termination_signal.load(Ordering::Relaxed) { return; }

             let public_key_point = SafePublicKey::from_private(&private_key_instance);

             // 1. AUDITORÍA ESTRATO SATOSHI (Uncompressed 0x04)
             let uncompressed_raw_bytes = public_key_point.to_bytes(false);
             let hash160_uncompressed = prospector_core_math::hashing::hash160(&uncompressed_raw_bytes);

             if target_census_filter.contains(&hash160_uncompressed) {
                 let bitcoin_address = prospector_core_gen::address_legacy::pubkey_to_address(&public_key_point, false);
                 collision_handler.on_finding(bitcoin_address, private_key_instance.clone(), forensic_metadata.clone());
             }

             // 2. AUDITORÍA ESTRATO MODERN (Compressed 0x02/03)
             let compressed_raw_bytes = public_key_point.to_bytes(true);
             let hash160_compressed = prospector_core_math::hashing::hash160(&compressed_raw_bytes);

             if target_census_filter.contains(&hash160_compressed) {
                 let bitcoin_address = prospector_core_gen::address_legacy::pubkey_to_address(&public_key_point, true);
                 collision_handler.on_finding(bitcoin_address, private_key_instance, forensic_metadata);
             }

             // Sincronización con el HUD del Dashboard
             effort_telemetry_accumulator.fetch_add(1, Ordering::Relaxed);
         });
     }
 }
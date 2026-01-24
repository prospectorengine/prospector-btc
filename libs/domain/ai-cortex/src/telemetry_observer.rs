/*!
 * APARATO: TELEMETRY OBSERVER
 * RESPONSABILIDAD: Transformación de logs crudos en percepciones cognitivas.
 */
 use crate::lib::TelemetrySnapshot;
 use prospector_domain_models::telemetry::SystemMetrics;

 pub struct TelemetryObserver;

 impl TelemetryObserver {
     pub fn crystallize_snapshot(metrics: &SystemMetrics, thermal: f32, load: f32) -> TelemetrySnapshot {
         TelemetrySnapshot {
             current_hashrate: metrics.cumulative_global_hashrate,
             cpu_temperature_celsius: thermal,
             cpu_load_percentage: load,
             timestamp_ms: metrics.timestamp_ms,
         }
     }
 }

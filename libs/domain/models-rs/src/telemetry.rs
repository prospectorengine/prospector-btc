// [libs/domain/models-rs/src/telemetry.rs]
/*!
 * =================================================================
 * APARATO: SOVEREIGN TELEMETRY CONTRACT (V47.0 - VISION SYNC)
 * CLASIFICACIÓN: DOMAIN MODELS (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DE SEÑALES DE ALTA DENSIDAD Y BIOMETRÍA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. VISUAL TUNNEL SYNC: Inyecta 'snapshot_base64_data' en el evento 'vr',
 *    permitiendo el transporte de frames de video-proxy hacia el Dashboard.
 * 2. HARDWARE AWARENESS: Mantiene el campo 'supports_avx2' para la
 *    certificación de nodos ELITE en el Panóptico.
 * 3. TYPE SOVEREIGNTY: Sello bit-perfect para 'typeshare', garantizando que
 *    TypeScript genere interfaces con soporte para ráfagas de imagen.
 * 4. HYGIENE: Nomenclatura nominal absoluta y rastro forense inalterado.
 *
 * # Mathematical Proof (Neural Consistency):
 * La inclusión del payload base64 en la unión discriminada garantiza que
 * el Neural Link sea el único canal de verdad visual, eliminando la
 * necesidad de peticiones HTTP GET redundantes para imágenes.
 * =================================================================
 */

 use serde::{Deserialize, Serialize};
 use typeshare::typeshare;
 use crate::work::AuditReport;
 use std::collections::HashMap;

 /// Definición del nivel de integridad de un aparato de infraestructura.
 #[typeshare]
 #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
 #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
 pub enum IntegrityStatus {
     /// El subsistema opera dentro de los parámetros nominales.
     Operational,
     /// El subsistema presenta latencia o errores no fatales.
     Degraded,
     /// El subsistema ha colapsado o es inalcanzable.
     Critical,
 }

 /// Telemetría detallada de hardware de un nodo individual.
 #[typeshare]
 #[derive(Debug, Clone, Serialize, Deserialize, Default)]
 pub struct NodeHardwareMetrics {
     /// Frecuencia actual del procesador en Megahercios.
     pub cpu_frequency_megahertz: u32,
     /// Porcentaje de carga de procesamiento (0.0 - 100.0).
     pub cpu_load_percentage: f32,
     /// Temperatura del núcleo en grados Celsius.
     pub cpu_temperature_celsius: f32,
     /// Uso actual de memoria volátil en Megabytes.
     #[typeshare(serialized_as = "number")]
     pub ram_usage_megabytes: u64,
     /// Indica si el silicio está bajo Throttling térmico.
     pub is_thermal_throttling_active: bool,
     /// Identificador de soporte para instrucciones AVX2 (SIMD 4-Way).
     pub supports_avx2: bool,
 }

 /// Métricas agregadas del enjambre (System Pulse).
 #[typeshare]
 #[derive(Debug, Clone, Serialize, Deserialize, Default)]
 pub struct SystemMetrics {
     /// Conteo total de nodos reportando latido activo.
     pub active_nodes_count: u32,
     /// Potencia de búsqueda acumulada en Hashes por segundo.
     #[typeshare(serialized_as = "number")]
     pub cumulative_global_hashrate: u64,
     /// Volumen de misiones actualmente en fase de ejecución.
     pub active_missions_in_flight: u32,
     /// Marca de tiempo milimétrica (Epoch MS) para el cálculo de latencia neural.
     #[typeshare(serialized_as = "number")]
     pub timestamp_ms: u64,
 }

 /// Registro de navegación del automatizador C2 (Playwright/Sentinel).
 #[typeshare]
 #[derive(Debug, Clone, Serialize, Deserialize)]
 pub struct ProvisioningLog {
     /// Índice de la instancia de navegación.
     pub node_index: u32,
     /// Contenido técnico de la traza de aprovisionamiento.
     pub message: String,
     /// Nivel de log: "INFO", "WARN", "CRITICAL".
     pub level: String,
     /// Timestamp ISO 8601 del evento.
     pub timestamp: String,
 }

 /// Estado del Escudo de Baneo basado en densidad de red.
 #[typeshare]
 #[derive(Debug, Clone, Serialize, Deserialize)]
 pub struct BanShieldStatus {
     /// Volumen total de identidades custodiadas en la Bóveda ZK.
     pub identities_in_vault: u32,
     /// Límite de nodos concurrentes calculado (Identidades * 3).
     pub safe_node_capacity: u32,
     /// Autorización de mando para nuevos despliegues.
     pub is_ignition_authorized: bool,
     /// Justificación técnica en caso de veto de ignición.
     pub restriction_reason: Option<String>,
 }

 /// Estructura de Log Unificado para el Proyecto Panóptico.
 #[typeshare]
 #[derive(Debug, Clone, Serialize, Deserialize)]
 pub struct SystemLog {
     /// Identificador unívoco del registro (UUID v4).
     pub id: String,
     /// Timestamp ISO 8601 de la emisión.
     pub timestamp: String,
     /// Capa de origen: "L1_CORE", "L3_ORCH", "L6_OPS", etc.
     pub stratum: String,
     /// Severidad semántica del evento.
     pub severity: String,
     /// Mensaje descriptivo para el operador.
     pub message: String,
     /// Metadatos técnicos estructurados para análisis por IA.
     pub metadata: Option<HashMap<String, serde_json::Value>>,
     /// ID de rastro para correlación distribuida.
     pub trace_id: Option<String>,
 }

 /// Reporte de integridad de subsistemas internos (Auditor, Inspector, Bridge).
 #[typeshare]
 #[derive(Debug, Clone, Serialize, Deserialize)]
 pub struct SystemIntegrityReport {
     /// Nombre nominal del aparato de infraestructura.
     pub apparatus_name: String,
     /// Estado de salud reportado.
     pub status: IntegrityStatus,
     /// Métricas específicas del diagnóstico.
     pub metrics: HashMap<String, serde_json::Value>,
     /// Fecha y hora de la detección.
     pub detected_at_timestamp: String,
 }

 /// UNIÓN DISCRIMINADA SOBERANA (RealTimeEvent)
 /// Orquestador central de todas las señales hacia el Dashboard Zenith.
 #[typeshare]
 #[derive(Debug, Clone, Serialize, Deserialize)]
 #[serde(tag = "t", content = "p")]
 pub enum RealTimeEvent {
     /// System Pulse: Pulso biométrico del enjambre.
     #[serde(rename = "sp")]
     SystemPulseUpdate(SystemMetrics),

     /// Audit Certified: Misión sellada con éxito.
     #[serde(rename = "ac")]
     MissionAuditCertified(AuditReport),

     /// Provisioning Trace: Trazas de ignición del automatizador L6.
     #[serde(rename = "pl")]
     ProvisioningTrace(ProvisioningLog),

     /// Ban Shield: Actualización del estado de protección de identidades.
     #[serde(rename = "bs")]
     BanShieldUpdate(BanShieldStatus),

     /// Integrity Report: Salud de estratos persistentes y red.
     #[serde(rename = "ir")]
     InfrastructureIntegrityReport(SystemIntegrityReport),

     /// Cryptographic Collision: Alerta máxima de hallazgo de clave.
     #[serde(rename = "cc")]
     CryptographicCollisionAlert {
         /// Dirección Bitcoin colisionada.
         target_bitcoin_address: String,
         /// Nodo que realizó el descubrimiento.
         discovery_node: String,
     },

     /// Node Visual Frame: Señal de refresco de captura visual.
     /// ✅ NIVELADO V47.0: Se añade el rastro de imagen base64.
     #[serde(rename = "vr")]
     NodeVisualFrameReady {
         /// ID del trabajador.
         worker_identifier: String,
         /// Estado operativo (running, captcha, error).
         operational_status: String,
         /// Datos de la imagen en formato Data URL Base64.
         snapshot_base64_data: String,
         /// Timestamp del servidor para sincronía visual.
         #[typeshare(serialized_as = "number")]
         system_timestamp: u64,
     },

     /// System Log: Entrada en el flujo unificado del Panóptico.
     #[serde(rename = "sl")]
     SystemLogEmission(SystemLog),

     /// Archival Drift: Deriva detectada entre el Ledger Táctico y Estratégico.
     #[serde(rename = "ad")]
     ArchivalDriftDetected {
         /// Conteo de misiones en diferido.
         #[typeshare(serialized_as = "number")]
         drift_gap_count: u64,
         /// Volumen total en el Ledger Táctico.
         #[typeshare(serialized_as = "number")]
         total_tactical_count: u64,
     },
 }

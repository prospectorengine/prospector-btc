// [libs/domain/models-rs/src/work.rs]
/*!
 * =================================================================
 * APARATO: WORK DOMAIN MODELS (V152.0 - PLAYGROUND ENABLED)
 * CLASIFICACIÓN: DOMAIN MODELS (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DE CONTRATOS SOBERANOS DE MISIÓN
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. SMOKE-TEST CAPABILITY: Inyecta la variante 'Playground' en el motor
 *    polimórfico para validación de handshakes sin consumo de CPU real.
 * 2. SILICON EVIDENCE: Mantiene la firma 'hardware_acceleration_signature'
 *    para certificar el uso de ráfagas SIMD/ADX en el reporte final.
 * 3. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta en todos los campos
 *    (mb -> megabytes, id -> identifier).
 * 4. HYGIENE: Erradicación de placeholders y cumplimiento estricto de Typeshare.
 *
 * # Mathematical Proof (Audit Immutability):
 * El AuditReport actúa como el bloque de sellado de una misión. Al incluir
 * el 'audit_footprint_checkpoint' y la firma de hardware, se garantiza
 * que el esfuerzo computacional es auditable y reproducible bit-perfecto.
 * =================================================================
 */

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// Clasificación geológica de los estratos de direcciones UTXO para la búsqueda.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TargetStrata {
    /// Direcciones del periodo 2009-2010 (Satoshi Standard).
    SatoshiEra,
    /// Direcciones vulnerables por fallos conocidos de PRNG (2011-2014).
    VulnerableLegacy,
    /// Set completo de direcciones P2PKH con balance positivo.
    StandardLegacy,
    /// Combinación de todos los estratos para auditoría masiva.
    FullTacticalSet,
}

/// Definición polimórfica de la Estrategia de Búsqueda Criptográfica.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "strategy_type", content = "parameters")]
pub enum SearchStrategy {
    /// Barrido lineal en el espacio de claves Jacobianas.
    Sequential {
        start_index_hexadecimal: String,
        end_index_hexadecimal: String,
    },
    /// Reconstrucción forense de OpenSSL 0.9.8h sobre Windows XP.
    SatoshiWindowsXpForensic {
        scenario_template_identifier: String,
        #[typeshare(serialized_as = "number")]
        uptime_seconds_start: u64,
        #[typeshare(serialized_as = "number")]
        uptime_seconds_end: u64,
        #[typeshare(serialized_as = "number")]
        hardware_clock_frequency: u64,
    },
    /// Explotación de la vulnerabilidad LCG de Java (CVE-2013-7372).
    AndroidLcgForensic {
        #[typeshare(serialized_as = "number")]
        seed_range_start: u64,
        #[typeshare(serialized_as = "number")]
        seed_range_end: u64,
    },
    /// Resolución de logaritmo discreto mediante Pollard's Lambda (Kangaroo).
    KangarooLambda {
        target_public_key_hexadecimal: String,
        #[typeshare(serialized_as = "number")]
        range_width_max: u64,
    },
    /// Auditoría basada en diccionarios y patrones humanos (Brainwallets).
    Dictionary {
        dataset_resource_locator: String,
        #[typeshare(serialized_as = "number")]
        processing_batch_size: usize,
    },
    /// ✅ NUEVO: Modo de validación de Handshake y Telemetría.
    /// No realiza cómputo pesado; simula una ráfaga para certificar el enlace neural.
    Playground {
        #[typeshare(serialized_as = "number")]
        target_mock_iterations: u64,
        diagnostic_seed: String,
    },
}

/// Orden de Trabajo soberana despachada por el Orquestador al Enjambre.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOrder {
    /// Identificador único universal de la misión de auditoría.
    pub job_mission_identifier: String,
    /// Tiempo de concesión antes de que la misión sea reclamada por el servicio Reaper.
    #[typeshare(serialized_as = "number")]
    pub lease_duration_seconds: u64,
    /// Configuración técnica del motor de búsqueda estratégica.
    pub strategy: SearchStrategy,
    /// Estrato de datos UTXO (Censo) objetivo de la misión.
    pub required_strata: TargetStrata,
}

/// Reporte inmutable de certificación de misión finalizada.
/// Constituye la prueba física de la auditoría para la Tesis Doctoral.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    /// Identificador de la misión certificada.
    pub job_mission_identifier: String,
    /// Identificador del nodo que completó el cómputo.
    pub worker_node_identifier: String,
    /// Volumen total de llaves auditadas (Representado como String para BigInt L5).
    pub total_wallets_audited: String,
    /// Duración física del cómputo en milisegundos.
    #[typeshare(serialized_as = "number")]
    pub execution_duration_milliseconds: u64,
    /// Estado final del proceso (ej: "completed", "halted_by_preemption").
    pub final_mission_status: String,
    /// Último escalar procesado o firma de estado del pool (Punto de reanudación).
    pub audit_footprint_checkpoint: String,
    /// Marca de tiempo UTC de la cristalización del reporte.
    pub completed_at_timestamp: String,
    /// Hashes por milisegundo alcanzados durante la ráfaga.
    pub average_computational_efficiency: f64,
    /// Firma técnica del hardware utilizado (ej: ELITE_SIMD_ADX).
    pub hardware_acceleration_signature: String,
}

/// Payload de solicitud de misión (Handshake Táctico Worker -> Server).
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionRequestPayload {
    /// ID unívoco del nodo solicitante.
    pub worker_id: String,
    /// Telemetría de capacidad física para el balanceo inteligente de carga.
    pub hardware_capacity: NodeHardwareCapacity,
}

/// Telemetría de capacidad física y silicio del nodo solicitante.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeHardwareCapacity {
    /// Memoria RAM física disponible en el contenedor.
    #[typeshare(serialized_as = "number")]
    pub ram_available_megabytes: u64,
    /// Número de núcleos lógicos detectados por el sistema operativo.
    pub cpu_cores: u32,
    /// Flag de soporte para instrucciones vectoriales avanzadas (AVX2/ADX).
    pub supports_avx2: bool,
}

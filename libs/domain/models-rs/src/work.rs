// [libs/domain/models-rs/src/work.rs]
/*!
 * =================================================================
 * APARATO: WORK DOMAIN MODELS (V153.0 - ATOMIC CONSTRUCTORS)
 * CLASIFICACIÓN: DOMAIN MODELS (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DE CONTRATOS SOBERANOS DE MISIÓN
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. ENCAPSULATED METRICS: Inyección de métodos 'calculate_from_raw' en
 *    NodeHardwareCapacity. Abstrae la física de la RAM del handshake.
 * 2. CONTRACT HARDENING: Garantiza que 'ram_available_megabytes' sea la
 *    única vía de reporte, sanando el error de Severidad 8 del Worker.
 * 3. BIGINT BRIDGE COMPLIANCE: Mantiene 'total_wallets_audited' como String
 *    para la paridad bit-perfecta con el Dashboard L5 (JavaScript BigInt).
 * 4. NOMINAL PURITY: Erradicación total de lógica de cálculo en DTOs.
 *
 * # Mathematical Proof (Byte-to-MB Conversion):
 * Se utiliza una división saturante (saturating_div) con el factor
 * 1_048_576 (2^20) para garantizar que el reporte de memoria sea
 * determinista incluso en arquitecturas de memoria limitada.
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
    /// Modo de validación de Handshake y Telemetría.
    /// Simulación táctica para certificar el enlace neural L3-L5.
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
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    pub job_mission_identifier: String,
    pub worker_node_identifier: String,
    pub total_wallets_audited: String,
    #[typeshare(serialized_as = "number")]
    pub execution_duration_milliseconds: u64,
    pub final_mission_status: String,
    pub audit_footprint_checkpoint: String,
    pub completed_at_timestamp: String,
    pub average_computational_efficiency: f64,
    /// Firma técnica del hardware utilizado (ej: ELITE_SIMD_ADX).
    pub hardware_acceleration_signature: String,
}

/// Payload de solicitud de misión (Handshake Táctico Worker -> Server).
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionRequestPayload {
    pub worker_id: String,
    pub hardware_capacity: NodeHardwareCapacity,
}

/// Telemetría de capacidad física y silicio del nodo solicitante.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeHardwareCapacity {
    /// Memoria RAM física disponible en el contenedor (en Megabytes).
    #[typeshare(serialized_as = "number")]
    pub ram_available_megabytes: u64,
    /// Número de núcleos lógicos detectados por el sistema operativo.
    pub cpu_cores: u32,
    /// Flag de soporte para instrucciones vectoriales avanzadas (AVX2/ADX).
    pub supports_avx2: bool,
}

impl NodeHardwareCapacity {
    /// Factor de conversión binaria (1024 * 1024).
    const BYTES_TO_MB: u64 = 1_048_576;

    /**
     * Constructor de élite: Abstrae el cálculo de métricas de silicio.
     *
     * @param raw_ram_bytes Memoria disponible en bytes crudos.
     * @param cpu_cores Conteo de hilos del SO.
     * @param avx2_flag Estado de detección de registros YMM.
     */
    pub fn calculate_from_raw(
        raw_ram_bytes: u64,
        cpu_cores: u32,
        avx2_flag: bool
    ) -> Self {
        Self {
            ram_available_megabytes: raw_ram_bytes.saturating_div(Self::BYTES_TO_MB),
            cpu_cores,
            supports_avx2: avx2_flag,
        }
    }
}

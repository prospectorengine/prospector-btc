// [libs/domain/models-rs/src/work.rs]
/*!
 * =================================================================
 * APARATO: WORK DOMAIN MODELS (V151.0 - PRODUCTION MASTER)
 * CLASIFICACIÓN: DOMAIN MODELS (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DE CONTRATOS SOBERANOS DE MISIÓN
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. SILICON EVIDENCE: Inyecta 'hardware_acceleration_signature' en el
 *    AuditReport para certificar el uso de ráfagas SIMD/ADX.
 * 2. PRODUCTION PURITY: Erradicación total de placeholders y comentarios
 *    de depuración. Datos reales para despliegue Cloud-Native.
 * 3. TYPESHARE SYNERGY: Mapeo determinista de tipos numéricos para el
 *    Frontend Next.js 16, garantizando paridad en la Tríada.
 * 4. NOMINAL PRECISION: Nomenclatura nominal absoluta en todos los campos.
 *
 * # Mathematical Proof (Audit Immutability):
 * El AuditReport actúa como el bloque de sellado de una misión. Al incluir
 * el 'audit_footprint_checkpoint' y la firma de hardware, se garantiza
 * que el esfuerzo computacional es auditable y reproducible bit-perfecto.
 * =================================================================
 */

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// Clasificación geológica de los estratos de direcciones UTXO.
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
    /// Resolución de logaritmo discreto mediante Pollard's Lambda.
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
}

/// Orden de Trabajo soberana despachada por el Orquestador.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOrder {
    /// Identificador único universal de la misión.
    pub job_mission_identifier: String,
    /// Tiempo de concesión antes de que la misión se considere huérfana.
    #[typeshare(serialized_as = "number")]
    pub lease_duration_seconds: u64,
    /// Configuración técnica del motor de búsqueda.
    pub strategy: SearchStrategy,
    /// Estrato de datos UTXO objetivo.
    pub required_strata: TargetStrata,
}

/// Reporte inmutable de certificación de misión finalizada.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    pub job_mission_identifier: String,
    pub worker_node_identifier: String,
    /// Volumen total de llaves auditadas (Representado como String para BigInt L5).
    pub total_wallets_audited: String,
    /// Duración física del cómputo.
    #[typeshare(serialized_as = "number")]
    pub execution_duration_milliseconds: u64,
    pub final_mission_status: String,
    /// Último escalar procesado (Punto de reanudación).
    pub audit_footprint_checkpoint: String,
    pub completed_at_timestamp: String,
    /// Hashes por milisegundo alcanzados.
    pub average_computational_efficiency: f64,
    /// Firma técnica del hardware utilizado (SIMD, ADX, Software).
    pub hardware_acceleration_signature: String,
}

/// Payload de solicitud de misión (Handshake Táctico).
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionRequestPayload {
    pub worker_id: String,
    pub hardware_capacity: NodeHardwareCapacity,
}

/// Telemetría de capacidad física del nodo solicitante.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeHardwareCapacity {
    #[typeshare(serialized_as = "number")]
    pub ram_available_mb: u64,
    pub cpu_cores: u32,
    pub supports_avx2: bool,
}

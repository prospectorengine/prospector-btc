// [libs/domain/models-rs/src/lab.rs]
/**
 * =================================================================
 * APARATO: LAB PROVING DOMAIN MODELS (V16.0 - ARCHITECTURAL SYNC)
 * CLASIFICACIÓN: DOMAIN MODELS (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DE LA GRAMÁTICA DE PRUEBAS Y VERDAD
 *
 * VISION HIPER-HOLÍSTICA:
 * Centraliza los modelos de reporte de integridad. Resuelve la
 * dependencia circular al extraer los DTOs de reporte del Handler.
 * =================================================================
 */

use serde::{Deserialize, Serialize};
use typeshare::typeshare;


// --- ESTRATO 1: EL INTERCEPTOR ---

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyEntropyPayload {
    pub entropy_vector: String,
    pub vector_type: String,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntropyResult {
    pub derived_bitcoin_address: String,
    pub derived_wallet_import_format: String,
    pub is_target_collision: bool,
    pub matched_scenario_name: Option<String>,
}

// --- ESTRATO 2: REALITY CHECK ---

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkRealityData {
    pub final_balance_satoshis: u64,
    pub total_received_satoshis: u64,
    pub confirmed_transaction_count: u32,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifiedVectorAuditReport {
    pub vector_identifier: u32,
    pub source_passphrase: String,
    pub derived_wallet_import_format: String,
    pub derived_bitcoin_address: String,
    pub mathematical_integrity_verified: bool,
    pub network_reality_data: Option<NetworkRealityData>,
}

// --- ESTRATO 3: PROVING GROUNDS (CERTIFICACIÓN) ---

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProvingVerdict {
    GoldMaster,
    Stable,
    Degraded,
    Failed,
}

/**
 * Reporte Maestro de Certificación.
 * Consumido por el Prover para informar al Orquestador L3.
 */
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvingReport {
    pub stratum: String,
    pub test_name: String,
    pub verdict: ProvingVerdict,
    pub forensic_log: String,
    pub metrics: serde_json::Value,
    pub environment: String,
    pub timestamp: String,
}

impl ProvingReport {
    pub fn build_report(
        stratum: &str,
        name: &str,
        verdict: ProvingVerdict,
        log: String,
        metrics: serde_json::Value,
        env: String,
    ) -> Self {
        Self {
            stratum: stratum.to_string(),
            test_name: name.to_string(),
            verdict,
            forensic_log: log,
            metrics,
            environment: env,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

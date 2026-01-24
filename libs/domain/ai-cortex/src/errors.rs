use thiserror::Error;
use typeshare::typeshare;
use serde::{Deserialize, Serialize};

#[typeshare]
#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum CortexError {
    #[error("[L9_CORTEX_FAULT]: INSUFFICIENT_TELEMETRY_DATA")]
    IncompleteStrata,
    #[error("[L9_CORTEX_FAULT]: INFERENCE_DIVERGENCE_DETECTED")]
    InferenceDrift,
    #[error("[L9_CORTEX_FAULT]: OPTIMIZATION_VETOED_BY_NEXUS")]
    DirectiveRejected,
}

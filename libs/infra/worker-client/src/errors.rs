// INICIO DEL ARCHIVO [libs/infra/worker-client/src/errors.rs]
//! =================================================================
//! APARATO: WORKER CLIENT ERRORS (V10.4 - DOCS FIXED)
//! RESPONSABILIDAD: CATALOGACIÓN DE FALLOS DE COMUNICACIÓN
//! =================================================================

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("NETWORK_UNREACHABLE: Failed to connect to Command Center: {0}")]
    NetworkFault(#[from] reqwest::Error),

    #[error("IO_VAULT_FAULT: Disk access denied or full: {0}")]
    IoFault(#[from] std::io::Error),

    #[error("ENVELOPE_CORRUPTION: Failed to decode mission assignment: {0}")]
    DecodingFault(#[from] serde_json::Error),

    #[error("COMMAND_REJECTION: Server returned status {0}")]
    ServerRejection(String),

    #[error("IDENTITY_REVOKED: Session material invalid or expired")]
    Unauthorized,

    #[error("STRATUM_HYDRATION_FAILED: Multi-shard download collapsed")]
    HydrationFailed,
}
// FIN DEL ARCHIVO [libs/infra/worker-client/src/errors.rs]

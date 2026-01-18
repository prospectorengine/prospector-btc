
pub mod finding;
pub mod identity;
pub mod telemetry;
pub mod work;
pub mod worker;
pub mod scenario;
pub mod stratum;
pub mod lab;

pub use stratum::StratumManifest;
pub use finding::Finding;
pub use identity::{Identity, IdentityStatus, EncryptedIdentityPayload};
pub use telemetry::{RealTimeEvent, SystemMetrics, ProvisioningLog, BanShieldStatus};
pub use work::{WorkOrder, SearchStrategy, TargetStrata, AuditReport};
pub use worker::{WorkerHeartbeat, WorkerSnapshot, HardwareStats};
pub use scenario::SystemTemplateRegistry;

pub use lab::{VerifyEntropyPayload, EntropyResult, VerifiedVectorAuditReport};

pub mod academy;
pub use academy::{KnowledgeModule, DifficultyLevel, ModuleStatus};

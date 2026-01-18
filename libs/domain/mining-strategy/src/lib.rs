// INICIO DEL ARCHIVO [libs/domain/mining-strategy/src/lib.rs]
/*!
 * =================================================================
 * APARATO: STRATEGY DOMAIN HUB (V112.2 - EXPORTS FIXED)
 * =================================================================
 */

pub mod brainwallet;
pub mod combinatoric;
pub mod dictionary;
pub mod engines;
pub mod executor;
pub mod kangaroo;
pub mod forensic_auditor;

// --- RE-EXPORTACIONES SOBERANAS ---

// Executor y Handlers
pub use executor::{StrategyExecutor, FindingHandler};

// Motores Específicos (Corrección E0432)
pub use engines::sequential_engine::ProjectiveSequentialEngine;
pub use engines::satoshi_xp_engine::SatoshiWindowsXpForensicEngine;
pub use engines::android_lcg_engine::AndroidLcgForensicEngine;
pub use engines::dictionary_engine::EntropyDictionaryEngine;
pub use kangaroo::KangarooRunner;

// Utilidades
pub use brainwallet::phrase_to_private_key;
// FIN DEL ARCHIVO [libs/domain/mining-strategy/src/lib.rs]

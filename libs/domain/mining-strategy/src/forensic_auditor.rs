// INICIO DEL ARCHIVO [libs/domain/mining-strategy/src/forensic_auditor.rs]
/*!
 * =================================================================
 * APARATO: FORENSIC VECTOR AUDITOR (V12.0 - INTEGRITY FIELD ADDED)
 * CLASIFICACIÓN: DOMAIN STRATEGY (ESTRATO L2)
 * RESPONSABILIDAD: AUDITORÍA DE ESPECTRO COMPLETO
 * =================================================================
 */

use prospector_core_math::prelude::*;
use prospector_core_gen::address_legacy::pubkey_to_address;
use prospector_core_gen::wif::private_to_wif;
use crate::phrase_to_private_key;
use prospector_blockchain_client::{BitcoinNetworkUplinkClient, BitcoinAddressNetworkState};
use serde::{Serialize, Deserialize};
use futures::future::join_all;

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifiedVectorAuditReport {
    pub vector_id: u32,
    pub source_passphrase: String,
    // Resultados Duales
    pub address_compressed: String,
    pub address_uncompressed: String,
    pub wif_compressed: String,
    pub wif_uncompressed: String,

    // ✅ NUEVO CAMPO: Requerido por el contrato L3
    pub mathematical_integrity_verified: bool,

    // Telemetría de Red
    pub active_network_state: Option<BitcoinAddressNetworkState>,
    pub target_type_found: Option<String>,
}

pub struct ForensicVectorAuditor;

impl ForensicVectorAuditor {
    pub async fn execute_dataset_certification(
        input_vectors: Vec<(u32, String, String, String, String)>
    ) -> Vec<VerifiedVectorAuditReport> {
        let network_client = BitcoinNetworkUplinkClient::new();

        let audit_tasks = input_vectors.into_iter().map(|(id, _type, phrase, expected_wif, expected_addr)| {
            let client_ref = &network_client;
            async move {
                // 1. DERIVACIÓN SOBERANA
                let private_key = phrase_to_private_key(&phrase);
                let public_key = SafePublicKey::from_private(&private_key);

                // 2. GENERACIÓN DUAL
                let addr_comp = pubkey_to_address(&public_key, true);
                let wif_comp = private_to_wif(&private_key, true);

                let addr_uncomp = pubkey_to_address(&public_key, false);
                let wif_uncomp = private_to_wif(&private_key, false);

                // 3. VERIFICACIÓN DE INTEGRIDAD
                // Comparamos contra el vector esperado (Uncompressed o Compressed)
                let integrity_check = (addr_uncomp == expected_addr && wif_uncomp == expected_wif) ||
                                      (addr_comp == expected_addr && wif_comp == expected_wif);

                // 4. CONSULTA DE RED
                let (state_comp, state_uncomp) = futures::join!(
                    client_ref.fetch_bitcoin_address_activity(&addr_comp),
                    client_ref.fetch_bitcoin_address_activity(&addr_uncomp)
                );

                let (final_state, type_found) = if let Ok(s) = state_uncomp {
                    if s.confirmed_transaction_count > 0 {
                        (Some(s), Some("uncompressed".to_string()))
                    } else if let Ok(s_c) = state_comp {
                        (Some(s_c), Some("compressed".to_string()))
                    } else {
                        (Some(s), Some("uncompressed".to_string()))
                    }
                } else if let Ok(s) = state_comp {
                    (Some(s), Some("compressed".to_string()))
                } else {
                    (None, None)
                };

                VerifiedVectorAuditReport {
                    vector_id: id,
                    source_passphrase: phrase,
                    address_compressed: addr_comp,
                    address_uncompressed: addr_uncomp,
                    wif_compressed: wif_comp,
                    wif_uncompressed: wif_uncomp,
                    mathematical_integrity_verified: integrity_check, // ✅ Poblado
                    active_network_state: final_state,
                    target_type_found: type_found,
                }
            }
        });

        join_all(audit_tasks).await
    }
}
// FIN DEL ARCHIVO [libs/domain/mining-strategy/src/forensic_auditor.rs]

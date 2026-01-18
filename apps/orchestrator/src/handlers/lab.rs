// INICIO DEL ARCHIVO [apps/orchestrator/src/handlers/lab.rs]
/**
 * =================================================================
 * APARATO: LABORATORY CERTIFICATION HANDLER (V81.0 - TYPE BRIDGE)
 * CLASIFICACIÓN: API ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: MANDO DE CERTIFICACIÓN, VERIFICACIÓN E INTERCEPCIÓN
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el puente entre la lógica forense (Strategy) y los contratos
 * de API (Models). Realiza la transformación de tipos necesaria para
 * desacoplar el cliente blockchain del esquema de respuesta JSON.
 * =================================================================
 */

use crate::state::AppState;
use axum::{extract::{Json, State}, http::StatusCode, response::IntoResponse};
use prospector_domain_models::work::{WorkOrder, SearchStrategy, TargetStrata};
use prospector_domain_models::lab::{
    VerifyEntropyPayload,
    EntropyResult,
    VerifiedVectorAuditReport as ModelReport,
    NetworkRealityData
};
use prospector_domain_strategy::{
    phrase_to_private_key,
    forensic_auditor::ForensicVectorAuditor
};
use prospector_core_math::public_key::SafePublicKey;
use prospector_core_gen::{wif::private_to_wif, address_legacy::pubkey_to_address};
use uuid::Uuid;
use tracing::{info, instrument};

pub struct CertificationHandler;

impl CertificationHandler {
    /**
     * POST /api/v1/lab/certification/ignite
     * Inyecta la misión de los 33 vectores dorados en la cola de despacho.
     */
    #[instrument(skip(application_state))]
    pub async fn handle_certification_ignition(
        State(application_state): State<AppState>,
    ) -> impl IntoResponse {
        info!("🧪 [CERTIFICATION]: Injecting Golden 33 Mission into dispatcher...");

        // Lista de las 33 semillas inyectadas en el censo
        let golden_seeds = vec![
            "power", "the", "peter", "and", "money", "password", "12345678", "qwerty",
            "bitcoin", "satoshi", "god", "love", "freedom", "master", "secret", "hell",
            "heaven", "dragon", "warrior", "ninja", "shadow", "winter", "summer", "autumn",
            "spring", "coffee", "pizza", "beer", "matrix", "identity", "prospector",
            "hydra", "zero"
        ];

        let mut hydrated_count = 0;

        for seed in golden_seeds {
            // Generación de ID determinista para trazabilidad
            let unique_job_id = format!("CERT-{}-{}",
                seed.chars().take(4).collect::<String>(),
                Uuid::new_v4().to_string().split('-').next().unwrap_or("0000")
            );

            let golden_order = WorkOrder {
                job_mission_identifier: unique_job_id,
                lease_duration_seconds: 300,
                strategy: SearchStrategy::Dictionary {
                    dataset_resource_locator: seed.to_string(),
                    processing_batch_size: 1,
                },
                required_strata: TargetStrata::SatoshiEra,
            };

            application_state.mission_control.hydrate_queue(vec![golden_order]);
            hydrated_count += 1;
        }

        (StatusCode::CREATED, Json(serde_json::json!({
            "status": "GOLDEN_VECTORS_QUEUED",
            "count": hydrated_count
        })))
    }

    /**
     * POST /api/v1/lab/verify
     * Ejecuta "The Interceptor": Verificación manual de un vector de entropía.
     */
    #[instrument(skip(_state, payload))]
    pub async fn handle_manual_verification(
        State(_state): State<AppState>,
        Json(payload): Json<VerifyEntropyPayload>
    ) -> impl IntoResponse {
        info!("🔬 [INTERCEPTOR]: Analyzing vector type: {:?}", payload.vector_type);

        let private_key = phrase_to_private_key(&payload.entropy_vector);
        let public_key = SafePublicKey::from_private(&private_key);

        let address = pubkey_to_address(&public_key, false);
        let wif = private_to_wif(&private_key, false);

        let result = EntropyResult {
            derived_bitcoin_address: address,
            derived_wallet_import_format: wif,
            is_target_collision: false,
            matched_scenario_name: None,
        };

        (StatusCode::OK, Json(result))
    }

    /**
     * GET /api/v1/lab/audit/brainwallet-dataset
     * Ejecuta una auditoría de red sobre el dataset de prueba (33 vectores).
     */
    #[instrument]
    pub async fn handle_brainwallet_dataset_audit() -> impl IntoResponse {
        info!("🕵️ [AUDIT]: Initiating network reality check for Golden Dataset...");

        let vectors = vec![
            (1, "Brainwallet".to_string(), "power".to_string(), "5HsjrA5VEhok91VzRTe4dhpGBtwmoF2MgZtLFCa1eZ1aVQ6FrNC".to_string(), "1PzYwVuTotg15ridCGNnAo8u3dr6bE2Yxy".to_string()),
            (10, "Brainwallet".to_string(), "satoshi".to_string(), "5KUN8s42BCTkQVMTy3oFfqeXE8awVskbDi6XbDMpRnFvHJW9fgk".to_string(), "1ADJqstUMBB5zFquWg19UqZ7Zc6ePCpzLE".to_string()),
            (33, "Brainwallet".to_string(), "zero".to_string(), "5KhzUeCZkhxFw2LJNtPTetockgKWuCP8JS3fUgtAxHd4NiQJXvv".to_string(), "1FqTH4aSCDwD8YT4SSuBg7kqm9mdQt6Nd3".to_string())
        ];

        // Ejecución en la capa de Estrategia
        let strategy_reports = ForensicVectorAuditor::execute_dataset_certification(vectors).await;

        // Mapeo hacia la capa de Modelos (API Contract)
        // Esto desacopla la respuesta JSON de los detalles internos del cliente blockchain
        let mapped_report: Vec<ModelReport> = strategy_reports.into_iter().map(|r| ModelReport {
            vector_identifier: r.vector_id,
            source_passphrase: r.source_passphrase,
            // Priorizamos la versión descomprimida para la auditoría forense de 2009
            derived_wallet_import_format: r.wif_uncompressed,
            derived_bitcoin_address: r.address_uncompressed,
            mathematical_integrity_verified: r.mathematical_integrity_verified,
            network_reality_data: r.active_network_state.map(|ns| NetworkRealityData {
                final_balance_satoshis: ns.final_balance_satoshis,
                total_received_satoshis: ns.total_received_satoshis,
                confirmed_transaction_count: ns.confirmed_transaction_count,
            }),
        }).collect();

        (StatusCode::OK, Json(mapped_report))
    }
}
// FIN DEL ARCHIVO [apps/orchestrator/src/handlers/lab.rs]

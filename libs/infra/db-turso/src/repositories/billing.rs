// [libs/infra/db-turso/src/repositories/billing.rs]
/*!
 * =================================================================
 * APARATO: BILLING TACTICAL REPOSITORY (V1.2 - OWNERSHIP FIXED)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN DE CUOTAS Y PERSISTENCIA ACID DE ENERGÍA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. OWNERSHIP SYNC: Resuelve el error 'use of moved value' en 'cache_key'
 *    mediante clonación determinista en la ráfaga transaccional.
 * 2. ATOMIC DEDUCTION: Garantiza la integridad del balance ante fallos de red.
 * 3. HYGIENE: Eliminación de importaciones no utilizadas (error).
 * 4. NOMINAL PRECISION: Operaciones sobre 'value_text' para preservar f64.
 * =================================================================
 */

use crate::errors::DbError;
use crate::TursoClient;
use libsql::{params, Connection};
use tracing::{instrument, info, debug}; // ✅ REPARADO: 'error' eliminado

/// Identificador de tabla estratégica para el Strategic Relay.
const BILLING_STRATUM_TARGET: &str = "BILLING_CONSUMPTION";

/// Repositorio especializado en la gestión de créditos de energía del operador.
pub struct BillingRepository {
    database_client: TursoClient,
}

impl BillingRepository {
    /**
     * Construye una nueva instancia del repositorio inyectando el cliente de Turso.
     */
    pub fn new(client: TursoClient) -> Self {
        Self { database_client: client }
    }

    /**
     * Ejecuta el Protocolo de Consumo de Energía Criptográfica (Deducción Atómica).
     *
     * # Mathematical Proof (Double-Entry Atomic Seal):
     * Realiza la sustracción en 'system_state' y sella el evento en 'outbox_strategic'.
     *
     * # Errors:
     * - `DbError::TransactionError`: Si falla el túnel ACID.
     */
    #[instrument(skip(self, operator_identifier, credit_magnitude), fields(op = %operator_identifier))]
    pub async fn execute_credit_deduction_sequence(
        &self,
        operator_identifier: &str,
        credit_magnitude: f64,
        associated_mission_identifier: &str
    ) -> Result<(), DbError> {
        let database_connection: Connection = self.database_client.get_connection()?;
        let atomic_transaction = database_connection.transaction().await?;

        debug!("💳 [BILLING]: Initiating atomic deduction for operator [{}].", operator_identifier);

        // 1. PREPARACIÓN DE LLAVE SOBERANA
        let cache_key = format!("balance_{}", operator_identifier);

        // 2. ACTUALIZACIÓN DEL CACHÉ LOCAL (L3 Táctico)
        let update_cache_sql = "
            UPDATE system_state
            SET value_text = CAST((CAST(value_text AS REAL) - ?2) AS TEXT),
                updated_at = CURRENT_TIMESTAMP
            WHERE key = ?1
        ";

        // ✅ REPARACIÓN CRÍTICA: Clonamos cache_key aquí para que esté disponible en el INSERT posterior
        if atomic_transaction.execute(update_cache_sql, params![cache_key.clone(), credit_magnitude]).await? == 0 {
            // Caso de borde: Si la cuenta no tiene registro, inicializamos el estrato.
            atomic_transaction.execute(
                "INSERT OR IGNORE INTO system_state (key, value_text) VALUES (?1, '100.0')",
                params![cache_key] // Aquí se consume finalmente la propiedad de la llave
            ).await?;
        }

        // 3. INYECCIÓN EN EL OUTBOX (Sincronía Motor B)
        let outbox_payload = serde_json::json!({
            "operator_id": operator_identifier,
            "credit_delta": -credit_magnitude,
            "mission_id": associated_mission_identifier,
            "timestamp_utc": chrono::Utc::now().to_rfc3339()
        });

        atomic_transaction.execute(
            "INSERT INTO outbox_strategic (outbox_identifier, payload_json, target_stratum, status) VALUES (?1, ?2, ?3, 'pending')",
            params![
                uuid::Uuid::new_v4().to_string(),
                outbox_payload.to_string(),
                BILLING_STRATUM_TARGET
            ]
        ).await?;

        // 4. SELLADO DEFINITIVO
        atomic_transaction.commit().await?;

        info!("✅ [BILLING_SYNC]: Energy secure deduction finalized for [{}].", operator_identifier);
        Ok(())
    }

    /**
     * Recupera el balance de créditos del estrato táctico.
     *
     * # Performance: O(1) vía Index Scan.
     */
    pub async fn get_cached_balance(&self, operator_identifier: &str) -> Result<f64, DbError> {
        let database_connection = self.database_client.get_connection()?;
        let cache_key = format!("balance_{}", operator_identifier);

        let mut query_results = database_connection.query(
            "SELECT value_text FROM system_state WHERE key = ?1",
            params![cache_key]
        ).await?;

        if let Some(data_row) = query_results.next().await? {
            let balance_string: String = data_row.get(0)?;
            balance_string.parse::<f64>().map_err(|_| {
                DbError::MappingError("INVALID_CREDIT_FORMAT_IN_LEDGER".into())
            })
        } else {
            Ok(0.0)
        }
    }

    /**
     * Sincroniza el balance local tras una recarga estratégica (Motor B -> A).
     */
    pub async fn sync_local_balance(&self, operator_identifier: &str, new_total: f64) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;
        let cache_key = format!("balance_{}", operator_identifier);

        database_connection.execute(
            "INSERT INTO system_state (key, value_text) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value_text = excluded.value_text, updated_at = CURRENT_TIMESTAMP",
            params![cache_key, new_total.to_string()]
        ).await?;

        Ok(())
    }
}

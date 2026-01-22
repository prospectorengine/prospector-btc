// [libs/infra/db-turso/src/repositories/scenarios.rs]
/**
 * =================================================================
 * APARATO: TEST SCENARIO REPOSITORY (V2.0 - SYNCHRONIZED MASTER)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: PERSISTENCIA Y AUDITORÍA DE VECTORES DE PRUEBA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. INDEX PARITY: Corrige el desvío de índices entre el esquema L3
 *    y el mapeo de Rust, asegurando la integridad del Interceptor.
 * 2. EXPLICIT SQL: Sustitución de 'SELECT *' por proyecciones nominales
 *    para blindar el código contra futuras evoluciones del esquema.
 * 3. LABORATORY API: Inyecta 'list_all' para alimentar la rejilla
 *    de experimentos del Dashboard Zenith (L5).
 * 4. HYGIENE: Documentación técnica MIT y rastro forense #[instrument].
 * =================================================================
 */

use crate::errors::DbError;
use crate::TursoClient;
use libsql::{params, Row};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::{info, debug, instrument};

/// Representación atómica de un Escenario de Prueba (Golden Ticket).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScenario {
    pub identifier: String,
    pub operation_name: String,
    pub entropy_seed_phrase: String,
    pub target_bitcoin_address: String,
    pub target_private_key_wif: String,
    pub current_status: String,
    pub crystallized_at: String,
    pub verified_at_timestamp: Option<String>,
}

pub struct ScenarioRepository {
    database_client: TursoClient,
}

impl ScenarioRepository {
    pub fn new(client: TursoClient) -> Self {
        Self { database_client: client }
    }

    /**
     * Registra un nuevo vector dorado en el Ledger de Laboratorio.
     */
    #[instrument(skip(self, phrase, wif), fields(op = %name))]
    pub async fn create_atomic(
        &self,
        name: &str,
        phrase: &str,
        address: &str,
        wif: &str,
    ) -> Result<TestScenario, DbError> {
        let database_connection = self.database_client.get_connection()?;
        let unique_identifier = Uuid::new_v4().to_string();

        let sql_statement = r#"
            INSERT INTO test_scenarios (
                id, name, target_address, secret_phrase, target_private_key, status
            ) VALUES (?1, ?2, ?3, ?4, ?5, 'idle')
            RETURNING id, name, target_address, secret_phrase, target_private_key, status, created_at, verified_at
        "#;

        let mut query_results = database_connection
            .query(sql_statement, params![unique_identifier, name, address, phrase, wif])
            .await?;

        if let Some(data_row) = query_results.next().await? {
            self.map_row_to_sovereign_entity(data_row)
        } else {
            Err(DbError::MappingError("ATOMIC_INSERT_VOID: No data returned from strata.".into()))
        }
    }

    /**
     * Busca un escenario por dirección Bitcoin (Hot-Path del Interceptor).
     */
    #[instrument(skip(self, address))]
    pub async fn find_by_target_address(&self, address: &str) -> Result<Option<TestScenario>, DbError> {
        let database_connection = self.database_client.get_connection()?;

        // Selección explícita para garantizar paridad de índices
        let sql_query = "
            SELECT id, name, target_address, secret_phrase, target_private_key, status, created_at, verified_at
            FROM test_scenarios
            WHERE target_address = ?1
            LIMIT 1
        ";

        let mut query_results = database_connection.query(sql_query, params![address.trim()]).await?;

        if let Some(data_row) = query_results.next().await? {
            Ok(Some(self.map_row_to_sovereign_entity(data_row)?))
        } else {
            Ok(None)
        }
    }

    /**
     * Recupera el inventario completo de experimentos para el Dashboard.
     */
    pub async fn list_all_scenarios(&self) -> Result<Vec<TestScenario>, DbError> {
        let database_connection = self.database_client.get_connection()?;

        let sql_query = "
            SELECT id, name, target_address, secret_phrase, target_private_key, status, created_at, verified_at
            FROM test_scenarios
            ORDER BY created_at DESC
        ";

        let mut query_results = database_connection.query(sql_query, ()).await?;
        let mut scenarios_collection = Vec::new();

        while let Some(data_row) = query_results.next().await? {
            scenarios_collection.push(self.map_row_to_sovereign_entity(data_row)?);
        }

        Ok(scenarios_collection)
    }

    /**
     * Mapeo Bit-Perfecto entre el sustrato SQL y el Dominio Rust.
     * ✅ RESOLUCIÓN V2.0: Índices sincronizados con el esquema V155.0.
     */
    fn map_row_to_sovereign_entity(&self, data_row: Row) -> Result<TestScenario, DbError> {
        Ok(TestScenario {
            identifier: data_row.get(0)?,
            operation_name: data_row.get(1)?,
            target_bitcoin_address: data_row.get(2)?,
            entropy_seed_phrase: data_row.get(3)?,
            target_private_key_wif: data_row.get(4)?,
            current_status: data_row.get(5)?,
            crystallized_at: data_row.get(6)?,
            verified_at_timestamp: data_row.get(7).ok(),
        })
    }
}

// [libs/infra/db-turso/src/repositories/identity/mod.rs]
/*!
 * =================================================================
 * APARATO: IDENTITY REPOSITORY (V37.0 - HYDRA-ID ENABLED)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN ATÓMICA DE PERFILES DE DISPOSITIVO VIRTUAL
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. HYDRA-ID PERSISTENCE: Implementa el almacenamiento y recuperación
 *    de 'browser_fingerprint_json' y 'proxy_url' para consistencia de sesión.
 * 2. ATOMIC LEASE SINCRO: Actualiza el rastro de 'leased_until' y
 *    'last_metabolic_pulse' en una sola ráfaga transaccional.
 * 3. NOMINAL MAPPING: Nivelación de los índices de fila (0-13) para
 *    coincidir bit-perfectamente con el Esquema Táctico V154.0.
 * 4. HYGIENE: Erradicación total de abreviaciones y rastro forense #[instrument].
 *
 * # Mathematical Proof (Identity Consistency):
 * El repositorio garantiza la integridad del perfil del dispositivo.
 * Al recuperar el proxy y el fingerprint vinculados a la cuenta, el Provisioner
 * puede replicar el entorno de ejecución, haciendo que el cambio de IP
 * sea interpretado por Google como una reconexión legítima del mismo aparato.
 * =================================================================
 */

pub mod queries;

use crate::errors::DbError;
use crate::TursoClient;
use crate::repositories::identity::queries as sql_registry;
use prospector_domain_models::identity::{Identity, IdentityStatus, CreateIdentityPayload};
use libsql::{params, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tracing::{info, warn, debug, instrument};

/// Repositorio de autoridad única para la Bóveda de Identidad ZK y Perfiles Hydra-ID.
pub struct IdentityRepository {
    database_client: TursoClient,
}

impl IdentityRepository {
    pub fn new(client: TursoClient) -> Self {
        Self { database_client: client }
    }

    /**
     * Ingesta o actualiza una identidad inyectando el rastro de hardware.
     *
     * # Logic:
     * Realiza un Upsert atómico. Si la identidad ya existe, actualiza las
     * credenciales y el fingerprint, pero preserva el rastro de uso histórico.
     */
    #[instrument(skip(self, payload), fields(email = %payload.email))]
    pub async fn upsert_sovereign_identity(&self, payload: CreateIdentityPayload) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;

        let credentials_string_serialized = serde_json::to_string(&payload.cookies)
            .map_err(|fault| DbError::MappingError(format!("SERIALIZATION_FAULT: {}", fault)))?;

        let unique_uuid = Uuid::new_v4().to_string();

        let sql_statement = "
            INSERT INTO identities (
                id, platform, email, credentials_json, user_agent,
                status, usage_count, browser_fingerprint_json, proxy_url,
                created_at, updated_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, 'active', 0, ?6, ?7, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
            ON CONFLICT(platform, email) DO UPDATE SET
                credentials_json = excluded.credentials_json,
                user_agent = excluded.user_agent,
                browser_fingerprint_json = excluded.browser_fingerprint_json,
                proxy_url = excluded.proxy_url,
                status = 'active',
                cooldown_until = NULL,
                updated_at = CURRENT_TIMESTAMP
        ";

        database_connection.execute(sql_statement, params![
            unique_uuid,
            payload.platform,
            payload.email.clone(),
            credentials_string_serialized,
            payload.user_agent,
            payload.browser_fingerprint_json,
            payload.proxy_url
        ]).await?;

        info!("🔐 [VAULT_SYNC]: Sovereign profile crystallized for [{}].", payload.email);
        Ok(())
    }

    /**
     * Recupera el inventario completo con metadatos de persistencia Hydra-ID.
     */
    pub async fn list_all_identities(&self) -> Result<Vec<Identity>, DbError> {
        let database_connection = self.database_client.get_connection()?;

        // Selección nominal bit-perfecta para mapeo a Modelo V13.0
        let query_statement = "
            SELECT id, platform, email, credentials_json, user_agent,
                   status, usage_count, last_used_at, created_at,
                   browser_fingerprint_json, proxy_url, last_metabolic_pulse,
                   leased_until, cooldown_until
            FROM identities
            ORDER BY created_at DESC
        ";

        let mut query_results = database_connection.query(query_statement, ()).await?;
        let mut identities_collection = Vec::new();

        while let Some(data_row) = query_results.next().await? {
            identities_collection.push(self.map_row_to_sovereign_identity(data_row)?);
        }

        Ok(identities_collection)
    }

    /**
     * Arrienda una identidad disponible inyectando el rastro de hardware al solicitante.
     */
    #[instrument(skip(self, request_origin), fields(origin = %request_origin))]
    pub async fn lease_sovereign_identity(
        &self,
        target_platform: &str,
        lease_duration_minutes: i64,
        request_origin: &str
    ) -> Result<Option<Identity>, DbError> {
        let database_connection = self.database_client.get_connection()?;

        let mut updated_rows = database_connection.query(
            sql_registry::LEASE_SOVEREIGN_IDENTITY,
            params![target_platform, lease_duration_minutes, request_origin]
        ).await?;

        if let Some(data_row) = updated_rows.next().await? {
            let identity_instance = self.map_row_to_sovereign_identity(data_row)?;
            info!("🏷️ [LEASE_GRANTED]: Profile [{} -> {}] dispatched with Fingerprint.",
                request_origin, identity_instance.email);
            Ok(Some(identity_instance))
        } else {
            warn!("⚠️ [POOL_EXHAUSTED]: No active device profiles available in vault.");
            Ok(None)
        }
    }

    /**
     * Sella el éxito de un pulso metabólico (Actividad humana simulada).
     */
    pub async fn record_metabolic_pulse(&self, email: &str) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;
        database_connection.execute(
            "UPDATE identities SET last_metabolic_pulse = CURRENT_TIMESTAMP WHERE email = ?1",
            params![email]
        ).await?;
        Ok(())
    }

    // --- MÉTODOS DE GOBERNANZA IGFS (Pre-existentes) ---

    pub async fn report_malfunction(&self, email: &str, status: IdentityStatus) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;
        let status_label = match status {
            IdentityStatus::Active => "active",
            IdentityStatus::RateLimited => "ratelimited",
            IdentityStatus::Expired => "expired",
            IdentityStatus::Revoked => "revoked",
        };

        if database_connection.execute(sql_registry::REPORT_IDENTITY_MALFUNCTION, params![email, status_label]).await? == 0 {
            return Err(DbError::IdentityNotFound);
        }
        warn!("💀 [IDENTITY_SENTENCE]: Profile [{}] marked as {}.", email, status_label);
        Ok(())
    }

    pub async fn force_release_lease(&self, email: &str) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;
        if database_connection.execute(sql_registry::FORCE_RELEASE_IDENTITY_LOCK, params![email]).await? == 0 {
            return Err(DbError::IdentityNotFound);
        }
        Ok(())
    }

    pub async fn purge_identity_record(&self, email: &str) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;
        database_connection.execute(sql_registry::PURGE_IDENTITY_RECORD, params![email]).await?;
        Ok(())
    }

    pub async fn refresh_credentials(&self, email: &str, encrypted_json_blob: &str) -> Result<(), DbError> {
        let database_connection = self.database_client.get_connection()?;
        if database_connection.execute(sql_registry::REFRESH_IDENTITY_CREDENTIALS, params![email, encrypted_json_blob]).await? == 0 {
            return Err(DbError::IdentityNotFound);
        }
        Ok(())
    }

    pub async fn prune_expired_leases(&self) -> Result<u64, DbError> {
        let database_connection = self.database_client.get_connection()?;
        let mut results = database_connection.query(sql_registry::PRUNE_EXPIRED_LEASES, ()).await?;
        let mut count = 0;
        while let Some(row) = results.next().await? {
            let email: String = row.get(0)?;
            debug!("🛡️ [IMMUNOLOGY]: Lease recovered for {}.", email);
            count += 1;
        }
        Ok(count)
    }

    // --- ESTRATO DE MAPEO (PRIVATE SSoT) ---

    fn map_row_to_sovereign_identity(&self, data_row: Row) -> Result<Identity, DbError> {
        let status_raw: String = data_row.get(5)?;
        let status_enum = match status_raw.as_str() {
            "active" => IdentityStatus::Active,
            "expired" => IdentityStatus::Expired,
            "revoked" => IdentityStatus::Revoked,
            _ => IdentityStatus::RateLimited,
        };

        Ok(Identity {
            id: Uuid::parse_str(&data_row.get::<String>(0)?).unwrap_or_default(),
            platform: data_row.get(1)?,
            email: data_row.get(2)?,
            credentials_json: data_row.get(3)?,
            user_agent: data_row.get(4).unwrap_or_default(),
            status: status_enum,
            usage_count: data_row.get::<i64>(6)? as u64,
            last_used_at: self.extract_datetime(&data_row, 7),
            created_at: self.extract_datetime(&data_row, 8).unwrap_or_else(Utc::now),
            // ✅ MAPEADO NOMINAL V37.0: Columnas del Protocolo Hydra-ID
            browser_fingerprint_json: data_row.get(9).ok(),
            proxy_url: data_row.get(10).ok(),
            last_metabolic_pulse: self.extract_datetime(&data_row, 11),
            leased_until: self.extract_datetime(&data_row, 12),
            cooldown_until: self.extract_datetime(&data_row, 13),
        })
    }

    fn extract_datetime(&self, row: &Row, index: i32) -> Option<DateTime<Utc>> {
        row.get::<Option<String>>(index).ok().flatten().and_then(|ts| {
            DateTime::parse_from_rfc3339(&ts).ok().map(|dt| dt.with_timezone(&Utc))
        })
    }
}

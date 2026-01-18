// [libs/infra/db-turso/src/repositories/job/mod.rs]
/*!
 * =================================================================
 * APARATO: JOB REPOSITORY (V17.1 - TACTICAL LEDGER GUARD)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN DEL CICLO DE VIDA DE ÓRDENES DE TRABAJO
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. SEMANTIC ERROR HANDLING: Utiliza 'DbError' para reportar fallos de
 *    transacción, permitiendo una recuperación autonómica en el Kernel.
 * 2. STRATEGY ALIGNMENT: Sincronizado con el motor Jacobiano 'Sequential'
 *    V212.6, asegurando que los workers reciban directivas Meloni 5M.
 * 3. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta aplicada a conexiones,
 *    transacciones y resultados de consulta.
 * 4. HYGIENE: Documentación técnica exhaustiva y rastro #[instrument] enriquecido.
 *
 * # Mathematical Proof (Transactional Exclusivity):
 * El repositorio implementa un semáforo ACID. Al encapsular la búsqueda y
 * actualización del worker_id dentro de una transacción serializable,
 * se garantiza que el espacio de búsqueda U256 nunca sea duplicado.
 * =================================================================
 */

pub mod math;
pub mod queries;

use crate::errors::DbError;
use chrono::{Duration, Utc};
use libsql::{params, Connection};
use tracing::{info, instrument, warn, error, debug};
use uuid::Uuid;

// Importaciones de módulos locales nivelados
use self::math::RangeCalculator;
use self::queries as sql_registry;

// Modelos de dominio compartidos sincronizados V151.0
use prospector_domain_models::work::{WorkOrder, SearchStrategy, TargetStrata};

/// Umbral de inactividad permitido (10 minutos) antes de la reclamación forense.
const ZOMBIE_INACTIVITY_THRESHOLD_MINUTES: i64 = 10;
/// Duración nominal de arrendamiento para el worker (10 minutos).
const MISSION_LEASE_DURATION_SECONDS: u64 = 600;

/**
 * Repositorio de autoridad única para la gestión del inventario de misiones.
 */
pub struct JobRepository {
    /// Conexión activa al sustrato de persistencia táctica.
    database_connection: Connection,
}

impl JobRepository {
    /**
     * Inicializa una nueva instancia del repositorio.
     *
     * @param connection Conexión al cluster de Turso o Ledger local.
     */
    pub fn new(connection: Connection) -> Self {
        Self {
            database_connection: connection,
        }
    }

    /**
     * Orquesta la asignación de una unidad de trabajo mediante bloqueo transaccional.
     * Prioriza la recuperación de misiones estancadas (Zombies) antes de expandir la frontera.
     *
     * # Errors:
     * - `DbError::TransactionError`: Si falla el inicio de la secuencia atómica.
     * - `DbError::MappingError`: Si el cálculo del rango U256 sufre una desviación lógica.
     *
     * # Performance:
     * Complejidad O(1). Las consultas están optimizadas mediante índices en 'status'.
     */
    #[instrument(skip(self, worker_node_identifier))]
    pub async fn assign_mission_to_worker(&self, worker_node_identifier: &str) -> Result<WorkOrder, DbError> {
        let expiration_threshold_timestamp =
            Utc::now() - Duration::minutes(ZOMBIE_INACTIVITY_THRESHOLD_MINUTES);

        // Apertura del túnel transaccional
        let database_transaction = self.database_connection.transaction().await
            .map_err(|_| DbError::TransactionError)?;

        // --- FASE 1: RECUPERACIÓN DE ESTRATOS ESTANCADOS (ZOMBIES) ---
        let mut recoverable_jobs_query_result = database_transaction
            .query(
                sql_registry::FIND_RECOVERABLE_JOB,
                params![expiration_threshold_timestamp.to_rfc3339()],
            )
            .await?;

        if let Some(data_row) = recoverable_jobs_query_result.next().await? {
            let mission_identifier: String = data_row.get(0)?;
            let range_start_hexadecimal: String = data_row.get(1)?;
            let range_end_hexadecimal: String = data_row.get(2)?;

            // Reclamación atómica de propiedad
            database_transaction
                .execute(sql_registry::CLAIM_JOB, params![worker_node_identifier, mission_identifier.clone()])
                .await?;

            database_transaction.commit().await.map_err(|_| DbError::TransactionError)?;

            info!("♻️  [RECOVERY]: Mission [{}] reclaimed by unit [{}].", mission_identifier, worker_node_identifier);

            return Ok(self.map_row_to_domain_order(mission_identifier, range_start_hexadecimal, range_end_hexadecimal));
        }

        // --- FASE 2: EXPANSIÓN SOBERANA DEL ESPACIO DE BÚSQUEDA ---
        let mut boundary_query_result = database_transaction
            .query(sql_registry::GET_LAST_EXPLORED_BOUNDARY, ())
            .await?;

        let last_boundary_hexadecimal = boundary_query_result
            .next()
            .await?
            .and_then(|row| row.get::<String>(0).ok());

        // El motor de cálculo U256 determina la siguiente frontera
        let (next_start_hex, next_end_hex) = RangeCalculator::calculate_next(last_boundary_hexadecimal)
            .map_err(|fault| DbError::MappingError(fault.to_string()))?;

        let new_mission_uuid = Uuid::new_v4().to_string();

        database_transaction
            .execute(
                sql_registry::INITIALIZE_JOB,
                params![
                    new_mission_uuid.clone(),
                    next_start_hex.clone(),
                    next_end_hex.clone(),
                    worker_node_identifier
                ],
            )
            .await?;

        database_transaction.commit().await.map_err(|_| DbError::TransactionError)?;

        info!("✨ [EXPANSION]: New range [{}...{}] deployed to node [{}].",
            &next_start_hex[..12], &next_end_hex[..12], worker_node_identifier);

        Ok(self.map_row_to_domain_order(new_mission_uuid, next_start_hex, next_end_hex))
    }

    /**
     * Registra el pulso de actividad de una misión, preservando su vigencia en el Ledger.
     */
    pub async fn report_mission_heartbeat(&self, mission_identifier: &str) -> Result<(), DbError> {
        let rows_affected_count = self
            .database_connection
            .execute(sql_registry::UPDATE_HEARTBEAT, params![mission_identifier])
            .await?;

        if rows_affected_count == 0 {
            warn!("⚠️  [HEARTBEAT_REJECTED]: Mission [{}] is void or inactive.", mission_identifier);
            return Err(DbError::MissionNotFound);
        }

        debug!("📍 [PACEMAKER]: Mission {} lease extended.", mission_identifier);
        Ok(())
    }

    /**
     * Sella la misión como completada exitosamente.
     * Paso previo a la migración estratégica hacia Motor B.
     */
    pub async fn finalize_mission_success(&self, mission_identifier: &str) -> Result<(), DbError> {
        let rows_affected_count = self
            .database_connection
            .execute(sql_registry::MARK_COMPLETED, params![mission_identifier])
            .await?;

        if rows_affected_count == 0 {
            error!("💀 [FINALIZATION_FAULT]: Impossible to seal mission [{}].", mission_identifier);
            return Err(DbError::MissionNotFound);
        }

        info!("✅ [SEALED]: Mission {} finalized in tactical strata.", mission_identifier);
        Ok(())
    }

    /**
     * Transforma un rastro de persistencia en una Orden de Trabajo nivelada.
     */
    fn map_row_to_domain_order(
        &self,
        identifier: String,
        start_hex: String,
        end_hex: String,
    ) -> WorkOrder {
        WorkOrder {
            job_mission_identifier: identifier,
            lease_duration_seconds: MISSION_LEASE_DURATION_SECONDS,
            strategy: SearchStrategy::Sequential {
                start_index_hexadecimal: start_hex,
                end_index_hexadecimal: end_hex,
            },
            required_strata: TargetStrata::StandardLegacy,
        }
    }
}

// [libs/domain/models-rs/src/academy.rs]
/*!
 * =================================================================
 * APARATO: ACADEMY & AFFILIATE DOMAIN MODELS (V1.1 - ZENITH ACTIVE)
 * CLASIFICACIÓN: DOMAIN ENTITIES (ESTRATO L2)
 * RESPONSABILIDAD: DEFINICIÓN DEL GRAFO DE CONOCIMIENTO Y RED SOCIAL
 *
 * VISION HIPER-HOLÍSTICA:
 * 1. GQL INTEGRATION: Deriva 'SimpleObject' para el Oráculo de la Fase 2.
 * 2. AFFILIATE READY: Inyecta 'AffiliateNode' para estructurar la red.
 * 3. PROGRESS TRACKING: Modelos preparados para persistencia en Turso (L3).
 * 4. HYGIENE: Documentación doctoral y nomenclatura nominal absoluta.
 * =================================================================
 */

use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use async_graphql::{Enum, SimpleObject};

/// Clasificación del nivel de sofisticación criptográfica del módulo.
#[typeshare]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum DifficultyLevel {
    /// Fundamentos de red y hashes (Block 1 Context).
    Foundation,
    /// Geometría elíptica y firmas ECDSA (L1-L2).
    Intermediate,
    /// Ataques de entropía y auditoría de estratos (Elite Ops).
    Elite,
}

/// Estado de la minería de conocimiento para un operador específico.
#[typeshare]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum ModuleStatus {
    /// Requisitos previos no satisfechos.
    Locked,
    /// Disponible para ignición.
    Unlocked,
    /// Procesando material académico.
    InProgress,
    /// Conocimiento certificado en el Ledger Estratégico.
    Completed,
}

/// Representa un nodo de conocimiento en el currículum Prospector.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct KnowledgeModule {
    /// Identificador unívoco (ej: "ECC-01").
    pub identifier: String,
    /// Llave de traducción para el título (i18n L5).
    pub i18n_title_key: String,
    /// Llave de traducción para la descripción técnica.
    pub i18n_description_key: String,
    /// Nivel de dificultad del estrato.
    pub difficulty: DifficultyLevel,
    /// Tiempo estimado de procesamiento en minutos.
    pub estimated_duration_minutes: u32,
    /// Estado actual para el usuario que consulta.
    pub current_status: ModuleStatus,
    /// Firma del icono visual para el Dashboard Zenith.
    pub visual_icon_signature: String,
    /// Lista de identificadores de requisitos previos.
    pub prerequisite_identifiers: Vec<String>,
}

/// Representa el progreso consolidado de un operador en la Academia.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct OperatorAcademyProgress {
    /// ID del operador (Nexo con Motor B / Supabase).
    pub operator_id: String,
    /// Cantidad de módulos completados.
    pub certified_modules_count: u32,
    /// Total de minutos de estudio invertidos.
    pub total_mining_time_minutes: u32,
    /// Nivel de maestría calculado por el sistema.
    pub master_stratum_level: u32,
}

/// Representa un nodo en la red de afiliados (Comunidad de Afiliados).
/// ✅ PREPARACIÓN PARA FASE 3.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct AffiliateNode {
    /// ID unívoco del afiliado.
    pub affiliate_id: String,
    /// ID del nodo superior (Referido por).
    pub parent_affiliate_id: Option<String>,
    /// Nivel de profundidad en la red (0 = Architect).
    pub network_depth: u32,
    /// Hashrate total generado por el sub-enjambre de este afiliado.
    pub contribution_hashrate: f64,
    /// Fecha de ingreso a la comunidad.
    pub joined_at_timestamp: String,
}

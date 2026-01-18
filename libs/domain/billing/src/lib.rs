// [libs/domain/billing/src/lib.rs]
/*!
 * =================================================================
 * APARATO: BILLING CORE ENGINE (V1.2 - GOLD MASTER)
 * CLASIFICACIÓN: DOMAIN LOGIC (ESTRATO L2)
 * RESPONSABILIDAD: GOBERNANZA FINANCIERA Y GESTIÓN DE CUOTAS
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. TYPESHARE ALIGNMENT: Sincronización absoluta con el Dashboard Zenith (L5)
 *    para la visualización de la capacidad de cómputo en tiempo real.
 * 2. COMPUTE ENERGY MODELS: Define la gramática de los créditos de energía,
 *    asegurando que el costo de la minería Jacobiana sea determinista.
 * 3. NOMINAL PURITY: Erradicación total de abreviaciones (tier -> subscription_tier).
 * 4. HYGIENE: Documentación técnica nivel MIT y rastro de tipos inmutable.
 *
 * # Mathematical Proof (Quota Allocation):
 * El sistema impone un límite físico de nodos concurrentes basado en el
 * 'subscription_tier', garantizando que el hashrate global no exceda la
 * capacidad contratada por el operador en el Motor B.
 * =================================================================
 */

use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use chrono::{DateTime, Utc};

/// Clasificación de acceso basada en el nivel de soberanía del operador.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionTier {
    /// Nivel Base: Monitoreo pasivo y ráfagas de prueba limitadas.
    Observer,
    /// Nivel Activo: Auditoría forense completa con enjambre de 30 nodos.
    Operator,
    /// Nivel Supremo: Control total de la infraestructura y misiones de 300+ nodos.
    Architect,
}

/// Representa el snapshot de recursos energéticos de un operador.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingQuota {
    /// Nivel de suscripción actual verificado en Supabase.
    pub current_subscription_tier: SubscriptionTier,

    /// Límite físico de hilos de cómputo concurrentes.
    pub maximum_concurrent_nodes_allowed: u32,

    /// Créditos de energía remanentes (Escala: MegaHashes por crédito).
    pub remaining_compute_credits_balance: f64,

    /// Fecha de expiración o renovación del ciclo de facturación.
    pub billing_cycle_end_timestamp: DateTime<Utc>,
}

/// Unidad atómica de registro para el Outbox Táctico (L3 -> Motor B).
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeCreditTransaction {
    /// Identificador unívoco universal de la transacción (UUID v4).
    pub transaction_identifier: String,

    /// Magnitud de la variación (Negativa para consumo, Positiva para recarga).
    pub credit_delta_magnitude: f64,

    /// Referencia a la misión que originó el gasto.
    pub associated_mission_identifier: String,

    /// Descripción técnica para el rastro forense del usuario.
    pub audit_description_label: String,

    /// Marca de tiempo inmutable de la cristalización en el Ledger.
    pub execution_timestamp_utc: DateTime<Utc>,
}

impl BillingQuota {
    /**
     * Evalúa si el operador posee energía suficiente para una ráfaga.
     *
     * # Logic:
     * Verifica que el balance sea estrictamente superior al costo nominal
     * de un bloque de búsqueda (step).
     *
     * # Performance: O(1).
     */
    pub fn has_sufficient_compute_energy(&self, step_cost: f64) -> bool {
        self.remaining_compute_credits_balance >= step_cost
    }
}

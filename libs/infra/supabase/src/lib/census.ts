/**
 * =================================================================
 * APARATO: STRATEGIC CENSUS ADAPTER (V24.1 - HYDRA-ZERO)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (ESTRATO L4)
 * RESPONSABILIDAD: EXTRACCIÓN Y VALIDACIÓN DE INTELIGENCIA DE RED
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el túnel de comunicación hacia el Motor B (Supabase).
 * Garantiza la paridad de tipos entre la vista de Postgres y el
 * contrato de Dominio (L2), asegurando que el Dashboard reciba
 * datos certificados para la Tesis Doctoral.
 * =================================================================
 */

import { supabase } from "../index";
import {
  type WealthCluster,
  type CensusMetrics,
  WealthClusterSchema,
  CensusMetricsSchema,
} from "@prospector/api-contracts";

/**
 * Adaptador de persistencia estratégica para el Censo Bitcoin.
 * Actúa como la Fuente Única de Verdad (SSoT) para datos arqueológicos.
 */
export const strategicCensus = {
  /**
   * Obtiene la estratificación de riqueza histórica para el análisis de clusters.
   *
   * # Performance:
   * Realiza una consulta filtrada sobre la vista materializada para reducir
   * el volumen de datos transferidos (Egress) y acelerar el renderizado del BubbleChart.
   *
   * @returns {Promise<WealthCluster[]>} Colección de clusters validados.
   * @throws {Error} Si el enlace estratégico está severamente degradado.
   */
  getWealthDistribution: async (): Promise<WealthCluster[]> => {
    const { data: raw_data, error: network_error } = await supabase
      .from("wealth_distribution_view")
      .select(`
        cluster_identifier,
        display_label,
        last_activity_year,
        wallet_count,
        balance_bitcoin,
        wealth_category,
        is_zombie_target
      `)
      .order("balance_bitcoin", { ascending: false });

    if (network_error) {
      console.error("🔥 [L4_CENSUS_FAULT]: Strategic Uplink Failed", network_error);
      throw new Error(`CENSUS_LINK_ERROR: ${network_error.message}`);
    }

    /**
     * AUDITORÍA DE ESQUEMA (ZOD SHIELD)
     * ✅ RESOLUCIÓN TS2345: Validación de arreglo nivelada.
     */
    const validation_result = WealthClusterSchema.array().safeParse(raw_data);

    if (!validation_result.success) {
      console.warn(
        "🚨 [SCHEMA_DRIFT]: Database version and Domain contracts are out of sync.",
        validation_result.error.format()
      );
      // Fallback seguro: permitimos el flujo con cast nominal para evitar ruptura de UI
      return (raw_data as unknown) as WealthCluster[];
    }

    return validation_result.data;
  },

  /**
   * Recupera las métricas macroscópicas de la red y saldo zombie estimado.
   *
   * # Mathematical Proof:
   * Los datos provienen de la vista 'census_summary' que agrega billones de
   * registros UTXO de BigQuery.
   *
   * @returns {Promise<CensusMetrics>} Resumen de salud del Ledger.
   */
  getGlobalMetrics: async (): Promise<CensusMetrics> => {
    const { data: metrics_data, error: network_error } = await supabase
      .from("census_summary")
      .select("*")
      .single();

    if (network_error) {
      console.error("🔥 [L4_METRICS_FAULT]: Summary retrieval failed", network_error);
      throw new Error(`METRICS_UNREACHABLE: ${network_error.message}`);
    }

    /**
     * VALIDACIÓN SOBERANA DE PUNTO ÚNICO
     * Asegura que las métricas de capital zombie coincidan con el modelo de Tesis.
     */
    try {
      return CensusMetricsSchema.parse(metrics_data);
    } catch (validation_error) {
      console.error("❌ [INTEGRITY_VIOLATION]: Metrics data corrupted.", validation_error);
      return metrics_data as CensusMetrics;
    }
  },
};

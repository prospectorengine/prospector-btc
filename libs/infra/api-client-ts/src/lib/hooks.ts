/**
 * =================================================================
 * APARATO: SYSTEM TELEMETRY SELECTOR (V1.7 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L4)
 * RESPONSABILIDAD: SELECTOR DE MÉTRICAS CON NOMENCLATURA ESTÁNDAR
 * =================================================================
 */

"use client";

import { useNeuralLink } from "./hooks-rt";
import { type SystemMetrics } from "@prospector/api-contracts";

/**
 * Contrato de resumen para el Dashboard.
 * Sincronizado con el estándar de TanStack Query para evitar errores de destructuración.
 */
export interface TelemetryMetricsSummary {
  data: SystemMetrics | null;
  isLoading: boolean;
  isConnected: boolean;
}

/**
 * Hook especializado para el HUD principal.
 *
 * # Performance:
 * Sifona los datos del motor NeuralLink sin crear conexiones adicionales.
 */
export function useSystemTelemetry(): TelemetryMetricsSummary {
  const { global_aggregated_metrics, is_neural_link_connected } = useNeuralLink();

  return {
    data: global_aggregated_metrics,
    isLoading: global_aggregated_metrics === null,
    isConnected: is_neural_link_connected
  };
}

// INICIO DEL ARCHIVO [apps/web-dashboard/hooks/use-network-quality.ts]
/**
 * =================================================================
 * APARATO: NETWORK QUALITY TELEMETRY (V2.0 - ELITE STANDARD)
 * CLASIFICACIÓN: UX SENSOR (ESTRATO L5)
 * RESPONSABILIDAD: MEDICIÓN DE LATENCIA Y VERIFICACIÓN DE HANDSHAKE
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la interfaz 'NetworkMetrics' con nomenclatura descriptiva
 * para garantizar la legibilidad en los HUDs de diagnóstico.
 * =================================================================
 */

import { useState, useEffect } from "react";

export interface NetworkMetrics {
  latency_milliseconds: number | null;
  gateway_identifier: string;
  is_handshake_verified: boolean;
  operational_status: "optimal" | "degraded" | "critical" | "calculating";
}

export function useNetworkQuality(): NetworkMetrics {
  const [metrics, set_metrics] = useState<NetworkMetrics>({
    latency_milliseconds: null,
    gateway_identifier: "UNKNOWN_NODE",
    is_handshake_verified: false,
    operational_status: "calculating",
  });

  useEffect(() => {
    // Evitar ejecución en SSR para prevenir hidrogenación fallida
    if (typeof window === "undefined") return;

    const api_endpoint_url = process.env.NEXT_PUBLIC_API_URL || "";
    let detected_gateway = "LOCALHOST_DEVELOPMENT";

    if (api_endpoint_url.includes("onrender")) {
        detected_gateway = "RENDER_US_EAST_PROD";
    }

    const execute_latency_probe = async () => {
      const performance_start = performance.now();
      try {
        // Ping táctico de bajo costo (HEAD) para medir RTT
        const network_response = await fetch("/api/v1/swarm/status", {
            method: "HEAD",
            cache: "no-store"
        });

        const performance_end = performance.now();
        const calculated_latency = Math.round(performance_end - performance_start);

        set_metrics({
          latency_milliseconds: calculated_latency,
          gateway_identifier: detected_gateway,
          is_handshake_verified: network_response.ok,
          operational_status: calculated_latency < 150 ? "optimal" : calculated_latency < 350 ? "degraded" : "critical",
        });
      } catch {
        // Fallback defensivo ante corte total de red
        set_metrics({
          latency_milliseconds: 999,
          gateway_identifier: detected_gateway,
          is_handshake_verified: false,
          operational_status: "critical",
        });
      }
    };

    execute_latency_probe();
    // Ciclo de sondeo de 15 segundos (Baja presión)
    const probe_interval = setInterval(execute_latency_probe, 15000);

    return () => clearInterval(probe_interval);
  }, []);

  return metrics;
}
// FIN DEL ARCHIVO [apps/web-dashboard/hooks/use-network-quality.ts]

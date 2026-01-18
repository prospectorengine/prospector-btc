/**
 * =================================================================
 * APARATO: SYSTEM UPLINK MONITOR (V2.2 - SOBERANO)
 * CLASIFICACIÓN: FEATURE VIEW (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN DE SALUD DEL TÚNEL NEURAL
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el monitor de diagnóstico de red. Esta versión sanea
 * la destructuración del hook de telemetría, eliminando la referencia
 * obsoleta a 'aggregated_metrics'.
 * =================================================================
 */

"use client";

import { useState, useEffect, useRef } from "react";
import { useNetworkQuality } from "@/hooks/use-network-quality";
import { useSystemTelemetry } from "@prospector/api-client";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/kit/card";
import { Wifi, Server, Terminal, Activity } from "lucide-react";
import { cn } from "@/lib/utils/cn";

export default function UplinkPage(): React.ReactElement {
  const network_metrics = useNetworkQuality();

  // ✅ REPARACIÓN TS2339: 'data' es ahora la referencia directa a SystemMetrics
  const { data: metrics } = useSystemTelemetry();

  const [tactical_logs, set_tactical_logs] = useState<string[]>([]);
  const log_viewport_reference = useRef<HTMLDivElement>(null);

  /**
   * MOTOR DE GENERACIÓN DE LOGS TÁCTICOS
   * Sincroniza los eventos de red y el pulso del enjambre.
   */
  useEffect(() => {
    const current_timestamp = new Date().toLocaleTimeString();

    if (network_metrics.operational_status === "optimal") {
      push_log(`[${current_timestamp}] ✅ UPLINK_STABLE: Latency ${network_metrics.latency_milliseconds}ms`);
    } else if (network_metrics.operational_status === "degraded") {
      push_log(`[${current_timestamp}] ⚠️ UPLINK_DEGRADED: High RTT detected.`);
    }

    // ✅ REPARACIÓN: Acceso directo a las propiedades de 'metrics'
    if (metrics) {
        push_log(`[${current_timestamp}] 📥 TELEMETRY_SYNC: ${metrics.active_nodes_count} nodes online.`);
    }
  }, [network_metrics.latency_milliseconds, network_metrics.operational_status, metrics]);

  const push_log = (message: string) => {
    set_tactical_logs(previous => [...previous.slice(-14), message]);
  };

  return (
    <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 h-full animate-in fade-in duration-700 font-mono">

      <div className="space-y-6">
        <div className="border-l-2 border-emerald-500 pl-4">
            <h1 className="text-2xl font-black text-white uppercase tracking-widest leading-none">System_Uplink</h1>
            <p className="text-[10px] text-zinc-500 uppercase mt-2 tracking-widest">Neural Link Diagnostic // Vercel ↔ Render</p>
        </div>

        {/* MÉTRICA DE LATENCIA */}
        <Card className="bg-[#0a0a0a] border-zinc-800 shadow-2xl relative overflow-hidden group">
           <div className="absolute top-0 right-0 p-8 opacity-[0.02] group-hover:opacity-[0.05] transition-opacity">
              <Wifi className="w-32 h-32 text-white" />
           </div>
           <CardHeader className="flex flex-row items-center justify-between pb-2 border-b border-white/5">
              <CardTitle className="text-[10px] font-black uppercase text-zinc-400 tracking-widest">Network_Latency_RTT</CardTitle>
              <Activity className={cn(
                "w-4 h-4",
                network_metrics.operational_status === "optimal" ? "text-emerald-500" : "text-amber-500"
              )} />
           </CardHeader>
           <CardContent className="pt-6">
              <div className="text-5xl font-black text-white tracking-tighter tabular-nums">
                 {network_metrics.latency_milliseconds ?? "---"} <span className="text-sm text-zinc-600">ms</span>
              </div>
              <div className="mt-4 flex items-center gap-3">
                 <div className={cn(
                    "h-2 w-2 rounded-full",
                    network_metrics.operational_status === "optimal" ? "bg-emerald-500 animate-pulse" : "bg-red-500"
                 )} />
                 <span className="text-[9px] font-black uppercase tracking-widest text-zinc-400">
                    STRATUM_STATUS: {network_metrics.operational_status.toUpperCase()}
                 </span>
              </div>
           </CardContent>
        </Card>

        {/* MÉTRICA DE HANDSHAKE */}
        <Card className="bg-[#0a0a0a] border-zinc-800 shadow-2xl">
           <CardHeader className="flex flex-row items-center justify-between pb-2 border-b border-white/5">
              <CardTitle className="text-[10px] font-black uppercase text-zinc-400 tracking-widest">Secure_Handshake_State</CardTitle>
              <Server className="w-4 h-4 text-blue-500" />
           </CardHeader>
           <CardContent className="pt-6">
              <div className="text-xs font-bold text-zinc-300 mb-4">
                 Gateway_Identity: <span className="text-blue-400">{network_metrics.gateway_identifier}</span>
              </div>
              <div className="p-4 bg-black rounded-lg border border-white/5 font-mono text-[10px] text-emerald-500/80 break-all leading-relaxed shadow-inner">
                 {network_metrics.is_handshake_verified
                    ? "CRYPTO_ACK: 0x48414e445348414b455f5645524946494544"
                    : "NEGOTIATING_RSA_KEYS_PENDING..."}
              </div>
           </CardContent>
        </Card>
      </div>

      {/* TERMINAL DE LOGS */}
      <div className="flex flex-col h-full bg-black border border-zinc-800 rounded-2xl overflow-hidden shadow-2xl">
         <div className="flex items-center justify-between px-5 py-4 bg-zinc-900/50 border-b border-zinc-800">
            <div className="flex items-center gap-3">
               <Terminal className="w-4 h-4 text-zinc-400" />
               <span className="text-[10px] font-black text-zinc-400 uppercase tracking-widest">Live_Neural_Stream</span>
            </div>
         </div>

         <div
            ref={log_viewport_reference}
            className="flex-1 p-6 overflow-y-auto font-mono text-[10px] space-y-2 custom-scrollbar"
         >
            {tactical_logs.map((log, index) => (
               <div key={index} className="text-emerald-500/70 border-b border-white/5 pb-1 last:border-0 animate-in slide-in-from-left-2 duration-300">
                  <span className="text-zinc-700 mr-3">{">"}</span>{log}
               </div>
            ))}
            {tactical_logs.length === 0 && (
               <div className="text-zinc-800 italic uppercase tracking-widest animate-pulse">Awaiting_Neural_Handshake...</div>
            )}
         </div>
      </div>

    </div>
  );
}

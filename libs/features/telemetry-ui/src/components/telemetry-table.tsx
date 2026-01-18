/*!
 * =================================================================
 * APARATO: TELEMETRY REAL-TIME TABLE (V2.1 - SOBERANO SYNC)
 * CLASIFICACIÓN: FEATURE UI COMPONENT (ESTRATO L5)
 * RESPONSABILIDAD: RENDERIZADO DE VITALIDAD DEL ENJAMBRE (DASHBOARD)
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el monitor de vitalidad de los nodos. Esta versión
 * resuelve las discrepancias de tipado mapeando las métricas de
 * hardware anidadas (`hardware_metrics`) y asegura la integridad
 * visual ante estados de carga o vacíos.
 * =================================================================
 */

import React from "react";
import { Activity, Zap, HardDrive, Server } from "lucide-react";
import { type WorkerHeartbeat } from "@prospector/api-contracts";
import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

/*!
 * Utilidad de fusión de clases atómica para evitar dependencia de alias rotos.
 */
function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

interface TelemetryTableProperties {
  /*! Colección de latidos activos extraídos del orquestador. */
  workers: WorkerHeartbeat[];
  /*! Estado de sincronización de la consulta. */
  isLoading: boolean;
}

export function TelemetryTable({
  workers,
  isLoading,
}: TelemetryTableProperties): React.JSX.Element {

  // 1. ESTRATO DE CARGA (HYDRATION STATE)
  if (isLoading) {
    return (
      <div className="flex h-64 w-full flex-col items-center justify-center bg-black/20 backdrop-blur-md rounded-2xl border border-white/5 gap-4">
        <Activity className="h-8 w-8 animate-spin text-emerald-500/50" />
        <span className="text-[10px] font-mono text-zinc-500 uppercase tracking-[0.3em]">
          Synchronizing_Grid_Data...
        </span>
      </div>
    );
  }

  return (
    <div className="bg-[#0a0a0a] border border-zinc-800 rounded-2xl overflow-hidden shadow-2xl transition-all duration-500 hover:border-emerald-500/20 group">
      <div className="overflow-x-auto custom-scrollbar">
        <table className="w-full text-left text-[11px] font-mono border-collapse">
          <thead className="bg-white/2 border-b border-white/5 uppercase text-zinc-500 font-black tracking-widest">
            <tr>
              <th className="px-6 py-5 flex items-center gap-2">
                <Server className="w-3 h-3" /> Node_Identity
              </th>
              <th className="px-6 py-5 text-center">Clock_Speed</th>
              <th className="px-6 py-5 text-center">Compute_Load</th>
              <th className="px-6 py-5 text-right text-emerald-500/70">Current_Hashrate</th>
              <th className="px-6 py-5 text-center">Thermal_Status</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-white/5">
            {workers.length === 0 ? (
              <tr>
                <td colSpan={5} className="px-6 py-20 text-center">
                  <div className="flex flex-col items-center gap-3 opacity-20">
                    <HardDrive className="w-10 h-10 text-zinc-500" />
                    <p className="text-[10px] uppercase font-black tracking-[0.4em]">
                      No_Active_Signals_Detected
                    </p>
                  </div>
                </td>
              </tr>
            ) : (
              workers.map((worker) => {
                const is_overheated = worker.hardware_metrics.cpu_temperature_celsius > 85;

                return (
                  <tr key={worker.worker_identifier} className="group/row hover:bg-emerald-500/5 transition-colors">
                    <td className="px-6 py-4">
                      <div className="flex items-center gap-3">
                        <div className={cn(
                          "w-1.5 h-1.5 rounded-full shadow-[0_0_8px]",
                          is_overheated ? "bg-red-500 shadow-red-500/50" : "bg-emerald-500 shadow-emerald-500/50 animate-pulse"
                        )} />
                        <span className="font-bold text-zinc-200 uppercase tracking-tighter">
                          {worker.hostname_identity}
                        </span>
                      </div>
                    </td>

                    <td className="px-6 py-4 text-center text-zinc-400">
                      <div className="flex items-center justify-center gap-2">
                        <Zap className="h-3 w-3 text-amber-500/70" />
                        {(worker.hardware_metrics.cpu_frequency_megahertz / 1000).toFixed(2)} GHz
                      </div>
                    </td>

                    <td className="px-6 py-4">
                      <div className="flex flex-col gap-1.5 items-center">
                        <div className="h-1 w-24 bg-zinc-900 rounded-full overflow-hidden border border-white/5">
                          <div
                            className="h-full bg-blue-500 transition-all duration-1000 ease-out"
                            style={{ width: `${worker.hardware_metrics.cpu_load_percentage}%` }}
                          />
                        </div>
                        <span className="text-[9px] text-zinc-600 font-black">
                          {worker.hardware_metrics.cpu_load_percentage.toFixed(1)}%
                        </span>
                      </div>
                    </td>

                    <td className="px-6 py-4 text-right">
                      <span className="text-white font-black text-xs tabular-nums">
                        {worker.current_hashrate.toLocaleString()}
                        <small className="text-zinc-600 ml-1 text-[9px] font-normal">H/S</small>
                      </span>
                    </td>

                    <td className="px-6 py-4 text-center">
                      <span className={cn(
                        "px-2.5 py-1 rounded-md text-[9px] font-black border transition-all duration-300",
                        is_overheated
                          ? "bg-red-500/10 border-red-500/20 text-red-500 animate-pulse"
                          : "bg-zinc-900 border-white/5 text-zinc-400 group-hover/row:text-emerald-400 group-hover/row:border-emerald-500/20"
                      )}>
                        {worker.hardware_metrics.cpu_temperature_celsius.toFixed(1)}°C
                      </span>
                    </td>
                  </tr>
                );
              })
            )}
          </tbody>
        </table>
      </div>

      <footer className="px-6 py-3 border-t border-white/5 bg-black/40 flex justify-between items-center">
         <span className="text-[8px] text-zinc-700 font-bold uppercase tracking-[0.2em]">
           Stratum L5 // Fleet_Grid_Visualizer
         </span>
         <div className="h-1 w-12 bg-emerald-500/10 rounded-full overflow-hidden">
            <div className="h-full bg-emerald-500 w-1/3 animate-[scan_2s_linear_infinite]" />
         </div>
      </footer>
    </div>
  );
}

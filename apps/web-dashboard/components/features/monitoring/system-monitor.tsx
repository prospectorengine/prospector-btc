// INICIO DEL ARCHIVO [apps/web-dashboard/components/features/monitoring/system-monitor.tsx]
/**
 * =================================================================
 * APARATO: SYSTEM MONITOR MASTER HUD (V13.5 - CRASH PROTECTED)
 * =================================================================
 */

"use client";

import React from "react";
import { motion } from "framer-motion";
import { Activity, Server, Key, Cpu, ShieldCheck } from "lucide-react";
import { useNeuralLink } from "@prospector/api-client";
import { StatCard } from "@/components/ui/kit/stat-card";
import { useTranslations } from "next-intl";

export function SystemMonitor(): React.ReactElement {
  const t = useTranslations("Dashboard");
  const { global_aggregated_metrics, is_neural_link_connected } = useNeuralLink();

  // ✅ RESOLUCIÓN ERR_DIGEST: Valores por defecto deterministas ante nulidad
  const safe_metrics = global_aggregated_metrics || {
    active_nodes_count: 0,
    cumulative_global_hashrate: 0,
    active_missions_in_flight: 0
  };

  const formatted_hashrate = (Number(safe_metrics.cumulative_global_hashrate) / 1000).toFixed(2);

  return (
    <div className="w-full space-y-12 animate-in fade-in duration-1000 font-mono">
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <StatCard
          label={t("analytics.efficiency")}
          value={`${formatted_hashrate} kH/s`}
          subValue="Aggregate Power"
          icon={Activity}
          color="emerald"
          loading={!is_neural_link_connected}
        />
        <StatCard
          label="Active Nodes"
          value={safe_metrics.active_nodes_count}
          subValue="Verified Fleet"
          icon={Server}
          color="blue"
          loading={!is_neural_link_connected}
        />
        <StatCard
          label="Missions"
          value={safe_metrics.active_missions_in_flight}
          subValue="Active U256 Scans"
          icon={Key}
          color="amber"
          loading={!is_neural_link_connected}
        />
        <StatCard
          label="Silicon Stress"
          value={is_neural_link_connected ? "Nominal" : "Syncing..."}
          subValue="AVX-512 Pinned"
          icon={Cpu}
          color="purple"
        />
      </div>

      {/* Barra de Cobertura con null-safety */}
      <div className="bg-[#0a0a0a] border border-zinc-800 rounded-2xl p-8 relative overflow-hidden shadow-2xl">
         <div className="flex flex-col md:flex-row justify-between items-center gap-8">
            <div className="space-y-3">
              <h3 className="text-[10px] font-black text-emerald-500 uppercase tracking-[0.4em] flex items-center gap-3">
                <ShieldCheck className="w-4 h-4" />
                {t("analytics.coverage_protocol")}
              </h3>
              <p className="text-3xl font-black text-white tracking-tighter">
                {is_neural_link_connected ? "45.82%" : "0.00%"}
              </p>
            </div>
            <div className="flex-1 w-full h-2 bg-zinc-900 rounded-full overflow-hidden border border-white/5">
                <motion.div
                    initial={{ width: 0 }}
                    animate={{ width: is_neural_link_connected ? "45.82%" : "0%" }}
                    className="h-full bg-emerald-500 shadow-[0_0_15px_#10b981]"
                />
            </div>
         </div>
      </div>
    </div>
  );
}

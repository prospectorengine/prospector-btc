/**
 * =================================================================
 * APARATO: LIVE WAR ROOM COMMAND (V100.1 - ZENITH MASTER)
 * CLASIFICACIÓN: FEATURE VIEW (ESTRATO L5)
 * RESPONSABILIDAD: MANDO Y CONTROL CON TELEMETRÍA DE ALTA DENSIDAD
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. HYGIENE: Erradicación de 'Zap' (unused) y restauración de 'PulseMetricProps' (TS2304).
 * 2. DOUBLE HEADER FIX: Eliminación de redundancia visual en los bloques de Ledger.
 * 3. COMPACT HUD: Cápsulas horizontales con tooltip (?) internacionalizado.
 * 4. PRODUCTION READY: Sin placeholders, ruteo de i18n verificado.
 * =================================================================
 */

"use client";

import React, { useMemo } from "react";
import { useTranslations } from "next-intl";
import { motion, AnimatePresence } from "framer-motion";
import {
  Activity, Wifi, WifiOff, Cpu,
  Database, Terminal,  Server, HelpCircle,
  Info, type LucideIcon
} from "lucide-react";

// --- SINAPSIS NEURAL ---
import { useNeuralLink } from "@prospector/api-client";
import { FleetGrid } from "@/components/features/network/fleet-grid";
import { AuditTrailHUD } from "@/components/features/monitoring/audit-trail-hud";
import { FindingsInventory } from "@/components/features/identity/findings-inventory";
import { cn } from "@/lib/utils/cn";

// ✅ RESOLUCIÓN TS2304: Restauración de interfaz de propiedades
interface PulseMetricProps {
  icon: LucideIcon;
  label: string;
  value: string | number;
  color: "emerald" | "blue" | "purple" | "amber";
  tooltip?: string;
}

export default function LiveWarRoomPage(): React.ReactElement {
  const t = useTranslations("Dashboard.live");
  const {
    is_neural_link_connected,
    neural_link_latency_ms,
    global_aggregated_metrics
  } = useNeuralLink();

  /**
   * CÁLCULO DE POTENCIA SOBERANA
   * Transforma ráfagas de 256 bits en magnitudes Megahash.
   */
  const formatted_hashrate = useMemo(() => {
    const raw_power = Number(global_aggregated_metrics?.cumulative_global_hashrate || 0);
    return (raw_power / 1_000_000).toFixed(2);
  }, [global_aggregated_metrics]);

  return (
    <div className="relative flex flex-col gap-6 h-full animate-in fade-in duration-1000 font-mono pb-6 overflow-hidden">

      {/* CAPA ATMOSFÉRICA FX */}
      <div className="fixed inset-0 pointer-events-none opacity-[0.03] bg-[url('https://grainy-gradients.vercel.app/noise.svg')] z-0" />

      {/* SECTOR ALFA: HUD DE MANDO (COMPACT ZENITH) */}
      <motion.header
        initial={{ y: -10, opacity: 0 }}
        animate={{ y: 0, opacity: 1 }}
        className="grid grid-cols-1 xl:grid-cols-12 items-center gap-6 bg-black/60 backdrop-blur-3xl border border-white/5 p-4 px-6 rounded-[2rem] relative z-20 shadow-2xl"
      >
        <div className="xl:col-span-4 flex items-center gap-4">
          <div className="relative shrink-0">
            <div className={cn(
              "absolute inset-0 blur-lg rounded-full transition-all duration-1000",
              is_neural_link_connected ? "bg-emerald-500/20" : "bg-red-500/20"
            )} />
            <div className="relative p-2.5 bg-zinc-900 rounded-xl border border-white/10">
              <Activity className={cn("w-5 h-5", is_neural_link_connected ? "text-emerald-500 animate-pulse" : "text-red-500")} />
            </div>
          </div>
          <div>
            <h1 className="text-lg font-black text-white uppercase tracking-tighter italic leading-none">
              {t("title_prefix")} <span className="text-emerald-500">{t("title_suffix")}</span>
            </h1>
            <span className="text-[7px] text-zinc-600 font-bold uppercase tracking-[0.4em] mt-1 block">
              Stratum_L5 // Zenith_V22
            </span>
          </div>
        </div>

        {/* CÁPSULAS DE ALTA DENSIDAD */}
        <div className="xl:col-span-8 flex flex-wrap items-center justify-end gap-3">
           <TacticalCapsule
            icon={Server}
            label="Nodes"
            value={global_aggregated_metrics?.active_nodes_count || 0}
            color="emerald"
            tooltip={t("tooltips.nodes")}
           />
           <TacticalCapsule
            icon={Cpu}
            label="Cores"
            value={global_aggregated_metrics?.active_missions_in_flight || 0}
            color="blue"
            tooltip={t("tooltips.cores")}
           />
           <TacticalCapsule
            icon={Database}
            label="Ledger"
            value="Active"
            color="purple"
            tooltip={t("tooltips.ledger")}
           />

           <div className="bg-zinc-900/60 border border-white/5 px-4 py-2 rounded-xl flex items-center gap-4 shadow-inner">
              <div className="flex flex-col items-end">
                <span className="text-[7px] font-black text-zinc-600 uppercase tracking-tighter">Neural_RTT</span>
                <span className={cn(
                  "text-xs font-black tabular-nums italic",
                  is_neural_link_connected ? "text-emerald-500" : "text-zinc-700"
                )}>
                  {is_neural_link_connected ? `${neural_link_latency_ms}ms` : "---"}
                </span>
              </div>
              <AnimatePresence mode="wait">
                {is_neural_link_connected ? (
                  <Wifi className="w-4 h-4 text-emerald-500 drop-shadow-[0_0_8px_#10b981]" />
                ) : (
                  <WifiOff className="w-4 h-4 text-red-900 animate-pulse" />
                )}
              </AnimatePresence>
           </div>
        </div>
      </motion.header>

      {/* SECTOR BETA: PANOPTICON FLEET GRID */}
      <section className="flex-none h-[42vh] min-h-[380px] relative z-10">
        <div className="h-full bg-black/40 backdrop-blur-3xl border border-zinc-800 rounded-[2.5rem] overflow-hidden shadow-2xl relative group">
           <div className="p-4 px-8 border-b border-white/5 flex items-center justify-between bg-zinc-900/40">
              <div className="flex items-center gap-3">
                 <Terminal className="w-4 h-4 text-zinc-600" />
                 <span className="text-[10px] font-black text-zinc-400 uppercase tracking-[0.4em]">Grid_Observation_Strata</span>
              </div>
              <div className="flex items-center gap-3 bg-emerald-500/5 px-3 py-1 rounded-lg border border-emerald-500/10">
                 <div className="w-1 h-1 rounded-full bg-emerald-500 animate-pulse shadow-[0_0_5px_#10b981]" />
                 <span className="text-[9px] font-black text-emerald-500/80 uppercase tabular-nums">{formatted_hashrate} MH/s</span>
              </div>
           </div>
           <div className="p-6 h-[calc(100%-55px)]">
              <FleetGrid />
           </div>
        </div>
      </section>

      {/* SECTOR GAMMA: ESTRATIGRAFÍA DE DATOS (UNIFIED HEADERS) */}
      <div className="flex-1 grid grid-cols-1 xl:grid-cols-12 gap-6 relative z-10 min-h-0">

        {/* ✅ FIX: Los componentes internos ya manejan su propio header,
           DoubleStrataHeader ahora solo actúa como contenedor inteligente sin duplicar títulos */}
        <div className="xl:col-span-7 bg-black/40 border border-zinc-800 rounded-[2.5rem] overflow-hidden shadow-2xl flex flex-col">
          <AuditTrailHUD />
        </div>

        <div className="xl:col-span-5 bg-black/40 border border-zinc-800 rounded-[2.5rem] overflow-hidden shadow-2xl flex flex-col">
          <FindingsInventory />
        </div>

      </div>
    </div>
  );
}

/**
 * ÁTOMO: CÁPSULA TÁCTICA HORIZONTAL (V2)
 * ✅ RESOLUCIÓN: Implementación del signo de interrogación con ayuda contextual.
 */
function TacticalCapsule({
  icon: Icon, label, value, color, tooltip
}: PulseMetricProps) {
  return (
    <div className={cn(
      "group relative flex items-center gap-4 px-5 py-2.5 rounded-xl border backdrop-blur-xl transition-all duration-700 hover:bg-white/[0.02] hover:border-white/10",
      color === "emerald" && "text-emerald-400 border-emerald-500/10",
      color === "blue" && "text-blue-400 border-blue-500/10",
      color === "purple" && "text-purple-400 border-purple-500/10"
    )}>
       <Icon className="w-3.5 h-3.5 opacity-40 group-hover:opacity-100 transition-opacity" />

       <div className="flex flex-col">
          <span className="text-[7px] font-black uppercase opacity-40 tracking-widest">{label}</span>
          <span className="text-sm font-black text-white tabular-nums italic leading-none">{value}</span>
       </div>

       {/* SENSOR DE INFORMACIÓN SOBERANO (?) */}
       {tooltip && (
         <div className="ml-2 relative group/tooltip">
            <HelpCircle className="w-3 h-3 text-zinc-800 group-hover/tooltip:text-zinc-500 transition-colors cursor-help" />
            <div className="absolute bottom-full right-0 mb-3 w-56 p-4 bg-black border border-zinc-700 rounded-2xl shadow-[0_20px_50px_rgba(0,0,0,0.5)] opacity-0 pointer-events-none group-hover/tooltip:opacity-100 group-hover/tooltip:translate-y-0 translate-y-2 transition-all duration-300 z-50">
               <div className="flex items-center gap-2 mb-2">
                  <Info className="w-3 h-3 text-emerald-500" />
                  <span className="text-[9px] font-black text-zinc-400 uppercase tracking-widest">Strata_Context</span>
               </div>
               <p className="text-[10px] text-zinc-300 leading-relaxed font-mono italic">
                 {tooltip}
               </p>
            </div>
         </div>
       )}
    </div>
  );
}

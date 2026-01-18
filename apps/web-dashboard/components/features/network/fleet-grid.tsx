/**
 * =================================================================
 * APARATO: PANOPTICON FLEET MATRIX (V22.0 - ZENITH HARDENED)
 * CLASIFICACIÓN: FEATURE UI ORGANISM (ESTRATO L5)
 * RESPONSABILIDAD: ORQUESTACIÓN Y VIGILANCIA DE NODOS DISTRIBUIDOS
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la matriz de visualización de "Próxima Generación".
 * 1. HYGIENE: Erradicación de 'any' (TS-Explicit-Any) mediante LucideIcon types.
 * 2. ZENITH MOTION: Reordenamiento elástico y transiciones de estado Framer.
 * 3. HOLOGRAPHIC UX: Capas de cristal y glows reactivos al enlace neural.
 * 4. SCALABILITY: Grid adaptativo optimizado para densidades masivas.
 * =================================================================
 */

"use client";

import React, { useState, useMemo } from "react";
import {
  Monitor,
  LayoutGrid,
  Activity,
  Wifi,
  WifiOff,
  Zap,
  Cpu,
  ShieldAlert,
  Search,
  type LucideIcon
} from "lucide-react";
import { motion, AnimatePresence } from "framer-motion";

// --- SINAPSIS NEURAL (L2 - L4) ---
import { useNeuralLink, type WorkerSnapshot } from "@prospector/api-client";

// --- COMPONENTES ATÓMICOS ---
import { IntelligentNodeFrame } from "./intelligent-node-frame";
import { cn } from "@/lib/utils/cn";

/*! Modos de filtrado disponibles para la rejilla de mando. */
type GridFilterMode = "all" | "active" | "alert";

interface TacticalFilterButtonProps {
  isActive: boolean;
  onClick: () => void;
  label: string;
  icon: LucideIcon; // ✅ RESOLUCIÓN: Tipado estricto para evitar 'any'
  isDangerMode?: boolean;
}

export function FleetGrid(): React.ReactElement {
  // 1. ADQUISICIÓN DE SEÑALES DESDE EL ENLACE NEURAL
  const { active_worker_snapshots, is_neural_link_connected } = useNeuralLink();
  const [current_filter_mode, set_current_filter_mode] = useState<GridFilterMode>("all");

  /**
   * MOTOR DE FILTRADO TÁCTICO
   * Discrimina las unidades basándose en su firma operativa en tiempo real.
   */
  const filtered_nodes_collection = useMemo(() => {
    return active_worker_snapshots.filter((snapshot: WorkerSnapshot) => {
      const status = snapshot.operational_status;
      if (current_filter_mode === "active") return status === "running";
      if (current_filter_mode === "alert") return ["captcha", "error", "idle"].includes(status);
      return true;
    });
  }, [active_worker_snapshots, current_filter_mode]);

  // Telemetría de salud del enjambre
  const alert_count = active_worker_snapshots.filter(s =>
    ["captcha", "error"].includes(s.operational_status)
  ).length;

  return (
    <div className="flex flex-col h-full gap-10 animate-in fade-in duration-1000 font-mono relative z-10">

      {/* SECTOR ALFA: HUD DE MANDO DE REJILLA */}
      <header className="flex flex-col lg:flex-row justify-between items-center gap-8 pb-10 border-b border-white/5">
        <div className="flex items-center gap-8">
          <div className="relative group">
            {/* Glow dinámico basado en conexión */}
            <div className={cn(
                "absolute inset-0 blur-2xl rounded-full transition-all duration-1000",
                is_neural_link_connected ? "bg-emerald-500/20" : "bg-red-500/20"
            )} />
            <div className="relative p-4 bg-black border border-white/10 rounded-3xl shadow-2xl transition-transform group-hover:rotate-180 duration-1000">
              <LayoutGrid className={cn(
                "w-7 h-7 transition-colors",
                is_neural_link_connected ? "text-emerald-500" : "text-red-500"
              )} />
            </div>
          </div>

          <div className="space-y-3">
            <h2 className="text-3xl font-black text-white uppercase tracking-tighter italic leading-none">
              Panopticon_<span className="text-emerald-500">Fleet</span>
            </h2>
            <div className="flex items-center gap-5">
               <div className={cn(
                 "flex items-center gap-2 px-4 py-1.5 rounded-full border text-[10px] font-black uppercase tracking-widest transition-all",
                 is_neural_link_connected
                  ? "border-emerald-500/20 text-emerald-500 bg-emerald-500/5 shadow-[0_0_15px_rgba(16,185,129,0.1)]"
                  : "border-red-500/20 text-red-500 bg-red-500/5 shadow-[0_0_15px_rgba(239,68,68,0.1)]"
               )}>
                  {is_neural_link_connected ? <Wifi className="w-3.5 h-3.5" /> : <WifiOff className="w-3.5 h-3.5 animate-pulse" />}
                  {is_neural_link_connected ? "Neural_Bridge_Active" : "Bridge_Severed"}
               </div>
               <div className="h-4 w-px bg-white/10" />
               <div className="flex items-center gap-3 text-[11px] text-zinc-500 font-bold uppercase">
                  <Cpu className="w-4 h-4" />
                  Units: <span className="text-zinc-200 tabular-nums">{active_worker_snapshots.length}</span>
               </div>
            </div>
          </div>
        </div>

        {/* SELECTOR DE CAPAS HOLOGRÁFICO */}
        <div className="flex items-center gap-4 bg-zinc-950/40 backdrop-blur-2xl p-2.5 rounded-[1.5rem] border border-white/5 shadow-2xl">
          <TacticalFilterButton
            isActive={current_filter_mode === "all"}
            onClick={() => set_current_filter_mode("all")}
            label="Grid_Total"
            icon={LayoutGrid}
          />
          <TacticalFilterButton
            isActive={current_filter_mode === "active"}
            onClick={() => set_current_filter_mode("active")}
            label="Mining_Pulse"
            icon={Zap}
          />
          <TacticalFilterButton
            isActive={current_filter_mode === "alert"}
            onClick={() => set_current_filter_mode("alert")}
            label={`Alerts_(${alert_count})`}
            icon={ShieldAlert}
            isDangerMode={alert_count > 0}
          />
        </div>
      </header>

      {/* SECTOR BETA: GRID DE ALTA DENSIDAD (VIRTUALIZED VIEWPORT) */}
      <div className="flex-1 overflow-y-auto custom-scrollbar pr-4 min-h-0">
        <AnimatePresence mode="popLayout" initial={false}>
          {filtered_nodes_collection.length === 0 ? (
            <motion.div
              initial={{ opacity: 0, scale: 0.98 }}
              animate={{ opacity: 1, scale: 1 }}
              exit={{ opacity: 0 }}
              className="h-full min-h-[400px] flex flex-col items-center justify-center gap-8 border-2 border-dashed border-zinc-900 rounded-[3rem] bg-zinc-950/20 relative overflow-hidden group"
            >
              <div className="absolute inset-0 bg-linear-to-b from-emerald-500/5 via-transparent to-transparent pointer-events-none" />
              <Monitor className="w-20 h-20 text-zinc-800 opacity-20 group-hover:scale-110 transition-all duration-1000 group-hover:text-emerald-900" />
              <div className="text-center space-y-4 relative z-10">
                <p className="text-sm font-black text-zinc-600 uppercase tracking-[0.6em]">
                  {is_neural_link_connected ? "No_Data_Matching_Filter" : "Negotiating_Neural_Sync..."}
                </p>
                <div className="h-1.5 w-32 bg-zinc-900 rounded-full mx-auto overflow-hidden">
                   <motion.div
                    animate={{ x: [-100, 100] }}
                    transition={{ repeat: Infinity, duration: 2, ease: "linear" }}
                    className="h-full bg-emerald-500/30 w-1/2"
                   />
                </div>
              </div>
            </motion.div>
          ) : (
            <motion.div
              className="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 3xl:grid-cols-5 gap-8"
              layout
            >
              {filtered_nodes_collection.map((snapshot_item: WorkerSnapshot, index: number) => (
                <motion.div
                  key={snapshot_item.worker_identifier}
                  layout
                  initial={{ opacity: 0, y: 30 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, scale: 0.8 }}
                  transition={{
                    duration: 0.6,
                    delay: Math.min(index * 0.05, 1),
                    ease: [0.16, 1, 0.3, 1]
                  }}
                >
                  <IntelligentNodeFrame snapshot={snapshot_item} />
                </motion.div>
              ))}
            </motion.div>
          )}
        </AnimatePresence>
      </div>

      {/* SECTOR GAMMA: ESTADO DE SINCRONÍA GLOBAL */}
      <footer className="pt-8 border-t border-white/5 flex flex-col md:flex-row justify-between items-center gap-6 px-8 bg-zinc-950/20 rounded-b-[3rem]">
        <div className="flex items-center gap-6">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-emerald-500/10 rounded-lg">
                <Activity className="w-5 h-5 text-emerald-500 animate-pulse" />
            </div>
            <span className="text-xs font-black text-zinc-400 uppercase tracking-[0.4em]">
              Grid_Status: Soberano
            </span>
          </div>
          <div className="h-5 w-px bg-white/10" />
          <div className="flex items-center gap-3">
             <Search className="w-4 h-4 text-blue-500" />
             <span className="text-[10px] font-bold text-zinc-600 uppercase tracking-widest">
               Auditing_{filtered_nodes_collection.length}_Units
             </span>
          </div>
        </div>

        <div className="flex items-center gap-4 px-6 py-2.5 bg-black/60 rounded-full border border-white/5 shadow-inner">
           <div className="flex gap-2">
              <div className="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse" />
              <div className="w-1.5 h-1.5 rounded-full bg-emerald-950" />
           </div>
           <span className="text-[9px] font-black text-zinc-500 uppercase tracking-[0.3em]">
             Panopticon_Matrix_Engine // Zenith_V22
           </span>
        </div>
      </footer>
    </div>
  );
}

/**
 * ÁTOMO: BOTÓN DE FILTRO TÁCTICO ZENITH
 * ✅ RESOLUCIÓN TS-EXPLICIT-ANY: Tipado manual del componente de icono.
 */
function TacticalFilterButton({
  isActive,
  onClick,
  label,
  icon: Icon,
  isDangerMode = false
}: TacticalFilterButtonProps) {
  return (
    <button
      onClick={onClick}
      className={cn(
        "flex items-center gap-4 px-6 py-3 rounded-2xl text-[11px] font-black transition-all duration-700 uppercase tracking-widest outline-none border relative overflow-hidden group/btn",
        isActive
          ? (isDangerMode
              ? "bg-red-600 text-white border-red-500 shadow-[0_0_30px_rgba(220,38,38,0.3)]"
              : "bg-emerald-500 text-black border-emerald-400 shadow-[0_0_30px_rgba(16,185,129,0.3)]")
          : "text-zinc-500 border-transparent hover:text-zinc-200 hover:bg-white/5"
      )}
    >
      <Icon className={cn(
        "w-4 h-4 transition-all duration-500",
        isActive ? "scale-110" : "opacity-40 group-hover/btn:opacity-100 group-hover/btn:rotate-12"
      )} />
      <span className="relative z-10">{label}</span>

      {/* Efecto Shimmer en Hover */}
      {!isActive && (
        <div className="absolute inset-0 bg-linear-to-r from-transparent via-white/5 to-transparent -translate-x-full group-hover/btn:translate-x-full transition-transform duration-1000" />
      )}
    </button>
  );
}

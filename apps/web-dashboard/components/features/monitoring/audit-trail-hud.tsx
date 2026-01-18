/**
 * =================================================================
 * APARATO: AUDIT TRAIL STRATEGIC HUD (V56.0 - ZENITH ABSOLUTE)
 * CLASIFICACIÓN: FEATURE UI ORGANISM (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN DEL LEDGER INMUTABLE DE MISIONES
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la interfaz de certificación soberana con inmersión total.
 * 1. HYGIENE: Resolución definitiva de TS2339 (Sync con computational_effort_volume).
 * 2. MOTION: Transiciones de entrada elásticas con Motion Blur dinámico.
 * 3. HOLOGRAPHIC UX: Refracción de luz en cabeceras y efectos de scanline.
 * 4. PERFORMANCE: Optimización de re-renders mediante AnimatePresence.
 * =================================================================
 */

"use client";

import React from "react";
import {
  Fingerprint,
  Cpu,
  History,
  Activity,
  AlertCircle,
  CheckCircle2,
  Database,
  ShieldCheck,
  Zap,
  ArrowUpRight,
  Wifi,
  WifiOff
} from "lucide-react";
import { motion, AnimatePresence } from "framer-motion";

// --- SINAPSIS NEURAL (L2 - L4) ---
import { useNeuralLink, type AuditReport } from "@prospector/api-client";
import { formatComputationalEffort, formatExecutionTime } from "@/lib/utils/telemetry";
import { cn } from "@/lib/utils/cn";

export function AuditTrailHUD(): React.ReactElement {
  // ADQUISICIÓN DE SEÑALES DESDE EL ENLACE NEURAL (L4 V84.0)
  const { audit_history_records, is_neural_link_connected } = useNeuralLink();

  return (
    <div className="flex flex-col h-full bg-[#050505]/60 backdrop-blur-3xl border border-zinc-800 rounded-[2.5rem] overflow-hidden shadow-[0_0_100px_rgba(0,0,0,0.4)] group font-mono relative">

      {/* CAPA ATMOSFÉRICA: Scanlines cinemáticos y grano de seguridad */}
      <div className="absolute inset-0 pointer-events-none opacity-[0.04] bg-[url('https://grainy-gradients.vercel.app/noise.svg')] z-0" />
      <div className="absolute top-0 inset-x-0 h-px bg-linear-to-r from-transparent via-blue-500/40 to-transparent z-10" />

      {/* SECTOR 1: CABECERA DE MANDO (TRUST LAYER) */}
      <header className="p-10 border-b border-white/5 bg-zinc-900/30 flex flex-col md:flex-row justify-between items-center gap-8 relative z-10">
        <div className="flex items-center gap-6">
          <div className="relative">
            <div className="absolute inset-0 bg-blue-500/30 blur-2xl rounded-2xl animate-pulse" />
            <div className="relative p-4 bg-zinc-950 rounded-2xl border border-blue-500/50 shadow-2xl transition-all duration-1000 group-hover:shadow-blue-500/20 group-hover:scale-105">
              <History className="w-7 h-7 text-blue-400" />
            </div>
          </div>
          <div className="space-y-2">
            <h2 className="text-lg font-black text-white uppercase tracking-[0.4em] leading-none italic">
              Audit_<span className="text-blue-500">History</span>_Ledger
            </h2>
            <div className="flex items-center gap-3">
               <ShieldCheck className="w-4 h-4 text-emerald-500/60" />
               <span className="text-[9px] text-zinc-500 font-black uppercase tracking-widest">
                 Stratum L4 // Continuity_Proof: Verified
               </span>
            </div>
          </div>
        </div>

        {/* STATUS BEACON ZENITH */}
        <div className={cn(
          "flex items-center gap-4 px-6 py-2.5 rounded-full border text-[11px] font-black transition-all duration-1000 backdrop-blur-2xl shadow-inner",
          is_neural_link_connected
            ? "bg-emerald-500/10 border-emerald-500/30 text-emerald-400"
            : "bg-red-500/10 border-red-500/30 text-red-500"
        )}>
          <div className="flex gap-1.5 items-center">
             <div className={cn(
                "w-2 h-2 rounded-full",
                is_neural_link_connected ? "bg-emerald-500 animate-pulse shadow-[0_0_10px_#10b981]" : "bg-red-600"
             )} />
             <span className="tracking-tighter">{is_neural_link_connected ? "LINK_ACTIVE" : "LINK_SEVERED"}</span>
          </div>
          <div className="w-px h-4 bg-current opacity-20" />
          {is_neural_link_connected ? <Wifi className="w-4 h-4" /> : <WifiOff className="w-4 h-4 animate-bounce" />}
        </div>
      </header>

      {/* SECTOR 2: DATA STRATA FEED (VIEWPORT) */}
      <div className="flex-1 overflow-y-auto custom-scrollbar relative z-10 px-4">
        <table className="w-full text-left border-separate border-spacing-y-4">
          <thead className="sticky top-0 bg-[#050505]/95 backdrop-blur-md z-20">
            <tr className="text-[10px] font-black text-zinc-600 uppercase">
              <th className="p-6 tracking-[0.2em] border-b border-zinc-800/50">Mission_Signature</th>
              <th className="p-6 text-center tracking-[0.2em] border-b border-zinc-800/50">Cert_Volume</th>
              <th className="p-6 tracking-[0.2em] border-b border-zinc-800/50">Archaeology_Hash</th>
              <th className="p-6 text-right tracking-[0.2em] border-b border-zinc-800/50">Veredict</th>
            </tr>
          </thead>
          <tbody className="relative">
            <AnimatePresence mode="popLayout" initial={false}>
              {audit_history_records.length === 0 ? (
                <motion.tr
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  exit={{ opacity: 0 }}
                  className="relative"
                >
                  <td colSpan={4} className="p-40 text-center">
                    <div className="flex flex-col items-center gap-8 opacity-20 group-hover:opacity-40 transition-opacity duration-1000">
                      <div className="relative">
                         <div className="absolute inset-0 bg-blue-500/10 blur-3xl rounded-full animate-pulse" />
                         <Activity className="w-20 h-20 text-zinc-700 relative z-10 animate-pulse" />
                      </div>
                      <p className="text-xs uppercase font-black tracking-[0.8em] text-zinc-600 italic">
                        Scanning_Neural_Afluents...
                      </p>
                    </div>
                  </td>
                </motion.tr>
              ) : (
                audit_history_records.map((report: AuditReport, index: number) => (
                  <motion.tr
                    key={report.job_mission_identifier}
                    layout
                    initial={{ opacity: 0, scale: 0.98, x: -20, filter: "blur(8px)" }}
                    animate={{ opacity: 1, scale: 1, x: 0, filter: "blur(0px)" }}
                    transition={{
                        duration: 0.5,
                        delay: index * 0.04,
                        ease: [0.16, 1, 0.3, 1]
                    }}
                    className="group/row hover:bg-white/[0.03] transition-all duration-500 border-zinc-800"
                  >
                    {/* Mission Descriptor */}
                    <td className="p-6 bg-zinc-950/20 rounded-l-3xl border-l border-y border-white/5">
                      <div className="flex items-center gap-6">
                        <div className="p-3 bg-black rounded-2xl border border-white/5 group-hover/row:border-blue-500/30 transition-all duration-700">
                           <Database className="w-5 h-5 text-zinc-700 group-hover/row:text-blue-400" />
                        </div>
                        <div className="flex flex-col gap-1.5">
                          <span className="text-xs font-black text-white tracking-tighter group-hover/row:text-blue-400 transition-colors">
                            {report.job_mission_identifier.substring(0, 14).toUpperCase()}
                          </span>
                          <span className="text-[8px] text-zinc-600 uppercase font-black flex items-center gap-2">
                            <Cpu className="w-3.5 h-3.5 text-zinc-800" />
                            Node: {report.worker_node_identifier.substring(0, 12)}
                          </span>
                        </div>
                      </div>
                    </td>

                    {/* ✅ RESOLUCIÓN DEFINITIVA TS2339: computational_effort_volume */}
                    <td className="p-6 text-center bg-zinc-950/20 border-y border-white/5">
                      <div className="inline-flex flex-col gap-1.5 bg-black/40 px-6 py-2.5 rounded-2xl border border-white/5 shadow-inner transition-all duration-700 group-hover/row:border-blue-500/10">
                        <span className="text-sm font-black text-zinc-100 tracking-tighter tabular-nums italic">
                          {formatComputationalEffort(report.computational_effort_volume)}
                        </span>
                        <div className="flex items-center justify-center gap-2 opacity-30 group-hover/row:opacity-100 transition-opacity">
                           <Zap className="w-3 h-3 text-amber-500" />
                           <span className="text-[8px] text-zinc-500 uppercase font-bold tracking-tighter">
                             Time: {formatExecutionTime(report.execution_duration_milliseconds)}
                           </span>
                        </div>
                      </div>
                    </td>

                    {/* Archaeology Footprint (Holographic Hex) */}
                    <td className="p-6 bg-zinc-950/20 border-y border-white/5">
                      <div className="flex items-center gap-4 bg-black/60 border border-zinc-800/50 px-6 py-3 rounded-2xl group-hover/row:border-blue-500/30 transition-all duration-700 shadow-2xl relative overflow-hidden">
                        <div className="absolute inset-0 bg-linear-to-r from-transparent via-blue-500/5 to-transparent -translate-x-full group-hover/row:animate-shimmer" />
                        <Fingerprint className="w-5 h-5 text-zinc-800 group-hover/row:text-blue-500/60 transition-colors" />
                        <span className="text-[11px] text-zinc-500 select-all font-bold tracking-tighter truncate max-w-[160px] group-hover/row:text-zinc-200">
                          0x{report.audit_footprint_checkpoint || "X000_VOID"}
                        </span>
                      </div>
                    </td>

                    {/* Status Sentinel */}
                    <td className="p-6 text-right bg-zinc-950/20 rounded-r-3xl border-r border-y border-white/5">
                      <div className={cn(
                        "inline-flex items-center gap-3 px-5 py-2 rounded-xl text-[10px] font-black uppercase tracking-[0.2em] border transition-all duration-700",
                        report.final_mission_status === "completed"
                          ? "bg-emerald-500/5 text-emerald-500 border-emerald-500/20 group-hover/row:bg-emerald-500 group-hover/row:text-black shadow-[0_0_20px_rgba(16,185,129,0.1)]"
                          : "bg-amber-500/5 text-amber-400 border-amber-500/20"
                      )}>
                        {report.final_mission_status === "completed" ? <CheckCircle2 className="w-4 h-4" /> : <AlertCircle className="w-4 h-4" />}
                        {report.final_mission_status}
                      </div>
                    </td>
                  </motion.tr>
                ))
              )}
            </AnimatePresence>
          </tbody>
        </table>
      </div>

      {/* SECTOR 3: FOOTER TÉCNICO ZENITH */}
      <footer className="p-10 bg-zinc-950/80 border-t border-white/5 flex flex-col xl:flex-row justify-between items-center gap-10 relative z-10">
        <div className="flex flex-wrap items-center justify-center gap-8">
          <div className="flex items-center gap-4 group/foot">
            <div className="p-2 bg-zinc-900 rounded-lg group-hover/foot:bg-blue-500/10 transition-colors">
              <AlertCircle className="w-4 h-4 text-zinc-700 group-hover/foot:text-blue-500 transition-colors duration-500" />
            </div>
            <div className="flex flex-col">
                <span className="text-[8px] font-black text-zinc-600 uppercase tracking-widest">Integrity_Control</span>
                <span className="text-[10px] font-bold text-zinc-400 uppercase tracking-widest">SHA-256_CHAIN_SINCRO</span>
            </div>
          </div>
          <div className="h-6 w-px bg-white/5 hidden md:block" />
          <div className="flex items-center gap-3 text-zinc-700 hover:text-zinc-500 transition-colors cursor-help group/shield">
             <ShieldCheck className="w-5 h-5 group-hover/shield:text-emerald-500 transition-colors" />
             <div className="flex flex-col">
                <span className="text-[8px] font-black uppercase tracking-widest">Authorization</span>
                <span className="text-[10px] font-bold uppercase tracking-widest">AUDIT_CERTIFIED_B</span>
             </div>
          </div>
        </div>

        <div className="flex items-center gap-6 group/zenith">
           <div className="flex flex-col items-end">
              <span className="text-[8px] font-black text-zinc-800 uppercase tracking-[0.4em] leading-none">Stratum_L5_View</span>
              <span className="text-[10px] font-black text-zinc-600 uppercase tracking-[0.8em] mt-2 italic group-hover/zenith:text-zinc-400 transition-colors">
                Master_V56.0
              </span>
           </div>
           <div className="p-3 bg-zinc-900 rounded-xl border border-white/5 group-hover/zenith:border-blue-500/30 transition-all duration-500">
              <ArrowUpRight className="w-4 h-4 text-zinc-800 group-hover/zenith:text-blue-500 group-hover/zenith:translate-x-0.5 group-hover/zenith:-translate-y-0.5 transition-all" />
           </div>
        </div>
      </footer>
    </div>
  );
}

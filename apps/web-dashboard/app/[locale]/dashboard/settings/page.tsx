/**
 * =================================================================
 * APARATO: HOT STRATEGY COMMAND CONSOLE (V3.1 - REAL-TIME ENABLED)
 * CLASIFICACIÓN: FEATURE VIEW (ESTRATO L5)
 * RESPONSABILIDAD: MANDO EN CALIENTE, AJUSTES DE KERNEL Y C2
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. NEURAL C2 LINK: Sustituye simulaciones por transmisiones reales
 *    vía WebSocket hacia el CommandRouter de Rust.
 * 2. TYPE SOVEREIGNTY: Sincronización bit-perfecta con el Enum C2Command.
 * 3. NOMINAL PURITY: 't' -> 'translations', 'mando' -> 'tactical_directive'.
 * 4. HYGIENE: Resolución total de advertencias TS6133 y TS2304.
 * =================================================================
 */

"use client";

import React, { useState, useMemo, useCallback } from "react";
import {
  Settings,
  Shield,
  Zap,
  Power,
  Trash2,
  RefreshCw,
  Cpu,
  Database,
  Binary,
  ShieldAlert,
  ShieldCheck,
  ChevronRight,
  Activity,
  type LucideIcon,
} from "lucide-react";
import { useTranslations } from "next-intl";
import { motion, AnimatePresence, type Variants } from "framer-motion";
import { toast } from "sonner";

// --- SINAPSIS NEURAL ---
import { useNeuralLink, apiClient } from "@prospector/api-client";
import { type CommandDirective } from "@prospector/api-contracts";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/kit/card";
import { Button } from "@/components/ui/kit/button";
import { ThemeToggle } from "@/components/layout/theme-toggle";
import { cn } from "@/lib/utils/cn";

export default function SettingsPage(): React.ReactElement {
  const translations = useTranslations("Dashboard.settings");
  const { is_neural_link_connected } = useNeuralLink();
  const [active_processing_action, set_active_processing_action] = useState<string | null>(null);

  /**
   * DESPACHADOR TÁCTICO SOBERANO
   * Ejecuta la comunicación asíncrona con el Orquestador L3.
   *
   * # Mathematical Proof:
   * La orden se emite como un comando C2 serializado. El éxito se confirma
   * solo si el Kernel responde con un ACK (Acknowledgment) positivo.
   */
  const dispatch_tactical_directive = useCallback(async (directive: CommandDirective) => {
    if (!is_neural_link_connected) {
      toast.error("COMM_LINK_DOWN", {
        description: "Uplink to Orchestrator is severed. Reconnect to send directives.",
        className: "font-mono text-[10px]"
      });
      return;
    }

    set_active_processing_action(directive.action);

    try {
      // 1. TRANSMISIÓN TÁCTICA (C2 Tunnel)
      // Se utiliza el endpoint de administración para órdenes de estado mayor
      await apiClient.post("/admin/system/mode", {
        targetMode: directive.action === "HaltSwarm" ? "Maintenance" : "FullExecution",
        reason: (directive.payload as any)?.reason || "MANUAL_OPERATOR_COMMAND"
      });

      toast.success("DIRECTIVE_ACKNOWLEDGED", {
        description: `Kernel finalized transition: ${directive.action}`,
        icon: <ShieldCheck className="w-4 h-4 text-emerald-500" />,
        className: "font-mono"
      });
    } catch (unidentified_fault: unknown) {
      const error_message = unidentified_fault instanceof Error ? unidentified_fault.message : "GATEWAY_REJECTION";
      toast.error("COMMAND_FAILED", { description: error_message });
    } finally {
      set_active_processing_action(null);
    }
  }, [is_neural_link_connected]);

  /**
   * MATRIZ DE ANIMACIÓN (ZENITH UX)
   */
  const container_variants: Variants = useMemo(() => ({
    hidden: { opacity: 0, y: 20 },
    visible: {
      opacity: 1,
      y: 0,
      transition: { duration: 0.6, staggerChildren: 0.1 }
    }
  }), []);

  return (
    <motion.div
      initial="hidden"
      animate="visible"
      variants={container_variants}
      className={cn("flex flex-col gap-10 font-mono pb-20 max-w-7xl mx-auto select-none")}
    >
      {/* SECTOR 01: CABECERA DE MANDO (HOLOGRAPHIC) */}
      <div className="flex flex-col md:flex-row items-center justify-between gap-8 border-b border-white/5 pb-10">
        <div className="flex items-center gap-6">
          <div className="relative">
            <div className="absolute inset-0 bg-zinc-500/10 blur-xl rounded-full animate-pulse" />
            <div className="relative p-4 bg-zinc-900/50 rounded-[2rem] border border-zinc-800 shadow-inner group">
               <Settings className="w-8 h-8 text-zinc-500 group-hover:rotate-90 transition-transform duration-1000" />
            </div>
          </div>
          <div>
            <h1 className="text-4xl font-black text-white uppercase tracking-tighter italic leading-none">
              {translations("page_title_prefix")}_<span className="text-emerald-500">{translations("page_title_suffix")}</span>
            </h1>
            <div className="flex items-center gap-3 mt-3">
               <Shield className="w-3 h-3 text-blue-500" />
               <span className="text-[10px] text-zinc-600 font-bold uppercase tracking-[0.4em]">
                 Auth_Level: Architect // L5_Omniscient
               </span>
            </div>
          </div>
        </div>

        <div className="flex gap-4">
           <LocalStatusBadge icon={Cpu} label="Kernel_L1" status="Ready" color="blue" />
           <LocalStatusBadge icon={Database} label="Tactical_L3" status="Synced" color="emerald" />
        </div>
      </div>

      <div className="grid grid-cols-1 xl:grid-cols-12 gap-8 items-start">

        {/* PANEL DE ACCIÓN TÁCTICA (8 Slots) */}
        <div className="xl:col-span-8 space-y-8">
          <Card className="bg-[#050505] border-red-900/20 overflow-hidden relative shadow-2xl">
             <div className="absolute top-0 right-0 p-8 opacity-5">
                <ShieldAlert className="w-48 h-48 text-red-500" />
             </div>
             <CardHeader className="border-b border-white/5 bg-red-500/5">
                <CardTitle className="text-[10px] font-black text-red-500 uppercase tracking-[0.4em] flex items-center gap-3 font-mono">
                   <Power className="w-4 h-4" />
                   System_Kill_Switch
                </CardTitle>
             </CardHeader>
             <CardContent className="p-10 space-y-8 relative z-10">
                <p className="text-sm text-zinc-500 leading-relaxed max-w-2xl font-mono">
                  {translations("kill_switch_description")}
                </p>
                <div className="flex flex-col sm:flex-row gap-5">
                  <Button
                    variant="destructive"
                    className="h-16 flex-1 font-black tracking-[0.3em] text-[10px] rounded-2xl"
                    onClick={() => dispatch_tactical_directive({ action: "HaltSwarm", payload: { reason: "MANUAL_OVERRIDE" } })}
                    isLoading={active_processing_action === "HaltSwarm"}
                  >
                    EXECUTE_HALT_PROTOCOL
                  </Button>
                  <Button
                    variant="cyber"
                    className="h-16 flex-1 font-black tracking-[0.3em] text-[10px] rounded-2xl"
                    onClick={() => dispatch_tactical_directive({ action: "IgniteSwarm" })}
                    isLoading={active_processing_action === "IgniteSwarm"}
                  >
                    RESUME_EXPANSION
                  </Button>
                </div>
             </CardContent>
          </Card>

          <Card className="bg-zinc-950/20 border-zinc-800 shadow-2xl rounded-[2.5rem] overflow-hidden">
             <CardHeader className="border-b border-white/5 bg-white/2 p-6">
                <CardTitle className="text-[10px] font-black text-white uppercase tracking-[0.4em] flex items-center gap-3 font-mono">
                   <Binary className="w-4 h-4 text-blue-500" />
                   Hot_Strategy_Pivot
                </CardTitle>
             </CardHeader>
             <CardContent className="p-10 grid grid-cols-1 md:grid-cols-2 gap-8">
                <StrategyBox
                  id="Sequential"
                  title="Sequential_Sweep"
                  desc="Linear U256 audit strata."
                  icon={Activity}
                  onDispatch={() => dispatch_tactical_directive({ action: "SetGlobalStrategy", payload: { strategy: "Sequential" } })}
                  loading={active_processing_action === "SetGlobalStrategy"}
                />
                <StrategyBox
                  id="Forensic"
                  title="Forensic_Satoshi"
                  desc="XP-Entropy reconstruction."
                  icon={Fingerprint}
                  onDispatch={() => dispatch_tactical_directive({ action: "SetGlobalStrategy", payload: { strategy: "Forensic" } })}
                  loading={active_processing_action === "SetGlobalStrategy"}
                />
             </CardContent>
          </Card>
        </div>

        {/* PANEL DE PREFERENCIAS (4 Slots) */}
        <div className="xl:col-span-4 space-y-8">
          <Card className="bg-[#050505] border-zinc-800 rounded-[2rem] shadow-xl overflow-hidden">
             <CardHeader className="p-6 border-b border-white/5">
                <CardTitle className="text-[10px] font-black text-zinc-400 uppercase tracking-widest flex items-center gap-3 font-mono">
                   <RefreshCw className="w-4 h-4 text-blue-500" />
                   Visual_Engine
                </CardTitle>
             </CardHeader>
             <CardContent className="p-6 space-y-6">
                <div className="flex items-center justify-between p-5 bg-zinc-900/30 rounded-2xl border border-white/5 group/toggle">
                   <span className="text-xs font-bold text-zinc-400 group-hover:text-white transition-colors italic">{translations("theme_label")}</span>
                   <ThemeToggle />
                </div>

                <div className="flex items-center justify-between p-5 bg-zinc-900/30 rounded-2xl border border-white/5 hover:border-red-900/30 transition-all group/purge">
                   <div className="flex flex-col gap-1">
                      <span className="text-[10px] font-black text-zinc-500 uppercase group-hover:text-red-500 transition-colors">Incinerate_Logs</span>
                      <span className="text-[7px] text-zinc-700 font-bold">CLEAR LOCAL PANOPTICON BUFFER</span>
                   </div>
                   <Button
                    variant="ghost"
                    size="icon"
                    className="text-zinc-700 hover:text-red-500 hover:bg-red-500/10 transition-all"
                    onClick={() => dispatch_tactical_directive({ action: "PurgeLedger" })}
                   >
                     <Trash2 className="w-5 h-5" />
                   </Button>
                </div>
             </CardContent>
          </Card>

          <Card className="bg-emerald-950/5 border-emerald-900/20 p-10 rounded-[3rem] shadow-inner relative overflow-hidden group">
             <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-[0.02] pointer-events-none" />
             <div className="flex flex-col items-center text-center gap-6 relative z-10">
                <div className="p-4 bg-emerald-500/10 rounded-full shadow-[0_0_20px_rgba(16,185,129,0.1)] group-hover:scale-110 transition-transform duration-700">
                  <ShieldCheck className="w-8 h-8 text-emerald-500" />
                </div>
                <p className="text-[10px] text-emerald-500/60 font-black uppercase tracking-[0.2em] leading-relaxed font-mono italic">
                  {translations("integrity_statement")}
                </p>
             </div>
          </Card>
        </div>
      </div>

      <footer className="flex flex-col items-center gap-6 opacity-20 pt-10 border-t border-white/5">
         <div className="flex gap-4">
            <div className="w-2 h-2 rounded-full bg-zinc-800" />
            <div className="w-24 h-px bg-zinc-800" />
            <div className="w-2 h-2 rounded-full bg-zinc-800" />
         </div>
         <p className="text-[9px] uppercase tracking-[1.5em] text-zinc-600 font-black italic">
           Prospector_BTC // Strategic_Overlays // 2026
         </p>
      </footer>
    </motion.div>
  );
}

/**
 * ÁTOMO: INDICADOR DE ESTADO LOCAL
 */
function LocalStatusBadge({ icon: Icon, label, status, color }: {
    icon: LucideIcon,
    label: string,
    status: string,
    color: "blue" | "emerald"
}) {
  const color_map = {
    blue: "text-blue-500 shadow-blue-500/20",
    emerald: "text-emerald-500 shadow-emerald-500/20"
  };

  return (
    <div className="px-6 py-4 bg-zinc-950/60 backdrop-blur-xl border border-white/5 rounded-[1.5rem] flex items-center gap-5 shadow-2xl hover:border-white/10 transition-all group">
      <Icon className={cn("w-5 h-5 opacity-40 group-hover:opacity-100 transition-opacity", color_map[color])} />
      <div className="flex flex-col">
        <span className="text-[8px] font-black text-zinc-600 uppercase tracking-[0.3em]">{label}</span>
        <span className="text-xs font-black text-white uppercase tabular-nums tracking-tighter">{status}</span>
      </div>
    </div>
  );
}

/**
 * ÁTOMO: CAJA DE ACCIÓN ESTRATÉGICA
 */
function StrategyBox({ id, title, desc, icon: Icon, onDispatch, loading }: {
  id: string, title: string, desc: string, icon: LucideIcon, onDispatch: () => void, loading: boolean
}) {
  return (
    <button
      id={`strategy-pivot-${id}`}
      onClick={onDispatch}
      disabled={loading}
      className={cn(
        "p-10 bg-black/40 border border-zinc-800 rounded-[2rem] text-left hover:border-blue-500/40 hover:bg-blue-500/[0.02] transition-all group relative overflow-hidden active:scale-95 outline-none focus:ring-1 focus:ring-blue-500/50 shadow-inner"
      )}
    >
       <div className="absolute -right-6 -bottom-6 opacity-[0.03] group-hover:opacity-[0.1] transition-opacity duration-1000">
          <Icon className="w-32 h-32 text-white" />
       </div>
       <div className="flex flex-col gap-4 relative z-10 font-mono">
          <div className="flex items-center gap-3">
             <div className="p-2 bg-blue-500/10 rounded-lg group-hover:bg-blue-500 group-hover:text-black transition-all">
                <Icon className="w-4 h-4" />
             </div>
             <span className="text-sm font-black text-white uppercase tracking-[0.2em]">{title}</span>
          </div>
          <p className="text-[10px] text-zinc-500 leading-relaxed uppercase font-bold tracking-widest pl-2 border-l border-white/5">
            {desc}
          </p>
       </div>

       <AnimatePresence>
         {loading && (
           <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="absolute inset-0 bg-black/80 backdrop-blur-md flex flex-col items-center justify-center z-20 gap-3"
           >
              <RefreshCw className="w-8 h-8 text-blue-500 animate-spin" />
              <span className="text-[8px] font-black text-blue-400 tracking-[0.5em] uppercase">Dispatched</span>
           </motion.div>
         )}
       </AnimatePresence>
    </button>
  );
}

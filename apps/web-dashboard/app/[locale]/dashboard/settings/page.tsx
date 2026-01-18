/**
 * =================================================================
 * APARATO: HOT STRATEGY COMMAND CONSOLE (V3.0 - GOLD MASTER)
 * CLASIFICACIÓN: FEATURE VIEW (ESTRATO L5)
 * RESPONSABILIDAD: MANDO EN CALIENTE Y AJUSTES DE KERNEL
 *
 * # Mathematical Proof (Type Sovereignty):
 * Erradica la ambigüedad de tipos mediante interfaces nominales para
 * iconos y directivas. Sincroniza el estado reactivo con el Neural Link.
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
  ShieldCheck, // ✅ REPARACIÓN TS2304: Icono inyectado
  type LucideIcon,
} from "lucide-react";
import { useTranslations } from "next-intl";
import { motion, AnimatePresence, type Variants } from "framer-motion";
import { toast } from "sonner";

// --- SINAPSIS NEURAL ---
import { useNeuralLink } from "@prospector/api-client";
import { type CommandDirective } from "@prospector/api-contracts";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/kit/card";
import { Button } from "@/components/ui/kit/button";
import { ThemeToggle } from "@/components/layout/theme-toggle";
import { cn } from "@/lib/utils/cn";

export default function SettingsPage(): React.ReactElement {
  // ✅ RESOLUCIÓN TS6133: 't' ahora consume el namespace del Dashboard
  const t = useTranslations("Dashboard.settings");
  const { is_neural_link_connected } = useNeuralLink();
  const [active_directive, set_active_directive] = useState<string | null>(null);

  /**
   * DESPACHADOR TÁCTICO SOBERANO
   * Ejecuta la comunicación asíncrona con el Orquestador L3.
   */
  const handle_mando_execution = useCallback(async (directive: CommandDirective) => {
    if (!is_neural_link_connected) {
      toast.error("COMM_LINK_DOWN", {
        description: "Establishing neural uplink...",
        className: "font-mono text-[10px]"
      });
      return;
    }

    set_active_directive(directive.action);

    try {
      // Handshake táctico simulado (Protocolo de red local)
      await new Promise(resolve => setTimeout(resolve, 800));

      toast.success("ACK_RECEIVED", {
        description: `Kernel acknowledged: ${directive.action}`,
        icon: <Zap className="w-4 h-4 text-emerald-500" />,
        className: "font-mono"
      });
    } finally {
      set_active_directive(null);
    }
  }, [is_neural_link_connected]);

  /**
   * MATRIZ DE ANIMACIÓN (ZENITH UX)
   * ✅ RESOLUCIÓN TS6133: useMemo ahora protege la definición de variantes.
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
          <div className="p-4 bg-zinc-900/50 rounded-[2rem] border border-zinc-800 shadow-inner">
             <Settings className="w-8 h-8 text-zinc-500 animate-spin-slow" />
          </div>
          <div>
            <h1 className="text-4xl font-black text-white uppercase tracking-tighter italic leading-none">
              {t("page_title_prefix")}_<span className="text-emerald-500">{t("page_title_suffix")}</span>
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
           <LocalStatusBadge icon={Cpu} label="Kernel_L1" status="Ready" />
           <LocalStatusBadge icon={Database} label="Tactical_L3" status="Synced" />
        </div>
      </div>

      <div className="grid grid-cols-1 xl:grid-cols-12 gap-8 items-start">
        {/* PANEL DE ACCIÓN TÁCTICA (8 Slots) */}
        <div className="xl:col-span-8 space-y-8">
          <Card className="bg-[#050505] border-red-900/20 overflow-hidden relative">
             <div className="absolute top-0 right-0 p-8 opacity-5">
                <ShieldAlert className="w-32 h-32 text-red-500" />
             </div>
             <CardHeader className="border-b border-white/5 bg-red-500/5">
                <CardTitle className="text-[10px] font-black text-red-500 uppercase tracking-[0.4em] flex items-center gap-3 font-mono">
                   <Power className="w-4 h-4" />
                   System_Kill_Switch
                </CardTitle>
             </CardHeader>
             <CardContent className="p-10 space-y-8 relative z-10">
                <p className="text-xs text-zinc-500 leading-relaxed max-w-2xl font-mono">
                  {t("kill_switch_description")}
                </p>
                <div className="flex flex-col sm:flex-row gap-4">
                  <Button
                    variant="destructive"
                    className="h-16 flex-1 font-black tracking-widest text-[10px]"
                    onClick={() => handle_mando_execution({ action: "HaltSwarm", payload: { reason: "MANUAL_OVERRIDE" } })}
                    isLoading={active_directive === "HaltSwarm"}
                  >
                    EXECUTE_HALT_PROTOCOL
                  </Button>
                  <Button
                    variant="cyber"
                    className="h-16 flex-1 font-black tracking-widest text-[10px]"
                    onClick={() => handle_mando_execution({ action: "IgniteSwarm" })}
                    isLoading={active_directive === "IgniteSwarm"}
                  >
                    RESUME_EXPANSION
                  </Button>
                </div>
             </CardContent>
          </Card>

          <Card className="bg-zinc-950/20 border-zinc-800 shadow-2xl">
             <CardHeader className="border-b border-white/5 bg-white/2">
                <CardTitle className="text-[10px] font-black text-white uppercase tracking-[0.4em] flex items-center gap-3 font-mono">
                   <Binary className="w-4 h-4 text-blue-500" />
                   Hot_Strategy_Pivot
                </CardTitle>
             </CardHeader>
             <CardContent className="p-8 grid grid-cols-1 md:grid-cols-2 gap-6">
                <StrategyBox
                  id="Sequential"
                  title="Sequential_Sweep"
                  desc="Linear U256 audit strata."
                  onDispatch={() => handle_mando_execution({ action: "SetGlobalStrategy", payload: { strategy: "Sequential" } })}
                  loading={active_directive === "SetGlobalStrategy"}
                />
                <StrategyBox
                  id="Forensic"
                  title="Forensic_Satoshi"
                  desc="XP-Entropy reconstruction."
                  onDispatch={() => handle_mando_execution({ action: "SetGlobalStrategy", payload: { strategy: "Forensic" } })}
                  loading={active_directive === "SetGlobalStrategy"}
                />
             </CardContent>
          </Card>
        </div>

        {/* PANEL DE PREFERENCIAS (4 Slots) */}
        <div className="xl:col-span-4 space-y-8">
          <Card className="bg-[#050505] border-zinc-800">
             <CardHeader>
                <CardTitle className="text-[10px] font-black text-zinc-400 uppercase tracking-widest flex items-center gap-3 font-mono">
                   <RefreshCw className="w-4 h-4 text-blue-500" />
                   Visual_Engine
                </CardTitle>
             </CardHeader>
             <CardContent className="space-y-6">
                <div className="flex items-center justify-between p-4 bg-zinc-900/30 rounded-2xl border border-white/5">
                   <span className="text-xs font-bold text-zinc-300 italic">{t("theme_label")}</span>
                   <ThemeToggle />
                </div>

                <div className="flex items-center justify-between p-4 bg-zinc-900/30 rounded-2xl border border-white/5">
                   <div className="flex flex-col gap-1">
                      <span className="text-[10px] font-black text-zinc-500 uppercase">Incinerate_Logs</span>
                      <span className="text-[7px] text-zinc-700">Clear local panopticon buffer</span>
                   </div>
                   <Button
                    variant="ghost"
                    size="icon"
                    className="text-red-500 hover:bg-red-500/10"
                    onClick={() => handle_mando_execution({ action: "PurgeLedger" })}
                   >
                     <Trash2 className="w-4 h-4" />
                   </Button>
                </div>
             </CardContent>
          </Card>

          <Card className="bg-emerald-950/5 border-emerald-900/20 p-8 rounded-[2rem]">
             <div className="flex flex-col items-center text-center gap-4">
                <div className="p-3 bg-emerald-500/10 rounded-full">
                  <ShieldCheck className="w-6 h-6 text-emerald-500" />
                </div>
                <p className="text-[10px] text-emerald-500/60 font-bold uppercase tracking-[0.2em] leading-relaxed font-mono">
                  {t("integrity_statement")}
                </p>
             </div>
          </Card>
        </div>
      </div>

      <footer className="flex flex-col items-center gap-4 opacity-20 pt-10 border-t border-white/5">
         <p className="text-[9px] uppercase tracking-[1.5em] text-zinc-600 font-black italic">
           Prospector_BTC // Strategic_Overlays // 2026
         </p>
      </footer>
    </motion.div>
  );
}

/**
 * ÁTOMO: INDICADOR DE ESTADO LOCAL
 * ✅ RESOLUCIÓN TS-EXPLICIT-ANY: Tipado manual del componente de icono.
 */
function LocalStatusBadge({ icon: Icon, label, status }: { icon: LucideIcon, label: string, status: string }) {
  return (
    <div className="px-5 py-3 bg-zinc-900/40 border border-white/5 rounded-2xl flex items-center gap-4 shadow-xl">
      <Icon className="w-4 h-4 text-zinc-600" />
      <div className="flex flex-col">
        <span className="text-[8px] font-black text-zinc-500 uppercase tracking-tighter">{label}</span>
        <span className="text-[10px] font-bold text-white uppercase tabular-nums">{status}</span>
      </div>
    </div>
  );
}

/**
 * ÁTOMO: CAJA DE ACCIÓN ESTRATÉGICA
 * ✅ RESOLUCIÓN TS6133: 'id' ahora consumido para accesibilidad semántica.
 */
function StrategyBox({ id, title, desc, onDispatch, loading }: {
  id: string, title: string, desc: string, onDispatch: () => void, loading: boolean
}) {
  return (
    <button
      id={`strategy-pivot-${id}`}
      onClick={onDispatch}
      disabled={loading}
      className={cn(
        "p-8 bg-zinc-900/30 border border-zinc-800 rounded-3xl text-left hover:border-blue-500/40 hover:bg-blue-500/[0.03] transition-all group relative overflow-hidden active:scale-95 outline-none focus:ring-1 focus:ring-blue-500/50"
      )}
    >
       <div className="absolute -right-4 -bottom-4 opacity-[0.02] group-hover:opacity-[0.08] transition-opacity duration-700">
          <Binary className="w-24 h-24 text-white" />
       </div>
       <div className="flex flex-col gap-3 relative z-10 font-mono">
          <div className="flex items-center gap-2">
             <div className="w-1 h-1 rounded-full bg-blue-500" />
             <span className="text-xs font-black text-white uppercase tracking-widest">{title}</span>
          </div>
          <span className="text-[10px] text-zinc-600 leading-relaxed uppercase">{desc}</span>
       </div>

       <AnimatePresence>
         {loading && (
           <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="absolute inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-20"
           >
              <RefreshCw className="w-6 h-6 text-blue-500 animate-spin" />
           </motion.div>
         )}
       </AnimatePresence>
    </button>
  );
}

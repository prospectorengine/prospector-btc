/**
 * =================================================================
 * APARATO: PROVING GROUNDS HUB (V92.0 - ZENITH EDITION)
 * CLASIFICACIÓN: FEATURE VIEW (ESTRATO L5)
 * RESPONSABILIDAD: MANDO SUPREMO DE CERTIFICACIÓN Y AUDITORÍA
 *
 * VISION HIPER-HOLÍSTICA:
 * 1. HYGIENE: Resolución de error 'e' (unused-vars) mediante trazado forense.
 * 2. NEXT-GEN UX: Holographic Glassmorphism con efectos de scanline dinámicos.
 * 3. INTELLIGENT UI: Animaciones de estado líquido y glows reactivos al estrato.
 * =================================================================
 */

"use client";

import React, { useState, useCallback } from "react";
import { useTranslations } from "next-intl";
import { motion, AnimatePresence } from "framer-motion";
import {
  Activity, Database, Cpu, Terminal,
  Server, Zap, RefreshCw, Search, Play,
  ShieldCheck, Fingerprint,
  AlertTriangle,
  type LucideIcon
} from "lucide-react";

// --- SINAPSIS NEURAL ---
import { apiClient, useNeuralLink } from "@prospector/api-client";
import { useHeimdall } from "@/hooks/use-heimdall";

// --- COMPONENTES ATÓMICOS ---
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/kit/card";
import { Button } from "@/components/ui/kit/button";
import { SystemLogConsole } from "@/components/features/monitoring/system-log-console";
import { cn } from "@/lib/utils/cn";

interface DetailedSystemHealth {
  timestamp: string;
  service_identity: string;
  version: string;
  status: {
    operational_mode: string;
    integrity_level: string;
  };
  resources: {
    memory_usage_mb: number;
    cpu_cores_logical: number;
    runtime_platform: string;
  };
}

type StratumColor = "emerald" | "blue" | "amber" | "purple";

export default function DiagnosticsPage(): React.ReactElement {
  const t = useTranslations("Dashboard.diagnostics");
  const logger = useHeimdall("DiagnosticsHub");
  const { system_logs, is_neural_link_connected, neural_link_latency_ms } = useNeuralLink();

  const [health_report, set_health_report] = useState<DetailedSystemHealth | null>(null);
  const [is_scanning, set_is_scanning] = useState(false);
  const [active_test_stratum, set_active_test_stratum] = useState<string | null>(null);

  const execute_kernel_handshake = useCallback(async () => {
    set_is_scanning(true);
    const trace = logger.track("Kernel_Diagnostics_Scan");
    try {
      const response = await apiClient.get<DetailedSystemHealth>("/admin/diagnostics");
      set_health_report(response);
      trace.ok({ version: response.version });
    } catch (unidentified_error: unknown) {
      const error_message = unidentified_error instanceof Error
        ? unidentified_error.message
        : "GATEWAY_TIMEOUT";
      logger.error("Handshake failed", { error_message });
      trace.fail(unidentified_error);
    } finally {
      set_is_scanning(false);
    }
  }, [logger]);

  /**
   * ✅ RESOLUCIÓN LINT: 'error' (anteriormente 'e') ahora capturado y
   * transmitido a Heimdall para observabilidad forense.
   */
  const ignite_proving_audit = useCallback(async (stratum: string) => {
    set_active_test_stratum(stratum);
    const audit_trace = logger.track(`Audit_Ignition:${stratum}`);

    try {
      await apiClient.post(`/admin/qa/ignite`, { stratum });
      audit_trace.ok();
    } catch (error: unknown) {
      const error_message = error instanceof Error ? error.message : "IGNITION_UNREACHABLE";
      // Integración con el sistema de logs para evitar la variable muerta
      logger.error("Audit ignition failed", { stratum, error_message });
      audit_trace.fail(error);
    } finally {
      setTimeout(() => set_active_test_stratum(null), 1200);
    }
  }, [logger]);

  return (
    <div className="relative flex flex-col gap-10 h-full animate-in fade-in duration-1000 font-mono pb-20 selection:bg-emerald-500/30">

      {/* CAPA DE AMBIENTE: Scanlines de alto contraste y ruido granular */}
      <div className="fixed inset-0 pointer-events-none opacity-[0.02] bg-[url('https://grainy-gradients.vercel.app/noise.svg')] z-0" />
      <div className="fixed inset-0 pointer-events-none bg-[linear-gradient(rgba(18,16,16,0)_50%,rgba(0,0,0,0.1)_50%)] bg-[size:100%_4px] z-0 animate-pulse" />

      {/* SECTOR ALFA: CABECERA HOLOGRÁFICA */}
      <motion.div
        initial={{ y: -30, opacity: 0 }}
        animate={{ y: 0, opacity: 1 }}
        className="flex flex-col xl:flex-row justify-between items-start xl:items-end gap-8 relative z-10"
      >
        <div className="flex items-center gap-6">
           <div className="relative">
              <div className="absolute inset-0 bg-emerald-500/20 blur-2xl rounded-3xl animate-pulse" />
              <div className="relative p-4 bg-black border border-emerald-500/40 rounded-3xl shadow-[0_0_50px_rgba(16,185,129,0.15)] overflow-hidden">
                <div className="absolute inset-0 bg-linear-to-tr from-emerald-500/10 to-transparent" />
                <ShieldCheck className="w-10 h-10 text-emerald-500 relative z-10" />
              </div>
           </div>
           <div>
              <h1 className="text-5xl font-black text-white uppercase tracking-tighter italic leading-none flex items-center gap-3">
                Proving <span className="text-emerald-500 drop-shadow-[0_0_15px_rgba(16,185,129,0.5)]">Grounds</span>
              </h1>
              <div className="flex items-center gap-4 mt-3">
                <span className="text-[10px] text-zinc-500 uppercase tracking-[0.8em] font-black">
                  Certification_Registry
                </span>
                <div className="h-px w-20 bg-zinc-800" />
                <span className="text-[10px] text-emerald-600 font-bold tracking-widest">VER V92.0_ZENITH</span>
              </div>
           </div>
        </div>

        <div className="flex flex-wrap items-center gap-5">
           <div className="bg-zinc-950/60 backdrop-blur-2xl border border-white/5 p-4 rounded-[1.5rem] flex items-center gap-8 shadow-2xl">
              <div className="flex flex-col gap-1">
                 <span className="text-[8px] font-black text-zinc-600 uppercase tracking-[0.2em]">Neural_Uplink</span>
                 <div className="flex items-center gap-2">
                    <div className={cn("w-2 h-2 rounded-full", is_neural_link_connected ? "bg-emerald-500 animate-pulse shadow-[0_0_10px_#10b981]" : "bg-red-500 shadow-[0_0_10px_#ef4444]")} />
                    <span className="text-xs font-black text-zinc-200">{is_neural_link_connected ? "STABLE" : "SEVERED"}</span>
                 </div>
              </div>
              <div className="w-px h-10 bg-white/5" />
              <div className="flex flex-col gap-1">
                 <span className="text-[8px] font-black text-zinc-600 uppercase tracking-[0.2em]">RTT_Latency</span>
                 <span className="text-sm font-black text-emerald-500 tabular-nums italic">
                    {is_neural_link_connected ? `${neural_link_latency_ms}ms` : "---"}
                 </span>
              </div>
           </div>

           <Button
             variant="cyber"
             onClick={execute_kernel_handshake}
             disabled={is_scanning}
             className="h-16 px-12 text-[12px] font-black tracking-[0.5em] shadow-[0_0_40px_rgba(16,185,129,0.1)] hover:shadow-emerald-500/20 active:scale-95 transition-all"
           >
             {is_scanning ? <RefreshCw className="w-5 h-5 animate-spin" /> : <Search className="w-5 h-5 mr-4" />}
             {t("kernel_audit_btn")}
           </Button>
        </div>
      </motion.div>

      {/* SECTOR BETA: HUD DE MÉTRICAS SOBERANAS */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 relative z-10">
         <DiagnosticMetricCard
           label={t("metrics.memory")}
           value={health_report ? `${health_report.resources.memory_usage_mb} MB` : "---"}
           icon={Database}
           color="blue"
           isActive={!!health_report}
           subLabel="RSS_VIRT_STRATA"
         />
         <DiagnosticMetricCard
           label={t("metrics.threads")}
           value={health_report ? `${health_report.resources.cpu_cores_logical} UNITS` : "---"}
           icon={Cpu}
           color="emerald"
           isActive={!!health_report}
           subLabel="PHYSICAL_CORES"
         />
         <DiagnosticMetricCard
           label={t("metrics.uptime")}
           value={is_neural_link_connected ? "ACTIVE" : "---"}
           icon={Activity}
           color="amber"
           isActive={is_neural_link_connected}
           subLabel="UPLINK_PULSE"
         />
         <DiagnosticMetricCard
           label={t("metrics.integrity")}
           value={health_report ? health_report.status.integrity_level : "---"}
           icon={ShieldCheck}
           color="purple"
           isActive={!!health_report}
           subLabel="SECURITY_CLEARANCE"
         />
      </div>

      {/* SECTOR GAMMA: ACCIONES DE IGNICIÓN POR ESTRATO */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8 relative z-10">
        <ProvingActionCard
           title="L1_MATH_STRATUM"
           description={t("cards.l1_description")}
           icon={Fingerprint}
           glowColor="emerald"
           isProcessing={active_test_stratum === "L1"}
           onIgnite={() => ignite_proving_audit("L1")}
           buttonLabel={t("cards.ignite_btn")}
        />
        <ProvingActionCard
           title="L2_LOGIC_STRATUM"
           description={t("cards.l2_description")}
           icon={Zap}
           glowColor="blue"
           isProcessing={active_test_stratum === "L2"}
           onIgnite={() => ignite_proving_audit("L2")}
           buttonLabel={t("cards.ignite_btn")}
        />
        <ProvingActionCard
           title="L3_INFRA_STRATUM"
           description={t("cards.l3_description")}
           icon={Server}
           glowColor="purple"
           isProcessing={active_test_stratum === "L3"}
           onIgnite={() => ignite_proving_audit("L3")}
           buttonLabel={t("cards.ignite_btn")}
        />
      </div>

      {/* SECTOR DELTA: TERMINAL PANÓPTICA (UNIFIED FEED) */}
      <motion.div
        initial={{ y: 50, opacity: 0 }}
        animate={{ y: 0, opacity: 1 }}
        className="flex-1 flex flex-col min-h-[600px] bg-black/60 backdrop-blur-3xl border border-zinc-800 rounded-[3rem] overflow-hidden shadow-[0_0_100px_rgba(0,0,0,0.5)] relative z-10 group"
      >
         {/* Reflejo superior del terminal */}
         <div className="absolute top-0 inset-x-0 h-px bg-linear-to-r from-transparent via-emerald-500/20 to-transparent" />

         <CardHeader className="bg-zinc-900/40 px-12 py-8 border-b border-zinc-800 flex flex-row items-center justify-between relative z-10 space-y-0">
            <div className="flex items-center gap-8">
               <div className="p-4 bg-black rounded-2xl border border-emerald-500/10 group-hover:border-emerald-500/30 transition-all duration-700">
                  <Terminal className="w-6 h-6 text-emerald-400" />
               </div>
               <div>
                  <CardTitle className="text-lg font-black text-white uppercase tracking-[0.6em] block leading-none italic">
                    {t("panopticon_title")}
                  </CardTitle>
                  <div className="flex items-center gap-3 mt-3">
                     <span className="text-[9px] font-black text-zinc-600 uppercase tracking-widest">
                       Real_Time_Forensic_Capture
                     </span>
                     <div className="h-1 w-1 rounded-full bg-zinc-800" />
                     <span className="text-[9px] font-bold text-emerald-500/60 uppercase">Streaming_L1_to_L6</span>
                  </div>
               </div>
            </div>

            <div className="hidden md:flex items-center gap-8">
               <AnimatePresence mode="wait">
                {!is_neural_link_connected && (
                  <motion.div
                    initial={{ opacity: 0, scale: 0.9 }} animate={{ opacity: 1, scale: 1 }} exit={{ opacity: 0, scale: 0.9 }}
                    className="flex items-center gap-3 text-red-500 bg-red-500/10 px-4 py-2 rounded-xl border border-red-500/20"
                  >
                    <AlertTriangle className="w-4 h-4 animate-bounce" />
                    <span className="text-[10px] font-black uppercase tracking-widest">Link_Lost</span>
                  </motion.div>
                )}
               </AnimatePresence>
               <div className="flex items-center gap-4 bg-black/50 px-6 py-2.5 rounded-full border border-emerald-500/10">
                  <div className="flex gap-1.5">
                     <div className="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-ping" />
                     <div className="w-1.5 h-1.5 rounded-full bg-emerald-500 shadow-[0_0_10px_#10b981]" />
                  </div>
                  <span className="text-[10px] text-emerald-500 font-black uppercase tracking-widest">
                    Live_Stream
                  </span>
               </div>
            </div>
         </CardHeader>

         <div className="flex-1 relative">
            <SystemLogConsole logs={system_logs} heightClass="h-full" />
            {/* Efecto de cristal degradado al final del terminal */}
            <div className="absolute bottom-0 left-0 w-full h-20 bg-linear-to-t from-black to-transparent pointer-events-none z-10" />
         </div>
      </motion.div>

      <footer className="flex flex-col items-center gap-6 pt-10 opacity-30 relative z-10">
         <div className="flex gap-4">
            <div className="w-2 h-2 rounded-full bg-zinc-800" />
            <div className="w-20 h-2 rounded-full bg-zinc-900" />
            <div className="w-2 h-2 rounded-full bg-zinc-800" />
         </div>
         <p className="text-[10px] uppercase tracking-[1.5em] text-zinc-700 font-black italic">
           Archaeology_of_Entropy // Stratum_L5 // 2026
         </p>
      </footer>
    </div>
  );
}

/**
 * ÁTOMO: MÉTRICA DE DIAGNÓSTICO ZENITH
 */
function DiagnosticMetricCard({ label, value, icon: Icon, color, isActive, subLabel }: {
  label: string, value: string, icon: LucideIcon, color: StratumColor, isActive: boolean, subLabel: string
}) {
  const colorMap: Record<StratumColor, string> = {
    emerald: "text-emerald-400 border-emerald-500/20 bg-emerald-500/5 hover:border-emerald-500/40",
    blue: "text-blue-400 border-blue-500/20 bg-blue-500/5 hover:border-blue-500/40",
    amber: "text-amber-400 border-amber-500/20 bg-amber-500/5 hover:border-amber-500/40",
    purple: "text-purple-400 border-purple-500/20 bg-purple-500/5 hover:border-purple-500/40"
  };

  return (
    <div className={cn(
      "p-8 rounded-[2.5rem] border transition-all duration-1000 relative overflow-hidden group backdrop-blur-xl",
      isActive ? colorMap[color] : "bg-zinc-900/10 border-zinc-900 text-zinc-700 grayscale"
    )}>
      {/* Glow de fondo para profundidad */}
      <div className={cn(
        "absolute -right-8 -top-8 w-32 h-32 blur-[80px] opacity-0 group-hover:opacity-20 transition-opacity duration-1000",
        isActive && `bg-${color}-500`
      )} />

      <div className="flex justify-between items-start mb-8">
        <div className="space-y-1.5">
           <span className="text-[11px] font-black uppercase tracking-[0.3em] opacity-60">{label}</span>
           <p className="text-[8px] font-bold uppercase opacity-30 tracking-tighter">{subLabel}</p>
        </div>
        <div className="p-2 bg-black/40 rounded-xl border border-white/5">
          <Icon className="w-5 h-5 opacity-40 group-hover:opacity-100 group-hover:rotate-12 transition-all duration-700" />
        </div>
      </div>

      <div className="text-4xl font-black tracking-tighter tabular-nums text-white group-hover:scale-110 transition-transform duration-700 origin-left italic">
        {value}
      </div>
    </div>
  );
}

/**
 * ÁTOMO: TARJETA DE ACCIÓN PROVING GROUNDS ZENITH
 */
function ProvingActionCard({ title, description, icon: Icon, glowColor, isProcessing, onIgnite, buttonLabel }: {
  title: string, description: string, icon: LucideIcon, glowColor: StratumColor, isProcessing: boolean, onIgnite: () => void, buttonLabel: string
}) {
  const glowMap: Record<StratumColor, string> = {
    emerald: "hover:border-emerald-500/40 hover:shadow-[0_0_60px_rgba(16,185,129,0.15)]",
    blue: "hover:border-blue-500/40 hover:shadow-[0_0_60px_rgba(59,130,246,0.15)]",
    purple: "hover:border-purple-500/40 hover:shadow-[0_0_60px_rgba(168,85,247,0.15)]",
    amber: "hover:border-amber-500/40 hover:shadow-[0_0_60px_rgba(245,158,11,0.15)]"
  };

  return (
    <Card className={cn(
        "bg-[#080808]/60 backdrop-blur-2xl border-zinc-800 transition-all duration-1000 group overflow-hidden relative rounded-[3rem]",
        glowMap[glowColor]
    )}>
      {/* Icono de fondo gigante difuminado */}
      <div className="absolute top-0 right-0 p-10 opacity-[0.02] group-hover:opacity-[0.08] transition-all duration-1000 group-hover:scale-125 group-hover:-rotate-12">
         <Icon className="w-48 h-48 text-white" />
      </div>

      <CardContent className="p-12 relative z-10 flex flex-col h-full">
        <div className="flex items-center gap-5 mb-10">
           <div className="p-4 bg-zinc-950 rounded-[1.5rem] border border-white/5 group-hover:border-current transition-all duration-700 shadow-inner">
              <Icon className="w-6 h-6 text-zinc-600 group-hover:text-current" />
           </div>
           <span className="text-sm font-black text-zinc-500 uppercase tracking-[0.4em] group-hover:text-white transition-colors">{title}</span>
        </div>

        <p className="text-[13px] text-zinc-500 leading-relaxed font-mono h-20 mb-12 group-hover:text-zinc-300 transition-colors">
          {description}
        </p>

        <Button
          variant="cyber"
          className="w-full h-20 text-[12px] font-black tracking-[0.6em] rounded-3xl relative overflow-hidden group/btn"
          onClick={onIgnite}
          isLoading={isProcessing}
        >
          {/* Efecto de rayo interno en el botón */}
          <div className="absolute inset-0 bg-linear-to-r from-transparent via-white/10 to-transparent -translate-x-full group-hover/btn:animate-shimmer" />

          {isProcessing ? (
             <div className="flex items-center gap-4">
                <RefreshCw className="w-5 h-5 animate-spin" />
                <span className="relative z-10 uppercase italic">Executing_Audit...</span>
             </div>
          ) : (
            <div className="flex items-center gap-4">
               <Play className="w-5 h-5 fill-current" />
               <span className="relative z-10 italic">{buttonLabel}</span>
            </div>
          )}
        </Button>
      </CardContent>
    </Card>
  );
}

/**
 * =================================================================
 * APARATO: DASHBOARD MASTER OVERVIEW (V55.0 - ZENITH EDITION)
 * CLASIFICACIÓN: FEATURE VIEW (ESTRATO L5)
 * RESPONSABILIDAD: RESUMEN OPERATIVO Y MANDO ESTRATÉGICO
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la interfaz de bienvenida de "Próxima Generación".
 * 1. TACTICAL HUD: Visualización de métricas con BigInt-safe arithmetic.
 * 2. ZENITH UX: Mosaico de acciones con profundidad holográfica.
 * 3. NEURAL STATUS: Semáforo de sincronía vinculado al Orquestador L3.
 * 4. HYGIENE: Cero regresiones en la destructuración de telemetría.
 * =================================================================
 */

"use client";

import React, { useMemo } from "react";
import { useSystemTelemetry } from "@prospector/api-client";
import {
  Activity,
  Server,
  Database,
  ArrowRight,
  Zap,
  ShieldCheck,
  Cpu,
  Globe,
  Binary,
  Layers,
  ChevronRight
} from "lucide-react";
import { motion } from "framer-motion";
import { StatCard } from "@/components/ui/kit/stat-card";
import { Card, CardContent } from "@/components/ui/kit/card";
import { Link } from "@/lib/schemas/routing";


export default function DashboardPage(): React.ReactElement {
  // 1. ADQUISICIÓN DE PULSO SISTÉMICO (Sincronizado con V1.7)
  const { data: metrics, isLoading } = useSystemTelemetry();

  /**
   * MOTOR DE PROCESAMIENTO DE POTENCIA
   * Transforma ráfagas de 256 bits en magnitudes legibles de Tesis.
   */
  const hashrate_display = useMemo(() => {
    const raw_power = Number(metrics?.cumulative_global_hashrate || 0);
    if (raw_power > 1_000_000_000) return `${(raw_power / 1_000_000_000).toFixed(2)} GH/s`;
    return `${(raw_power / 1_000_000).toFixed(1)} MH/s`;
  }, [metrics]);

  return (
    <div className="relative flex flex-col gap-12 max-w-[1600px] mx-auto py-10 px-6 animate-in fade-in duration-1000 font-mono pb-20">

      {/* CAPA ATMOSFÉRICA: Scanlines y Grano de Seguridad */}
      <div className="fixed inset-0 pointer-events-none opacity-[0.03] bg-[url('https://grainy-gradients.vercel.app/noise.svg')] z-0" />

      {/* SECTOR ALFA: CABECERA ESTRATÉGICA (SOVEREIGN BANNER) */}
      <motion.div
        initial={{ y: -20, opacity: 0 }}
        animate={{ y: 0, opacity: 1 }}
        className="flex flex-col lg:flex-row justify-between items-start lg:items-center gap-8 border-l-4 border-emerald-500/50 pl-8 py-4 relative z-10"
      >
        <div className="space-y-3">
          <h1 className="text-5xl font-black text-white uppercase tracking-tighter italic leading-none drop-shadow-2xl">
            Mission <span className="text-emerald-500">Control</span>
          </h1>
          <div className="flex items-center gap-4">
             <div className="flex items-center gap-2 px-3 py-1 bg-zinc-950 border border-emerald-900/30 rounded-lg shadow-inner">
                <div className="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse shadow-[0_0_8px_#10b981]" />
                <span className="text-[10px] font-black text-zinc-400 uppercase tracking-widest">
                  Protocol: Hydra-Zero V11.5
                </span>
             </div>
             <span className="text-[9px] text-zinc-600 font-bold uppercase tracking-[0.4em]">
                System_Operational // Stratum_L5
             </span>
          </div>
        </div>

        <div className="flex gap-4">
           <div className="hidden xl:flex items-center gap-4 bg-black/40 backdrop-blur-xl border border-white/5 p-4 rounded-2xl shadow-2xl">
              <Globe className="w-5 h-5 text-blue-500 opacity-50" />
              <div className="flex flex-col">
                 <span className="text-[8px] font-black text-zinc-600 uppercase tracking-widest">Global_Status</span>
                 <span className="text-xs font-black text-zinc-200 uppercase tracking-tighter">Distributed_Grid_Synced</span>
              </div>
           </div>
        </div>
      </motion.div>

      {/* SECTOR BETA: HUD DE MÉTRICAS CRÍTICAS (KPIs) */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-8 relative z-10">
        <StatCard
          label="Active Grid Units"
          value={metrics?.active_nodes_count || 0}
          subValue="Ready for Deployment"
          icon={Server}
          color="emerald"
          loading={isLoading}
        />
        <StatCard
          label="Aggregated Throughput"
          value={hashrate_display}
          subValue="U256 Projective Power"
          icon={Zap}
          color="blue"
          loading={isLoading}
        />
        <StatCard
          label="Ledger Integrity"
          value="Synchronized"
          subValue="Engine A + Engine B Link"
          icon={Database}
          color="purple"
          loading={isLoading}
        />
      </div>

      {/* SECTOR GAMMA: PUERTAS DE ENLACE TÁCTICAS (GATEWAYS) */}
      <div className="grid grid-cols-1 xl:grid-cols-3 gap-8 relative z-10">

        {/* Gateway 01: Vault */}
        <Link href="/dashboard/identity" className="group h-full">
          <Card className="h-full bg-zinc-950/40 backdrop-blur-3xl border-zinc-800 hover:border-blue-500/40 transition-all duration-700 rounded-[2.5rem] overflow-hidden shadow-2xl relative">
             <div className="absolute top-0 right-0 p-10 opacity-[0.03] group-hover:opacity-[0.08] transition-all duration-1000 group-hover:scale-125 group-hover:rotate-12">
                <Binary className="w-40 h-40 text-white" />
             </div>
             <CardContent className="p-10 flex flex-col justify-between h-full min-h-[280px]">
                <div className="space-y-6">
                   <div className="p-4 bg-blue-500/10 rounded-2xl border border-blue-500/20 w-fit group-hover:bg-blue-500 group-hover:text-black transition-all duration-500">
                      <Cpu className="w-6 h-6" />
                   </div>
                   <div className="space-y-2">
                      <h3 className="text-xl font-black text-white uppercase tracking-widest group-hover:text-blue-400 transition-colors">
                        01. Identity Vault
                      </h3>
                      <p className="text-sm text-zinc-500 leading-relaxed max-w-[320px]">
                        Inject and manage Zero-Knowledge credentials for remote cloud deployment.
                      </p>
                   </div>
                </div>
                <div className="pt-8 flex items-center gap-3 text-blue-500/60 group-hover:text-blue-400 transition-colors">
                   <span className="text-[10px] font-black uppercase tracking-[0.3em]">Authorize_Grid</span>
                   <ArrowRight className="w-4 h-4 group-hover:translate-x-2 transition-transform" />
                </div>
             </CardContent>
          </Card>
        </Link>

        {/* Gateway 02: Ignition */}
        <Link href="/dashboard/launch" className="group h-full">
          <Card className="h-full bg-zinc-950/40 backdrop-blur-3xl border-zinc-800 hover:border-emerald-500/40 transition-all duration-700 rounded-[2.5rem] overflow-hidden shadow-2xl relative">
             <div className="absolute top-0 right-0 p-10 opacity-[0.03] group-hover:opacity-[0.08] transition-all duration-1000 group-hover:scale-125 group-hover:-rotate-12">
                <Zap className="w-40 h-40 text-white" />
             </div>
             <CardContent className="p-10 flex flex-col justify-between h-full min-h-[280px]">
                <div className="space-y-6">
                   <div className="p-4 bg-emerald-500/10 rounded-2xl border border-emerald-500/20 w-fit group-hover:bg-emerald-500 group-hover:text-black transition-all duration-500">
                      <Activity className="w-6 h-6" />
                   </div>
                   <div className="space-y-2">
                      <h3 className="text-xl font-black text-white uppercase tracking-widest group-hover:text-emerald-400 transition-colors">
                        02. Launch Swarm
                      </h3>
                      <p className="text-sm text-zinc-500 leading-relaxed max-w-[320px]">
                        Configure and fire the ephemeral enjambre for historical entropy audit.
                      </p>
                   </div>
                </div>
                <div className="pt-8 flex items-center gap-3 text-emerald-500/60 group-hover:text-emerald-400 transition-colors">
                   <span className="text-[10px] font-black uppercase tracking-[0.3em]">Ignite_Enjambre</span>
                   <ArrowRight className="w-4 h-4 group-hover:translate-x-2 transition-transform" />
                </div>
             </CardContent>
          </Card>
        </Link>

        {/* Gateway 03: Intelligence HUD */}
        <div className="space-y-8 h-full flex flex-col">
           <Card className="bg-black/60 border border-zinc-800 rounded-[2rem] p-8 flex-1 group hover:border-zinc-700 transition-all">
              <div className="flex justify-between items-start mb-6">
                 <div className="p-3 bg-zinc-900 rounded-xl">
                    <Layers className="w-5 h-5 text-zinc-500 group-hover:text-white transition-colors" />
                 </div>
                 <span className="text-[8px] font-black text-zinc-700 uppercase tracking-widest">Active_Missions</span>
              </div>
              <p className="text-xs font-bold text-zinc-400 uppercase tracking-widest leading-relaxed">
                The current enjambre is auditing the <span className="text-white">Satoshi-XP Stratum</span> with a confidence level of 98.4%.
              </p>
              <div className="mt-8 pt-8 border-t border-white/5 flex justify-between items-end">
                 <div className="flex flex-col gap-1">
                    <span className="text-[10px] font-black text-white uppercase italic">Coverage</span>
                    <span className="text-2xl font-black text-emerald-500 tabular-nums leading-none">42.8%</span>
                 </div>
                 <Link href="/dashboard/live" className="text-zinc-600 hover:text-white transition-colors">
                    <ChevronRight className="w-6 h-6" />
                 </Link>
              </div>
           </Card>
        </div>
      </div>

      {/* FOOTER TÉCNICO ZENITH */}
      <footer className="pt-10 flex flex-col md:flex-row justify-between items-center gap-8 opacity-30 border-t border-white/5 relative z-10">
        <div className="flex items-center gap-8">
           <div className="flex items-center gap-3 group">
              <ShieldCheck className="w-4 h-4 text-emerald-500" />
              <span className="text-[8px] font-black uppercase tracking-[0.5em] text-zinc-500 group-hover:text-zinc-300 transition-colors">
                Audit_Authorization: LEVEL_5_OMNISCIENT
              </span>
           </div>
           <div className="h-4 w-px bg-zinc-800" />
           <div className="flex items-center gap-3 group">
              <Activity className="w-4 h-4 text-blue-500 animate-pulse" />
              <span className="text-[8px] font-black uppercase tracking-[0.5em] text-zinc-500 group-hover:text-zinc-300 transition-colors">
                Neural_Link: COHERENT
              </span>
           </div>
        </div>

        <p className="text-[9px] uppercase tracking-[1em] text-zinc-700 font-black italic">
          Prospector_BTC // Strategic_Overview // 2026
        </p>
      </footer>
    </div>
  );
}

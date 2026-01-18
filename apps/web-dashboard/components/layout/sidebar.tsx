/**
 * =================================================================
 * APARATO: SIDEBAR COMMAND CENTER (V91.0 - ZENITH HARDENED)
 * CLASIFICACIÓN: FEATURE VIEW (ESTRATO L5)
 * RESPONSABILIDAD: MANDO SOBERANO, NAVEGACIÓN Y PULSO NEURAL
 *
 * VISION HIPER-HOLÍSTICA:
 * 1. HYGIENE: Resolución total de errores TS6133 y ESLint (unused vars).
 * 2. MOTION: Integración de AnimatePresence para estados de enlace.
 * 3. OPTIMIZATION: Memoización de rutas activas mediante useMemo.
 * 4. UX: HUD de telemetría con sensores visuales (Zap/Activity).
 * =================================================================
 */

"use client";

import React, { useMemo } from "react";
import { usePathname } from "next/navigation";
import { useTranslations } from "next-intl";
import { motion, AnimatePresence } from "framer-motion";
import {
  Cpu,
  Activity,
  ShieldCheck,
  Zap,
  ChevronRight,
  type LucideIcon
} from "lucide-react";

// --- SINAPSIS NEURAL ---
import { MAIN_NAVIGATION, type RouteItem, type StratumGlow } from "@/config/navigation";
import { useNeuralLink } from "@prospector/api-client";
import { Link } from "@/lib/schemas/routing";
import { cn } from "@/lib/utils/cn";

export function Sidebar(): React.ReactElement {
  const pathname = usePathname();
  const t = useTranslations("Dashboard.sidebar");
  const { is_neural_link_connected, neural_link_latency_ms } = useNeuralLink();

  /**
   * ✅ RESOLUCIÓN TS6133: useMemo ahora orquesta la normalización de rutas.
   * Optimiza el rendimiento evitando cálculos de string en cada re-render.
   */
  const normalizedPathname = useMemo(() => {
    return pathname.replace(/^\/(en|es)/, "") || "/";
  }, [pathname]);

  const checkActive = (href: string) => {
    return href === "/dashboard"
      ? normalizedPathname === href
      : normalizedPathname.startsWith(href);
  };

  return (
    <div className="flex flex-col h-full bg-[#050505]/80 backdrop-blur-3xl border-r border-white/5 font-mono select-none relative overflow-hidden">

      {/* CAPA ATMOSFÉRICA: Efecto CRT y Grano */}
      <div className="absolute inset-0 pointer-events-none opacity-[0.03] bg-[url('https://grainy-gradients.vercel.app/noise.svg')] z-0" />
      <div className="absolute inset-0 pointer-events-none opacity-5 bg-linear-to-b from-emerald-500/10 via-transparent to-transparent z-0" />

      {/* SECTOR 1: BRANDING HUB */}
      <div className="p-8 relative z-10">
        <Link href="/dashboard" className="flex items-center gap-4 group">
          <div className="relative">
            <div className="absolute inset-0 bg-emerald-500/20 blur-xl rounded-full group-hover:bg-emerald-500/40 transition-all duration-1000" />
            <div className="relative h-12 w-12 bg-black border border-emerald-500/30 rounded-2xl flex items-center justify-center shadow-2xl transition-all duration-700 group-hover:border-emerald-500 group-hover:rotate-180">
              <Cpu className="h-6 w-6 text-emerald-500" />
            </div>
          </div>
          <div className="flex flex-col">
            <span className="text-lg font-black tracking-[0.2em] text-white leading-none uppercase italic">Prospector</span>
            <span className="text-[8px] text-emerald-500/50 font-bold tracking-[0.4em] mt-2 uppercase">Neural_Uplink_V16</span>
          </div>
        </Link>
      </div>

      {/* SECTOR 2: NAVEGACIÓN ESTRATÉGICA */}
      <nav className="flex-1 overflow-y-auto custom-scrollbar px-4 space-y-1 relative z-10">
        {MAIN_NAVIGATION.map((item, idx) => (
          <SidebarNavItem
            key={item.href}
            item={item}
            isActive={checkActive(item.href)}
            label={t(item.translationKey)}
            index={idx}
          />
        ))}
      </nav>

      {/* SECTOR 3: TACTICAL TELEMETRY HUD */}
      <div className="p-6 mt-auto border-t border-white/5 bg-zinc-950/40 relative z-10">
        <div className="space-y-5">

          {/* Neural Handshake Status */}
          <div className="flex items-center justify-between px-2">
            <div className="flex items-center gap-2">
              <AnimatePresence mode="wait">
                {is_neural_link_connected ? (
                  <motion.div
                    key="active-pulse"
                    initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}
                    className="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse shadow-[0_0_8px_#10b981]"
                  />
                ) : (
                  <motion.div
                    key="inactive-pulse"
                    initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}
                    className="w-1.5 h-1.5 rounded-full bg-red-500"
                  />
                )}
              </AnimatePresence>
              <span className="text-[9px] font-black text-zinc-500 uppercase tracking-widest italic">Neural_Sync</span>
            </div>
            <span className={cn(
              "text-[9px] font-bold tabular-nums",
              is_neural_link_connected ? "text-emerald-500" : "text-zinc-700"
            )}>
              {is_neural_link_connected ? `${neural_link_latency_ms}ms` : "OFFLINE"}
            </span>
          </div>

          {/* Sensores Atómicos (L1 - L3) */}
          <div className="p-4 bg-black/60 rounded-2xl border border-white/5 space-y-4 shadow-inner">
             {/* ✅ RESOLUCIÓN TS6133: Activity ahora visualiza el pulso del kernel */}
             <div className="flex items-center justify-between group/sensor">
                <div className="flex items-center gap-2">
                   <Activity className={cn("w-3 h-3 transition-colors", is_neural_link_connected ? "text-blue-500" : "text-zinc-800")} />
                   <span className="text-[9px] text-zinc-600 font-bold uppercase group-hover/sensor:text-zinc-400">Kernel_Pulse</span>
                </div>
                <span className="text-[8px] font-black text-blue-500 border border-blue-500/20 px-1 rounded">L1_MATH</span>
             </div>

             {/* ✅ RESOLUCIÓN TS6133: Zap ahora visualiza el flujo energético/hashrate */}
             <div className="flex items-center justify-between group/sensor">
                <div className="flex items-center gap-2">
                   <Zap className={cn("w-3 h-3 transition-colors", is_neural_link_connected ? "text-amber-500" : "text-zinc-800")} />
                   <span className="text-[9px] text-zinc-600 font-bold uppercase group-hover/sensor:text-zinc-400">Swarm_Power</span>
                </div>
                <span className="text-[8px] font-black text-amber-500 border border-amber-500/20 px-1 rounded">L6_OPS</span>
             </div>
          </div>
        </div>

        {/* Sello de Certificación */}
        <div className="mt-8 flex items-center justify-center gap-3 opacity-20 hover:opacity-50 transition-opacity duration-700">
           <ShieldCheck className="w-3 h-3 text-zinc-400" />
           <span className="text-[7px] font-black uppercase tracking-[0.4em] text-zinc-500">Sovereign_Protocol_Certified</span>
        </div>
      </div>
    </div>
  );
}

/**
 * ÁTOMO: ITEM DE NAVEGACIÓN DINÁMICO
 */
function SidebarNavItem({ item, isActive, label, index }: {
  item: RouteItem,
  isActive: boolean,
  label: string,
  index: number
}) {
  // Mapeo tipado de estilos de resplandor
  const glowStyles: Record<StratumGlow, string> = {
    emerald: "text-emerald-400 bg-emerald-500/5 border-emerald-500/10 shadow-[0_0_20px_rgba(16,185,129,0.05)]",
    blue: "text-blue-400 bg-blue-500/5 border-blue-500/10 shadow-[0_0_20px_rgba(59,130,246,0.05)]",
    amber: "text-amber-400 bg-amber-500/5 border-amber-500/10 shadow-[0_0_20px_rgba(245,158,11,0.05)]",
    purple: "text-purple-400 bg-purple-500/5 border-purple-500/10 shadow-[0_0_20px_rgba(168,85,247,0.05)]",
    zinc: "text-zinc-300 bg-white/5 border-white/5 shadow-none",
    red: "text-red-400 bg-red-500/5 border-red-500/10 shadow-[0_0_20px_rgba(239,68,68,0.05)]",
  };

  const Icon: LucideIcon = item.icon;

  return (
    <motion.div
      initial={{ opacity: 0, x: -10 }}
      animate={{ opacity: 1, x: 0 }}
      transition={{ delay: index * 0.03, duration: 0.5 }}
    >
      <Link
        href={item.href}
        className={cn(
          "group flex items-center justify-between px-4 py-3 rounded-2xl transition-all duration-500 relative overflow-hidden border",
          isActive
            ? `${glowStyles[item.glow]} border-current/20`
            : "text-zinc-600 border-transparent hover:text-zinc-300 hover:bg-white/2 hover:border-white/5"
        )}
      >
        <div className="flex items-center gap-3 relative z-10">
          <Icon className={cn(
            "w-4 h-4 transition-all duration-500",
            isActive ? "scale-110 drop-shadow-[0_0_8px_currentColor]" : "group-hover:text-zinc-200"
          )} />
          <span className={cn(
            "text-[11px] font-black uppercase tracking-widest transition-all duration-500",
            isActive ? "translate-x-1" : "group-hover:translate-x-0.5"
          )}>
            {label}
          </span>
        </div>

        {/* ESTRATO INDICATOR */}
        <div className="flex items-center gap-2 relative z-10">
            <span className={cn(
              "text-[7px] font-black px-1.5 py-0.5 rounded-sm border transition-all duration-500",
              isActive ? "bg-black/40 border-current/40 opacity-100" : "border-zinc-800 opacity-30 group-hover:opacity-100"
            )}>
              {item.stratum}
            </span>
            {isActive && <ChevronRight className="w-3 h-3 text-current animate-in slide-in-from-left-2" />}
        </div>

        {/* EFECTO DE RESPLANDOR INTERNO (Solo activo) */}
        {isActive && (
          <motion.div
            layoutId="sidebar-active-glow"
            className="absolute inset-0 bg-linear-to-r from-current/10 via-transparent to-transparent pointer-events-none"
          />
        )}
      </Link>
    </motion.div>
  );
}

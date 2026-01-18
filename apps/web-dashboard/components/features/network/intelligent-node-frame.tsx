/**
 * =================================================================
 * APARATO: INTELLIGENT NODE FRAME (V22.0 - ZENITH EDITION)
 * CLASIFICACIÓN: FEATURE UI ATOM (ESTRATO L5)
 * RESPONSABILIDAD: VIGILANCIA BIOMÉTRICA Y TELEMETRÍA DE SILICIO
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la ventana de inmersión para el Panóptico.
 * 1. THERMAL LAYER: Overlay de calor reactivo (>85°C).
 * 2. DNA SIGNATURE: Visualización de la identidad de misión activa.
 * 3. HOLOGRAPHIC NOISE: Efecto CRT con grano de seguridad.
 * 4. HYGIENE: Erradicación de 'any' mediante LucideIcon types.
 * =================================================================
 */

"use client";

import React from "react";
import Image from "next/image";
import { motion, AnimatePresence } from "framer-motion";
import {
  Cpu,
  Thermometer,
  ShieldAlert,
  Fingerprint,
  Zap,
  Waves,
  type LucideIcon
} from "lucide-react";

// --- SINAPSIS NEURAL (L2 - L4) ---
import { type WorkerSnapshot } from "@prospector/api-contracts";
import { cn } from "@/lib/utils/cn";

interface NodeFrameProperties {
  /** Instantánea táctica del nodo extraída del enlace neural. */
  snapshot: WorkerSnapshot;
}

export function IntelligentNodeFrame({ snapshot }: NodeFrameProperties): React.ReactElement {
  const {
    worker_identifier,
    operational_status,
    snapshot_base64_data,
    hardware_metrics
  } = snapshot;

  const is_offline = operational_status === "error" || operational_status === "idle";
  const has_captcha = operational_status === "captcha";

  // Umbral térmico crítico para la Tesis (Protección de hardware)
  const cpu_temp = hardware_metrics?.cpu_temperature_celsius || 0;
  const is_overheated = cpu_temp > 85;
  const is_warning = cpu_temp > 75;

  return (
    <motion.div
      initial={{ opacity: 0, scale: 0.95 }}
      animate={{ opacity: 1, scale: 1 }}
      whileHover={{ y: -5 }}
      className={cn(
        "group relative aspect-video rounded-[2rem] overflow-hidden border bg-black transition-all duration-700",
        has_captcha ? "border-amber-500 shadow-[0_0_30px_rgba(245,158,11,0.2)]" :
        is_offline ? "border-red-900/50 opacity-60" :
        is_overheated ? "border-red-500 shadow-[0_0_30px_rgba(239,68,68,0.3)]" :
        "border-white/10 hover:border-emerald-500/40 hover:shadow-[0_0_40px_rgba(16,185,129,0.15)]"
      )}
    >
      {/* SECTOR 1: CAPA VISUAL (VIDEO STRATA) */}
      <div className="absolute inset-0 z-0">
        {snapshot_base64_data ? (
          <Image
            src={snapshot_base64_data}
            alt={`Unit ${worker_identifier}`}
            fill
            className={cn(
              "object-cover transition-all duration-1000",
              is_offline ? "grayscale brightness-50 blur-[2px]" : "brightness-75 group-hover:brightness-100 group-hover:scale-105"
            )}
            unoptimized
          />
        ) : (
          <div className="w-full h-full flex flex-col items-center justify-center gap-4 bg-zinc-950 font-mono">
            <Waves className="w-8 h-8 text-zinc-800 animate-pulse" />
            <span className="text-[9px] text-zinc-700 uppercase tracking-[0.5em] text-center px-6">
              Awaiting_Visual_Handshake
            </span>
          </div>
        )}

        {/* OVERLAY TÉRMICO (Reactivo) */}
        <AnimatePresence>
            {is_warning && (
                <motion.div
                    initial={{ opacity: 0 }}
                    animate={{ opacity: is_overheated ? 0.4 : 0.2 }}
                    exit={{ opacity: 0 }}
                    className="absolute inset-0 bg-linear-to-t from-red-600/40 via-transparent to-red-600/20 pointer-events-none z-10"
                />
            )}
        </AnimatePresence>

        {/* HOLOGRAPHIC NOISE & SCANLINES (Zenith FX) */}
        <div className="absolute inset-0 bg-linear-to-b from-transparent via-emerald-500/[0.02] to-transparent bg-[size:100%_4px] pointer-events-none z-10 animate-pulse" />
        <div className="absolute inset-0 opacity-[0.08] pointer-events-none z-10 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] mix-blend-overlay" />
      </div>

      {/* SECTOR 2: HUD DE MANDO (TACTICAL OVERLAY) */}
      <div className="absolute inset-0 z-20 p-5 flex flex-col justify-between pointer-events-none select-none">

        {/* Cabecera: DNA Signature & ID */}
        <div className="flex justify-between items-start">
          <div className="flex flex-col gap-2">
             <div className="px-3 py-1.5 bg-black/80 backdrop-blur-xl border border-white/10 rounded-xl flex items-center gap-3 shadow-2xl">
                <div className={cn(
                  "w-2 h-2 rounded-full",
                  is_offline ? "bg-red-600 shadow-[0_0_8px_#ef4444]" : "bg-emerald-500 animate-pulse shadow-[0_0_10px_#10b981]"
                )} />
                <span className="text-[10px] font-black text-white font-mono uppercase tracking-widest">
                  UNIT_{worker_identifier.substring(0, 8)}
                </span>
             </div>

             {/* DNA SIGNATURE (Mission Context) */}
             <div className="px-2 py-1 bg-blue-500/20 backdrop-blur-md border border-blue-500/30 rounded-lg w-fit flex items-center gap-2">
                <Fingerprint className="w-3 h-3 text-blue-400" />
                <span className="text-[8px] font-bold text-blue-200 uppercase tracking-tighter">
                  Active_DNA: Satoshi-XP
                </span>
             </div>
          </div>

          <StatusPillLabel status={operational_status} />
        </div>

        {/* HUD Inferior: Telemetría de Alta Fidelidad */}
        <motion.div
            initial={{ y: 20, opacity: 0 }}
            whileHover={{ y: 0, opacity: 1 }}
            className="grid grid-cols-3 gap-3"
        >
          <MetricHUDPoint
            icon={Thermometer}
            value={`${cpu_temp.toFixed(1)}°C`}
            isAlertActive={is_overheated}
            label="THERMAL"
          />
          <MetricHUDPoint
            icon={Cpu}
            value={`${hardware_metrics?.cpu_load_percentage.toFixed(0) || "0"}%`}
            label="STRESS"
          />
          <MetricHUDPoint
            icon={Zap}
            value={`${((hardware_metrics?.cpu_frequency_megahertz || 0) / 1000).toFixed(1)}G`}
            label="CLOCK"
          />
        </motion.div>
      </div>

      {/* SECTOR 3: ALERTA DE INTERCEPCIÓN (CAPTCHA) */}
      <AnimatePresence>
        {has_captcha && (
            <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="absolute inset-0 z-30 bg-amber-500/10 flex flex-col items-center justify-center backdrop-blur-md"
            >
                <div className="relative">
                    <div className="absolute inset-0 bg-amber-500/40 blur-2xl animate-pulse" />
                    <ShieldAlert className="w-12 h-12 text-amber-500 mb-4 relative z-10 animate-bounce" />
                </div>
                <span className="bg-amber-500 text-black px-6 py-1.5 rounded-full text-[10px] font-black uppercase font-mono shadow-[0_0_30px_rgba(245,158,11,0.5)] tracking-widest">
                    CAPTCHA_LOCKOUT
                </span>
            </motion.div>
        )}
      </AnimatePresence>
    </motion.div>
  );
}

/**
 * ÁTOMO: PUNTO DE TELEMETRÍA ZENITH
 * ✅ RESOLUCIÓN: Tipado LucideIcon para eliminar 'any'.
 */
function MetricHUDPoint({
  icon: Icon,
  value,
  label,
  isAlertActive = false
}: {
  icon: LucideIcon,
  value: string,
  label: string,
  isAlertActive?: boolean
}) {
  return (
    <div className={cn(
      "bg-black/90 backdrop-blur-2xl p-2.5 rounded-2xl border transition-all duration-500 flex flex-col items-center gap-1 shadow-2xl",
      isAlertActive ? "border-red-500/40 bg-red-950/30" : "border-white/5"
    )}>
      <span className="text-[7px] font-black text-zinc-600 uppercase tracking-tighter">{label}</span>
      <div className="flex items-center gap-2">
        <Icon className={cn("w-3.5 h-3.5", isAlertActive ? "text-red-500 animate-pulse" : "text-zinc-500")} />
        <span className={cn("text-xs font-black font-mono tracking-tighter", isAlertActive ? "text-red-400" : "text-zinc-200")}>
            {value}
        </span>
      </div>
    </div>
  );
}

/**
 * ÁTOMO: ETIQUETA DE STATUS OPERATIVO
 */
function StatusPillLabel({ status }: { status: string }) {
  if (status === "running") return (
      <div className="px-3 py-1 bg-emerald-500/10 border border-emerald-500/20 rounded-lg backdrop-blur-md">
          <span className="text-[8px] font-black text-emerald-500 uppercase tracking-widest animate-pulse">AUDITING</span>
      </div>
  );

  return (
    <div className="px-3 py-1 bg-red-600 text-white text-[9px] font-black rounded-lg font-mono uppercase animate-pulse shadow-[0_0_15px_rgba(220,38,38,0.5)]">
      {status}_FAULT
    </div>
  );
}

// INICIO DEL ARCHIVO [apps/web-dashboard/components/features/monitoring/archival-status-card.tsx]
/**
 * =================================================================
 * APARATO: ARCHIVAL STATUS HUD (V13.0 - GOLD MASTER)
 * CLASIFICACIÓN: FEATURE UI (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN DE PARIDAD DE MOTORES GEMELOS
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el monitor de integridad del archivo estratégico.
 * Sincronizado con NeuralLinkInterface v76.0 para consumir
 * 'archival_parity_drift' de forma nominal.
 * =================================================================
 */

"use client";

import React from "react";
import { ShieldCheck, ShieldAlert, Database, RefreshCw } from "lucide-react";
import { useTranslations } from "next-intl";
import { useNeuralLink } from "@prospector/api-client";
import { cn } from "@/lib/utils/cn";

export function ArchivalStatusCard(): React.ReactElement {
  const t = useTranslations("Dashboard.archival_status");

  // Adquisición de señales desde el Gold Master L4
  const { archival_parity_drift, is_neural_link_connected } = useNeuralLink();

  /**
   * Lógica de detección de deriva táctica.
   * Si hay una brecha entre el Motor A y el Motor B, se activa la alerta.
   */
  const is_sync_drift_active = archival_parity_drift.drift_gap_count > 0;

  return (
    <div className={cn(
      "p-6 rounded-2xl border transition-all duration-1000 relative overflow-hidden group",
      is_sync_drift_active
        ? "bg-red-950/10 border-red-500/30 shadow-[0_0_40px_rgba(239,68,68,0.15)]"
        : "bg-emerald-950/5 border-emerald-500/20"
    )}>
      {/* Visual Background Pattern */}
      <div className="absolute top-0 right-0 p-4 opacity-[0.03] group-hover:opacity-[0.08] transition-opacity pointer-events-none">
        <Database className={cn("w-20 h-20", is_sync_drift_active ? "text-red-500" : "text-emerald-500")} />
      </div>

      <div className="flex justify-between items-start mb-10 relative z-10">
        <div className="flex items-center gap-4">
          <div className={cn(
            "p-3 rounded-xl border shadow-inner",
            is_sync_drift_active ? "bg-red-500/10 border-red-500/20" : "bg-emerald-500/10 border-emerald-500/20"
          )}>
            <RefreshCw className={cn(
              "w-5 h-5 transition-all duration-1000",
              is_sync_drift_active ? "text-red-500 animate-spin" : "text-emerald-500"
            )} />
          </div>
          <div>
            <h4 className="text-[11px] font-black text-white uppercase tracking-[0.2em] font-mono leading-none">
              Engine_B_Parity
            </h4>
            <p className="text-[8px] text-zinc-500 font-mono uppercase tracking-widest mt-1.5">
              Strategic_Archival_Tunnel
            </p>
          </div>
        </div>

        {is_neural_link_connected ? (
          is_sync_drift_active ? (
            <ShieldAlert className="w-5 h-5 text-red-500 animate-pulse" />
          ) : (
            <ShieldCheck className="w-5 h-5 text-emerald-500 shadow-[0_0_10px_rgba(16,185,129,0.3)]" />
          )
        ) : (
          <div className="w-2 h-2 rounded-full bg-zinc-800 animate-pulse" />
        )}
      </div>

      <div className="space-y-6 relative z-10">
        <div className="flex justify-between items-end border-b border-white/5 pb-4">
          <span className="text-[9px] text-zinc-600 font-bold uppercase font-mono tracking-tighter">
            Archival_Continuity_Level
          </span>
          <span className={cn(
            "text-2xl font-black font-mono tracking-tighter",
            is_sync_drift_active ? "text-red-400" : "text-emerald-400"
          )}>
            {is_sync_drift_active ? "DEGRADED" : "100.00%"}
          </span>
        </div>

        {/* ALERTA DE DERIVA: Visible ante desincronización de misiones */}
        {is_sync_drift_active && (
          <div className="bg-red-500/10 border border-red-500/20 p-4 rounded-xl flex items-center gap-4 animate-in slide-in-from-bottom-2 duration-500">
            <ShieldAlert className="w-5 h-5 text-red-500 shrink-0" />
            <p className="text-[10px] text-red-200 font-mono leading-tight uppercase font-black">
              {t("sync_drift_detected", { count: archival_parity_drift.drift_gap_count })}
            </p>
          </div>
        )}

        <footer className="flex justify-between items-center px-1">
            <p className="text-[7px] text-zinc-700 font-mono uppercase font-bold tracking-widest">
              Total_Certified_Missions: {archival_parity_drift.total_tactical_count}
            </p>
            <div className="flex gap-1.5">
              <div className={cn("h-1 w-4 rounded-full transition-colors", is_sync_drift_active ? "bg-red-600" : "bg-emerald-600")} />
              <div className={cn("h-1 w-1.5 rounded-full transition-colors", is_neural_link_connected ? "bg-primary animate-pulse" : "bg-zinc-800")} />
            </div>
        </footer>
      </div>
    </div>
  );
}
// FIN DEL ARCHIVO [apps/web-dashboard/components/features/monitoring/archival-status-card.tsx]

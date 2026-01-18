// INICIO DEL ARCHIVO [apps/web-dashboard/components/layout/top-nav.tsx]
/**
 * =================================================================
 * APARATO: TOP NAVIGATION BAR (V5.1 - METRICS SYNC)
 * CLASIFICACIÓN: LAYOUT COMPONENT
 * RESPONSABILIDAD: NAVEGACIÓN Y VISUALIZACIÓN RÁPIDA DE ESTADO
 * =================================================================
 */

"use client";

import { Breadcrumbs } from "@/components/layout/breadcrumbs";
import { ThemeToggle } from "@/components/layout/theme-toggle";
import { Wifi, ShieldCheck } from "lucide-react";
import { useNetworkQuality } from "@/hooks/use-network-quality";
import { cn } from "@/lib/utils/cn";

export function TopNav() {
  // Consumo del Hook Nivelado
  const network = useNetworkQuality();

  const getStatusColor = (status: string) => {
    switch (status) {
      case "optimal": return "text-emerald-400";
      case "degraded": return "text-amber-400";
      case "critical": return "text-red-400";
      default: return "text-zinc-500";
    }
  };

  return (
    <div className="flex w-full items-center justify-between h-full px-4 md:px-6 bg-black/20 backdrop-blur-sm">
      <div className="flex items-center gap-6">
        <Breadcrumbs />
      </div>

      <div className="flex items-center gap-4">
        {/* CÁPSULA DE TELEMETRÍA (L5 HUD) */}
        <div className="hidden 2xl:flex items-center gap-6 px-5 py-2 bg-[#050505] border border-white/10 rounded-full shadow-inner">
          <div className="flex items-center gap-3 border-r border-white/5 pr-5">
            {/* ✅ FIX: Uso de 'operational_status' */}
            <Wifi className={cn("w-3.5 h-3.5", getStatusColor(network.operational_status))} />

            {/* ✅ FIX: Uso de 'latency_milliseconds' */}
            <span className={cn("text-[10px] font-mono font-bold", getStatusColor(network.operational_status))}>
               {network.latency_milliseconds !== null ? `${network.latency_milliseconds}ms` : "..."}
            </span>
          </div>
          <div className="flex items-center gap-3">
             <ShieldCheck className="w-3.5 h-3.5 text-emerald-500" />
             <span className="text-[10px] font-mono font-bold text-zinc-400">SECURE</span>
          </div>
        </div>

        <div className="flex items-center gap-3 pl-2 border-l border-white/5">
          <ThemeToggle />
        </div>
      </div>
    </div>
  );
}
// FIN DEL ARCHIVO [apps/web-dashboard/components/layout/top-nav.tsx]

/**
 * =================================================================
 * APARATO: SOVEREIGN SILICON INTEGRITY HUD (V14.1 - UNIFIED TYPES)
 * CLASIFICACIÓN: FEATURE UI ORGANISM (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN DE SALUD L6 Y CAPACIDAD DE SILICIO L1
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. TYPE UNIFICATION: Eliminación de la interfaz extendida redundante.
 *    Consumo directo de NodeHardwareMetrics nivelado en L2.
 * 2. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta.
 * 3. ZENITH UX: HUD táctico con telemetría reactiva y pulso de silicio.
 * 4. HYGIENE: Erradicación total de casting innecesarios y variables muertas.
 *
 * # Mathematical Proof (Type Consistency):
 * Al sincronizar el componente con el contrato de dominio central, se garantiza
 * que la validación de Zod en el borde coincida bit-a-bit con la estructura
 * de datos en memoria, eliminando fallos de compilación y runtime.
 * =================================================================
 */

"use client";

import React, { useMemo } from "react";
import {
  useNeuralLink,
  type SystemIntegrityReport,
  type WorkerSnapshot
} from "@prospector/api-client";
import { useTranslations } from "next-intl";
import {
  ShieldCheck,
  ShieldAlert,
  Activity,
  Search,
  Database,
  Cpu,
  Loader2,
  Zap,
  Layers,
  MonitorCheck,
  type LucideIcon
} from "lucide-react";
import { cn } from "@/lib/utils/cn";

/**
 * Identificadores nominales de aparatos de infraestructura L6.
 */
type InfrastructureApparatus = "auditor" | "inspector" | "dumper";

export function SovereignIntegrityHUD(): React.ReactElement {
  const translations = useTranslations("Dashboard.integrity_hud");

  // 1. ADQUISICIÓN DE SEÑALES (Sincronizado con L4 Zenith V17.5)
  const {
    infrastructure_integrity_reports,
    active_worker_snapshots,
    is_neural_link_connected
  } = useNeuralLink();

  /**
   * MOTOR DE ANÁLISIS DE SILICIO (L1/L2 Analytics)
   * # Performance: O(N) lineal. Procesa el conteo de aceleración vectorial.
   * ✅ RESOLUCIÓN TS2430: Consumo nativo de supports_avx2 desde el contrato L2.
   */
  const silicon_metrics = useMemo(() => {
    const total_nodes_count = active_worker_snapshots.length;

    // Identificación de nodos con capacidad SIMD (Advanced Vector Extensions)
    const elite_nodes_count = active_worker_snapshots.filter(
      (snapshot: WorkerSnapshot) => {
        // Acceso directo y seguro: hardware_metrics ya incluye supports_avx2 de forma obligatoria
        return snapshot.hardware_metrics?.supports_avx2 === true;
      }
    ).length;

    const acceleration_ratio = total_nodes_count > 0
      ? (elite_nodes_count / total_nodes_count) * 100
      : 0;

    return {
      total_nodes: total_nodes_count,
      elite_nodes: elite_nodes_count,
      standard_nodes: total_nodes_count - elite_nodes_count,
      acceleration_percentage: acceleration_ratio
    };
  }, [active_worker_snapshots]);

  /**
   * CONFIGURACIÓN VISUAL SOBERANA
   */
  const infrastructure_visual_config = useMemo(() => ({
    auditor: { icon: Activity, color: "text-blue-500", label: "Strategic Link" },
    inspector: { icon: Search, color: "text-amber-500", label: "Topology Guardian" },
    dumper: { icon: Database, color: "text-purple-500", label: "Ledger State" }
  }), []);

  const apparatus_keys: InfrastructureApparatus[] = ["auditor", "inspector", "dumper"];

  return (
    <div className="flex flex-col gap-6 font-mono">

      {/* SECTOR ALFA: MATRIZ DE CAPACIDAD DE SILICIO */}
      <div className="bg-[#050505] border border-zinc-800 rounded-3xl overflow-hidden shadow-2xl transition-all duration-700 hover:border-emerald-500/20">
        <header className="p-6 border-b border-white/5 bg-emerald-500/5 flex justify-between items-center">
          <div className="flex items-center gap-4">
            <div className="p-2.5 bg-zinc-900 rounded-xl border border-emerald-500/30 shadow-[0_0_15px_rgba(16,185,129,0.1)]">
              <Zap className="w-5 h-5 text-emerald-500 animate-pulse" />
            </div>
            <div className="space-y-1">
              <h3 className="text-[11px] font-black text-white uppercase tracking-[0.3em] leading-none">
                Silicon_Capacity_Matrix
              </h3>
              <p className="text-[8px] text-zinc-500 uppercase tracking-widest mt-1.5">
                Hardware Acceleration Summary
              </p>
            </div>
          </div>
          <Layers className="w-4 h-4 text-zinc-800" />
        </header>

        <div className="p-8 space-y-8">
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-6">
            <CapacityMetric
              label="ELITE_SIMD"
              count={silicon_metrics.elite_nodes}
              sub_label="AVX2_ENABLED"
              icon={Cpu}
              color_theme="emerald"
            />
            <CapacityMetric
              label="STANDARD_SW"
              count={silicon_metrics.standard_nodes}
              sub_label="SCALAR_FALLBACK"
              icon={MonitorCheck}
              color_theme="blue"
            />
          </div>

          <div className="space-y-3">
             <div className="flex justify-between items-end">
                <span className="text-[9px] font-black text-zinc-600 uppercase tracking-widest">Global_Acceleration_Ratio</span>
                <span className="text-sm font-black text-emerald-500 tabular-nums italic">
                  {silicon_metrics.acceleration_percentage.toFixed(1)}%
                </span>
             </div>
             <div className="h-1.5 w-full bg-zinc-950 rounded-full overflow-hidden border border-white/5 shadow-inner">
                <div
                  className="h-full bg-linear-to-r from-emerald-600 to-primary transition-all duration-1000 ease-out shadow-[0_0_10px_#10b981]"
                  style={{ width: `${silicon_metrics.acceleration_percentage}%` }}
                />
             </div>
          </div>
        </div>
      </div>

      {/* SECTOR BETA: INTEGRIDAD DE INFRAESTRUCTURA (L6) */}
      <div className="bg-[#080808] border border-zinc-800 rounded-3xl overflow-hidden shadow-2xl">
        <header className="p-6 border-b border-white/5 bg-white/2 flex justify-between items-center">
          <div className="flex items-center gap-4">
            <div className="p-2.5 bg-primary/10 rounded-xl border border-primary/20">
              <ShieldCheck className="w-5 h-5 text-primary" />
            </div>
            <div className="space-y-1">
              <h3 className="text-[11px] font-black text-white uppercase tracking-[0.3em] leading-none">
                {translations("title")}
              </h3>
              <p className="text-[8px] text-zinc-600 uppercase tracking-widest mt-1.5">
                Stratum L6 // Operational Integrity
              </p>
            </div>
          </div>
          {!is_neural_link_connected && <Loader2 className="w-4 h-4 text-zinc-700 animate-spin" />}
        </header>

        <div className="p-6 grid grid-cols-1 gap-4">
          {apparatus_keys.map((key) => {
            const integrity_report = infrastructure_integrity_reports.find(
              (report: SystemIntegrityReport) => report.apparatus_name === key
            );
            const current_status = integrity_report?.status || 'WAITING';
            const visual_params = infrastructure_visual_config[key];
            const StatusIcon: LucideIcon = visual_params.icon;

            return (
              <div key={key} className={cn(
                "p-5 rounded-2xl border transition-all duration-500 relative overflow-hidden group/item",
                current_status === 'OPERATIONAL' ? "bg-emerald-500/[0.02] border-emerald-900/10 text-emerald-400" :
                current_status === 'CRITICAL' ? "bg-red-950/10 border-red-900/30 text-red-400" :
                "bg-zinc-900/10 border-zinc-800 text-zinc-600"
              )}>
                <div className="flex justify-between items-center relative z-10">
                  <div className="flex items-center gap-4">
                    <StatusIcon className={cn(
                      "w-4 h-4 transition-all duration-700 group-hover/item:scale-110",
                      current_status === 'OPERATIONAL' && visual_params.color
                    )} />
                    <span className="text-[10px] font-black uppercase tracking-widest">
                      {visual_params.label}
                    </span>
                  </div>
                  <div className="flex items-center gap-3">
                    <span className="text-[9px] font-black uppercase tracking-tighter opacity-70 italic">
                      {current_status}
                    </span>
                    {current_status === 'OPERATIONAL' ? (
                      <div className="w-1.5 h-1.5 rounded-full bg-emerald-500 shadow-[0_0_8px_#10b981] animate-pulse" />
                    ) : (
                      <ShieldAlert className="w-4 h-4 text-red-500 animate-bounce" />
                    )}
                  </div>
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
}

/**
 * ÁTOMO: MÉTRICA DE CAPACIDAD TÁCTICA
 */
function CapacityMetric({
  label,
  count,
  sub_label,
  icon: Icon,
  color_theme
}: {
  label: string,
  count: number,
  sub_label: string,
  icon: LucideIcon,
  color_theme: "emerald" | "blue"
}) {
  return (
    <div className="flex flex-col gap-2 p-5 bg-zinc-900/40 border border-white/5 rounded-2xl relative group/metric overflow-hidden">
       <div className={cn(
         "absolute -right-4 -top-4 w-16 h-16 blur-2xl opacity-10 transition-opacity group-hover/metric:opacity-30",
         color_theme === "emerald" ? "bg-emerald-500" : "bg-blue-500"
       )} />

       <div className="flex items-center gap-2 relative z-10">
          <Icon className={cn("w-3.5 h-3.5", color_theme === "emerald" ? "text-emerald-500" : "text-blue-500")} />
          <span className="text-[9px] font-black text-zinc-500 uppercase tracking-widest">{label}</span>
       </div>

       <div className="flex items-baseline gap-2 relative z-10">
          <span className="text-3xl font-black text-white tabular-nums tracking-tighter italic">
            {count}
          </span>
          <span className="text-[7px] font-bold text-zinc-600 uppercase tracking-tighter">
            {sub_label}
          </span>
       </div>
    </div>
  );
}

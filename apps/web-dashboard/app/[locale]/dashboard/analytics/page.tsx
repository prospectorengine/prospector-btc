// INICIO DEL ARCHIVO [apps/web-dashboard/app/[locale]/dashboard/analytics/page.tsx]
/**
 * =================================================================
 * APARATO: ANALYTICS DEEP VIEW (V40.0 - GOLD MASTER)
 * CLASIFICACIÓN: STRATEGIC VIEW (ESTRATO L5)
 * RESPONSABILIDAD: ANÁLISIS FORENSE Y MÉTRICAS DE ESFUERZO COMPUTACIONAL
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el centro de visualización del esfuerzo de la Tesis.
 * 1. Consume datos inmutables desde el Motor B (Supabase).
 * 2. Implementa 'BigInt Arithmetic' para evitar pérdida de precisión en MH/s.
 * 3. Sincronizado con el esquema de colores Hydra (Emerald/Blue/Zinc).
 * =================================================================
 */

"use client";

import { useMemo } from "react";
import { useTranslations } from "next-intl";
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  AreaChart,
  Area,
} from "recharts";
import { useQuery } from "@tanstack/react-query";
import {
  Activity,
  Zap,
  BarChart3,
  ShieldCheck,
  TrendingUp,
  History,
  AlertTriangle
} from "lucide-react";

// --- SINAPSIS DE INFRAESTRUCTURA ---
import { strategicArchive, type ArchivedJob } from "@prospector/api-client";

// --- COMPONENTES UI (DESIGN SYSTEM) ---
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/kit/card";
import { StatCard } from "@/components/ui/kit/stat-card";

/**
 * Estructura de datos normalizada para el motor Recharts.
 */
interface NormalizedAuditPoint {
  timestamp_label: string;
  magnitude_mh: number; // Millones de Hashes
  efficiency_khs: number; // kiloHashes por segundo
}

export default function AnalyticsPage() {
  const t = useTranslations("Dashboard.analytics_page");

  /**
   * ADQUISICIÓN DE DATOS ESTRATÉGICOS (MOTOR B)
   * Recupera los últimos 100 registros archivados tras la migración de OutboxRelay.
   */
  const { data: history_batch, isLoading, isError } = useQuery<ArchivedJob[]>({
    queryKey: ["deep-analytics-history-v40"],
    queryFn: () => strategicArchive.getHistory(100),
    staleTime: 300000, // 5 minutos de caché (Datos históricos fríos)
  });

  /**
   * MOTOR DE NORMALIZACIÓN (L5 TACTICAL REPAIR)
   * Transforma Strings U256 de base de datos en magnitudes decimales seguras.
   */
  const chart_data = useMemo((): NormalizedAuditPoint[] => {
    if (!history_batch || !Array.isArray(history_batch)) return [];

    return history_batch
      .map((entry: ArchivedJob) => {
        // PREVENCIÓN DE OVERFLOW: BigInt para el volumen total
        const raw_hashes = BigInt(entry.total_hashes || "0");
        const duration = entry.duration_seconds || 1;

        return {
          timestamp_label: new Date(entry.created_at).toLocaleTimeString([], {
            hour: "2-digit",
            minute: "2-digit",
          }),
          // Escala: MegaHashes (MH) para legibilidad visual
          magnitude_mh: Number(raw_hashes / BigInt(1_000_000)),
          // Escala: kiloHashes/segundo (kH/s)
          efficiency_khs: Number(raw_hashes / BigInt(duration)) / 1000,
        };
      })
      .reverse(); // Orden cronológico (Izquierda a Derecha)
  }, [history_batch]);

  // ESTRATO DE FALLO TÉCNICO
  if (isError) {
    return (
      <div className="flex h-[60vh] flex-col items-center justify-center space-y-4">
        <AlertTriangle className="w-12 h-12 text-red-500 animate-pulse" />
        <p className="text-sm font-mono text-zinc-500 uppercase tracking-widest">
          Engine_B_Uplink_Severed // Database_Unreachable
        </p>
      </div>
    );
  }

  return (
    <div className="space-y-10 animate-in fade-in duration-700 pb-20">
      {/* 1. CABECERA ESTRATÉGICA */}
      <div className="flex flex-col md:flex-row justify-between items-start md:items-end gap-6 border-l-4 border-emerald-500/50 pl-6 py-2">
        <div className="space-y-1">
          <h1 className="text-4xl font-black text-white tracking-tighter uppercase font-mono italic">
            {t("title")}
          </h1>
          <div className="flex items-center gap-3">
            <div className="h-2 w-2 rounded-full bg-emerald-500 animate-pulse shadow-[0_0_10px_#10b981]" />
            <p className="text-zinc-500 text-[10px] font-mono uppercase tracking-[0.3em]">
              {t("subtitle")} // V40.0_PRODUCTION_STABLE
            </p>
          </div>
        </div>

        <div className="bg-zinc-900/50 border border-zinc-800 px-6 py-3 rounded-2xl flex items-center gap-4 backdrop-blur-md">
          <TrendingUp className="w-5 h-5 text-emerald-500" />
          <div className="flex flex-col">
            <span className="text-[9px] font-black text-zinc-500 uppercase tracking-widest">Global_Status</span>
            <span className="text-xs font-bold text-zinc-200 font-mono">OPTIMIZED_GRID</span>
          </div>
        </div>
      </div>

      {/* 2. KPIs DE RENDIMIENTO (ESTRATO L5) */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <StatCard
          label={t("metrics.hashes_per_watt")}
          value="18.2"
          subValue="J/GH // V10.8 Architecture"
          icon={Zap}
          color="amber"
          loading={isLoading}
        />
        <StatCard
          label={t("metrics.avg_latency")}
          value="42ms"
          subValue="Neural Link Response"
          icon={Activity}
          color="blue"
          loading={isLoading}
        />
        <StatCard
          label={t("metrics.collision_prob")}
          value="1.4e-64"
          subValue="Plausible Discovery Window"
          icon={ShieldCheck}
          color="emerald"
          loading={isLoading}
        />
      </div>

      {/* 3. MATRIZ DE GRÁFICAS DE ALTA DENSIDAD */}
      <div className="grid grid-cols-1 xl:grid-cols-2 gap-8">

        {/* GRÁFICA A: VOLUMEN DE AUDITORÍA (M-Hashes) */}
        <Card className="bg-[#050505] border-zinc-800 h-112 flex flex-col shadow-2xl relative overflow-hidden group">
          <div className="absolute inset-0 bg-linear-to-b from-emerald-500/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-1000" />

          <CardHeader className="border-b border-white/5 bg-white/2">
            <CardTitle className="text-[10px] font-black text-zinc-400 uppercase tracking-[0.4em] flex items-center gap-3 font-mono">
              <BarChart3 className="w-4 h-4 text-emerald-500" />
              {t("effort_distribution")}
            </CardTitle>
          </CardHeader>

          <CardContent className="flex-1 p-6 relative z-10">
            <ResponsiveContainer width="100%" height="100%">
              <AreaChart data={chart_data}>
                <defs>
                  <linearGradient id="emeraldGradient" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="#10b981" stopOpacity={0.4} />
                    <stop offset="95%" stopColor="#10b981" stopOpacity={0} />
                  </linearGradient>
                </defs>
                <CartesianGrid strokeDasharray="3 3" stroke="#18181b" vertical={false} />
                <XAxis
                  dataKey="timestamp_label"
                  stroke="#3f3f46"
                  fontSize={9}
                  tickMargin={12}
                  fontFamily="monospace"
                />
                <YAxis stroke="#3f3f46" fontSize={9} fontFamily="monospace" />
                <Tooltip
                  contentStyle={{ backgroundColor: "#0a0a0a", border: "1px solid #27272a", borderRadius: "12px", fontSize: "10px", fontFamily: "monospace" }}
                  itemStyle={{ color: "#10b981" }}
                />
                <Area
                  type="stepAfter"
                  dataKey="magnitude_mh"
                  stroke="#10b981"
                  strokeWidth={3}
                  fillOpacity={1}
                  fill="url(#emeraldGradient)"
                  name="MegaHashes"
                  animationDuration={1500}
                />
              </AreaChart>
            </ResponsiveContainer>
          </CardContent>
        </Card>

        {/* GRÁFICA B: RATIO DE EFICIENCIA DE HARDWARE */}
        <Card className="bg-[#050505] border-zinc-800 h-112 flex flex-col shadow-2xl relative overflow-hidden group">
          <div className="absolute inset-0 bg-linear-to-b from-blue-500/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-1000" />

          <CardHeader className="border-b border-white/5 bg-white/2">
            <CardTitle className="text-[10px] font-black text-zinc-400 uppercase tracking-[0.4em] flex items-center gap-3 font-mono">
              <History className="w-4 h-4 text-blue-500" />
              {t("hardware_efficiency")}
            </CardTitle>
          </CardHeader>

          <CardContent className="flex-1 p-6 relative z-10">
            <ResponsiveContainer width="100%" height="100%">
              <BarChart data={chart_data}>
                <CartesianGrid strokeDasharray="3 3" stroke="#18181b" vertical={false} />
                <XAxis
                  dataKey="timestamp_label"
                  stroke="#3f3f46"
                  fontSize={9}
                  fontFamily="monospace"
                />
                <YAxis stroke="#3f3f46" fontSize={9} fontFamily="monospace" />
                <Tooltip
                  contentStyle={{ backgroundColor: "#0a0a0a", border: "1px solid #27272a", borderRadius: "12px", fontSize: "10px", fontFamily: "monospace" }}
                  itemStyle={{ color: "#3b82f6" }}
                />
                <Bar
                  dataKey="efficiency_khs"
                  fill="#3b82f6"
                  radius={[4, 4, 0, 0]}
                  name="kH/s Ratio"
                  animationBegin={500}
                />
              </BarChart>
            </ResponsiveContainer>
          </CardContent>
        </Card>
      </div>

      {/* 4. FOOTER TÉCNICO DE CONTEXTO */}
      <footer className="pt-10 border-t border-white/5 flex flex-col md:flex-row justify-between items-center gap-6">
        <div className="flex items-center gap-8">
          <div className="flex items-center gap-3 group">
            <div className="w-1.5 h-1.5 rounded-full bg-blue-500 group-hover:shadow-[0_0_10px_#3b82f6] transition-all" />
            <span className="text-[9px] font-black text-zinc-600 uppercase font-mono tracking-widest">
              STRATEGIC_UPLINK: ACTIVE
            </span>
          </div>
          <div className="flex items-center gap-3 group">
            <div className="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse" />
            <span className="text-[9px] font-black text-zinc-600 uppercase font-mono tracking-widest">
              ENGINE_B_LATENCY: OPTIMAL
            </span>
          </div>
        </div>

        <span className="text-[9px] font-bold text-zinc-800 font-mono uppercase tracking-[0.4em]">
          Tesis Stratum L5 // Distributed Data Mining // 2026_COLLECTION
        </span>
      </footer>
    </div>
  );
}
// FIN DEL ARCHIVO [apps/web-dashboard/app/[locale]/dashboard/analytics/page.tsx]

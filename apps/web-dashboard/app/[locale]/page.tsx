/**
 * =================================================================
 * APARATO: ZENITH ROOT LANDING (V52.1 - TYPE SOVEREIGN)
 * CLASIFICACIÓN: VIEW LAYER (ESTRATO L5)
 * RESPONSABILIDAD: PUNTO DE ENTRADA SOBERANO Y COMPOSICIÓN DE MARCA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. ZERO ANY: Erradicación total de lints mediante tipos nominales.
 * 2. FULL SYNERGY: Integración de iconografía LucideIcon.
 * 3. ATMOSPHERIC DEPTH: Capas EMI y Grid FX para inmersión total.
 * 4. BIGINT PRECISION: Mapeo de potencia del enjambre sin pérdida.
 * =================================================================
 */

"use client";

import React, { useMemo } from "react";
import { motion } from "framer-motion";
import { useTranslations } from "next-intl";
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
  Search,
  FlaskConical,
  type LucideIcon // ✅ INYECCIÓN DE TIPO SOBERANO
} from "lucide-react";

// --- SINAPSIS DE INFRAESTRUCTURA ---
import { useSystemTelemetry } from "@prospector/api-client";
import { formatComputationalEffort } from "@/lib/utils/telemetry";
import { Link } from "@/lib/schemas/routing";

// --- COMPONENTES DE ESTRATO ---
import { PublicHeader } from "@/components/layout/public-header";
import { PublicFooter } from "@/components/layout/public-footer";
import { StatCard } from "@/components/ui/kit/stat-card";
import { Card } from "@/components/ui/kit/card";
import { Button } from "@/components/ui/kit/button";
import { cn } from "@/lib/utils/cn";

export default function RootLandingPage(): React.ReactElement {
  const t = useTranslations("Landing");
  const tDash = useTranslations("Dashboard");

  // 1. ADQUISICIÓN DE PULSO SISTÉMICO
  const { data: metrics, isLoading } = useSystemTelemetry();

  // 2. PROCESAMIENTO DE POTENCIA SOBERANA
  const hashrate_display = useMemo(() => {
    if (!metrics) return "0.0 MH/s";
    return formatComputationalEffort(metrics.cumulative_global_hashrate.toString());
  }, [metrics]);

  return (
    <div className="relative min-h-screen bg-[#050505] flex flex-col font-mono selection:bg-emerald-500/30 overflow-x-hidden">

      {/* CAPA 0: INFRAESTRUCTURA VISUAL (FX) */}
      <div className="fixed inset-0 pointer-events-none z-0">
        <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-[0.03] mix-blend-overlay" />
        <div className="absolute inset-0 bg-radial-at-t from-emerald-500/10 via-transparent to-transparent opacity-50" />
        <div className="absolute inset-0 bg-grid-pattern opacity-10" />
      </div>

      <PublicHeader />

      <main className="flex-1 relative z-10 pt-32 pb-20">
        {/* SECTOR ALFA: HERO IGNITION */}
        <section className="max-w-7xl mx-auto px-6">
          <div className="grid grid-cols-1 lg:grid-cols-12 gap-12 items-center">

            <motion.div
              initial={{ x: -50, opacity: 0, filter: "blur(10px)" }}
              animate={{ x: 0, opacity: 1, filter: "blur(0px)" }}
              transition={{ duration: 0.8, ease: "easeOut" }}
              className="lg:col-span-7 space-y-8"
            >
              <div className="inline-flex items-center gap-3 px-4 py-2 bg-emerald-500/5 border border-emerald-500/20 rounded-full shadow-[0_0_20px_rgba(16,185,129,0.1)]">
                <Globe className="w-3 h-3 text-emerald-500 animate-spin-slow" />
                <span className="text-[10px] font-black text-emerald-500 uppercase tracking-[0.3em]">
                  {t("hero.badge")}
                </span>
              </div>

              <h1 className="text-6xl md:text-8xl font-black text-white uppercase tracking-tighter italic leading-[0.85] drop-shadow-2xl">
                {t("hero.title").split(' ')[0]} <br />
                <span className="text-emerald-500">{t("hero.title").split(' ').slice(1).join(' ')}</span>
              </h1>

              <p className="text-zinc-500 text-lg md:text-xl max-w-2xl leading-relaxed font-mono">
                {t("hero.subtitle")}
              </p>

              <div className="flex flex-wrap gap-6 pt-4">
                <Link href="/register">
                  <Button variant="cyber" className="h-16 px-10 text-xs font-black tracking-[0.4em]">
                    {t("hero.cta_primary.label")}
                    <ArrowRight className="ml-3 w-4 h-4" />
                  </Button>
                </Link>
                <div className="flex items-center gap-4 px-6 border-l border-white/10 group cursor-help">
                    <Cpu className="w-5 h-5 text-zinc-700 group-hover:text-blue-500 transition-colors" />
                    <div className="flex flex-col">
                        <span className="text-[8px] font-black text-zinc-600 uppercase tracking-widest">Clearance_Required</span>
                        <span className="text-[10px] font-bold text-zinc-400 uppercase">Stratum_L5_Architect</span>
                    </div>
                </div>
              </div>
            </motion.div>

            <motion.div
              initial={{ scale: 0.8, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              transition={{ duration: 1, delay: 0.2 }}
              className="lg:col-span-5 relative hidden lg:block"
            >
              <div className="relative aspect-square bg-zinc-900/20 border border-white/5 rounded-[4rem] backdrop-blur-3xl overflow-hidden group shadow-[0_0_100px_rgba(0,0,0,0.5)]">
                <div className="absolute inset-0 bg-linear-to-br from-emerald-500/5 via-transparent to-blue-500/5" />
                <div className="absolute inset-0 flex items-center justify-center">
                  <Binary className="w-64 h-64 text-emerald-500/10 group-hover:scale-110 group-hover:rotate-12 transition-all duration-1000" />
                </div>
                <div className="absolute bottom-10 left-10 right-10">
                   <div className="p-6 bg-black/60 border border-white/5 rounded-3xl backdrop-blur-md">
                      <div className="flex justify-between items-center mb-4">
                        <span className="text-[8px] font-black text-zinc-600 uppercase tracking-widest">{tDash("header.status_online")}</span>
                        <Activity className="w-3 h-3 text-emerald-500 animate-pulse" />
                      </div>
                      <div className="space-y-1 font-mono text-[10px] text-emerald-500/60 uppercase">
                        <p>{">"} SECP256K1_STRATA_IDENTIFIED</p>
                        <p>{">"} ENTROPY_GAP: 2.44%</p>
                        <p className="animate-pulse">{">"} SEARCHING_ZOMBIE_CLUSTERS...</p>
                      </div>
                   </div>
                </div>
              </div>
            </motion.div>
          </div>
        </section>

        {/* SECTOR BETA: LIVE TELEMETRY HUD */}
        <section className="max-w-7xl mx-auto px-6 mt-32">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            <StatCard
              label={tDash("sidebar.network")}
              value={metrics?.active_nodes_count || 0}
              subValue="Verified Ephemeral Units"
              icon={Server}
              color="emerald"
              loading={isLoading}
            />
            <StatCard
              label="Enjambre_Throughput"
              value={hashrate_display}
              subValue="Combined Projective Power"
              icon={Zap}
              color="blue"
              loading={isLoading}
            />
            <StatCard
              label={tDash("sidebar.uplink")}
              value="Sovereign"
              subValue="Engine A + B Handshake"
              icon={Database}
              color="purple"
              loading={isLoading}
            />
          </div>
        </section>

        {/* SECTOR GAMMA: ESTRATIGRAFÍA (DETERMINISTIC GRID) */}
        <section id="features" className="max-w-7xl mx-auto px-6 mt-40 space-y-20">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            <FeatureApparatus
              icon={Layers}
              title="Jacobian Acceleration"
              desc="U256 Projective space computation eliminating the modular inverse bottleneck."
              stratum="L1_MATH"
            />
            <FeatureApparatus
              icon={Search}
              title="Entropy Archaeology"
              desc="Forensic reconstruction of historical PRNG failures: Debian and Android pattern recognition."
              stratum="L2_STRATEGY"
            />
            <FeatureApparatus
              icon={FlaskConical}
              title="Deterministic Sharding"
              desc="Probabilistic UTXO mapping via parallel Bloom Filters in ephemeral RAM."
              stratum="L1_PROB"
            />
          </div>
        </section>

        {/* SECTOR DELTA: CTAs DE DECISIÓN */}
        <section className="max-w-7xl mx-auto px-6 mt-40">
           <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
             <LandingActionCard
                href="/dashboard/identity"
                icon={ShieldCheck}
                title="01. Identity Vault"
                desc="Inject credentials via local AES-256-GCM. Cookies never touch the server."
                color="blue"
             />
             <LandingActionCard
                href="/dashboard/launch"
                icon={Zap}
                title="02. Launch Swarm"
                desc="Configure node density and fire the swarm via C2 GitHub Actions signal."
                color="emerald"
             />
           </div>
        </section>
      </main>

      <PublicFooter />
    </div>
  );
}

/**
 * ÁTOMO: FEATURE APPARATUS
 * ✅ RESOLUCIÓN TS-EXPLICIT-ANY: Uso de LucideIcon como tipo nominal.
 */
interface FeatureApparatusProps {
  icon: LucideIcon;
  title: string;
  desc: string;
  stratum: string;
}

function FeatureApparatus({ icon: Icon, title, desc, stratum }: FeatureApparatusProps) {
  return (
    <div className="p-10 bg-zinc-950/20 border border-zinc-900 rounded-[2.5rem] hover:border-emerald-500/20 transition-all duration-700 group shadow-lg">
      <div className="p-4 bg-zinc-900 rounded-2xl border border-white/5 w-fit mb-8 group-hover:bg-emerald-500 group-hover:text-black transition-all duration-500">
        <Icon className="w-6 h-6" />
      </div>
      <div className="space-y-4">
        <div className="flex items-center gap-3">
           <span className="text-[10px] font-black text-zinc-700 uppercase tracking-widest">{stratum}</span>
        </div>
        <h4 className="text-xl font-black text-white uppercase tracking-tight italic group-hover:text-emerald-400 transition-colors">
          {title}
        </h4>
        <p className="text-sm text-zinc-500 leading-relaxed font-mono">
          {desc}
        </p>
      </div>
    </div>
  );
}

/**
 * ÁTOMO: ACCIÓN TÁCTICA
 * ✅ RESOLUCIÓN TS-EXPLICIT-ANY: Uso de LucideIcon.
 */
interface LandingActionCardProps {
  href: string;
  icon: LucideIcon;
  title: string;
  desc: string;
  color: "blue" | "emerald";
}

function LandingActionCard({ href, icon: Icon, title, desc, color }: LandingActionCardProps) {
    return (
        <Link href={href} className="group">
            <Card className={cn(
                "p-10 bg-zinc-900/10 border-zinc-800 transition-all duration-700 h-full relative overflow-hidden",
                color === "blue" ? "hover:border-blue-500/30" : "hover:border-emerald-500/30"
            )}>
                <div className="flex justify-between items-start relative z-10">
                    <div className="space-y-4">
                        <h3 className="text-lg font-black text-white uppercase tracking-widest group-hover:text-primary transition-colors">
                            {title}
                        </h3>
                        <p className="text-xs text-zinc-500 max-w-[300px] leading-relaxed">
                            {desc}
                        </p>
                    </div>
                    <Icon className={cn(
                        "w-12 h-12 opacity-5 group-hover:opacity-20 transition-all duration-700",
                        color === "blue" ? "text-blue-500" : "text-emerald-500"
                    )} />
                </div>
                <div className="mt-8 flex items-center gap-3 text-zinc-700 group-hover:text-white transition-colors">
                    <span className="text-[10px] font-black uppercase tracking-widest">Enter_Stratum</span>
                    <ArrowRight className="w-4 h-4 group-hover:translate-x-2 transition-transform" />
                </div>
            </Card>
        </Link>
    );
}

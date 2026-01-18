/**
 * =================================================================
 * APARATO: ACADEMY ZENITH HUB (V2.2 - LINT PURGED)
 * CLASIFICACIÓN: FEATURE VIEW (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN DEL GRAFO DE CONOCIMIENTO
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. HYGIENE: Erradicación total de 'unused-vars'. 'BrainCircuit' integrado como motor visual.
 * 2. TYPE SOBERANEITY: Consumo de tipos exportados nominalmente desde L2.
 * 3. ZENITH UX: Implementa 'Bento Grid' con refracción y profundidad Z.
 * =================================================================
 */

"use client";

import React, { useMemo } from "react";
import { useQuery } from "@tanstack/react-query";
import { useTranslations } from "next-intl";
import { motion, AnimatePresence } from "framer-motion";
import {
  GraduationCap, Zap, Lock, Unlock, CheckCircle2,
  Cpu, Fingerprint, Network, Binary, ArrowRight,
  ShieldCheck, BrainCircuit, Star
} from "lucide-react";

// --- SINAPSIS DE INFRAESTRUCTURA ---
import { neuralOracle } from "@prospector/api-client";
import {
  type KnowledgeModule,
  type OperatorAcademyProgress
} from "@prospector/api-contracts";
import { Card, CardContent } from "@/components/ui/kit/card";
import { Button } from "@/components/ui/kit/button";
import { cn } from "@/lib/utils/cn";

export default function AcademyPage(): React.ReactElement {
  const t = useTranslations("Dashboard.academy_portal");
  const operator_id = "OPERATOR_GÉN_01";

  /**
   * ADQUISICIÓN DE CONOCIMIENTO (NEURAL ORACLE)
   * Hidratación del currículum y estatus de maestría.
   */
  const { data: academy_data, isLoading } = useQuery({
    queryKey: ["neural-academy-strata", operator_id],
    queryFn: async () => {
      const query = `
        query GetAcademy($opId: String!) {
          getAdaptiveCurriculum(operatorId: $opId) {
            identifier
            i18nTitleKey
            i18nDescriptionKey
            difficulty
            estimatedDurationMinutes
            currentStatus
            visualIconSignature
            prerequisiteIdentifiers
          }
          getOperatorMastery(operatorId: $opId) {
            certifiedModulesCount
            totalMiningTimeMinutes
            masterStratumLevel
          }
        }
      `;
      return await neuralOracle.query<{
        getAdaptiveCurriculum: KnowledgeModule[],
        getOperatorMastery: OperatorAcademyProgress
      }>(query, { opId: operator_id });
    },
  });

  const modules = useMemo(() => academy_data?.getAdaptiveCurriculum || [], [academy_data]);
  const mastery = useMemo(() => academy_data?.getOperatorMastery, [academy_data]);

  if (isLoading) return <AcademyLoadingSkeleton />;

  return (
    <div className="relative flex flex-col gap-10 font-mono pb-20 select-none animate-in fade-in duration-1000">
      {/* CAPA AMBIENTAL: Grano de seguridad e interferencia */}
      <div className="fixed inset-0 pointer-events-none opacity-[0.03] bg-[url('https://grainy-gradients.vercel.app/noise.svg')] z-0" />

      {/* SECTOR ALFA: MASTERY HUD (THE BENTO HEADER) */}
      <div className="grid grid-cols-1 lg:grid-cols-12 gap-6 relative z-10">
        <motion.div
          initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }}
          className="lg:col-span-8 bg-linear-to-br from-blue-600/10 to-emerald-600/5 border border-white/10 rounded-[2.5rem] p-10 flex flex-col justify-between relative overflow-hidden group shadow-2xl"
        >
          {/* ✅ RESOLUCIÓN LINT: 'BrainCircuit' utilizado como marca de agua cinética */}
          <div className="absolute top-0 right-0 p-12 opacity-5 group-hover:opacity-15 transition-all duration-1000 group-hover:rotate-12 group-hover:scale-110">
            <BrainCircuit className="w-64 h-64 text-emerald-500" />
          </div>

          <div className="space-y-4 relative z-10">
            <div className="flex items-center gap-3">
              <div className="px-3 py-1 bg-blue-500/20 rounded-full border border-blue-500/30">
                <span className="text-[10px] font-black text-blue-400 uppercase tracking-widest animate-pulse">Neural_Sync_Active</span>
              </div>
            </div>
            <h1 className="text-5xl font-black text-white uppercase tracking-tighter italic leading-tight drop-shadow-lg">
              {t("page_title")}
            </h1>
            <p className="text-zinc-500 text-sm max-w-xl leading-relaxed font-mono">
              {t("page_subtitle")}
            </p>
          </div>

          <div className="flex flex-wrap gap-10 mt-12 border-t border-white/5 pt-8 relative z-10">
            <MasteryMetric label="CERTIFIED_UNITS" value={mastery?.certifiedModulesCount || 0} icon={ShieldCheck} />
            <MasteryMetric label="MINING_TIME" value={`${mastery?.totalMiningTimeMinutes || 0}m`} icon={Zap} />
            <MasteryMetric label="STRATUM_LVL" value={mastery?.masterStratumLevel || 1} icon={Star} />
          </div>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, scale: 0.95 }} animate={{ opacity: 1, scale: 1 }}
          className="lg:col-span-4 bg-[#080808]/60 backdrop-blur-3xl border border-zinc-800 rounded-[2.5rem] p-10 flex flex-col items-center justify-center text-center gap-8 shadow-2xl border-t-white/10"
        >
           <div className="relative w-32 h-32 flex items-center justify-center">
              <div className="absolute inset-0 border-2 border-dashed border-emerald-500/10 rounded-full animate-spin-slow" />
              <div className="absolute inset-4 border border-emerald-500/30 rounded-full shadow-[0_0_30px_rgba(16,185,129,0.1)]" />
              <GraduationCap className="w-12 h-12 text-emerald-500 drop-shadow-[0_0_10px_#10b981]" />
           </div>
           <div className="space-y-2">
              <span className="text-[9px] font-black text-zinc-600 uppercase tracking-[0.4em]">Current_Operator_Rank</span>
              <h3 className="text-xl font-bold text-white uppercase tracking-widest italic">Stratum_Archaeologist</h3>
           </div>
           <Button variant="cyber" className="w-full h-14 text-[10px] tracking-[0.4em] shadow-lg">
              RESUME_BREADCRUMB
           </Button>
        </motion.div>
      </div>

      {/* SECTOR BETA: MODULE NEURAL MAP (THE GRID) */}
      <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-8 relative z-10">
        <AnimatePresence mode="popLayout">
          {modules.map((module, index) => (
            <ModuleCard
              key={module.identifier}
              module={module}
              index={index}
              label_start={t("actions.start")}
              label_locked={t("actions.locked")}
            />
          ))}
        </AnimatePresence>
      </div>
    </div>
  );
}

/**
 * ÁTOMO: MÉTRICA DE MAESTRÍA SOBERANA
 */
function MasteryMetric({ label, value, icon: Icon }: { label: string, value: string | number, icon: React.ElementType }) {
  return (
    <div className="flex flex-col gap-1 group/metric">
      <div className="flex items-center gap-2">
        <Icon className="w-3.5 h-3.5 text-zinc-500 group-hover/metric:text-blue-400 transition-colors" />
        <span className="text-[9px] font-black text-zinc-600 uppercase tracking-widest">{label}</span>
      </div>
      <span className="text-3xl font-black text-white tabular-nums tracking-tighter italic">{value}</span>
    </div>
  );
}

/**
 * ÁTOMO: TARJETA DE MÓDULO ZENITH
 */
function ModuleCard({ module, index, label_start, label_locked }: {
  module: KnowledgeModule,
  index: number,
  label_start: string,
  label_locked: string
}) {
  const isLocked = module.currentStatus === 'Locked';
  const isCompleted = module.currentStatus === 'Completed';
  const isUnlocked = module.currentStatus === 'Unlocked';

  const icons: Record<string, React.ElementType> = {
    network: Network,
    fingerprint: Fingerprint,
    zap: Zap,
    cpu: Cpu,
    binary: Binary
  };

  const Icon = icons[module.visualIconSignature] || Binary;

  return (
    <motion.div
      initial={{ opacity: 0, y: 30 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ delay: index * 0.08, duration: 0.8, ease: [0.16, 1, 0.3, 1] }}
      whileHover={{ y: -10 }}
    >
      <Card className={cn(
        "h-full bg-zinc-950/40 border-zinc-800 rounded-[2.5rem] transition-all duration-1000 relative overflow-hidden group/card shadow-xl",
        isUnlocked && "hover:border-blue-500/40 hover:bg-blue-500/[0.02] hover:shadow-[0_0_60px_rgba(59,130,246,0.1)]",
        isCompleted && "border-emerald-500/30 bg-emerald-500/[0.02]",
        isLocked && "grayscale opacity-50 border-red-900/10 cursor-not-allowed"
      )}>
        {/* Glow de estado */}
        <div className={cn(
          "absolute top-0 left-0 w-full h-1 opacity-20",
          isCompleted ? "bg-emerald-500" : isUnlocked ? "bg-blue-500" : "bg-red-900"
        )} />

        <CardContent className="p-10 flex flex-col justify-between h-full min-h-[380px]">
          <div className="space-y-8">
            <div className="flex justify-between items-start">
               <div className={cn(
                 "p-4 rounded-2xl border transition-all duration-700 shadow-inner",
                 isCompleted ? "bg-emerald-500/10 border-emerald-500/20 text-emerald-400" :
                 isUnlocked ? "bg-blue-500/10 border-blue-500/20 text-blue-400" :
                 "bg-zinc-900 border-zinc-800 text-zinc-600"
               )}>
                  <Icon className="w-7 h-7" />
               </div>
               <div className="p-2 bg-black/40 rounded-lg border border-white/5">
                 {isLocked ? <Lock className="w-4 h-4 text-red-900" /> : isCompleted ? <CheckCircle2 className="w-4 h-4 text-emerald-500 animate-pulse" /> : <Unlock className="w-4 h-4 text-blue-500" />}
               </div>
            </div>

            <div className="space-y-4">
              <div className="flex items-center gap-3">
                <span className="text-[9px] font-black text-zinc-600 uppercase tracking-[0.2em]">{module.identifier}</span>
                <span className="h-px w-12 bg-zinc-900" />
                <span className={cn(
                  "text-[8px] font-bold uppercase px-2 py-0.5 rounded border",
                  module.difficulty === 'Elite' ? "text-purple-400 border-purple-900/30" : "text-zinc-500 border-zinc-800"
                )}>{module.difficulty}</span>
              </div>
              <h3 className="text-2xl font-black text-white uppercase tracking-tighter group-hover/card:text-primary transition-colors leading-none italic">
                {module.i18nTitleKey}
              </h3>
              <p className="text-[13px] text-zinc-500 leading-relaxed font-mono line-clamp-3 group-hover/card:text-zinc-400 transition-colors">
                {module.i18nDescriptionKey}
              </p>
            </div>
          </div>

          <div className="pt-10 flex flex-col gap-5 border-t border-white/5">
             <div className="flex justify-between items-center text-[10px] font-black text-zinc-600 uppercase tracking-widest">
                <span className="flex items-center gap-2"><Star className="w-3 h-3" /> {module.estimatedDurationMinutes} MINS</span>
                <span className={isCompleted ? "text-emerald-500" : ""}>{module.currentStatus}</span>
             </div>
             <Button
              variant={isCompleted ? "outline" : "cyber"}
              disabled={isLocked}
              className={cn(
                "h-16 w-full rounded-2xl font-black tracking-[0.4em] text-[10px] uppercase",
                isCompleted && "border-emerald-500/20 text-emerald-500 hover:bg-emerald-500 hover:text-black shadow-[0_0_20px_rgba(16,185,129,0.1)]"
              )}
             >
                {isLocked ? label_locked : isCompleted ? "RE-CERTIFY_MODULE" : label_start}
                {!isLocked && <ArrowRight className="w-4 h-4 ml-4 transition-transform group-hover/card:translate-x-2" />}
             </Button>
          </div>
        </CardContent>
      </Card>
    </motion.div>
  );
}

/**
 * SKELETON DE CARGA ZENITH
 */
function AcademyLoadingSkeleton(): React.ReactElement {
  return (
    <div className="space-y-12 p-10 animate-pulse font-mono">
      <div className="h-80 w-full bg-zinc-900/50 rounded-[3rem] border border-white/5" />
      <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
        {[1, 2, 3].map(i => (
          <div key={i} className="h-[450px] bg-zinc-900/20 rounded-[3rem] border border-white/5" />
        ))}
      </div>
    </div>
  );
}

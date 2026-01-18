/**
 * =================================================================
 * APARATO: COMMAND DECK VIEW (V1.1 - COMMAND VERIFIED)
 * CLASIFICACIÓN: FEATURE VIEW (ESTRATO L5)
 * RESPONSABILIDAD: INTERFAZ DE EJECUCIÓN CON SOPORTE WINDOWS/PNPM
 * =================================================================
 */

"use client";

import React, { useState } from "react";
import { useTranslations } from "next-intl";
import { motion } from "framer-motion";
import {
   Search, Copy, Check, Command as CommandIcon,
   Activity
} from "lucide-react";
import { toast } from "sonner";
import { cn } from "@/lib/utils/cn";
import { Card, CardContent } from "@/components/ui/kit/card";
import { Button } from "@/components/ui/kit/button";

interface ScriptItem {
  id: string;
  command: string;
  stratum: string;
  risk: "SAFE" | "CAUTION" | "CRITICAL";
}

export default function ScriptsPage() {
  const t = useTranslations("Dashboard.scripts");
  const [copiedId, setCopiedId] = useState<string | null>(null);

  /**
   * MATRIZ DE COMANDOS SOBERANOS
   * ✅ VERIFICACIÓN: El comando 'purge_github' ha sido inyectado con el
   * estándar de transpilación CommonJS certificado en su terminal.
   */
  const TACTICAL_SCRIPTS: ScriptItem[] = [
    {
      id: "purge_github",
      command: 'pnpm exec cross-env TS_NODE_COMPILER_OPTIONS="{\\"module\\":\\"commonjs\\",\\"esModuleInterop\\\":true}" ts-node tools/scripts/ops/purge-github-queue.ts',
      stratum: "L6_OPS",
      risk: "CRITICAL"
    },
    { id: "db_migrate", command: "pnpm db:migrate", stratum: "L3_INFRA", risk: "CAUTION" },
    { id: "audit_health", command: "pnpm audit:health", stratum: "L6_OPS", risk: "SAFE" },
    { id: "i18n_gen", command: "pnpm i18n:generate", stratum: "L5_VIEW", risk: "SAFE" },
    { id: "build_miner", command: "bash ./scripts/build_miner_static.sh", stratum: "L1_CORE", risk: "SAFE" },
  ];

  const handleCopy = (cmd: string, id: string) => {
    navigator.clipboard.writeText(cmd);
    setCopiedId(id);
    toast.success("COMMAND_SECURED", {
      description: "Ready for CMD terminal execution.",
      className: "font-mono text-[10px]"
    });
    setTimeout(() => setCopiedId(null), 2000);
  };

  return (
    <div className="relative flex flex-col gap-10 font-mono pb-20 animate-in fade-in duration-1000">
      {/* CAPA AMBIENTAL FX */}
      <div className="fixed inset-0 pointer-events-none opacity-[0.03] bg-[url('https://grainy-gradients.vercel.app/noise.svg')] z-0" />

      {/* HEADER TÁCTICO */}
      <motion.div
        initial={{ x: -20, opacity: 0 }}
        animate={{ x: 0, opacity: 1 }}
        className="flex flex-col gap-3 border-l-4 border-amber-500 pl-8 py-2 relative z-10"
      >
        <h1 className="text-5xl font-black text-white uppercase tracking-tighter italic leading-none">
          {t("page_title")}
        </h1>
        <div className="flex items-center gap-4">
           <p className="text-zinc-500 text-sm max-w-2xl">{t("page_subtitle")}</p>
           <div className="h-px flex-1 bg-zinc-900 hidden lg:block" />
           <div className="px-3 py-1 bg-amber-500/10 border border-amber-500/20 rounded-full">
              <span className="text-[10px] font-black text-amber-500 animate-pulse uppercase tracking-widest">C2_Terminal_Active</span>
           </div>
        </div>
      </motion.div>

      {/* GRID DE INSTRUMENTACIÓN */}
      <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-8 relative z-10">
        {TACTICAL_SCRIPTS.map((script, index) => (
          <motion.div
            key={script.id}
            initial={{ y: 20, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{ delay: index * 0.1 }}
            whileHover={{ y: -5 }}
          >
            <Card className={cn(
                "bg-[#080808]/60 backdrop-blur-2xl border-zinc-800 rounded-[2.5rem] overflow-hidden group transition-all duration-700 hover:shadow-[0_0_50px_rgba(245,158,11,0.05)]",
                script.risk === "CRITICAL" ? "hover:border-red-500/30" : "hover:border-amber-500/30"
            )}>
              <CardContent className="p-10 space-y-8">

                <div className="flex justify-between items-start">
                  <div className="flex items-center gap-5">
                    <div className={cn(
                        "p-4 rounded-2xl border transition-all duration-500 shadow-inner",
                        script.risk === "CRITICAL" ? "bg-red-500/10 border-red-500/20 text-red-500" : "bg-zinc-900 border-zinc-800 text-amber-500"
                    )}>
                      <CommandIcon className="w-6 h-6" />
                    </div>
                    <div>
                      <h3 className="text-md font-black text-white uppercase tracking-widest leading-none">
                        {t(`definitions.${script.id}.label`)}
                      </h3>
                      <div className="flex items-center gap-2 mt-2">
                         <span className="text-[9px] text-zinc-600 font-bold uppercase tracking-widest">{script.stratum}</span>
                         <div className="w-1 h-1 rounded-full bg-zinc-800" />
                         <span className="text-[9px] text-zinc-600 font-bold uppercase tracking-widest">v1.0</span>
                      </div>
                    </div>
                  </div>

                  {/* LA LUPA SOBERANA (Tooltip) */}
                  <div className="relative group/tooltip">
                    <div className="p-2.5 bg-black/40 rounded-full border border-white/5 cursor-help hover:bg-white/10 transition-all">
                      <Search className="w-4 h-4 text-zinc-500" />
                    </div>

                    <div className="absolute right-0 bottom-full mb-6 w-72 p-6 bg-black/95 border border-zinc-700 rounded-3xl shadow-[0_20px_50px_rgba(0,0,0,0.5)] opacity-0 pointer-events-none group-hover/tooltip:opacity-100 group-hover/tooltip:translate-y-0 translate-y-4 transition-all duration-500 z-50">
                       <div className="flex items-center gap-3 mb-4 border-b border-white/5 pb-3">
                          <Activity className="w-4 h-4 text-amber-500" />
                          <span className="text-[10px] font-black text-zinc-400 uppercase tracking-widest">Forense_Analysis</span>
                       </div>
                       <p className="text-[11px] text-zinc-300 leading-relaxed italic font-mono">
                         {t(`definitions.${script.id}.desc`)}
                       </p>
                    </div>
                  </div>
                </div>

                {/* VISUALIZADOR DE COMANDO (GOLD MASTER ALIGNED) */}
                <div className="space-y-3">
                   <div className="flex justify-between items-center px-1">
                      <span className="text-[8px] font-black text-zinc-700 uppercase tracking-widest">CLI_Source_String</span>
                      <span className="text-[8px] font-bold text-emerald-600 animate-pulse">VERIFIED_FOR_WIN10</span>
                   </div>
                   <div className="bg-black/80 rounded-2xl p-5 border border-white/5 relative group/code shadow-inner">
                      <code className="text-[10px] text-emerald-500/60 font-mono break-all leading-relaxed block pr-8">
                        {script.command}
                      </code>
                      <button
                        onClick={() => handleCopy(script.command, script.id)}
                        className="absolute right-4 top-4 p-2.5 rounded-xl bg-zinc-900 border border-white/10 text-zinc-400 hover:text-emerald-500 hover:border-emerald-500/30 transition-all opacity-0 group-hover/code:opacity-100"
                      >
                        {copiedId === script.id ? <Check className="w-4 h-4 text-emerald-500" /> : <Copy className="w-4 h-4" />}
                      </button>
                   </div>
                </div>

                {/* ESTRATO DE RIESGO */}
                <div className="flex items-center justify-between pt-6 border-t border-white/5">
                  <div className="flex items-center gap-3">
                    <div className={cn(
                      "w-2 h-2 rounded-full",
                      script.risk === "SAFE" ? "bg-emerald-500 shadow-[0_0_10px_#10b981]" :
                      script.risk === "CAUTION" ? "bg-amber-500 shadow-[0_0_10px_#f59e0b]" :
                      "bg-red-500 shadow-[0_0_10px_#ef4444] animate-pulse"
                    )} />
                    <span className={cn(
                        "text-[9px] font-black uppercase tracking-[0.2em]",
                        script.risk === "CRITICAL" ? "text-red-500" : "text-zinc-500"
                    )}>
                      {script.risk}
                    </span>
                  </div>
                  <Button
                    variant="ghost"
                    className="h-10 text-[10px] font-black uppercase tracking-widest hover:bg-white/5 hover:text-white"
                    onClick={() => handleCopy(script.command, script.id)}
                  >
                    {t("labels.copy_cmd")}
                  </Button>
                </div>
              </CardContent>
            </Card>
          </motion.div>
        ))}
      </div>

      <footer className="flex flex-col items-center gap-6 pt-20 opacity-20 relative z-10">
          <div className="flex gap-4">
              <div className="w-1.5 h-1.5 rounded-full bg-zinc-700" />
              <div className="w-32 h-1 rounded-full bg-zinc-900" />
              <div className="w-1.5 h-1.5 rounded-full bg-zinc-700" />
          </div>
          <p className="text-[10px] uppercase tracking-[1.5em] text-zinc-700 font-black italic">
            Sovereign_Protocol_Shell // V11.5 // 2026
          </p>
      </footer>
    </div>
  );
}

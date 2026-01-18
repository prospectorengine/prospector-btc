/**
 * =================================================================
 * APARATO: FINDINGS INVENTORY ZENITH (V27.1 - REPAIRED)
 * CLASIFICACIÓN: FEATURE UI ORGANISM (ESTRATO L5)
 * RESPONSABILIDAD: GESTIÓN VISUAL DE COLISIONES CRIPTOGRÁFICAS
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la interfaz de inspección de hallazgos definitivos.
 * 1. FIX: Inyección de iconos faltantes (CheckCircle2, AlertCircle).
 * 2. HYGIENE: Eliminación de imports muertos (Check, LucideIcon).
 * 3. ZENITH UX: Mantiene la estética de cristal y neón.
 * =================================================================
 */

"use client";

import React, { useState, useCallback } from "react";
import {
  ShieldCheck,
  Lock,
  Eye,
  EyeOff,
  Terminal,
  Database,
  Fingerprint,
  Activity,
  Copy,
  Zap,
  ExternalLink,
  Search,
  // ✅ FIX: Iconos requeridos inyectados, muertos eliminados
  CheckCircle2,
  AlertCircle
} from "lucide-react";
import { useQuery } from "@tanstack/react-query";
import { motion, AnimatePresence } from "framer-motion";
import { toast } from "sonner";

// --- SINAPSIS NEURAL (L2 - L4) ---
import { apiClient, type Finding } from "@prospector/api-client";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/kit/card";
import { Skeleton } from "@/components/ui/kit/skeleton";
import { cn } from "@/lib/utils/cn";

export function FindingsInventory(): React.ReactElement {
  const [visible_keys_map, set_visible_keys_map] = useState<Record<string, boolean>>({});

  // 1. ADQUISICIÓN DE DATOS TÁCTICOS (L3 -> L4)
  const { data: findings_collection, isLoading, isError } = useQuery<Finding[]>({
    queryKey: ["tactical-findings-zenith-v27"],
    queryFn: () => apiClient.get<Finding[]>("/swarm/findings"),
    refetchInterval: 15000,
  });

  const toggle_key_security = useCallback((address: string) => {
    set_visible_keys_map(prev => ({ ...prev, [address]: !prev[address] }));
  }, []);

  const copy_to_clipboard = (text: string, label: string) => {
    navigator.clipboard.writeText(text);
    toast.success(`${label}_COPIED`, {
        description: "Material secured in local clipboard.",
        icon: <Copy className="w-4 h-4 text-emerald-500" />
    });
  };

  if (isError) return <ErrorState />;

  return (
    <Card className="flex flex-col h-full bg-[#050505]/60 backdrop-blur-3xl border border-zinc-800 rounded-[2.5rem] overflow-hidden shadow-[0_0_100px_rgba(16,185,129,0.05)] group font-mono relative">

      {/* CAPA ATMOSFÉRICA */}
      <div className="absolute inset-0 pointer-events-none opacity-[0.03] bg-[url('https://grainy-gradients.vercel.app/noise.svg')] z-0" />
      <div className="absolute top-0 inset-x-0 h-px bg-linear-to-r from-transparent via-emerald-500/30 to-transparent z-10" />

      {/* SECTOR 1: CABECERA DE BÓVEDA (VAULT AUTHORITY) */}
      <CardHeader className="p-8 border-b border-white/5 bg-emerald-500/5 flex flex-row items-center justify-between relative z-10 space-y-0">
        <div className="flex items-center gap-6">
          <div className="relative">
            <div className="absolute inset-0 bg-emerald-500/20 blur-2xl rounded-2xl animate-pulse" />
            <div className="relative p-3.5 bg-zinc-950 rounded-2xl border border-emerald-500/40 shadow-2xl">
              <ShieldCheck className="w-6 h-6 text-emerald-500" />
            </div>
          </div>
          <div>
            <CardTitle className="text-sm font-black text-white uppercase tracking-[0.5em] leading-none italic">
              Tactical_<span className="text-emerald-500">Vault</span>_Findings
            </CardTitle>
            <p className="text-[9px] text-zinc-500 font-bold uppercase tracking-widest mt-3 flex items-center gap-2">
              <Database className="w-3.5 h-3.5 text-zinc-700" />
              Stratum L3 // Collision_Inventory // Synchronized
            </p>
          </div>
        </div>

        <div className="flex items-center gap-4 bg-black/60 px-5 py-2 rounded-full border border-emerald-500/10">
           <span className="text-[10px] font-black text-emerald-500 tabular-nums">
              {findings_collection?.length || 0} COLLISIONS
           </span>
        </div>
      </CardHeader>

      {/* SECTOR 2: FINDINGS STREAM (DATA ESTRATIGRAPHY) */}
      <CardContent className="flex-1 overflow-y-auto custom-scrollbar p-0 relative z-10">
        <AnimatePresence mode="popLayout">
          {isLoading ? (
            <VaultSkeleton />
          ) : findings_collection?.length === 0 ? (
            <div className="p-32 text-center flex flex-col items-center gap-8 opacity-20">
               <Activity className="w-16 h-16 text-zinc-600 animate-pulse" />
               <span className="text-xs font-black uppercase tracking-[0.8em] text-zinc-700">Awaiting_Collision_Signal...</span>
            </div>
          ) : (
            <div className="divide-y divide-white/5">
              {findings_collection?.map((finding, idx) => (
                <FindingRow
                  key={finding.address}
                  finding={finding}
                  index={idx}
                  is_visible={!!visible_keys_map[finding.address]}
                  onToggle={() => toggle_key_security(finding.address)}
                  onCopy={copy_to_clipboard}
                />
              ))}
            </div>
          )}
        </AnimatePresence>
      </CardContent>

      {/* SECTOR 3: FOOTER DE SEGURIDAD */}
      <footer className="p-8 bg-zinc-950/80 border-t border-white/5 flex flex-col xl:flex-row justify-between items-center gap-6 relative z-10">
        <div className="flex items-center gap-8">
           <div className="flex items-center gap-3 opacity-40 hover:opacity-100 transition-opacity duration-500 cursor-help">
              <Terminal className="w-4 h-4 text-zinc-500" />
              <span className="text-[8px] font-black text-zinc-600 uppercase tracking-widest">Vault_Security: GCM_AES_256</span>
           </div>
           <div className="h-4 w-px bg-white/5" />
           <div className="flex items-center gap-3">
              <Zap className="w-3.5 h-3.5 text-amber-500 animate-pulse" />
              <span className="text-[8px] font-black text-zinc-500 uppercase tracking-widest">Real_Time_Siphon: Online</span>
           </div>
        </div>
        <span className="text-[9px] font-black text-zinc-800 uppercase tracking-[0.8em] italic">
          Master_Vault_V27.0
        </span>
      </footer>
    </Card>
  );
}

/**
 * ÁTOMO: FILA DE HALLAZGO ZENITH
 */
function FindingRow({ finding, index, is_visible, onToggle, onCopy }: {
    finding: Finding,
    index: number,
    is_visible: boolean,
    onToggle: () => void,
    onCopy: (t: string, l: string) => void
}) {
    return (
        <motion.div
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ delay: index * 0.05, duration: 0.5 }}
            className="p-8 hover:bg-emerald-500/[0.02] transition-all duration-500 group/item relative overflow-hidden"
        >
            <div className="flex flex-col xl:flex-row justify-between items-start xl:items-center gap-8 relative z-10">

                {/* Identity Quadrant */}
                <div className="space-y-4 flex-1 w-full">
                    <div className="flex items-center gap-4">
                        <div className="p-2.5 bg-zinc-900 rounded-xl border border-white/5 shadow-inner group-hover/item:border-emerald-500/30 transition-all duration-700">
                           <Fingerprint className="w-4 h-4 text-zinc-600 group-hover/item:text-emerald-400" />
                        </div>
                        <div className="flex flex-col">
                            <span className="text-[9px] font-black text-zinc-600 uppercase tracking-widest mb-1">Target_Bitcoin_Address</span>
                            <div className="flex items-center gap-3">
                                <span className="text-sm font-black text-white tracking-tighter select-all">
                                    {finding.address}
                                </span>
                                <button onClick={() => onCopy(finding.address, "ADDRESS")} className="p-1 hover:bg-emerald-500/10 rounded transition-colors">
                                    <Copy className="w-3 h-3 text-zinc-700 hover:text-emerald-500" />
                                </button>
                            </div>
                        </div>
                    </div>

                    {/* WIF Security Stratum */}
                    <div className="relative group/wif w-full">
                        <div className="flex items-center justify-between mb-2 px-1">
                            <div className="flex items-center gap-2">
                                <Lock className="w-3 h-3 text-amber-600" />
                                <span className="text-[8px] font-black text-zinc-700 uppercase tracking-widest">Secret_WIF_Material</span>
                            </div>
                            <button onClick={onToggle} className="flex items-center gap-2 text-zinc-600 hover:text-white transition-colors">
                                {is_visible ? <EyeOff className="w-3.5 h-3.5" /> : <Eye className="w-3.5 h-3.5" />}
                                <span className="text-[8px] font-black uppercase tracking-widest">{is_visible ? 'Secure' : 'Reveal'}</span>
                            </button>
                        </div>
                        <div className={cn(
                            "p-4 rounded-2xl font-mono text-xs border transition-all duration-700 flex items-center justify-between overflow-hidden",
                            is_visible
                                ? "bg-zinc-900/50 border-amber-500/40 text-amber-400 shadow-[inset_0_0_20px_rgba(245,158,11,0.05)]"
                                : "bg-black border-white/5 text-zinc-800"
                        )}>
                            <span className={cn("select-all truncate pr-4", !is_visible && "blur-[6px] opacity-30")}>
                                {finding.private_key_wif}
                            </span>
                            {is_visible && (
                                <button onClick={() => onCopy(finding.private_key_wif, "PRIVATE_KEY")} className="shrink-0 hover:scale-110 transition-transform">
                                    <Zap className="w-4 h-4 text-amber-500 fill-amber-500" />
                                </button>
                            )}
                        </div>
                    </div>
                </div>

                {/* Context Quadrant */}
                <div className="xl:w-72 w-full space-y-4">
                    <div className="bg-zinc-950/50 border border-white/5 p-5 rounded-2xl space-y-3 shadow-inner group-hover/item:border-blue-500/20 transition-all duration-700">
                        <div className="flex items-center gap-3">
                            <Search className="w-3.5 h-3.5 text-blue-500" />
                            <span className="text-[9px] font-black text-zinc-500 uppercase tracking-widest">Entropy_Source</span>
                        </div>
                        <p className="text-[11px] text-zinc-300 italic leading-relaxed pl-6 border-l border-zinc-800">
                            {finding.source_entropy}
                        </p>
                    </div>

                    <div className="flex justify-between items-center px-2">
                        <div className="flex items-center gap-2">
                            {/* ✅ FIX: Uso activo del icono importado */}
                            <CheckCircle2 className="w-3 h-3 text-emerald-600" />
                            <span className="text-[8px] font-black text-zinc-700 uppercase tracking-widest">Verified_UTXO</span>
                        </div>
                        <a
                            href={`https://mempool.space/address/${finding.address}`}
                            target="_blank"
                            rel="noreferrer"
                            className="p-2 bg-white/5 rounded-lg hover:bg-blue-500 hover:text-black transition-all group/link"
                        >
                            <ExternalLink className="w-3 h-3" />
                        </a>
                    </div>
                </div>
            </div>

            {/* Efecto decorativo de fila activa */}
            <div className="absolute left-0 top-0 bottom-0 w-1 bg-emerald-500 opacity-0 group-hover/item:opacity-100 transition-opacity" />
        </motion.div>
    );
}

function VaultSkeleton() {
  return (
    <div className="p-8 space-y-8">
      {[...Array(3)].map((_, i) => (
        <Skeleton key={i} className="h-40 w-full rounded-[2rem] bg-zinc-900/50" />
      ))}
    </div>
  );
}

function ErrorState() {
  return (
    <div className="h-full flex flex-col items-center justify-center p-20 text-center gap-6 font-mono">
       {/* ✅ FIX: Uso activo del icono importado */}
       <AlertCircle className="w-16 h-16 text-red-500 animate-bounce" />
       <div className="space-y-2">
          <h3 className="text-xl font-black text-white uppercase tracking-tighter">Vault_Link_Severed</h3>
          <p className="text-zinc-600 text-xs uppercase tracking-widest">Unable to synchronize collision strata</p>
       </div>
    </div>
  );
}

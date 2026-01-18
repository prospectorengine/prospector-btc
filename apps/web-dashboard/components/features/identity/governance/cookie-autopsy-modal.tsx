// INICIO DEL ARCHIVO [apps/web-dashboard/components/features/identity/governance/cookie-autopsy-modal.tsx]
/**
 * =================================================================
 * APARATO: COOKIE AUTOPSY MODAL (V1.1 - LINT FIXED)
 * CLASIFICACIÓN: FEATURE UI (ESTRATO L5)
 * RESPONSABILIDAD: INSPECCIÓN FORENSE CLIENT-SIDE DE CREDENCIALES
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementación "Self-Contained" del modal para eliminar dependencia
 * de UI Kits externos no existentes. Tipado estricto en manejo de errores.
 * =================================================================
 */

"use client";

import { useState } from "react";
import { Button } from "@/components/ui/kit/button";
import { Input } from "@/components/ui/kit/input";
import { ShieldAlert, Microscope, Lock, Unlock, FileJson, Clock } from "lucide-react";
import { type Identity } from "@prospector/api-client";
import { type DeepAuditResult } from "@/hooks/use-identity-governance";
import { cn } from "@/lib/utils/cn";

interface AutopsyProps {
  identity: Identity | null;
  isOpen: boolean;
  onClose: () => void;
  onAuditRequest: (identity: Identity, key: string) => Promise<DeepAuditResult>;
}

export function CookieAutopsyModal({ identity, isOpen, onClose, onAuditRequest }: AutopsyProps) {
  const [masterKey, setMasterKey] = useState("");
  const [auditResult, setAuditResult] = useState<DeepAuditResult | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleDecryption = async () => {
    if (!identity || !masterKey) return;
    setIsLoading(true);
    setError(null);
    try {
      const result = await onAuditRequest(identity, masterKey);
      setAuditResult(result);
    } catch (err: unknown) {
      // ✅ RESOLUCIÓN: Narrowing de tipo 'unknown' para evitar 'no-explicit-any'
      const msg = err instanceof Error ? err.message : "CIPHER_INTEGRITY_FAIL";
      setError(`DECRYPTION_FAILED: ${msg}`);
    } finally {
      setIsLoading(false);
    }
  };

  const handleClose = () => {
    setMasterKey("");
    setAuditResult(null);
    setError(null);
    onClose();
  };

  if (!identity) return null;

  return (
    // ESTRUCTURA MODAL ATÓMICA (Sin dependencias externas)
    <div className={cn(
        "fixed inset-0 z-50 flex items-center justify-center bg-black/90 backdrop-blur-sm transition-opacity duration-300",
        isOpen ? "opacity-100 pointer-events-auto" : "opacity-0 pointer-events-none"
    )}>
      <div className="bg-[#0a0a0a] border border-zinc-800 w-full max-w-2xl rounded-2xl shadow-2xl overflow-hidden flex flex-col max-h-[90vh] animate-in fade-in zoom-in-95 duration-300">

        {/* HEADER TÁCTICO */}
        <div className="p-6 border-b border-zinc-800 bg-zinc-900/30 flex justify-between items-start">
            <div className="space-y-1">
                <h3 className="text-sm font-black text-white uppercase tracking-widest flex items-center gap-2">
                    <Microscope className="w-4 h-4 text-purple-500" />
                    Identity_Forensics // {identity.platform.toUpperCase()}
                </h3>
                <p className="text-[10px] text-zinc-500 font-mono">
                    TARGET: <span className="text-zinc-300">{identity.email}</span>
                </p>
            </div>
            <button
                onClick={handleClose}
                className="text-zinc-500 hover:text-white transition-colors h-8 w-8 flex items-center justify-center rounded-full hover:bg-white/10"
            >
                ✕
            </button>
        </div>

        {/* CONTENT VIEWPORT */}
        <div className="p-6 flex-1 overflow-y-auto custom-scrollbar">
            {!auditResult ? (
                <div className="flex flex-col items-center justify-center space-y-6 py-10">
                    <div className="w-16 h-16 bg-zinc-900 rounded-full flex items-center justify-center border border-zinc-800 shadow-inner">
                        <Lock className="w-6 h-6 text-zinc-600" />
                    </div>
                    <div className="text-center space-y-2">
                        <h4 className="text-xs font-bold text-zinc-300 uppercase tracking-wider">Zero-Knowledge Protocol</h4>
                        <p className="text-[10px] text-zinc-500 max-w-xs mx-auto leading-relaxed">
                            This payload is encrypted with AES-256-GCM. The server does not possess the key.
                            Provide the Master Key to perform a client-side autopsy.
                        </p>
                    </div>
                    <div className="flex gap-2 w-full max-w-sm">
                        <Input
                            type="password"
                            placeholder="MASTER_VAULT_KEY"
                            value={masterKey}
                            onChange={(e) => setMasterKey(e.target.value)}
                            className="font-mono text-center tracking-[0.3em] text-emerald-500 bg-black border-zinc-800 focus:border-emerald-500/50 h-12"
                        />
                        <Button variant="cyber" onClick={handleDecryption} isLoading={isLoading} className="h-12 w-16">
                            <Unlock className="w-4 h-4" />
                        </Button>
                    </div>
                    {error && (
                        <div className="flex items-center gap-2 text-red-400 bg-red-950/20 px-4 py-3 rounded-lg border border-red-900/30 w-full max-w-sm animate-in slide-in-from-top-2">
                            <ShieldAlert className="w-4 h-4 shrink-0" />
                            <span className="text-[9px] font-bold font-mono">{error}</span>
                        </div>
                    )}
                </div>
            ) : (
                <div className="space-y-6 animate-in fade-in slide-in-from-bottom-4">
                    {/* METRICS GRID */}
                    <div className="grid grid-cols-3 gap-4">
                        <div className={cn(
                            "p-4 rounded-xl border flex flex-col items-center justify-center gap-2",
                            auditResult.health_metrics.status === 'HEALTHY' ? "bg-emerald-950/10 border-emerald-900/30 text-emerald-500" :
                            auditResult.health_metrics.status === 'CRITICAL' ? "bg-red-950/10 border-red-900/30 text-red-500" :
                            "bg-amber-950/10 border-amber-900/30 text-amber-500"
                        )}>
                            <span className="text-[9px] font-black uppercase tracking-widest opacity-70">Health_Status</span>
                            <span className="text-xl font-black tracking-tighter">{auditResult.health_metrics.status}</span>
                        </div>

                        <div className="p-4 rounded-xl border border-zinc-800 bg-zinc-900/20 flex flex-col items-center justify-center gap-2 text-zinc-400">
                            <span className="text-[9px] font-black uppercase tracking-widest opacity-70">TTL_Projection</span>
                            <div className="flex items-center gap-2">
                                <Clock className="w-4 h-4 text-blue-500" />
                                <span className="text-lg font-bold text-white tabular-nums">{auditResult.health_metrics.ttl_days ?? "?"} Days</span>
                            </div>
                        </div>

                        <div className="p-4 rounded-xl border border-zinc-800 bg-zinc-900/20 flex flex-col items-center justify-center gap-2 text-zinc-400">
                            <span className="text-[9px] font-black uppercase tracking-widest opacity-70">Cookie_Count</span>
                            <div className="flex items-center gap-2">
                                <FileJson className="w-4 h-4 text-purple-500" />
                                <span className="text-lg font-bold text-white tabular-nums">{auditResult.decrypted_cookies_count}</span>
                            </div>
                        </div>
                    </div>

                    {/* RISK FACTORS */}
                    {auditResult.health_metrics.risk_factors.length > 0 && (
                        <div className="space-y-2">
                            <label className="text-[9px] font-black text-red-500 uppercase tracking-widest pl-1">Risk Factors Detected</label>
                            <div className="bg-red-950/10 border border-red-900/20 rounded-xl p-4 space-y-2">
                                {auditResult.health_metrics.risk_factors.map((risk, idx) => (
                                    <div key={idx} className="flex items-center gap-2 text-[10px] text-red-400 font-mono">
                                        <div className="w-1.5 h-1.5 bg-red-500 rounded-full animate-pulse" />
                                        {risk}
                                    </div>
                                ))}
                            </div>
                        </div>
                    )}

                    {/* RAW DATA EXPIRATION */}
                    <div className="pt-4 border-t border-zinc-800 flex justify-between items-center px-2">
                        <span className="text-[9px] text-zinc-500 uppercase tracking-widest font-bold">
                            Calculated Expiration Date
                        </span>
                        <span className="text-xs font-mono text-white font-bold bg-zinc-900 px-2 py-1 rounded">
                            {auditResult.health_metrics.expiration_date?.toUTCString() ?? "N/A"}
                        </span>
                    </div>
                </div>
            )}
        </div>
      </div>
    </div>
  );
}
// FIN DEL ARCHIVO [apps/web-dashboard/components/features/identity/governance/cookie-autopsy-modal.tsx]

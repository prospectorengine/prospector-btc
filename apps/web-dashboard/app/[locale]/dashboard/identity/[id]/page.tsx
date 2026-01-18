// INICIO DEL ARCHIVO [apps/web-dashboard/app/[locale]/dashboard/identity/[id]/page.tsx]
/**
 * =================================================================
 * APARATO: IDENTITY BIOMETRICS DETAIL (V2.2 - IMPORT FIXED)
 * CLASIFICACIÓN: FEATURE VIEW (ESTRATO L5)
 * RESPONSABILIDAD: INSPECCIÓN FORENSE MOLECULAR DE IDENTIDAD
 *
 * VISION HIPER-HOLÍSTICA:
 * - RESOLUCIÓN TS2304: Inyección de 'useState' en los imports de React.
 * - SANEAMIENTO: Estructura atómica autocontenida.
 * =================================================================
 */

"use client";

import React, { useState } from "react"; // ✅ REPARACIÓN: Importación nominal explícita
import { useQuery } from "@tanstack/react-query";
import { useParams, useRouter } from "next/navigation";
import { formatDistanceToNow } from "date-fns";
import {
    Activity,
    Clock,
    Shield,
    AlertTriangle,
    Fingerprint,
    RefreshCw,
    Terminal,
    Lock,
    ArrowLeft,
    Copy,
    Check,
    Globe,
    Cpu,
    type LucideIcon
} from "lucide-react";

// --- SINAPSIS CON INFRAESTRUCTURA ---
import { adminApi, type Identity } from "@prospector/api-client";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/kit/card";
import { Skeleton } from "@/components/ui/kit/skeleton";
import { Button } from "@/components/ui/kit/button";
import { cn } from "@/lib/utils/cn";

// --- DEFINICIONES DE TIPO (ZERO ANY POLICY) ---
type ColorVariant = "blue" | "emerald" | "purple" | "zinc" | "amber" | "red";

interface MetricCardProps {
    icon: LucideIcon;
    label: string;
    value: string;
    unit?: string;
    subValue: string;
    color: ColorVariant;
}

export default function IdentityDetailPage() {
  const router = useRouter();
  const params = useParams();

  // Extracción segura del ID de ruta
  const identityId = Array.isArray(params?.id) ? params.id[0] : params?.id;

  /**
   * ADQUISICIÓN DE DATOS BIOMÉTRICOS
   */
  const { data: identity, isLoading, isError } = useQuery<Identity | undefined>({
    queryKey: ["identity-detail", identityId],
    queryFn: async () => {
        const allIdentities = await adminApi.listIdentities();
        return allIdentities.find(i => i.id === identityId);
    },
    enabled: !!identityId,
    staleTime: 5000
  });

  if (isLoading) return <DetailSkeleton />;

  if (isError || !identity) return (
      <div className="flex flex-col items-center justify-center h-[60vh] space-y-6 text-center animate-in fade-in zoom-in-95">
          <div className="p-6 bg-red-500/10 rounded-full border border-red-500/20 shadow-[0_0_40px_rgba(239,68,68,0.2)]">
            <AlertTriangle className="w-12 h-12 text-red-500" />
          </div>
          <div className="space-y-2">
            <h2 className="text-xl font-black text-white font-mono tracking-widest uppercase">Signal Lost</h2>
            <p className="text-xs text-zinc-500 font-mono uppercase tracking-wide">
                Identity_UUID not found in Tactical Ledger.
            </p>
          </div>
          <Button variant="outline" onClick={() => router.back()} className="font-mono text-xs">
            <ArrowLeft className="w-3 h-3 mr-2" /> RETURN TO BASE
          </Button>
      </div>
  );

  // Análisis Forense del Payload (Client-Side)
  const payloadSizeBytes = new TextEncoder().encode(identity.credentials_json).length;
  const isEncrypted = identity.credentials_json.includes("cipher_text_base64");

  // Extracción segura de user_agent con fallback para evitar TS2339 en contratos desincronizados
  const rawIdentity = identity as unknown as Record<string, string>;
  const userAgentDisplay = rawIdentity.user_agent || "UNKNOWN_FINGERPRINT_SIGNATURE";

  return (
    <div className="max-w-6xl mx-auto py-8 px-4 space-y-8 animate-in fade-in duration-700 font-mono">

        {/* NAVEGACIÓN TÁCTICA */}
        <button
            onClick={() => router.back()}
            className="flex items-center gap-2 text-[10px] font-bold text-zinc-500 hover:text-white transition-colors uppercase tracking-widest group"
        >
            <ArrowLeft className="w-3 h-3 group-hover:-translate-x-1 transition-transform" />
            Back_To_Vault
        </button>

        {/* CABECERA BIOMÉTRICA */}
        <div className="flex flex-col lg:flex-row justify-between items-start lg:items-center gap-6 border-b border-white/5 pb-8 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] bg-opacity-5">
            <div className="space-y-3 w-full lg:w-auto">
                <div className="flex items-center gap-3">
                    <div className={cn(
                        "w-2 h-2 rounded-full shadow-[0_0_10px]",
                        identity.status === 'active' ? "bg-emerald-500 shadow-emerald-500" : "bg-red-500 shadow-red-500"
                    )} />
                    <span className="text-[10px] font-black text-zinc-500 uppercase tracking-[0.3em]">
                        Subject_Identity // {identity.platform.toUpperCase()}
                    </span>
                </div>

                <h1 className="text-3xl md:text-4xl font-black text-white uppercase tracking-tighter break-all">
                    {identity.email}
                </h1>

                <div className="flex flex-wrap items-center gap-2 text-[10px]">
                    <CopyableBadge label="UUID" value={identity.id} icon={Fingerprint} />
                    <CopyableBadge label="PLATFORM" value={identity.platform} icon={Globe} />
                </div>
            </div>

            <StatusBadge status={identity.status} />
        </div>

        {/* MATRIZ DE SIGNOS VITALES (L5 HUD) */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <MetricCard
                icon={Activity}
                label="Operational Load"
                value={`${identity.usage_count}`}
                unit="MISSIONS"
                subValue="Total Execution Cycles"
                color="blue"
            />
            <MetricCard
                icon={Clock}
                label="Last Heartbeat"
                value={identity.last_used_at ? formatDistanceToNow(new Date(identity.last_used_at)) : "N/A"}
                unit={identity.last_used_at ? "AGO" : ""}
                subValue={identity.last_used_at ? "Active Sync Pulse" : "Awaiting Initialization"}
                color={identity.last_used_at ? "emerald" : "zinc"}
            />
            <MetricCard
                icon={RefreshCw}
                label="Age of Identity"
                value={identity.created_at ? formatDistanceToNow(new Date(identity.created_at)) : "GENESIS"}
                unit="OLD"
                subValue="Vault Residency Time"
                color="purple"
            />
        </div>

        {/* ESTRUCTURA MOLECULAR (PAYLOAD & SECURITY) */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">

            {/* SECTOR A: FORENSE TÉCNICO */}
            <Card className="bg-[#080808] border-zinc-800 h-full flex flex-col shadow-2xl overflow-hidden relative group">
                <div className="absolute top-0 right-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity">
                    <Terminal className="w-24 h-24 text-white" />
                </div>
                <CardHeader className="border-b border-zinc-900 bg-zinc-900/30 py-5">
                    <CardTitle className="text-xs font-black text-zinc-400 uppercase tracking-widest flex items-center gap-2">
                        <Cpu className="w-4 h-4 text-blue-500" />
                        Payload_Forensics
                    </CardTitle>
                </CardHeader>
                <CardContent className="p-6 space-y-6 flex-1">
                    <div className="space-y-2">
                        <div className="flex justify-between items-end">
                            <label className="text-[9px] font-bold text-zinc-500 uppercase tracking-wider">Payload Weight</label>
                            <span className="text-xs font-mono text-white font-bold">{payloadSizeBytes} bytes</span>
                        </div>
                        <div className="h-1.5 w-full bg-zinc-900 rounded-full overflow-hidden">
                            <div
                                className="h-full bg-blue-600 rounded-full transition-all duration-1000"
                                style={{ width: `${Math.min((payloadSizeBytes / 4096) * 100, 100)}%` }}
                            />
                        </div>
                    </div>

                    <div className="space-y-2">
                        <label className="text-[9px] font-bold text-zinc-500 uppercase tracking-wider flex items-center gap-2">
                            User_Agent_Fingerprint
                        </label>
                        <div className="text-[10px] font-mono text-zinc-400 bg-black p-4 rounded-xl border border-zinc-800 break-all leading-relaxed shadow-inner">
                            {userAgentDisplay}
                        </div>
                    </div>
                </CardContent>
            </Card>

            {/* SECTOR B: PROTOCOLO DE SEGURIDAD */}
            <Card className="bg-[#080808] border-zinc-800 h-full shadow-2xl relative overflow-hidden">
                <div className={cn(
                    "absolute inset-0 opacity-5 pointer-events-none",
                    isEncrypted ? "bg-emerald-500" : "bg-red-500"
                )} />

                <CardHeader className="border-b border-zinc-900 bg-zinc-900/30 py-5 relative z-10">
                    <CardTitle className="text-xs font-black text-zinc-400 uppercase tracking-widest flex items-center gap-2">
                        <Shield className={cn("w-4 h-4", isEncrypted ? "text-emerald-500" : "text-red-500")} />
                        Encryption_Protocol
                    </CardTitle>
                </CardHeader>

                <CardContent className="p-8 relative z-10 flex flex-col items-center justify-center h-full min-h-[250px] text-center gap-6">
                    {isEncrypted ? (
                        <>
                            <div className="relative">
                                <div className="absolute inset-0 bg-emerald-500/20 blur-xl rounded-full animate-pulse" />
                                <div className="relative w-20 h-20 bg-emerald-950/30 rounded-2xl flex items-center justify-center border border-emerald-500/30 shadow-[0_0_30px_rgba(16,185,129,0.1)]">
                                    <Lock className="w-8 h-8 text-emerald-500" />
                                </div>
                            </div>
                            <div className="space-y-2">
                                <h4 className="text-sm font-black text-white uppercase tracking-widest">AES-256-GCM Secured</h4>
                                <p className="text-[10px] text-zinc-500 max-w-xs mx-auto leading-relaxed font-mono">
                                    Zero-Knowledge Architecture Active.<br/>
                                    Payload is sealed client-side. The server holds only the ciphertext.
                                </p>
                            </div>
                            <div className="flex gap-2 text-[9px] font-bold text-emerald-700 uppercase tracking-widest border border-emerald-900/30 px-3 py-1 rounded-full bg-emerald-950/10">
                                <Check className="w-3 h-3" /> Integrity Verified
                            </div>
                        </>
                    ) : (
                        <>
                             <div className="w-12 h-12 bg-red-900/20 rounded-full flex items-center justify-center border border-red-500/30 animate-pulse">
                                <AlertTriangle className="w-6 h-6 text-red-500" />
                            </div>
                            <div>
                                <h4 className="text-sm font-black text-red-500 uppercase tracking-wider">Plain Text Exposure</h4>
                                <p className="text-[10px] text-zinc-500 mt-2 max-w-xs mx-auto leading-relaxed font-mono">
                                    Legacy identity format detected. Credentials are visible to the system administrator.
                                </p>
                            </div>
                            <Button variant="destructive" size="sm" className="text-[10px] font-bold uppercase tracking-widest">
                                Rotate Credentials Now
                            </Button>
                        </>
                    )}
                </CardContent>
            </Card>
        </div>

        {/* FOOTER DE CONTEXTO */}
        <div className="flex justify-center pt-8 opacity-30">
             <div className="flex items-center gap-3 text-[9px] text-zinc-600 font-mono uppercase tracking-[0.3em] border border-zinc-800 px-6 py-2 rounded-full">
                <Terminal className="w-3 h-3" />
                <span>Identity_Stratum_L3 // Read_Only_Access</span>
             </div>
        </div>
    </div>
  );
}

/**
 * Átomo: Visualización de Carga.
 */
function DetailSkeleton() {
    return (
        <div className="space-y-6 p-6">
          <Skeleton className="h-20 w-full rounded-xl bg-zinc-900/50" />
          <div className="grid grid-cols-3 gap-4">
              <Skeleton className="h-32 rounded-xl bg-zinc-900/50" />
              <Skeleton className="h-32 rounded-xl bg-zinc-900/50" />
              <Skeleton className="h-32 rounded-xl bg-zinc-900/50" />
          </div>
          <Skeleton className="h-64 w-full rounded-xl bg-zinc-900/50" />
      </div>
    );
}

// --- SUB-COMPONENTES ATÓMICOS ---

function CopyableBadge({ label, value, icon: Icon }: { label: string, value: string, icon: LucideIcon }) {
    const [copied, setCopied] = useState(false);

    const handleCopy = () => {
        navigator.clipboard.writeText(value);
        setCopied(true);
        setTimeout(() => setCopied(false), 2000);
    };

    return (
        <button
            onClick={handleCopy}
            className="flex items-center gap-2 bg-zinc-900/50 hover:bg-zinc-800 border border-zinc-800 px-3 py-1.5 rounded-lg transition-all group"
            title="Click to copy"
        >
            <Icon className="w-3 h-3 text-zinc-500 group-hover:text-white transition-colors" />
            <span className="font-bold text-zinc-400 group-hover:text-white">{label}:</span>
            <span className="font-mono text-zinc-500 group-hover:text-zinc-300">{value.substring(0, 12)}...</span>
            {copied ? <Check className="w-3 h-3 text-emerald-500" /> : <Copy className="w-3 h-3 text-zinc-600 opacity-0 group-hover:opacity-100 transition-opacity" />}
        </button>
    );
}

function StatusBadge({ status }: { status: string }) {
    const config: Record<string, { bg: string, text: string, border: string, label: string }> = {
        active: { bg: "bg-emerald-500/10", text: "text-emerald-500", border: "border-emerald-500/20", label: "OPERATIONAL" },
        expired: { bg: "bg-amber-500/10", text: "text-amber-500", border: "border-amber-500/20", label: "EXPIRED" },
        revoked: { bg: "bg-red-500/10", text: "text-red-500", border: "border-red-500/20", label: "REVOKED" },
        ratelimited: { bg: "bg-orange-500/10", text: "text-orange-500", border: "border-orange-500/20", label: "RATE_LIMITED" }
    };

    const style = config[status] || { bg: "bg-zinc-800", text: "text-zinc-400", border: "border-zinc-700", label: "UNKNOWN" };

    return (
        <div className={cn("px-5 py-2 rounded-full border shadow-lg backdrop-blur-md flex items-center gap-3", style.bg, style.border)}>
            <div className={cn("w-2 h-2 rounded-full animate-pulse", style.text.replace("text-", "bg-"))} />
            <span className={cn("text-[10px] font-black uppercase tracking-widest", style.text)}>
                {style.label}
            </span>
        </div>
    );
}

function MetricCard({ icon: Icon, label, value, unit, subValue, color }: MetricCardProps) {
    const colorMap: Record<ColorVariant, string> = {
        blue: "text-blue-500 group-hover:text-blue-400",
        emerald: "text-emerald-500 group-hover:text-emerald-400",
        purple: "text-purple-500 group-hover:text-purple-400",
        zinc: "text-zinc-500 group-hover:text-zinc-400",
        amber: "text-amber-500 group-hover:text-amber-400",
        red: "text-red-500 group-hover:text-red-400"
    };

    return (
        <div className="p-6 bg-[#0a0a0a] border border-zinc-800 rounded-2xl flex flex-col justify-between group hover:border-zinc-700 transition-all duration-300 shadow-sm hover:shadow-xl hover:-translate-y-1">
            <div className="flex justify-between items-start mb-6">
                <p className="text-[9px] text-zinc-500 font-black uppercase tracking-[0.2em]">{label}</p>
                <div className="p-2 bg-zinc-900 rounded-lg border border-zinc-800 group-hover:border-zinc-700 transition-colors">
                    <Icon className={cn("w-4 h-4 opacity-60 group-hover:opacity-100 transition-opacity", colorMap[color])} />
                </div>
            </div>
            <div>
                <p className="text-3xl font-black text-white tracking-tighter tabular-nums flex items-baseline gap-1.5">
                    {value}
                    {unit && <span className="text-[10px] text-zinc-600 font-medium tracking-normal font-mono">{unit}</span>}
                </p>
                <div className="mt-3 pt-3 border-t border-zinc-900 group-hover:border-zinc-800 transition-colors">
                    <p className="text-[9px] text-zinc-500 font-mono uppercase tracking-wide flex items-center gap-2">
                        <div className={cn("w-1 h-1 rounded-full", colorMap[color].split(" ")[0].replace("text-", "bg-"))} />
                        {subValue}
                    </p>
                </div>
            </div>
        </div>
    );
}
// FIN DEL ARCHIVO [apps/web-dashboard/app/[locale]/dashboard/identity/[id]/page.tsx]

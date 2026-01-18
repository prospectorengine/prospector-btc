/**
 * =================================================================
 * APARATO: IDENTITY GOVERNANCE GRID (V16.2 - HARDENED)
 * CLASIFICACIÓN: FEATURE UI ORGANISM (ESTRATO L5)
 * RESPONSABILIDAD: TABLERO DE MANDO PARA GESTIÓN DE CREDENCIALES
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la interfaz de control supremo. Sincronizado con el
 * contrato de estado del Hook V2.2 para bloqueo global de procesos.
 * =================================================================
 */

"use client";

import { useState } from "react";
import { isAfter } from "date-fns";
import {
  Users, Trash2, Unlock,
  Terminal, Activity, Copy, AlertOctagon,
  CheckCircle2, Fingerprint, Clock, ShieldAlert,
  Zap, ShieldX, type LucideIcon
} from "lucide-react";
import { motion, AnimatePresence } from "framer-motion";
import { toast } from "sonner";
import { useIdentityGovernance } from "@/hooks/use-identity-governance";
import { type Identity } from "@prospector/api-client";
import { CookieAutopsyModal } from "./cookie-autopsy-modal";

import { Card, CardHeader } from "@/components/ui/kit/card";
import { Button } from "@/components/ui/kit/button";
import { Skeleton } from "@/components/ui/kit/skeleton";
import { cn } from "@/lib/utils/cn";

/**
 * Interface para botones de acción táctica.
 */
interface ActionButtonProps {
  icon: LucideIcon;
  title: string;
  onClick: () => void;
  color: "blue" | "amber" | "red" | "purple";
  disabled?: boolean;
}

export function IdentityGovernanceGrid() {
  const { identities, isLoading, isError, actions, state } = useIdentityGovernance();
  const [selectedIdentity, setSelectedIdentity] = useState<Identity | null>(null);

  /**
   * Genera un reporte forense en JSON para análisis externo.
   */
  const handleGenerateAIReport = () => {
    if (!identities) return;
    const report = {
      timestamp: new Date().toISOString(),
      stratum: "L5_GOVERNANCE_AUDIT",
      total_units: identities.length,
      data: identities.map(i => ({
        id: i.email,
        load: i.usage_count,
        status: i.status.toUpperCase()
      }))
    };
    navigator.clipboard.writeText(JSON.stringify(report, null, 2));
    toast.success("STRATEGIC_REPORT_COPIED", {
        description: "Audit data crystallized for neural analysis.",
        icon: <Terminal className="w-4 h-4 text-emerald-500" />
    });
  };

  if (isLoading) return <Skeleton className="h-96 w-full rounded-3xl bg-zinc-900/20" />;

  if (isError) return (
    <div className="flex flex-col items-center justify-center p-20 border border-red-900/30 bg-red-950/5 rounded-3xl">
        <AlertOctagon className="w-16 h-16 text-red-500 mb-6 animate-pulse" />
        <h3 className="text-xl font-black text-white uppercase tracking-tighter font-mono">Uplink_Sync_Failure</h3>
        <p className="text-red-400/60 text-xs font-mono mt-2 uppercase tracking-widest">Unable to synchronize governance strata</p>
    </div>
  );

  return (
    <div className="space-y-8 animate-in fade-in duration-1000 font-mono">

      {/* 1. CABECERA Y CONTROL */}
      <div className="flex flex-col md:flex-row justify-between items-start md:items-end gap-6">
        <div className="space-y-2 border-l-2 border-emerald-500 pl-5">
            <h2 className="text-3xl font-black text-white uppercase tracking-tighter italic leading-none">
                Governance_Matrix
            </h2>
            <div className="flex items-center gap-3">
                <div className="h-1.5 w-1.5 rounded-full bg-emerald-500 animate-pulse shadow-[0_0_8px_#10b981]" />
                <p className="text-[10px] text-zinc-500 uppercase tracking-[0.3em]">Identity_Strata_L3 // Total_Units: {identities?.length}</p>
            </div>
        </div>

        <Button
            variant="outline"
            onClick={handleGenerateAIReport}
            className="h-12 text-[10px] font-black uppercase tracking-widest border-zinc-800 hover:bg-emerald-500/10 hover:border-emerald-500/30 text-zinc-400 hover:text-emerald-400 gap-3 transition-all duration-500"
        >
            <Copy className="w-4 h-4" /> Export_Forensic_Snapshot
        </Button>
      </div>

      {/* 2. REJILLA DE DATOS TÁCTICOS */}
      <Card className="bg-black/40 border-zinc-800 shadow-2xl rounded-3xl overflow-hidden backdrop-blur-xl relative">
        <div className="absolute inset-0 bg-linear-to-b from-white/[0.02] to-transparent pointer-events-none" />

        <CardHeader className="border-b border-white/5 bg-white/2 py-5 px-8">
            <div className="flex items-center gap-4">
                <Users className="w-5 h-5 text-blue-500" />
                <span className="text-[11px] font-black text-zinc-400 uppercase tracking-[0.4em]">Active_Operator_Registry</span>
            </div>
        </CardHeader>

        <div className="overflow-x-auto custom-scrollbar">
            <table className="w-full text-left border-collapse">
                <thead className="bg-zinc-950/50 text-[9px] text-zinc-600 uppercase font-black tracking-[0.2em] border-b border-zinc-800">
                    <tr>
                        <th className="p-6">Identity_Fingerprint</th>
                        <th className="p-6">Lease_Status</th>
                        <th className="p-6 text-center">Health_Pulse</th>
                        <th className="p-6">Load_Metrics</th>
                        <th className="p-6 text-right">Mando_Actions</th>
                    </tr>
                </thead>
                <tbody className="divide-y divide-white/5">
                    <AnimatePresence mode="popLayout">
                        {identities?.map((identity) => (
                            <motion.tr
                                key={identity.id}
                                layout
                                initial={{ opacity: 0 }}
                                animate={{ opacity: 1 }}
                                className="group hover:bg-white/[0.03] transition-all duration-300"
                            >
                                <td className="p-6">
                                    <div className="flex items-start gap-4">
                                        <div className="p-2 bg-zinc-900 rounded-lg border border-white/5 group-hover:border-emerald-500/30 transition-colors">
                                            <Fingerprint className="w-4 h-4 text-zinc-500" />
                                        </div>
                                        <div className="flex flex-col gap-1">
                                            <span className={cn(
                                                "text-sm font-bold tracking-tight transition-all",
                                                identity.status === "revoked" ? "text-red-500/50 line-through" : "text-zinc-100"
                                            )}>{identity.email}</span>
                                            <span className="text-[9px] text-zinc-500 uppercase font-black tracking-widest">{identity.platform}</span>
                                        </div>
                                    </div>
                                </td>

                                <td className="p-6">
                                    <LeaseTimer identity={identity} />
                                </td>

                                <td className="p-6 text-center">
                                    <IdentityStatusBadge status={identity.status} />
                                </td>

                                <td className="p-6">
                                    <div className="flex items-center gap-3 bg-zinc-900/30 w-fit px-3 py-1.5 rounded-lg border border-white/5">
                                        <Zap className="w-3 h-3 text-amber-500" />
                                        <span className="text-xs font-black text-zinc-300 tabular-nums">{identity.usage_count}</span>
                                    </div>
                                </td>

                                <td className="p-6">
                                    <div className="flex justify-end gap-2.5 opacity-30 group-hover:opacity-100 transition-opacity duration-500">
                                        <ActionButton
                                            icon={Terminal}
                                            title="Forensic Autopsy"
                                            onClick={() => setSelectedIdentity(identity)}
                                            color="blue"
                                            disabled={state.isProcessing}
                                        />
                                        <ActionButton
                                            icon={Unlock}
                                            title="Force Release Lease"
                                            onClick={() => actions.forceRelease(identity.email)}
                                            color="amber"
                                            disabled={!identity.leased_until || state.isProcessing}
                                        />
                                        <ActionButton
                                            icon={ShieldX}
                                            title="Flag as Compromised"
                                            onClick={() => actions.reportMalfunction(identity.email, "revoked")}
                                            color="purple"
                                            disabled={state.isProcessing}
                                        />
                                        <ActionButton
                                            icon={Trash2}
                                            title="Incinerate Record"
                                            onClick={() => actions.purge(identity.email)}
                                            color="red"
                                            disabled={state.isProcessing}
                                        />
                                    </div>
                                </td>
                            </motion.tr>
                        ))}
                    </AnimatePresence>
                </tbody>
            </table>
        </div>
      </Card>

      <footer className="flex justify-center pt-8 opacity-20">
          <div className="flex items-center gap-4 px-6 py-2 border border-white/10 rounded-full">
              <Activity className="w-3 h-3 text-emerald-500 animate-pulse" />
              <span className="text-[8px] font-black uppercase tracking-[0.5em] text-zinc-500">
                Sovereign_Identity_Stratum // ISO-27001_Agnostic
              </span>
          </div>
      </footer>

      <CookieAutopsyModal
        identity={selectedIdentity}
        isOpen={!!selectedIdentity}
        onClose={() => setSelectedIdentity(null)}
        onAuditRequest={actions.auditIdentity}
      />
    </div>
  );
}

/**
 * Átomo: Badge de estado tipado.
 */
function IdentityStatusBadge({ status }: { status: string }) {
    const config: Record<string, { color: string, label: string, icon: LucideIcon }> = {
        active: { color: "text-emerald-500 border-emerald-500/20 bg-emerald-500/5", label: "NOMINAL", icon: CheckCircle2 },
        revoked: { color: "text-red-500 border-red-500/20 bg-red-500/5", label: "REVOKED", icon: ShieldAlert },
        expired: { color: "text-zinc-500 border-zinc-800 bg-zinc-900/20", label: "EXPIRED", icon: Clock },
        ratelimited: { color: "text-amber-500 border-amber-500/20 bg-amber-500/5", label: "COOLDOWN", icon: Activity },
    };

    const { color, label, icon: Icon } = config[status] || config.expired;

    return (
        <div className={cn("inline-flex items-center gap-2 px-3 py-1 rounded-md border text-[9px] font-black uppercase tracking-widest", color)}>
            <Icon className="w-3 h-3" />
            {label}
        </div>
    );
}

/**
 * Átomo: Temporizador de arrendamiento.
 */
function LeaseTimer({ identity }: { identity: Identity }) {
    const isLeased = identity.leased_until && isAfter(new Date(identity.leased_until), new Date());

    if (!isLeased) {
        return (
            <div className="flex items-center gap-2 text-zinc-700">
                <Unlock className="w-3 h-3 opacity-30" />
                <span className="text-[10px] font-bold italic tracking-tighter">Available_For_Lease</span>
            </div>
        );
    }

    return (
        <div className="flex items-center gap-3">
            <div className="relative h-6 w-6 flex items-center justify-center">
                <div className="absolute inset-0 bg-amber-500/10 rounded-full animate-ping" />
                <Clock className="w-3.5 h-3.5 text-amber-500 relative z-10" />
            </div>
            <div className="flex flex-col">
                <span className="text-[9px] text-amber-500/70 font-black uppercase tracking-widest">Locked_Strata</span>
                <span className="text-[11px] text-white font-bold tabular-nums">
                    until {new Date(identity.leased_until!).toLocaleTimeString()}
                </span>
            </div>
        </div>
    );
}

/**
 * Átomo: Botón de acción con seguridad de tipos.
 */
function ActionButton({ icon: Icon, title, onClick, color, disabled = false }: ActionButtonProps) {
    const colorMap = {
        blue: "text-blue-400 hover:bg-blue-500/10 hover:text-blue-300",
        amber: "text-amber-500 hover:bg-amber-500/10 hover:text-amber-400",
        red: "text-red-500 hover:bg-red-500/20 hover:text-red-400",
        purple: "text-purple-400 hover:bg-purple-500/10 hover:text-purple-300"
    };

    return (
        <Button
            size="icon"
            variant="ghost"
            onClick={onClick}
            disabled={disabled}
            title={title}
            className={cn("h-9 w-9 border border-transparent transition-all", colorMap[color])}
        >
            <Icon className="w-4 h-4" />
        </Button>
    );
}

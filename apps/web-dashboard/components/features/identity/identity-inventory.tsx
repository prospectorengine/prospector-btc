// INICIO DEL ARCHIVO [apps/web-dashboard/components/features/identity/identity-inventory.tsx]
/**
 * =================================================================
 * APARATO: IDENTITY INVENTORY HUD (V23.0 - ACTIONABLE STATUS)
 * CLASIFICACIÓN: FEATURE UI ORGANISM (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN Y GESTIÓN DE CICLO DE VIDA
 *
 * VISION HIPER-HOLÍSTICA:
 * Muestra el estado real auditado. Permite la acción rápida de
 * "ROTACIÓN" que pre-carga el inyector para sobrescribir credenciales.
 * =================================================================
 */

"use client";

import React from "react";
import { useQuery } from "@tanstack/react-query";
import { formatDistanceToNow } from "date-fns";
import {
  Server,
  Clock,
  RefreshCw,
  Activity,
  UserCheck,
  AlertTriangle,
  Ban,
  RotateCcw,
  ShieldCheck
} from "lucide-react";
import { adminApi, type Identity } from "@prospector/api-client";
import { Skeleton } from "@/components/ui/kit/skeleton";
import { Button } from "@/components/ui/kit/button";
import { cn } from "@/lib/utils/cn";

// Contrato de evento para comunicar con el inyector hermano
export const IDENTITY_ROTATION_EVENT = "identity_rotation_request";

export function IdentityInventory(): React.ReactElement {
  const {
    data: identity_pool,
    isLoading,
    isError
  } = useQuery<Identity[]>({
    queryKey: ["identities-vault-inventory-v23"],
    queryFn: () => adminApi.listIdentities(),
    refetchInterval: 5000, // Polling rápido para reflejar cambios del script auditor
  });

  const trigger_rotation = (email: string) => {
    // Dispara evento al componente hermano (Injector) para purga por sobrescritura
    const event = new CustomEvent(IDENTITY_ROTATION_EVENT, { detail: { email } });
    window.dispatchEvent(event);
  };

  const getStatusConfig = (status: string) => {
    switch (status) {
      case "active":
        return { color: "text-emerald-500", bg: "bg-emerald-500", border: "border-emerald-500/30", icon: UserCheck, label: "OPERATIONAL" };
      case "expired":
        return { color: "text-amber-500", bg: "bg-amber-500", border: "border-amber-500/30", icon: Clock, label: "EXPIRED_COOKIES" };
      case "revoked":
      case "ratelimited":
        return { color: "text-red-500", bg: "bg-red-500", border: "border-red-500/30", icon: Ban, label: "REVOKED/BAN" };
      default:
        return { color: "text-zinc-500", bg: "bg-zinc-500", border: "border-zinc-500/30", icon: AlertTriangle, label: "UNKNOWN" };
    }
  };

  if (isLoading) {
    return (
      <div className="space-y-4">
        {[...Array(3)].map((_, i) => (
          <Skeleton key={i} className="h-32 w-full rounded-2xl bg-zinc-900/50 border border-white/5" />
        ))}
      </div>
    );
  }

  if (isError) {
    return (
      <div className="p-10 border border-red-900/30 bg-red-950/5 rounded-2xl text-center font-mono">
        <AlertTriangle className="w-8 h-8 text-red-500 mx-auto mb-4" />
        <p className="text-[10px] text-red-400 font-black uppercase tracking-widest">
          Vault_Uplink_Severed // Engine_A_Unresponsive
        </p>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-[#0a0a0a] border border-zinc-800 rounded-2xl overflow-hidden shadow-2xl group font-mono">
      <header className="p-5 border-b border-zinc-800 bg-white/2 flex justify-between items-center backdrop-blur-md">
        <div className="flex items-center gap-3">
          <Server className="w-4 h-4 text-emerald-500" />
          <h3 className="text-[10px] font-black text-zinc-400 uppercase tracking-[0.3em]">
            Vault_Registry // Audit_Live
          </h3>
        </div>
        <span className="text-[10px] font-bold bg-black text-emerald-500 px-3 py-1 rounded-lg border border-emerald-900/30">
          COUNT: {identity_pool?.length || 0}
        </span>
      </header>

      <div className="flex-1 overflow-y-auto max-h-[600px] p-6 space-y-4 custom-scrollbar">
        {identity_pool?.length === 0 ? (
          <div className="h-64 flex flex-col items-center justify-center text-zinc-800 gap-4">
            <Activity className="w-10 h-10 opacity-20" />
            <p className="text-[10px] font-black uppercase tracking-[0.4em] text-center px-10 leading-relaxed">
              No_Identities_Injected // Manual_Action_Required
            </p>
          </div>
        ) : (
          identity_pool?.map((identity) => {
            const config = getStatusConfig(identity.status);

            return (
              <div
                key={identity.id}
                className={cn(
                  "bg-zinc-950 border p-5 rounded-xl transition-all duration-500 group/card relative overflow-hidden",
                  config.border
                )}
              >
                <div className="flex justify-between items-start mb-4">
                  <div className="flex items-center gap-3">
                    <div className={cn("w-2 h-2 rounded-full shadow-[0_0_8px]", config.bg, config.color)} />
                    <div className="flex flex-col">
                         <span className="text-xs text-zinc-100 font-bold truncate max-w-[200px] tracking-tight">
                            {identity.email}
                         </span>
                         <span className={cn("text-[8px] font-black uppercase tracking-widest mt-1", config.color)}>
                            {config.label}
                         </span>
                    </div>
                  </div>

                  {/* BOTÓN DE ACCIÓN: ROTAR/REPARAR */}
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => trigger_rotation(identity.email)}
                    className="h-8 px-3 text-[9px] border border-white/5 bg-white/5 hover:bg-white/10 text-zinc-400 hover:text-white transition-all hover:border-emerald-500/50"
                  >
                    <RotateCcw className="w-3 h-3 mr-2" />
                    ROTATE_KEYS
                  </Button>
                </div>

                <div className="grid grid-cols-2 gap-4 pt-4 border-t border-white/5">
                  <div className="space-y-1">
                    <p className="text-[7px] text-zinc-600 uppercase font-black tracking-widest">Session_Weight</p>
                    <div className="flex items-center gap-2">
                      <RefreshCw className="w-3 h-3 text-zinc-800" />
                      <span className="text-[10px] font-bold text-zinc-400">{identity.usage_count} leases</span>
                    </div>
                  </div>
                  <div className="space-y-1 text-right">
                    <p className="text-[7px] text-zinc-600 uppercase font-black tracking-widest">Last_Sync</p>
                    <div className="flex items-center justify-end gap-2 text-zinc-400">
                      <span className="text-[10px] font-bold">
                        {identity.last_used_at ? formatDistanceToNow(new Date(identity.last_used_at)) : "Genesis"}
                      </span>
                      <Clock className="w-3 h-3 text-zinc-800" />
                    </div>
                  </div>
                </div>
              </div>
            );
          })
        )}
      </div>

      <footer className="p-4 bg-black/60 border-t border-zinc-900 flex justify-center items-center gap-3">
        <ShieldCheck className="w-3 h-3 text-emerald-500" />
        <p className="text-[7px] font-black text-zinc-700 uppercase tracking-[0.4em]">
          Zero-Knowledge Vault // V2.0 Audit
        </p>
      </footer>
    </div>
  );
}
// FIN DEL ARCHIVO [apps/web-dashboard/components/features/identity/identity-inventory.tsx]

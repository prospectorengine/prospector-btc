/**
 * =================================================================
 * APARATO: USER NAVIGATION ZENITH (V2.6 - MOTION RECOVERY)
 * CLASIFICACIÓN: FEATURE UI COMPONENT (ESTRATO L5)
 * RESPONSABILIDAD: MANDO DE IDENTIDAD, PRESTIGIO Y SINCRO ENERGÉTICA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. MOTION ENGINE RECOVERY: Resuelve el error TS2552 mediante la
 *    inyección formal de 'motion' desde el sustrato framer-motion.
 * 2. L7 REACTIVE SYNERGY: Mantiene la sincronía bit-perfecta con el
 *    motor Resilient_Uplink V18.8 para la gestión de energía.
 * 3. ENERGY VIGILANCE: Semáforo dinámico de cuota (Emerald -> Red).
 * 4. HYGIENE: Erradicación de abreviaciones y rastro de desarrollo.
 * =================================================================
 */

"use client";

import React, { useMemo } from "react";
import { Avatar, AvatarFallback, AvatarImage } from "@radix-ui/react-avatar";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@radix-ui/react-dropdown-menu";
import { useTranslations } from "next-intl";
import { signOut } from "next-auth/react";
import { useQuery } from "@tanstack/react-query";
import {
  motion,
} from "framer-motion"; // ✅ REPARACIÓN: Inyección de motor de animación
import {
  LogOut,
  Settings,
  Zap,
  Trophy,
  CreditCard,
  ChevronRight,
  Activity,
  AlertTriangle
} from "lucide-react";

// --- SINAPSIS CON EL NÚCLEO Y TELEMETRÍA (L4) ---
import { nexusApi, billingApi } from "@prospector/api-client";
import { useHeimdall } from "@/hooks/use-heimdall";
import { Link } from "@/lib/schemas/routing";
import { cn } from "@/lib/utils/cn";

interface UserNavProps {
  operator_identity: {
    name?: string | null;
    email?: string | null;
    image?: string | null;
  };
}

export function UserNav({ operator_identity }: UserNavProps): React.ReactElement {
  const translations = useTranslations("Dashboard.user_nav");
  const logger = useHeimdall("L5:UserNav_Sovereign");

  // 1. ADQUISICIÓN DE PRESTIGIO (NEXUS STRATUM)
  const {
    data: operator_rank_artifact,
    isError: has_nexus_fault
  } = useQuery({
    queryKey: ["operator-prestige-v2.6"],
    queryFn: () => nexusApi.getPrestige(),
    staleTime: 60000,
  });

  // 2. ADQUISICIÓN DE CUOTA (BILLING STRATUM)
  const {
    data: energy_quota_artifact,
    isLoading: is_energy_syncing,
    isError: has_billing_fault
  } = useQuery({
    queryKey: ["operator-energy-v2.6"],
    queryFn: () => billingApi.getQuota(),
    refetchInterval: 20000,
  });

  const operator_initials_signature = useMemo(() =>
    operator_identity.name
      ?.split(" ")
      .map((name_part) => name_part[0])
      .join("")
      .toUpperCase()
      .slice(0, 2) || "OP",
  [operator_identity.name]);

  /**
   * Determina el color del estrato energético basado en la saturación.
   */
  const energy_status_color = useMemo(() => {
    const balance = energy_quota_artifact?.remaining_compute_credits_balance || 0;
    if (balance > 50) return "text-emerald-500 shadow-emerald-500/20";
    if (balance > 15) return "text-amber-500 shadow-amber-500/20";
    return "text-red-500 shadow-red-500/20";
  }, [energy_quota_artifact]);

  /**
   * Ejecuta el protocolo de terminación de sesión y limpieza de rastro local.
   */
  const execute_termination_sequence = async (): Promise<void> => {
    logger.info(`Terminating session strata for: ${operator_identity.email}`);
    sessionStorage.removeItem("ADMIN_SESSION_TOKEN");
    await signOut({ callbackUrl: "/" });
  };

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button
          className="relative h-10 w-10 rounded-full focus:outline-none focus:ring-2 focus:ring-emerald-500/50 transition-all hover:scale-105 active:scale-95 group border-2 border-zinc-800 hover:border-emerald-500/50"
          aria-label="Open sovereign command menu"
        >
          <Avatar className="h-full w-full rounded-full overflow-hidden">
            <AvatarImage
              src={operator_identity.image || ""}
              alt={operator_identity.name || "Operator"}
              className="object-cover"
            />
            <AvatarFallback className="flex h-full w-full items-center justify-center bg-zinc-900 text-[10px] font-black text-zinc-500 font-mono">
              {operator_initials_signature}
            </AvatarFallback>
          </Avatar>
          <span className={cn(
            "absolute bottom-0 right-0 h-3 w-3 rounded-full border-2 border-black",
            has_billing_fault ? "bg-red-600 shadow-[0_0_8px_#ef4444]" : "bg-emerald-500 shadow-[0_0_8px_#10b981]"
          )}></span>
        </button>
      </DropdownMenuTrigger>

      <DropdownMenuContent
        className="w-80 bg-[#080808]/95 backdrop-blur-2xl border border-zinc-800 text-zinc-200 p-3 shadow-[0_25px_50px_-12px_rgba(0,0,0,0.8)] rounded-[2rem] animate-in fade-in zoom-in-95 slide-in-from-top-3 z-50 mr-6 font-mono"
        align="end"
        sideOffset={12}
      >
        {/* SECTOR ALFA: IDENTIDAD SOBERANA */}
        <div className="bg-zinc-900/40 rounded-2xl p-5 mb-3 border border-white/5 relative overflow-hidden group/profile">
          <div className="absolute inset-0 bg-linear-to-br from-emerald-500/5 to-transparent opacity-0 group-hover/profile:opacity-100 transition-opacity duration-1000" />
          <DropdownMenuLabel className="font-normal relative z-10 p-0">
            <div className="flex flex-col space-y-1">
              <p className="text-sm font-black text-white uppercase tracking-wider italic">
                {operator_identity.name || "Unknown_Operator"}
              </p>
              <p className="text-[9px] text-zinc-500 truncate lowercase opacity-70">
                {operator_identity.email}
              </p>
            </div>
          </DropdownMenuLabel>
        </div>

        {/* SECTOR BETA: PRESTIGIO (NEXUS HUD) */}
        <div className="px-3 py-4 space-y-3">
            <div className="flex justify-between items-center px-1">
                <div className="flex items-center gap-2.5">
                    <Trophy className={cn("w-4 h-4", has_nexus_fault ? "text-zinc-700" : "text-amber-500")} />
                    <span className="text-[10px] font-black text-zinc-400 uppercase tracking-widest">Prestige_Mastery</span>
                </div>
                <span className="text-xs font-black text-white tabular-nums italic">
                  {has_nexus_fault ? "UPLINK_OFF" : `LVL ${operator_rank_artifact?.level || 1}`}
                </span>
            </div>
            <div className="p-4 bg-black/40 border border-zinc-800 rounded-2xl flex items-center justify-between group/rank transition-colors hover:border-amber-500/20">
                <span className="text-[9px] font-bold text-zinc-500 uppercase tracking-tighter">
                  {operator_rank_artifact?.title || "Novice_Archaeologist"}
                </span>
                <ChevronRight className="w-3 h-3 text-zinc-800 group-hover/rank:text-amber-500 transition-all" />
            </div>
        </div>

        <DropdownMenuSeparator className="h-px bg-white/5 my-2 mx-1" />

        {/* SECTOR GAMMA: ENERGÍA (BILLING HUD) */}
        <div className="px-3 py-4 space-y-4">
            <div className="flex justify-between items-center px-1">
                <div className="flex items-center gap-2.5">
                    <Zap className={cn("w-4 h-4", is_energy_syncing ? "animate-pulse" : "", energy_status_color)} />
                    <span className="text-[10px] font-black text-zinc-400 uppercase tracking-widest">Compute_Energy</span>
                </div>
                <div className="flex items-center gap-2">
                  {has_billing_fault && <AlertTriangle className="w-3 h-3 text-red-500 animate-bounce" />}
                  <span className={cn("text-sm font-black tabular-nums", energy_status_color)}>
                      {energy_quota_artifact?.remaining_compute_credits_balance.toFixed(2) || "0.00"}
                  </span>
                </div>
            </div>
            <div className="h-1.5 w-full bg-zinc-950 rounded-full overflow-hidden border border-white/5 shadow-inner">
                <motion.div
                    initial={{ width: 0 }}
                    animate={{ width: `${Math.min((energy_quota_artifact?.remaining_compute_credits_balance || 0), 100)}%` }}
                    className={cn("h-full transition-all duration-1000 shadow-[0_0_10px_currentColor]", energy_status_color.replace("text-", "bg-"))}
                />
            </div>
        </div>

        <DropdownMenuSeparator className="h-px bg-white/5 my-2 mx-1" />

        {/* SECTOR DELTA: ACCIONES DE MANDO */}
        <DropdownMenuGroup className="space-y-1">
          <Link href="/dashboard/settings" passHref>
             <DropdownMenuItem className="cursor-pointer hover:bg-white/5 text-zinc-400 hover:text-white p-3 rounded-xl flex gap-4 text-xs items-center transition-all outline-none group/item">
                <Settings className="w-4 h-4 group-hover/item:rotate-90 transition-transform duration-700" />
                <span className="font-bold uppercase tracking-widest">{translations("settings")}</span>
             </DropdownMenuItem>
          </Link>
          <Link href="/dashboard/analytics" passHref>
             <DropdownMenuItem className="cursor-pointer hover:bg-white/5 text-zinc-400 hover:text-white p-3 rounded-xl flex gap-4 text-xs items-center transition-all outline-none">
                <CreditCard className="w-4 h-4" />
                <span className="font-bold uppercase tracking-widest">Billing_Center</span>
             </DropdownMenuItem>
          </Link>
        </DropdownMenuGroup>

        <DropdownMenuSeparator className="h-px bg-white/5 my-2 mx-1" />

        <DropdownMenuItem
          className="cursor-pointer bg-red-500/5 hover:bg-red-500/10 text-red-500 hover:text-red-400 p-4 rounded-2xl flex gap-4 text-xs items-center transition-all outline-none group/logout"
          onClick={execute_termination_sequence}
        >
          <LogOut className="w-4 h-4 group-hover/logout:-translate-x-1 transition-transform" />
          <span className="font-black uppercase tracking-[0.2em]">{translations("logout")}</span>
        </DropdownMenuItem>

        {/* SELLO DE INTEGRIDAD SESIÓN */}
        <div className="px-4 py-3 mt-2 flex items-center justify-between text-[7px] text-zinc-800 font-mono uppercase tracking-[0.4em] border-t border-white/5">
          <span className="flex items-center gap-2">
            <Activity className="w-3 h-3 opacity-30" />
            Session_Hardened_Uplink
          </span>
          <span className="font-black opacity-30 italic">L5_ZENITH_V2.6</span>
        </div>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

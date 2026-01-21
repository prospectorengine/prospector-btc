/**
 * =================================================================
 * APARATO: USER NAVIGATION ZENITH (V2.0 - L7 INTEGRATED)
 * CLASIFICACIÓN: FEATURE UI COMPONENT (ESTRATO L5)
 * RESPONSABILIDAD: MANDO DE IDENTIDAD, PRESTIGIO Y CUOTAS
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. L7 SYNERGY: Integra los túneles de Billing y Nexus para mostrar
 *    el rango y la energía del operador en tiempo real.
 * 2. ZENITH UX: Implementa micro-HUDs dentro del Dropdown con
 *    estética de cristal y neón (Emerald/Blue).
 * 3. NOMINAL PURITY: Erradicación total de abreviaciones.
 * 4. HYGIENE: Gestión de estados de carga y fallos de enlace neural.
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
  LogOut,
  Settings,
  ShieldAlert,
  Zap,
  Trophy,
  CreditCard,
  ChevronRight,
  Activity
} from "lucide-react";

// --- SINAPSIS CON EL NÚCLEO Y TELEMETRÍA ---
import { nexusApi, billingApi } from "@prospector/api-client";
import { useHeimdall } from "@/hooks/use-heimdall";
import { Link } from "@/lib/schemas/routing";
import { cn } from "@/lib/utils/cn";

interface UserNavigationProperties {
  user_identity: {
    name?: string | null;
    email?: string | null;
    image?: string | null;
  };
}

export function UserNav({ user_identity }: UserNavigationProperties): React.ReactElement {
  const translations = useTranslations("Dashboard.user_nav");
  const logger = useHeimdall("UserNav:Zenith");

  // 1. ADQUISICIÓN DE PRESTIGIO (NEXUS STRATUM)
  const { data: operator_rank_artifact } = useQuery({
    queryKey: ["operator-prestige-v2"],
    queryFn: () => nexusApi.getPrestige(),
    staleTime: 30000,
  });

  // 2. ADQUISICIÓN DE CUOTA (BILLING STRATUM)
  const { data: energy_quota_artifact } = useQuery({
    queryKey: ["operator-energy-v2"],
    queryFn: () => billingApi.getQuota(),
    refetchInterval: 15000, // Refresco frecuente para telemetría de consumo
  });

  const user_initials_artifact = useMemo(() =>
    user_identity.name
      ?.split(" ")
      .map((name_part) => name_part[0])
      .join("")
      .toUpperCase()
      .slice(0, 2) || "OP",
  [user_identity.name]);

  /**
   * Ejecuta el protocolo de terminación de sesión y limpieza de rastro.
   */
  const execute_termination_sequence = async (): Promise<void> => {
    logger.info(`Initiating session termination for operator: ${user_identity.email}`);
    // Limpieza de tokens de sesión táctica en el navegador
    sessionStorage.removeItem("ADMIN_SESSION_TOKEN");
    await signOut({ callbackUrl: "/" });
  };

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button
          className="relative h-10 w-10 rounded-full focus:outline-none focus:ring-2 focus:ring-emerald-500/50 transition-all hover:scale-105 active:scale-95 group"
          aria-label="Open sovereign menu"
        >
          <Avatar className="h-10 w-10 rounded-full overflow-hidden border-2 border-zinc-800 group-hover:border-emerald-500/50 transition-colors">
            <AvatarImage
              src={user_identity.image || ""}
              alt={user_identity.name || "Operator"}
              className="object-cover h-full w-full"
            />
            <AvatarFallback className="flex h-full w-full items-center justify-center bg-zinc-900 text-xs font-black text-zinc-400 group-hover:text-emerald-500 font-mono transition-colors">
              {user_initials_artifact}
            </AvatarFallback>
          </Avatar>
          <span className="absolute bottom-0 right-0 h-3 w-3 rounded-full bg-emerald-500 border-2 border-black shadow-[0_0_8px_#10b981]"></span>
        </button>
      </DropdownMenuTrigger>

      <DropdownMenuContent
        className="w-80 bg-[#080808]/95 backdrop-blur-2xl border border-zinc-800 text-zinc-200 p-3 shadow-[0_25px_50px_-12px_rgba(0,0,0,0.7)] rounded-[2rem] animate-in fade-in zoom-in-95 slide-in-from-top-3 z-50 mr-6"
        align="end"
        sideOffset={12}
      >
        {/* SECTOR ALFA: PERFIL E IDENTIDAD */}
        <div className="bg-zinc-900/40 rounded-2xl p-4 mb-3 border border-white/5 relative overflow-hidden group/profile">
          <div className="absolute inset-0 bg-linear-to-br from-emerald-500/5 to-transparent opacity-0 group-hover/profile:opacity-100 transition-opacity duration-1000" />
          <DropdownMenuLabel className="font-normal relative z-10">
            <div className="flex flex-col space-y-1">
              <p className="text-sm font-black text-white uppercase tracking-wider italic">
                {user_identity.name}
              </p>
              <p className="text-[10px] text-zinc-500 font-mono truncate">
                {user_identity.email}
              </p>
            </div>
          </DropdownMenuLabel>
        </div>

        {/* SECTOR BETA: NEXUS HUD (PRESTIGIO) */}
        <div className="px-2 py-3 space-y-3">
            <div className="flex justify-between items-center px-1">
                <div className="flex items-center gap-2">
                    <Trophy className="w-3.5 h-3.5 text-amber-500" />
                    <span className="text-[10px] font-black text-zinc-400 uppercase tracking-widest">Prestige_Level</span>
                </div>
                <span className="text-xs font-black text-white tabular-nums italic">
                  LVL {operator_rank_artifact?.level || 1}
                </span>
            </div>
            <div className="p-3 bg-black/40 border border-zinc-800 rounded-xl flex items-center justify-between">
                <span className="text-[9px] font-bold text-zinc-500 uppercase">{operator_rank_artifact?.title || "Novice_Archaeologist"}</span>
                <ChevronRight className="w-3 h-3 text-zinc-700" />
            </div>
        </div>

        <DropdownMenuSeparator className="h-px bg-zinc-800 my-2 mx-1" />

        {/* SECTOR GAMMA: BILLING HUD (ENERGÍA) */}
        <div className="px-2 py-3 space-y-3">
            <div className="flex justify-between items-center px-1">
                <div className="flex items-center gap-2">
                    <Zap className="w-3.5 h-3.5 text-emerald-500" />
                    <span className="text-[10px] font-black text-zinc-400 uppercase tracking-widest">Energy_Credits</span>
                </div>
                <span className="text-xs font-black text-emerald-500 tabular-nums">
                    {energy_quota_artifact?.remaining_compute_credits_balance.toFixed(1) || "0.0"}
                </span>
            </div>
            <div className="h-1.5 w-full bg-zinc-900 rounded-full overflow-hidden border border-white/5">
                <motion.div
                    initial={{ width: 0 }}
                    animate={{ width: `${Math.min((energy_quota_artifact?.remaining_compute_credits_balance || 0), 100)}%` }}
                    className="h-full bg-emerald-500 shadow-[0_0_10px_#10b981]"
                />
            </div>
        </div>

        <DropdownMenuSeparator className="h-px bg-zinc-800 my-2 mx-1" />

        {/* SECTOR DELTA: ACCIONES ESTRATÉGICAS */}
        <DropdownMenuGroup className="space-y-1">
          <Link href="/dashboard/settings" passHref>
             <DropdownMenuItem className="cursor-pointer hover:bg-white/5 text-zinc-400 hover:text-white p-3 rounded-xl flex gap-3 text-xs items-center transition-all outline-none group/item">
                <Settings className="w-4 h-4 group-hover/item:rotate-90 transition-transform duration-500" />
                <span className="font-bold uppercase tracking-widest">{translations("settings")}</span>
             </DropdownMenuItem>
          </Link>
          <Link href="/dashboard/billing" passHref>
             <DropdownMenuItem className="cursor-pointer hover:bg-white/5 text-zinc-400 hover:text-white p-3 rounded-xl flex gap-3 text-xs items-center transition-all outline-none">
                <CreditCard className="w-4 h-4" />
                <span className="font-bold uppercase tracking-widest">Billing_Center</span>
             </DropdownMenuItem>
          </Link>
        </DropdownMenuGroup>

        <DropdownMenuSeparator className="h-px bg-zinc-800 my-2 mx-1" />

        <DropdownMenuItem
          className="cursor-pointer bg-red-500/5 hover:bg-red-500/10 text-red-500 hover:text-red-400 p-3 rounded-xl flex gap-3 text-xs items-center transition-all outline-none group/logout"
          onClick={execute_termination_sequence}
        >
          <LogOut className="w-4 h-4 group-hover/logout:-translate-x-1 transition-transform" />
          <span className="font-black uppercase tracking-[0.2em]">{translations("logout")}</span>
        </DropdownMenuItem>

        <div className="px-4 py-3 mt-2 flex items-center justify-between text-[8px] text-zinc-700 font-mono uppercase tracking-[0.3em] border-t border-white/5">
          <span className="flex items-center gap-2">
            <ShieldAlert className="w-3 h-3 text-emerald-950" />
            Session_Hardened
          </span>
          <div className="flex items-center gap-1">
            <Activity className="w-2.5 h-2.5 text-emerald-900 animate-pulse" />
            V2.0
          </div>
        </div>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

/**
 * =================================================================
 * APARATO: OMNISCIENT SWARM COMMANDER (V16.2 - STRICTLY TYPED)
 * CLASIFICACIÓN: FEATURE COMPONENT (ESTRATO L5)
 * RESPONSABILIDAD: MANDO, VALIDACIÓN SOBERANA Y LIVE CONSOLE
 *
 * VISION HIPER-HOLÍSTICA:
 * 1. API ALIGNMENT: Sincronización con TanStack Query v5 y Hook Form.
 * 2. TYPE SOVEREIGNTY: Erradicación de 'any' en filtros e inyecciones.
 * 3. UI KIT COMPLIANCE: Mapeo exacto de props (hasError, isLoading).
 * 4. ERROR REDUCTION: Resolución de 11 fallos de compilación TS.
 * =================================================================
 */

"use client";

import React, { useState, useRef, useEffect, useMemo } from "react";
import { useForm, type SubmitHandler } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { useMutation, useQuery } from "@tanstack/react-query";
import {
  Rocket, Activity, Terminal, Database,
  Github, Cpu, Zap, ShieldAlert,
  type LucideIcon
} from "lucide-react";
import { motion, AnimatePresence } from "framer-motion";
import { toast } from "sonner";

import {
  SwarmLaunchSchema,
  type SwarmLaunchConfig,
  type DispatchResponse,
  type ProvisioningLog,
  type Identity // ✅ Importado para tipado de filtros
} from "@prospector/api-contracts";
import { controlApi, adminApi, useNeuralLink } from "@prospector/api-client";
import { useHeimdall } from "@/hooks/use-heimdall";

import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/kit/card";
import { Button } from "@/components/ui/kit/button";
import { Input } from "@/components/ui/kit/input";
import { PreFlightModal } from "./pre-flight-modal";
import { cn } from "@/lib/utils/cn";

export function SwarmLauncher(): React.ReactElement {
  const logger = useHeimdall("L5:SwarmLauncher");
  const console_viewport = useRef<HTMLDivElement>(null);

  const [is_pre_flight_visible, set_is_pre_flight_visible] = useState(false);
  const [pending_config, set_pending_config] = useState<SwarmLaunchConfig | null>(null);

  // 1. SINAPSIS NEURAL (Telemetría y Escudo de Baneo)
  const {
    is_neural_link_connected,
    provisioning_logs,
    global_aggregated_metrics,
    ban_shield_status
  } = useNeuralLink();

  // 2. AUDITORÍA DE BÓVEDA (Polling de capacidad real)
  const { data: identities_collection } = useQuery({
    queryKey: ["vault-capacity-audit-v16"], // ✅ CORRECCIÓN: queryKey (CamelCase)
    queryFn: () => adminApi.listIdentities(), // ✅ CORRECCIÓN: queryFn (CamelCase)
    refetchInterval: 30000
  });

  // ✅ CORRECCIÓN: Tipado explícito de 'identity' para evitar implicit any y TS2339
  const active_identity_count = useMemo(() =>
    identities_collection?.filter((identity: Identity) => identity.status === "active").length || 0,
    [identities_collection]
  );

  const { register, handleSubmit, watch, formState: { errors, isValid } } = useForm<SwarmLaunchConfig>({
    resolver: zodResolver(SwarmLaunchSchema),
    mode: "onChange",
    defaultValues: { // ✅ CORRECCIÓN: defaultValues (CamelCase)
      worker_count: 5,
      shard_count: 1,
      ref: "main"
    }
  });

  const requested_workers = watch("worker_count");

  /**
   * MOTOR DE EVALUACIÓN DE RIESGO (DYNAMIC SHIELD)
   */
  const ignition_restriction = useMemo(() => {
    if (!is_neural_link_connected) return { blocked: true, reason: "LINK_SEVERED" };

    if (ban_shield_status && !ban_shield_status.is_ignition_authorized) {
      return {
        blocked: true,
        reason: ban_shield_status.restriction_reason || "SHIELD_ACTIVE"
      };
    }

    const local_limit = ban_shield_status?.safe_node_capacity || (active_identity_count * 3);
    if (requested_workers > local_limit && local_limit > 0) {
      return { blocked: true, reason: "CAPACITY_EXCEEDED" };
    }

    return { blocked: false, reason: null };
  }, [requested_workers, ban_shield_status, is_neural_link_connected, active_identity_count]);

  useEffect(() => {
    if (console_viewport.current) {
      console_viewport.current.scrollTop = console_viewport.current.scrollHeight;
    }
  }, [provisioning_logs]);

  // ✅ CORRECCIÓN: Definición de genéricos para admitir SwarmLaunchConfig en mutate()
  const ignition_mutation = useMutation<DispatchResponse, Error, SwarmLaunchConfig>({
    mutationFn: (config: SwarmLaunchConfig) => controlApi.launchSwarm(config), // ✅ CORRECCIÓN: mutationFn (CamelCase)
    onSuccess: (data: DispatchResponse) => {
      logger.info(`Ignition pulse accepted. Trace: ${data.trace_id}`);
      toast.success("IGNITION_ACCEPTED", {
        description: "Swarm expansion sequence crystallized.",
        icon: <Rocket className="w-4 h-4 text-emerald-500" />
      });
      set_is_pre_flight_visible(false);
    },
    onError: (e: Error) => {
      logger.error("Ignition failed", { message: e.message });
      toast.error("IGNITION_REJECTED", { description: "Uplink to GitHub Forge severed." });
    }
  });

  const handle_form_submit: SubmitHandler<SwarmLaunchConfig> = (data) => {
    set_pending_config(data);
    set_is_pre_flight_visible(true);
  };

  return (
    <div className="space-y-6 font-mono">
      {/* SECTOR 1: SEMÁFOROS DE INTEGRIDAD */}
      <div className="grid grid-cols-2 lg:grid-cols-4 gap-4">
        <HealthStatusPill label="API_UPLINK" status={is_neural_link_connected ? "nominal" : "critical"} icon={Activity} />
        <HealthStatusPill
          label="VAULT_CAPACITY"
          status={active_identity_count > 0 ? "nominal" : "critical"}
          icon={Database}
        />
        <HealthStatusPill label="C2_AUTHORITY" status="nominal" icon={Github} />
        <HealthStatusPill
          label="GRID_SATURATION"
          status={global_aggregated_metrics?.active_nodes_count ? "active" : "standby"}
          icon={Cpu}
        />
      </div>

      <Card className={cn(
        "bg-[#0a0a0a] border-zinc-800 shadow-2xl relative overflow-hidden transition-all duration-700",
        ignition_restriction.blocked && is_neural_link_connected && "border-red-900/30"
      )}>
        <CardHeader className="border-b border-white/5 bg-white/2 p-6">
          <CardTitle className="text-[10px] font-black text-white uppercase tracking-[0.4em] flex items-center gap-3">
            <Zap className={cn("w-4 h-4", !ignition_restriction.blocked ? "text-emerald-500 animate-pulse" : "text-zinc-700")} />
            C2_Swarm_Launcher_V16.2
          </CardTitle>
        </CardHeader>

        <CardContent className="p-8">
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-12">

            {/* LÓGICA DE MANDO */}
            <form onSubmit={handleSubmit(handle_form_submit)} className="space-y-6">
              <div className="space-y-4">
                <div className="flex justify-between items-end">
                    <label className="text-[9px] font-black text-zinc-500 uppercase tracking-widest">Node_Density</label>
                    <span className={cn(
                      "text-[9px] font-bold px-2 py-0.5 rounded border",
                      ignition_restriction.blocked ? "text-red-500 border-red-900/50" : "text-emerald-500 border-emerald-900/50"
                    )}>
                        MAX_ALLOWED: {ban_shield_status?.safe_node_capacity || (active_identity_count * 3)}
                    </span>
                </div>
                <Input
                    type="number"
                    {...register("worker_count", { valueAsNumber: true })} // ✅ CORRECCIÓN: valueAsNumber (CamelCase)
                    className={cn(
                      "bg-black border-zinc-800 text-lg font-black h-14",
                      ignition_restriction.blocked && "border-red-500/50 text-red-500"
                    )}
                    hasError={ignition_restriction.blocked || !!errors.worker_count} // ✅ CORRECCIÓN: hasError (CamelCase)
                />

                <AnimatePresence>
                  {ignition_restriction.blocked && (
                    <motion.div
                      initial={{ opacity: 0, y: -10 }}
                      animate={{ opacity: 1, y: 0 }}
                      exit={{ opacity: 0, y: -10 }}
                      className="p-4 bg-red-950/10 border border-red-900/30 rounded-xl flex items-start gap-3"
                    >
                        <ShieldAlert className="w-4 h-4 text-red-500 mt-0.5" />
                        <div className="space-y-1">
                          <p className="text-[9px] font-black text-red-400 uppercase tracking-widest">Ban_Shield_Active</p>
                          <p className="text-[10px] text-zinc-500 font-bold uppercase italic">{ignition_restriction.reason}</p>
                        </div>
                    </motion.div>
                  )}
                </AnimatePresence>
              </div>

              <Button
                type="submit"
                variant="cyber"
                className="w-full h-16 font-black tracking-[0.5em] text-xs"
                disabled={ignition_restriction.blocked || !isValid || ignition_mutation.isPending}
                isLoading={ignition_mutation.isPending} // ✅ CORRECCIÓN: isLoading e isPending (CamelCase)
              >
                EXECUTE_IGNITION_SEQUENCE
              </Button>
            </form>

            {/* LIVE C2 CONSOLE */}
            <div className="flex flex-col gap-3">
                <div className="flex items-center gap-2 text-[9px] font-black text-zinc-600 uppercase tracking-widest">
                    <Terminal className="w-3 h-3" /> Raw_Provisioning_Stream
                </div>
                <div
                    ref={console_viewport}
                    className="bg-black border border-zinc-900 rounded-2xl p-5 h-72 overflow-y-auto custom-scrollbar font-mono text-[10px] leading-relaxed shadow-inner relative"
                >
                    <div className="absolute top-0 left-0 w-full h-1 bg-linear-to-r from-transparent via-emerald-500/10 to-transparent" />
                    <AnimatePresence mode="popLayout">
                      {provisioning_logs.length === 0 ? (
                          <motion.div
                            initial={{ opacity: 0 }} animate={{ opacity: 1 }}
                            className="h-full flex flex-col items-center justify-center text-zinc-800 italic uppercase tracking-[0.3em] gap-4"
                          >
                              <Activity className="w-8 h-8 opacity-10 animate-pulse" />
                              Awaiting_Dispatch...
                          </motion.div>
                      ) : (
                          provisioning_logs.map((log: ProvisioningLog, i: number) => (
                              <motion.div
                                key={`${log.timestamp}-${i}`}
                                initial={{ opacity: 0, x: -10 }}
                                animate={{ opacity: 1, x: 0 }}
                                className={cn(
                                  "mb-1 border-l-2 pl-3 py-1 transition-all",
                                  log.level === "CRITICAL" ? "border-red-600 text-red-500 bg-red-950/20" :
                                  log.level === "WARN" ? "border-amber-600 text-amber-400" : "border-emerald-500/20 text-emerald-500/80"
                                )}
                              >
                                  <span className="text-zinc-700 mr-2">[{new Date(log.timestamp).toLocaleTimeString()}]</span>
                                  <span className="font-black text-[9px]">U{log.node_index}:</span> {log.message}
                              </motion.div>
                          ))
                      )}
                    </AnimatePresence>
                </div>
            </div>
          </div>
        </CardContent>
      </Card>

      {is_pre_flight_visible && pending_config && (
        <PreFlightModal
          is_modal_open={is_pre_flight_visible}
          on_close_request={() => set_is_pre_flight_visible(false)}
          on_ignition_confirmed={() => ignition_mutation.mutate(pending_config)}
          deployment_configuration={{
            worker_count_per_shard: pending_config.worker_count,
            shard_count: pending_config.shard_count,
          }}
        />
      )}
    </div>
  );
}

/**
 * ÁTOMO: INDICADOR DE SALUD DE RED
 */
function HealthStatusPill({ label, status, icon: Icon }: { label: string, status: string, icon: LucideIcon }) {
    const is_nominal = status === "nominal" || status === "active";
    const is_critical = status === "critical";

    return (
        <div className={cn(
            "p-4 rounded-[1.25rem] border flex items-center justify-between transition-all duration-700 bg-zinc-950/40 backdrop-blur-md",
            is_nominal ? "border-emerald-900/30 text-emerald-500" :
            is_critical ? "border-red-900/40 text-red-500" : "border-zinc-800 text-zinc-600"
        )}>
            <div className="flex items-center gap-3">
                <Icon className={cn("w-4 h-4", is_nominal ? "opacity-100" : "opacity-30")} />
                <span className="text-[10px] font-black uppercase tracking-widest leading-none">{label}</span>
            </div>
            <div className={cn(
              "w-1.5 h-1.5 rounded-full",
              is_nominal ? "bg-emerald-500 shadow-[0_0_8px_#10b981] animate-pulse" :
              is_critical ? "bg-red-500 shadow-[0_0_8px_#ef4444]" : "bg-zinc-800"
            )} />
        </div>
    );
}

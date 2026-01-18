/**
 * =================================================================
 * APARATO: IDENTITY INJECTION ENGINE (V25.2 - ZENITH RECOVERY)
 * CLASIFICACIÓN: FEATURE UI COMPONENT (ESTRATO L5)
 * RESPONSABILIDAD: PURIFICACIÓN, CIFRADO Y SINCRONIZACIÓN DE IDENTIDAD
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. SYMBOL RECOVERY: Re-inyección de 'motion', 'AnimatePresence' y 'Activity'
 *    para restaurar la funcionalidad del Dashboard y el build de Vercel.
 * 2. TYPE SOVEREIGNTY: Eliminación de 'as any' en el mapeo de traducciones
 *    mediante narrowing de tipos para satisfacer @typescript-eslint.
 * 3. ZERO ABBREVIATIONS: Nomenclatura nominal absoluta en todo el componente.
 * 4. HYGIENE: Erradicación total de variables muertas y placeholders.
 *
 * # Logic:
 * Implementa el protocolo de inyección de tres fases:
 * - Fase A: Purificación de ruido (Client-side).
 * - Fase B: Cifrado AES-256-GCM Zero-Knowledge (Client-side).
 * - Fase C: Transmisión de ciphertext al Orquestador L3.
 * =================================================================
 */

"use client";

import React, { useState, useCallback, useEffect } from "react";
import { useForm, type SubmitHandler } from "react-hook-form";
import { useQueryClient } from "@tanstack/react-query";
import { useTranslations } from "next-intl";
import { motion, AnimatePresence } from "framer-motion"; // ✅ REPARADO: Motores de animación reintegrados
import { toast } from "sonner";
import {
  Key,
  ShieldCheck,
  Lock,
  FileJson,
  ShieldAlert,
  Terminal,
  RefreshCw,
  CheckCircle2,
  Trash2,
  AlertTriangle,
  Zap,
  ArrowRight,
  Activity // ✅ REPARADO: Icono de pulso reintegrado
} from "lucide-react";

// --- SINAPSIS CON EL NÚCLEO CRIPTOGRÁFICO Y API ---
import { VaultCryptoEngine, type EncryptedVaultPayload } from "@prospector/crypto-vault";
import { adminApi } from "@prospector/api-client";
import { analyzeIdentity, type PurificationReport } from "@/lib/utils/cookie-cleaner";
import { cn } from "@/lib/utils/cn";

// --- COMPONENTES DEL DESIGN SYSTEM ---
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/kit/card";
import { Button } from "@/components/ui/kit/button";
import { Input } from "@/components/ui/kit/input";

/** Identificador del evento global para disparar la rotación de identidad. */
const IDENTITY_ROTATION_EVENT = "identity_rotation_request";

/**
 * Estados de la máquina de inyección.
 * Permite al operador visualizar la profundidad del proceso actual.
 */
type InjectionStage = "IDLE" | "PURIFYING" | "ENCRYPTING" | "TRANSMITTING";

/** Estructura interna del formulario de inyección. */
interface InjectionSequenceValues {
  operator_email: string;
  security_passphrase: string;
  raw_cookie_material_json: string;
}

/** Interfaz para el triaje de errores de ingesta. */
interface IngestionFault {
  response?: {
    status?: number;
    data?: {
      error?: string;
    };
  };
  message: string;
}

export function IdentityInjector(): React.ReactElement {
  const translations = useTranslations("Dashboard.vault");
  const query_client = useQueryClient();

  // --- ESTRATOS DE ESTADO ---
  const [current_injection_stage, set_current_injection_stage] = useState<InjectionStage>("IDLE");
  const [is_identity_rotation_mode_active, set_is_identity_rotation_mode_active] = useState(false);
  const [purification_forensic_report, set_purification_forensic_report] = useState<PurificationReport | null>(null);

  const {
    register,
    handleSubmit,
    reset,
    setValue,
    setFocus,
    watch

  } = useForm<InjectionSequenceValues>({
    mode: "onChange"
  });

  const current_raw_json_input = watch("raw_cookie_material_json");

  /**
   * MOTOR DE ANÁLISIS EN TIEMPO REAL
   * # Performance: O(N) sobre el tamaño del JSON de entrada.
   * Ejecuta el escrutinio de cookies mientras el usuario escribe.
   */
  useEffect(() => {
    if (!current_raw_json_input || current_raw_json_input.length < 10) {
      set_purification_forensic_report(null);
      return;
    }
    const generated_report = analyzeIdentity(current_raw_json_input);
    set_purification_forensic_report(generated_report);
  }, [current_raw_json_input]);

  /**
   * SENSOR DE PROTOCOLO PHOENIX
   * Escucha solicitudes de rotación emitidas por el inventario L5.
   */
  useEffect(() => {
    const handle_rotation_request = (custom_event: Event) => {
      const event_data = custom_event as CustomEvent<{ email: string }>;
      setValue("operator_email", event_data.detail.email);
      set_is_identity_rotation_mode_active(true);
      setFocus("raw_cookie_material_json");

      toast.info("ROTATION_PROTOCOL_INITIATED", {
          description: `Ready to overwrite credentials for ${event_data.detail.email}.`
      });
    };

    window.addEventListener(IDENTITY_ROTATION_EVENT, handle_rotation_request);
    return () => window.removeEventListener(IDENTITY_ROTATION_EVENT, handle_rotation_request);
  }, [setValue, setFocus]);

  /**
   * EJECUTOR SOBERANO DE INYECCIÓN
   */
  const execute_secure_injection_sequence: SubmitHandler<InjectionSequenceValues> = useCallback(async (form_data) => {
    if (purification_forensic_report?.status === "CRITICAL" || purification_forensic_report?.status === "INVALID") {
      toast.error("INJECTION_BLOCKED", { description: "Identify material fails integrity audit." });
      return;
    }

    try {
      set_current_injection_stage("PURIFYING");
      const cookies_to_secure = purification_forensic_report
        ? purification_forensic_report.cleanCookies
        : analyzeIdentity(form_data.raw_cookie_material_json).cleanCookies;

      if (cookies_to_secure.length === 0) {
        throw new Error("PURIFIER_FAULT: No valid Google session cookies detected.");
      }

      set_current_injection_stage("ENCRYPTING");
      const encrypted_payload: EncryptedVaultPayload = await VaultCryptoEngine.encryptPortable(
        JSON.stringify(cookies_to_secure),
        form_data.security_passphrase,
        form_data.operator_email
      );

      set_current_injection_stage("TRANSMITTING");
      await adminApi.uploadIdentity({
        platform: "google_colab",
        email: form_data.operator_email,
        cookies: encrypted_payload,
        userAgent: navigator.userAgent,
      });

      toast.success(is_identity_rotation_mode_active ? "IDENTITY_ROTATED" : "VAULT_SYNCHRONIZED");

      query_client.invalidateQueries({ queryKey: ["identities-vault-inventory-v23"] });
      reset();
      set_purification_forensic_report(null);
      set_is_identity_rotation_mode_active(false);

    } catch (unidentified_fault: unknown) {
      const error_bridge = unidentified_fault as IngestionFault;
      const error_label = error_bridge.response?.data?.error || error_bridge.message;

      toast.error("INJECTION_FAILED", { description: `FAULT: ${error_label.toUpperCase()}` });
    } finally {
      set_current_injection_stage("IDLE");
    }
  }, [query_client, reset, is_identity_rotation_mode_active, purification_forensic_report]);

  return (
    <Card className={cn(
        "h-full border-zinc-800 shadow-2xl relative group overflow-hidden font-mono transition-all duration-700",
        is_identity_rotation_mode_active ? "bg-amber-950/10 border-amber-900/30" : "bg-[#0a0a0a]"
    )}>
      <div className={cn(
          "absolute top-0 right-0 p-32 blur-[120px] rounded-full pointer-events-none transition-all duration-1000",
          is_identity_rotation_mode_active ? "bg-amber-500/10" : "bg-emerald-500/5"
      )} />

      <CardHeader className="border-b border-white/5 bg-white/2 pb-6 relative z-10">
        <div className="flex justify-between items-start">
          <div className="space-y-1.5">
            <CardTitle className="flex items-center gap-4 text-white text-xl uppercase tracking-widest leading-none italic">
              {is_identity_rotation_mode_active
                ? <RefreshCw className="w-6 h-6 text-amber-500 animate-spin-slow" />
                : <Key className="w-6 h-6 text-emerald-500" />
              }
              {is_identity_rotation_mode_active ? "PHOENIX_ROTATION" : translations("title")}
            </CardTitle>
            <CardDescription className="text-zinc-600 text-[10px] font-black uppercase tracking-[0.2em]">
              {is_identity_rotation_mode_active ? "Overwriting Active Strata" : translations("injection_badge")}
            </CardDescription>
          </div>
          <div className="flex gap-3 text-zinc-700">
             <Lock className="w-5 h-5" />
             <ShieldCheck className="w-5 h-5" />
          </div>
        </div>
      </CardHeader>

      <CardContent className="p-10 space-y-8 relative z-10">
        <form onSubmit={handleSubmit(execute_secure_injection_sequence)} className="space-y-8">

          <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
            <div className="space-y-3">
              <label className="text-[10px] font-black text-zinc-500 uppercase tracking-widest flex items-center gap-2">
                <Terminal className="w-4 h-4 text-zinc-700" /> Operator_ID
              </label>
              <Input
                {...register("operator_email", { required: true })}
                placeholder="operator@prospector.io"
                readOnly={is_identity_rotation_mode_active}
                className={cn(
                    "bg-black border-zinc-800 text-xs h-14",
                    is_identity_rotation_mode_active && "opacity-60 text-amber-500 border-amber-900/20"
                )}
              />
            </div>

            <div className="space-y-3">
              <label className="text-[10px] font-black text-zinc-500 uppercase tracking-widest flex items-center gap-2">
                <ShieldAlert className="w-4 h-4 text-amber-600" /> Master_Key
              </label>
              <Input
                type="password"
                {...register("security_passphrase", { required: true })}
                placeholder="••••••••••••••••"
                className="bg-black border-zinc-800 text-xs h-14 text-amber-500"
              />
            </div>
          </div>

          <div className="space-y-3">
            <label className="text-[10px] font-black text-zinc-500 uppercase tracking-widest flex justify-between items-center px-1">
              <span className="flex items-center gap-2">
                <FileJson className="w-4 h-4 text-zinc-700"/>
                Identity_Strata (JSON)
              </span>
              <span className="flex items-center gap-2 bg-emerald-500/5 px-3 py-1 rounded-full border border-emerald-500/20">
                <Zap className="w-3 h-3 text-emerald-500 animate-pulse" />
                <span className="text-[8px] text-emerald-400 font-bold">Purifier_V2_Active</span>
              </span>
            </label>

            <textarea
              {...register("raw_cookie_material_json", { required: true })}
              className={cn(
                "w-full h-40 bg-black/60 border border-zinc-800 rounded-2xl p-6 text-[11px] text-emerald-500/80 font-mono outline-none transition-all duration-500 resize-none custom-scrollbar",
                purification_forensic_report?.status === "OPTIMAL" && "border-emerald-500/40",
                purification_forensic_report?.status === "DEGRADED" && "border-amber-500/40",
                purification_forensic_report?.status === "CRITICAL" && "border-red-500/40"
              )}
              placeholder="Paste cookie JSON material..."
              spellCheck={false}
            />

            <AnimatePresence>
              {purification_forensic_report && purification_forensic_report.isValidStructure && (
                <motion.div
                  initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0 }}
                  className={cn(
                    "p-5 rounded-2xl border flex flex-col gap-4",
                    purification_forensic_report.status === "OPTIMAL" ? "bg-emerald-950/10 border-emerald-900/30 text-emerald-400" :
                    purification_forensic_report.status === "DEGRADED" ? "bg-amber-950/10 border-amber-900/30 text-amber-400" :
                    "bg-red-950/10 border-red-900/30 text-red-400"
                  )}
                >
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-3">
                      {purification_forensic_report.status === "OPTIMAL" && <CheckCircle2 className="w-5 h-5 shadow-[0_0_10px_#10b981]" />}
                      {purification_forensic_report.status === "DEGRADED" && <AlertTriangle className="w-5 h-5 animate-pulse" />}
                      {purification_forensic_report.status === "CRITICAL" && <ShieldAlert className="w-5 h-5" />}

                      <span className="text-[10px] font-black uppercase tracking-widest italic">
                        {/* ✅ RESOLUCIÓN ESLINT: Narrowing del tipo para evitar 'any' */}
                        {translations(`cookie_report.status_${purification_forensic_report.status.toLowerCase()}` as "cookie_report.status_optimal" | "cookie_report.status_degraded" | "cookie_report.status_critical")}
                      </span>
                    </div>

                    <div className="flex gap-6 text-[9px] font-black uppercase opacity-80">
                      <span className="flex items-center gap-2 border-r border-current pr-4">
                        <CheckCircle2 className="w-3.5 h-3.5" />
                        {purification_forensic_report.validCookiesCount} {translations("cookie_report.stats_valid")}
                      </span>
                      <span className="flex items-center gap-2">
                        <Trash2 className="w-3.5 h-3.5" />
                        {purification_forensic_report.purgedGarbageCount} {translations("cookie_report.stats_garbage")}
                      </span>
                    </div>
                  </div>

                  {purification_forensic_report.missingCriticalKeys.length > 0 && (
                    <div className="text-[9px] font-mono bg-black/60 p-4 rounded-xl border border-red-500/20 text-red-300 shadow-inner">
                      <strong className="text-red-500 block mb-2 tracking-[0.2em]">{translations("cookie_report.missing_keys")}</strong>
                      <div className="grid grid-cols-2 gap-2">
                         {purification_forensic_report.missingCriticalKeys.map(key => (
                           <span key={key} className="flex items-center gap-2">
                             <div className="w-1 h-1 bg-red-800 rounded-full" /> {key}
                           </span>
                         ))}
                      </div>
                    </div>
                  )}

                  {purification_forensic_report.status !== "CRITICAL" && (
                    <div className="flex items-center gap-3 pt-3 border-t border-white/5 text-[9px] font-bold italic opacity-60">
                      <Activity className="w-3.5 h-3.5" /> {/* ✅ REINTEGRADO */}
                      {translations("cookie_report.recommendation")}
                    </div>
                  )}
                </motion.div>
              )}
            </AnimatePresence>
          </div>

          <div className="flex gap-4 pt-6 border-t border-white/5">
              {is_identity_rotation_mode_active && (
                  <Button
                    type="button"
                    variant="outline"
                    className="h-16 px-10 text-[10px] font-black tracking-widest border-zinc-800"
                    onClick={() => {
                        reset();
                        set_is_identity_rotation_mode_active(false);
                        set_purification_forensic_report(null);
                    }}
                  >
                    ABORT_ROTATION
                  </Button>
              )}

              <Button
                type="submit"
                variant="cyber"
                className={cn(
                    "w-full h-16 font-black tracking-[0.5em] uppercase text-xs transition-all",
                    is_identity_rotation_mode_active && "bg-amber-950 border-amber-600 text-amber-500",
                    (purification_forensic_report?.status === "CRITICAL" || purification_forensic_report?.status === "INVALID") && "opacity-40 cursor-not-allowed grayscale"
                )}
                isLoading={current_injection_stage !== "IDLE"}
                disabled={current_injection_stage !== "IDLE" || purification_forensic_report?.status === "CRITICAL" || purification_forensic_report?.status === "INVALID"}
              >
                {current_injection_stage !== "IDLE" ?
                  (current_injection_stage === "ENCRYPTING" ? translations("encrypting") : current_injection_stage)
                  : (is_identity_rotation_mode_active ? "CONFIRM_PHOENIX_ROTATION" : translations("secure_btn"))}

                {current_injection_stage === "IDLE" && <ArrowRight className="w-4 h-4 ml-4" />}
              </Button>
          </div>
        </form>
      </CardContent>
    </Card>
  );
}

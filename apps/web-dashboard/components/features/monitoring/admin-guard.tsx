// INICIO DEL ARCHIVO [apps/web-dashboard/components/features/monitoring/admin-guard.tsx]
/**
 * =================================================================
 * APARATO: ADMIN GUARD HUD (V5.2 - TYPE HARDENED)
 * CLASIFICACIÓN: ACCESS CONTROL (ESTRATO L4)
 * RESPONSABILIDAD: PROTECCIÓN PERIMETRAL Y HANDSHAKE TÁCTICO
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la última barrera de seguridad antes del acceso al mando.
 * 1. RESOLUCIÓN LINT: Erradicación total de 'any' en bloques catch.
 * 2. DIAGNÓSTICO: Mapeo de errores HTTP 401/404/Net para el HUD.
 * 3. PERSISTENCIA: Gestión de SessionToken en el almacenamiento volátil.
 * =================================================================
 */

"use client";

import React, { useState, useEffect, useCallback } from "react";
import { Lock, ArrowRight, AlertTriangle, Terminal, ShieldCheck } from "lucide-react";
import { apiClient } from "@prospector/api-client";
import { useHeimdall } from "@/hooks/use-heimdall";
import { Input } from "@/components/ui/kit/input";
import { Button } from "@/components/ui/kit/button";

/**
 * Estructura de error esperada desde el túnel de red (Axios/Fetch compatible).
 */
interface NetworkErrorStrata {
  response?: {
    status: number;
    data?: {
      error?: string;
    };
  };
}

interface AdminGuardProperties {
  children: React.ReactNode;
}

/**
 * Portero Lógico del Dashboard.
 */
export function AdminGuard({ children }: AdminGuardProperties): React.JSX.Element {
  const logger = useHeimdall("AdminGuard");
  const [is_vault_unlocked, set_is_vault_unlocked] = useState<boolean>(false);
  const [is_authenticating, set_is_authenticating] = useState<boolean>(false);
  const [passphrase_input, set_passphrase_input] = useState<string>("");
  const [error_message, set_error_message] = useState<string>("");

  // Adquisición de la Llave Maestra desde el entorno
  const MASTER_ADMIN_SECRET = process.env.NEXT_PUBLIC_ADMIN_PASSWORD || "";

  /**
   * Ejecuta el protocolo de autenticación bi-direccional.
   */
  const execute_authentication_protocol = useCallback(async () => {
    set_is_authenticating(true);
    set_error_message("");

    // 1. VERIFICACIÓN ESTRUCTURAL (Local)
    if (passphrase_input !== MASTER_ADMIN_SECRET) {
      set_error_message("ACCESS_DENIED: INVALID_MASTER_KEY");
      set_is_authenticating(false);
      return;
    }

    // 2. HANDSHAKE DE ESTRATO (Remoto)
    try {
      await apiClient.get("/admin/identities", {
        headers: { Authorization: `Bearer ${passphrase_input}` },
      });

      // Éxito: Cristalización de sesión en el navegador
      sessionStorage.setItem("ADMIN_SESSION_TOKEN", passphrase_input);
      set_is_vault_unlocked(true);
      logger.info("Handshake certified. Administrative strata level.");

    } catch (unidentified_fault: unknown) {
      // ✅ RESOLUCIÓN LINT: Narrowing del tipo 'unknown' a 'NetworkErrorStrata'
      const strata_error = unidentified_fault as NetworkErrorStrata;
      const status_code = strata_error.response?.status;

      let tactical_error_label = "CONNECTION_REJECTED: Check NEXT_PUBLIC_API_URL";

      if (status_code === 401) {
        tactical_error_label = "TOKEN_MISMATCH: Key rejected by Orchestrator.";
      } else if (status_code === 404) {
        tactical_error_label = "GATEWAY_PATH_ERROR: Endpoint not found.";
      }

      set_error_message(tactical_error_label);
      logger.error(`Handshake Failed [${status_code || "NET_FAULT"}]`);
      sessionStorage.removeItem("ADMIN_SESSION_TOKEN");
    } finally {
      set_is_authenticating(false);
    }
  }, [passphrase_input, MASTER_ADMIN_SECRET, logger]);

  /**
   * Vigilancia de sesión existente en el montaje.
   */
  useEffect(() => {
    const active_token = sessionStorage.getItem("ADMIN_SESSION_TOKEN");
    if (active_token && active_token === MASTER_ADMIN_SECRET) {
      set_is_vault_unlocked(true);
    }
  }, [MASTER_ADMIN_SECRET]);

  // RENDERIZADO DEL ESTRATO DASHBOARD (Si está desbloqueado)
  if (is_vault_unlocked) return <>{children}</>;

  // RENDERIZADO DEL SECURITY WALL
  return (
    <div className="min-h-screen bg-[#050505] flex items-center justify-center p-4 font-mono">
      <div className="max-w-md w-full bg-[#0a0a0a] border border-zinc-800 p-10 rounded-3xl shadow-2xl relative overflow-hidden group">

        {/* CABECERA VISUAL */}
        <div className="text-center mb-10 relative z-10">
          <div className="mx-auto w-20 h-20 bg-zinc-900/50 rounded-2xl flex items-center justify-center mb-6 border border-zinc-700 transition-colors group-hover:border-primary/50">
             <Lock className="w-8 h-8 text-zinc-500" />
          </div>
          <h1 className="text-2xl font-black text-white tracking-[0.2em] uppercase leading-none">Security_Wall</h1>
          <p className="text-[9px] text-zinc-600 mt-2 tracking-widest uppercase font-bold">Identity Stratum L4 // Handshake Required</p>
        </div>

        {/* ESTRATO DE INTERACCIÓN */}
        <div className="space-y-6 relative z-10">
          <div className="relative">
            <div className="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
              <Terminal className="h-4 w-4 text-zinc-600" />
            </div>
            <Input
              type="password"
              value={passphrase_input}
              onChange={(e) => {
                set_passphrase_input(e.target.value);
                set_error_message("");
              }}
              placeholder="ENTER_MASTER_KEY"
              className="pl-10 text-center tracking-[0.3em] font-black text-primary bg-black/50 border-zinc-800 h-14 selection:bg-primary/30"
              onKeyDown={(e) => e.key === "Enter" && execute_authentication_protocol()}
              autoFocus
            />
          </div>

          <Button
            onClick={execute_authentication_protocol}
            disabled={is_authenticating || passphrase_input.length < 4}
            variant="cyber"
            className="w-full h-14 text-xs font-black tracking-[0.3em] uppercase"
            isLoading={is_authenticating}
          >
            IGNITE_AUTHENTICATION
            <ArrowRight className="w-4 h-4 ml-3" />
          </Button>

          <AnimatePresence>
            {error_message && (
              <div className="flex items-center justify-center gap-3 text-red-500 text-[10px] bg-red-950/10 py-4 rounded-xl border border-red-900/20 animate-in zoom-in-95 duration-300">
                <AlertTriangle className="w-4 h-4 shrink-0" />
                <span className="font-black uppercase tracking-tighter">{error_message}</span>
              </div>
            )}
          </AnimatePresence>
        </div>

        {/* SELLO DE INTEGRIDAD */}
        <div className="mt-8 pt-6 border-t border-white/5 flex justify-center">
            <div className="flex items-center gap-2 opacity-30">
                <ShieldCheck className="w-3 h-3 text-emerald-500" />
                <span className="text-[8px] uppercase font-bold tracking-widest text-zinc-400">Hydra-Zero Protected</span>
            </div>
        </div>
      </div>
    </div>
  );
}

/**
 * Componente Placeholder para animaciones (Reemplazo táctico de Framer Motion si no se desea pesadez)
 */
function AnimatePresence({ children }: { children: React.ReactNode }) {
    return <>{children}</>;
}
// FIN DEL ARCHIVO [apps/web-dashboard/components/features/monitoring/admin-guard.tsx]

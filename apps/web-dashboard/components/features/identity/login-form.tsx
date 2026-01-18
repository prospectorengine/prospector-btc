// INICIO DEL ARCHIVO [apps/web-dashboard/components/features/identity/login-form.tsx]
/**
 * =================================================================
 * APARATO: SOVEREIGN LOGIN FORM (V8.0 - HEIMDALL OBSERVABILITY)
 * CLASIFICACIÓN: FEATURE UI (ESTRATO L5)
 * RESPONSABILIDAD: INICIO DE OAUTH CON TRAZABILIDAD COMPLETA
 *
 * VISION HIPER-HOLÍSTICA:
 * Integra el logger soberano para registrar intentos de acceso,
 * redirecciones y fallos del proveedor de identidad.
 * =================================================================
 */

"use client";

import { useState } from "react";
import { Button } from "@/components/ui/kit/button";
import { useTranslations } from "next-intl";
import { supabase } from "@prospector/infra-supabase";
import { LogIn, ShieldCheck } from "lucide-react";
import { useHeimdall } from "@/hooks/use-heimdall"; // ✅ Inyección de Observabilidad

export function LoginForm() {
  const t = useTranslations("Auth");
  const logger = useHeimdall("IdentityAccessGate"); // Contexto Específico
  const [is_authenticating, set_is_authenticating] = useState<boolean>(false);

  const handle_google_login = async (): Promise<void> => {
    set_is_authenticating(true);
    const trace = logger.track("OAuth_Handshake_Initiation"); // Inicio de Span

    try {
      const productionOrigin = process.env.NEXT_PUBLIC_SITE_URL;

      if (!productionOrigin) {
        throw new Error("CRITICAL: NEXT_PUBLIC_SITE_URL is not defined in environment.");
      }

      const callbackUrl = `${productionOrigin}/api/auth/callback`;

      logger.info(`Initiating OAuth redirection target: [${callbackUrl}]`, {
        origin: productionOrigin,
        provider: "google"
      });

      const { error: auth_error } = await supabase.auth.signInWithOAuth({
        provider: 'google',
        options: {
          redirectTo: callbackUrl,
          queryParams: {
            access_type: 'offline',
            prompt: 'consent',
          },
        },
      });

      if (auth_error) {
        trace.fail(auth_error); // Cierre de Span con error
        set_is_authenticating(false);
      } else {
        // No cerramos el Span aquí porque la redirección navegará fuera de la página
        logger.info("OAuth request dispatched to provider. Awaiting callback.");
      }
    } catch (err: unknown) {
      const error_message = err instanceof Error ? err.message : String(err);
      trace.fail(new Error(error_message));
      set_is_authenticating(false);
    }
  };

  return (
    <div className="space-y-4 w-full animate-in fade-in duration-500">
      <Button
        variant="cyber"
        className="w-full h-14 text-xs font-black tracking-[0.2em] gap-3 bg-black border-emerald-500/50 text-emerald-500 hover:bg-emerald-500 hover:text-black shadow-[0_0_20px_rgba(16,185,129,0.2)]"
        onClick={handle_google_login}
        isLoading={is_authenticating}
      >
        {!is_authenticating && <LogIn className="h-4 w-4" />}
        {t("login.google_btn")}
      </Button>

      <div className="flex items-center gap-2 justify-center py-2 opacity-40">
        <ShieldCheck className="w-3 h-3 text-emerald-500" />
        <span className="text-[8px] font-mono uppercase tracking-widest text-zinc-500">
          Uplink: Production_Secure
        </span>
      </div>
    </div>
  );
}
// FIN DEL ARCHIVO [apps/web-dashboard/components/features/identity/login-form.tsx]

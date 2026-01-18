/**
 * =================================================================
 * APARATO: LOGIN PAGE (V6.1 - LINT & SSR SECURED)
 * CLASIFICACIÓN: VIEW LAYER (ESTRATO L5)
 * RESPONSABILIDAD: GESTIÓN DE ACCESO Y VERIFICACIÓN DE SESIÓN SSR
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la puerta de enlace para el operador. Verifica la
 * existencia de una sesión en el Motor B antes del renderizado
 * para evitar accesos redundantes. Saneado contra TS6133.
 * =================================================================
 */

import { getTranslations } from "next-intl/server";
import { redirect } from "next/navigation";
import { cookies } from "next/headers";
import { createServerClient } from "@supabase/ssr";
import { ShieldCheck, Cpu } from "lucide-react";
import { LoginForm } from "@/components/features/identity/login-form";

export default async function LoginPage() {
  const t = await getTranslations("Auth");
  const cookie_store = await cookies();

  /**
   * 1. INICIALIZACIÓN DEL CLIENTE SOBERANO (READ-ONLY)
   * Se utiliza únicamente para verificar la legitimidad del operador
   * en el lado del servidor antes de servir la interfaz.
   */
  const supabase = createServerClient(
    process.env.NEXT_PUBLIC_SUPABASE_URL!,
    process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!,
    {
      cookies: {
        get(name: string) {
          return cookie_store.get(name)?.value;
        },
      },
    }
  );

  const { data: { user } } = await supabase.auth.getUser();

  // 2. PROTOCOLO DE REBOTE: Si el usuario ya está autenticado, enviar al mando.
  if (user) {
    redirect("/dashboard");
  }

  return (
    <div className="flex min-h-screen flex-col items-center justify-center p-4 bg-[#050505] selection:bg-emerald-500/30">
      {/* CAPA DE AMBIENTACIÓN VISUAL */}
      <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_center,_var(--tw-gradient-stops))] from-zinc-900/20 via-[#050505] to-[#050505] pointer-events-none" />

      <div className="w-full max-w-md space-y-8 relative z-10 p-8 border border-white/5 rounded-2xl bg-black/50 backdrop-blur-xl shadow-2xl animate-in fade-in zoom-in-95 duration-500">

        <div className="flex flex-col items-center justify-center text-center space-y-4">
          <div className="h-16 w-16 bg-emerald-500/10 rounded-2xl border border-emerald-500/20 flex items-center justify-center shadow-[0_0_30px_rgba(16,185,129,0.1)]">
            <Cpu className="w-8 h-8 text-emerald-500" />
          </div>

          <div className="space-y-2">
            <h2 className="text-2xl font-black tracking-tighter text-white font-mono uppercase">
              {t("login.title")}
            </h2>

            <div className="inline-flex items-center gap-2 text-[9px] font-mono text-zinc-400 uppercase tracking-[0.2em] bg-white/5 px-3 py-1 rounded-full border border-white/5">
              <ShieldCheck className="w-3 h-3 text-emerald-500" />
              Hydra-Zero Protocol
            </div>
          </div>
        </div>

        {/* COMPONENTE DE INTERACCIÓN OAUTH */}
        <div className="mt-8">
          <LoginForm />
        </div>

        <div className="pt-6 border-t border-white/5 text-center">
          <p className="text-[10px] text-zinc-600 font-mono uppercase tracking-wider">
            {t("login.footer_text")}
          </p>
        </div>
      </div>
    </div>
  );
}

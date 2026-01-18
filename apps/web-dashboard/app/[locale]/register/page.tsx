/**
 * =================================================================
 * APARATO: REGISTER PAGE (V6.0 - SUPABASE SSR & LINT SECURED)
 * CLASIFICACIÓN: VIEW LAYER (ESTRATO L5)
 * RESPONSABILIDAD: PROVISIÓN DE NUEVAS IDENTIDADES DE OPERADOR
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el portal de registro para el enjambre. Esta versión
 * elimina la dependencia de 'next-auth' y utiliza el cliente SSR
 * de Supabase para validar la integridad de la sesión en el servidor.
 * =================================================================
 */

import { getTranslations } from "next-intl/server";
import { redirect } from "next/navigation";
import { cookies } from "next/headers";
import { createServerClient } from "@supabase/ssr";
import { ShieldPlus, Cpu } from "lucide-react";
import { LoginForm } from "@/components/features/identity/login-form";

export default async function RegisterPage() {
  const t = await getTranslations("Auth");
  const cookie_store = await cookies();

  /**
   * 1. PROTOCOLO DE AUDITORÍA DE SESIÓN (L4)
   * Inicializamos el cliente SSR de Supabase para verificar si el operador
   * ya posee credenciales activas antes de permitir el registro.
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

  // 2. REDIRECCIÓN DE SEGURIDAD: Evita duplicidad de sesión
  if (user) {
    redirect("/dashboard");
  }

  return (
    <div className="flex min-h-screen flex-col items-center justify-center p-4 bg-[#050505] selection:bg-purple-500/30">
      {/* CAPA DE AMBIENTACIÓN VISUAL (ESTRATO L5) */}
      <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_top,_var(--tw-gradient-stops))] from-purple-900/20 via-[#050505] to-[#050505] pointer-events-none" />

      <div className="w-full max-w-md space-y-8 relative z-10 p-8 border border-white/5 rounded-2xl bg-black/50 backdrop-blur-xl shadow-2xl animate-in fade-in zoom-in-95 duration-500">

        <div className="flex flex-col items-center justify-center text-center space-y-4">
          <div className="h-16 w-16 bg-purple-500/10 rounded-2xl border border-purple-500/20 flex items-center justify-center shadow-[0_0_30px_rgba(168,85,247,0.1)]">
            <Cpu className="w-8 h-8 text-purple-500" />
          </div>

          <div className="space-y-2">
            <h2 className="text-2xl font-black tracking-tighter text-white font-mono uppercase">
              {t("register.title")}
            </h2>
            <div className="inline-flex items-center gap-2 text-[9px] font-mono text-zinc-400 uppercase tracking-[0.2em] bg-white/5 px-3 py-1 rounded-full border border-white/5">
              <ShieldPlus className="w-3 h-3 text-purple-500" />
              {t("register.badge")}
            </div>
          </div>
        </div>

        {/*
           REUTILIZACIÓN TÁCTICA: LoginForm actúa como puente OAuth
           tanto para login como para registro vía Google.
        */}
        <div className="mt-8">
          <LoginForm />
        </div>

        <div className="pt-6 border-t border-white/5 text-center">
          <p className="text-[10px] text-zinc-600 font-mono uppercase tracking-wider">
            {t("register.footer_text")}
          </p>
        </div>
      </div>
    </div>
  );
}

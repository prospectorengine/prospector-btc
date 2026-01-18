// INICIO DEL ARCHIVO [apps/web-dashboard/app/[locale]/dashboard/layout.tsx]
/**
 * =================================================================
 * APARATO: DASHBOARD LAYOUT (V20.1 - ADMIN GUARD SECURED)
 * CLASIFICACIÓN: ESTRUCTURAL (ESTRATO L5)
 * RESPONSABILIDAD: PROTECCIÓN GLOBAL DEL ESTRATO DE MANDO
 * =================================================================
 */

import { redirect } from "next/navigation";
import { cookies } from "next/headers";
import { createServerClient } from "@supabase/ssr";
import { Sidebar } from "@/components/layout/sidebar";
import { TopNav } from "@/components/layout/top-nav";
import { AdminGuard } from "@/components/features/monitoring/admin-guard";
import { OnboardingWizard } from "@/components/system/onboarding-wizard";

export default async function DashboardLayout({ children }: { children: React.ReactNode }) {
  const cookie_store = await cookies();

  const supabase = createServerClient(
    process.env.NEXT_PUBLIC_SUPABASE_URL!,
    process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!,
    {
      cookies: {
        get(name: string) { return cookie_store.get(name)?.value; },
      },
    }
  );

  const { data: { user } } = await supabase.auth.getUser();

  // 1. PROTECCIÓN DE SESIÓN OAUTH (MOTOR B)
  if (!user) {
    redirect("/login");
  }

  return (
    // 2. PROTECCIÓN DE MANDO (ADMIN GUARD)
    // Este componente intercepta al operador y exige la Master Key (Netflix69),
    // inyectándola en el sessionStorage para habilitar el túnel hacia Render.
    <AdminGuard>
      <div className="flex h-screen w-full bg-[#050505] text-white font-mono">
        <OnboardingWizard />

        <aside className="hidden md:flex w-72 border-r border-white/5 bg-black/40 h-full flex-col">
          <Sidebar />
        </aside>

        <div className="flex flex-1 flex-col relative min-w-0">
          <header className="h-16 border-b border-white/5 bg-black/20 flex items-center w-full z-40 sticky top-0 backdrop-blur-md">
            <TopNav />
          </header>

          <main className="flex-1 overflow-y-auto p-4 md:p-8 custom-scrollbar relative w-full">
            <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-[0.03] pointer-events-none fixed" />
            <div className="relative z-10 max-w-[1920px] mx-auto">
              {children}
            </div>
          </main>
        </div>
      </div>
    </AdminGuard>
  );
}
// FIN DEL ARCHIVO [apps/web-dashboard/app/[locale]/dashboard/layout.tsx]

// INICIO DEL ARCHIVO [apps/web-dashboard/app/[locale]/dashboard/network/page.tsx]
import { useTranslations } from "next-intl";
import { FleetGrid } from "@/components/features/network/fleet-grid";
import { Server, Activity } from "lucide-react";

export default function NetworkPage() {
  const t = useTranslations("Dashboard.fleet");

  return (
    <div className="space-y-8 animate-in fade-in duration-700">
      {/* Cabecera Local de Sección */}
      <div className="flex flex-col md:flex-row justify-between items-start md:items-end gap-4 border-b border-white/5 pb-6">
        <div className="space-y-2">
          <h1 className="text-3xl font-black text-white tracking-tighter uppercase font-mono flex items-center gap-3">
            <Server className="w-8 h-8 text-emerald-500" />
            {/* ✅ FIX: Uso activo de la variable 't' */}
            {t("title")}
          </h1>
          <p className="text-zinc-500 text-xs font-mono uppercase tracking-widest">
            {/* Usamos una clave genérica o fallback visual consistente */}
            Stratum L5 // Global Surveillance Matrix
          </p>
        </div>

        <div className="flex items-center gap-2 px-4 py-2 bg-emerald-500/5 border border-emerald-500/20 rounded-full">
           <Activity className="w-4 h-4 text-emerald-500 animate-pulse" />
           <span className="text-[10px] font-bold text-emerald-400 font-mono uppercase">
             {t("live_feed")}
           </span>
        </div>
      </div>

      {/* Grid de Nodos */}
      <FleetGrid />
    </div>
  );
}
// FIN DEL ARCHIVO [apps/web-dashboard/app/[locale]/dashboard/network/page.tsx]

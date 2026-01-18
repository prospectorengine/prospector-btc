// INICIO DEL ARCHIVO [apps/web-dashboard/app/[locale]/dashboard/identity/page.tsx]
"use client";

import { useQuery } from "@tanstack/react-query";
// ✅ FIX: Eliminado 'type Identity'
import { adminApi } from "@prospector/api-client";
import { IdentityInjector } from "@/components/features/identity/identity-injector";
import { IdentityInventory } from "@/components/features/identity/identity-inventory";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/kit/card";
// ✅ FIX: Eliminados 'Lock' y 'Users'
import { Info, Cpu, CheckCircle2 } from "lucide-react";

export default function IdentityPage() {
  const { data: identities } = useQuery({
    queryKey: ["identities-capacity-check"],
    queryFn: () => adminApi.listIdentities(),
  });

  const activeCount = identities?.filter(i => i.status === 'active').length || 0;
  const projectedCapacity = activeCount * 2;

  return (
    <div className="grid grid-cols-1 xl:grid-cols-12 gap-8 min-h-[calc(100vh-120px)] p-4">
      <div className="xl:col-span-4 space-y-6 overflow-y-auto custom-scrollbar pr-2">
        <div>
            <h1 className="text-2xl font-black text-white uppercase tracking-widest font-mono mb-2">
            Identity Vault
            </h1>
            <p className="text-xs text-zinc-500 font-mono leading-relaxed">
            Inyección de credenciales para operación en la nube.
            </p>
        </div>

        <Card className="bg-blue-950/10 border-blue-900/30">
           <CardContent className="pt-6">
              <div className="flex justify-between items-end mb-2">
                 <span className="text-[10px] font-bold text-blue-400 uppercase tracking-widest">Swarm Capacity</span>
                 <Cpu className="w-5 h-5 text-blue-500" />
              </div>
              <div className="text-3xl font-black text-white font-mono">
                 {projectedCapacity} <span className="text-sm text-zinc-500">Nodes</span>
              </div>
              <p className="text-[9px] text-zinc-500 mt-2 font-mono">
                 Based on {activeCount} active identities in the vault.
              </p>
           </CardContent>
        </Card>

        <Card className="bg-zinc-900/20 border-zinc-800">
          <CardHeader className="pb-3">
             <CardTitle className="text-xs font-bold text-zinc-400 uppercase tracking-wider flex items-center gap-2">
                <Info className="w-4 h-4" /> Procedimiento de Extracción
             </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4 text-[11px] text-zinc-500 font-mono leading-relaxed">
             <Step number={1} text="Instala la extensión 'EditThisCookie'." />
             <Step number={2} text="Login en colab.research.google.com (Incógnito)." />
             <Step number={3} text="Exporta cookies como JSON." />
          </CardContent>
        </Card>

        <div className="p-4 rounded-xl bg-emerald-950/10 border border-emerald-900/20 flex items-start gap-3">
           <CheckCircle2 className="w-4 h-4 text-emerald-500 mt-0.5 shrink-0" />
           <p className="text-[10px] text-emerald-500/80 font-mono leading-relaxed">
             <strong>Zero-Knowledge:</strong> Tus cookies se cifran localmente con AES-256-GCM.
           </p>
        </div>
      </div>

      <div className="xl:col-span-8 flex flex-col gap-6 h-full">
         <div className="flex-none">
            <IdentityInjector />
         </div>
         <div className="flex-1 min-h-[300px]">
            <IdentityInventory />
         </div>
      </div>
    </div>
  );
}

function Step({ number, text }: { number: number, text: string }) {
    return (
        <div className="flex gap-3 items-start">
            <span className="flex-none bg-zinc-800 w-5 h-5 rounded-full flex items-center justify-center text-[10px] text-white font-bold font-mono">
                {number}
            </span>
            <p className="text-[11px] text-zinc-400 font-mono leading-snug pt-0.5">{text}</p>
        </div>
    );
}
// FIN DEL ARCHIVO [apps/web-dashboard/app/[locale]/dashboard/identity/page.tsx]

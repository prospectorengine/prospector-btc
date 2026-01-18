// INICIO DEL ARCHIVO [apps/web-dashboard/app/[locale]/dashboard/launch/page.tsx]
"use client";

import { SwarmLauncher } from "@/components/features/control/swarm-launcher";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/kit/card";
// ✅ FIX: Eliminado 'Rocket'
import { Info, AlertTriangle } from "lucide-react";

export default function LaunchPage() {
  return (
    <div className="grid grid-cols-1 xl:grid-cols-12 gap-8 min-h-[calc(100vh-120px)] p-4">

      <div className="xl:col-span-4 space-y-6">
        <div>
            <h1 className="text-2xl font-black text-white uppercase tracking-widest font-mono mb-2">
            Launch Center
            </h1>
            <p className="text-xs text-zinc-500 font-mono leading-relaxed">
            Configure y autorice el despliegue del enjambre distribuido en la infraestructura efímera.
            </p>
        </div>

        <Card className="bg-zinc-900/30 border-zinc-800">
          <CardHeader className="pb-2">
             <CardTitle className="text-xs font-bold text-blue-400 uppercase tracking-wider flex items-center gap-2">
                <Info className="w-4 h-4" /> Parámetros de Misión
             </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4 text-[10px] text-zinc-400 font-mono">
             <div className="space-y-1">
                <strong className="text-white">Nodes / Shard:</strong>
                <p>Cantidad de navegadores concurrentes por contenedor. Recomendado: 30 para Colab Free.</p>
             </div>
             <div className="space-y-1">
                <strong className="text-white">Parallel Shards:</strong>
                <p>Número de trabajos de GitHub Actions a disparar en paralelo.</p>
             </div>
          </CardContent>
        </Card>

        <div className="p-4 rounded-xl bg-amber-950/10 border border-amber-900/20 flex items-start gap-3">
           <AlertTriangle className="w-4 h-4 text-amber-500 mt-0.5 shrink-0" />
           <p className="text-[10px] text-amber-500/80 font-mono leading-relaxed">
             <strong>Advertencia:</strong> Inyecte credenciales válidas antes de iniciar. Sin cookies, los nodos operarán en modo degradado.
           </p>
        </div>
      </div>

      <div className="xl:col-span-8 h-full flex flex-col justify-center">
         <div className="max-w-2xl w-full mx-auto">
            <SwarmLauncher />
         </div>
      </div>
    </div>
  );
}
// FIN DEL ARCHIVO [apps/web-dashboard/app/[locale]/dashboard/launch/page.tsx]

// INICIO DEL ARCHIVO [apps/web-dashboard/components/features/identity/identity-vault.tsx]
/**
 * =================================================================
 * APARATO: IDENTITY VAULT LAYOUT (V20.0 - GOLD MASTER)
 * RESPONSABILIDAD: ORQUESTACIÓN DEL SUBSISTEMA DE IDENTIDAD
 * =================================================================
 */

"use client";

import { IdentityInjector } from "./identity-injector";
import { IdentityInventory } from "./identity-inventory";

export function IdentityVault(): React.ReactElement {
  return (
    <div className="grid grid-cols-1 xl:grid-cols-3 gap-8 h-full items-start animate-in fade-in slide-in-from-bottom-4 duration-1000">

      {/* AREA DE COMANDO: ESCRITURA Y CIFRADO (2/3) */}
      <div className="xl:col-span-2 h-full">
        <IdentityInjector />
      </div>

      {/* AREA DE VIGILANCIA: ESTADO DEL POOL (1/3) */}
      <div className="xl:col-span-1 h-full min-h-[500px]">
        <IdentityInventory />
      </div>

    </div>
  );
}
// FIN DEL ARCHIVO [apps/web-dashboard/components/features/identity/identity-vault.tsx]

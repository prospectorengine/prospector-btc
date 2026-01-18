// INICIO DEL ARCHIVO [apps/web-dashboard/app/[locale]/dashboard/identity/governance/page.tsx]
/**
 * =================================================================
 * APARATO: IDENTITY GOVERNANCE PAGE (V1.0)
 * CLASIFICACIÓN: VIEW LAYER (ESTRATO L5)
 * RESPONSABILIDAD: RENDERIZADO DEL CENTRO DE MANDO IGFS
 * =================================================================
 */

import { IdentityGovernanceGrid } from "@/components/features/identity/governance/identity-governance-grid";

export default function GovernancePage() {
  return (
    <div className="h-full flex flex-col gap-8 pb-20">
      <IdentityGovernanceGrid />

      {/* FOOTER TÉCNICO */}
      <div className="flex justify-center opacity-30 pt-10">
        <p className="text-[9px] text-zinc-500 font-mono uppercase tracking-[0.5em]">
          Identity_Strata_L3 // Write_Access_Authorized
        </p>
      </div>
    </div>
  );
}
// FIN DEL ARCHIVO [apps/web-dashboard/app/[locale]/dashboard/identity/governance/page.tsx]

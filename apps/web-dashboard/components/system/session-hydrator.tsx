// INICIO DEL ARCHIVO [apps/web-dashboard/components/system/session-hydrator.tsx]
"use client";

import { useEffect } from "react";
import { useSession } from "next-auth/react";

/**
 * =================================================================
 * APARATO: SESSION HYDRATOR (V1.1 - ENV ENFORCED)
 * RESPONSABILIDAD: PUENTE DE AUTORIZACIÓN (OAUTH -> TACTICAL TOKEN)
 * =================================================================
 */
export function SessionHydrator() {
  const { data: session } = useSession();

  useEffect(() => {
    if (session?.user) {
      const currentToken = sessionStorage.getItem("ADMIN_SESSION_TOKEN");

      // ✅ CORRECCIÓN: Extracción estricta. Si no hay variable, no hay acceso.
      const defaultToken = process.env.NEXT_PUBLIC_API_TOKEN;

      if (!currentToken && defaultToken) {
        console.info("🔌 [HYDRATOR]: Injecting Observer Token for Neural Link...");
        sessionStorage.setItem("ADMIN_SESSION_TOKEN", defaultToken);
        window.dispatchEvent(new Event("storage"));
      } else if (!defaultToken) {
        console.warn("⚠️ [HYDRATOR]: NEXT_PUBLIC_API_TOKEN is missing in Vercel Env.");
      }
    }
  }, [session]);

  return null;
}
// FIN DEL ARCHIVO [apps/web-dashboard/components/system/session-hydrator.tsx]

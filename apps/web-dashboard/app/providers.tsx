// INICIO DEL ARCHIVO [apps/web-dashboard/app/providers.tsx]
/**
 * =================================================================
 * APARATO: GLOBAL SYSTEM PROVIDERS (V43.1 - IMPORT FIXED)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (ESTRATO L4)
 * RESPONSABILIDAD: ORQUESTACIÓN DE ESTADO, TEMAS Y TELEMETRÍA
 *
 * VISION HIPER-HOLÍSTICA:
 * Inicializa el transporte de logs remoto (Heimdall Uplink).
 * ✅ FIX: Importación canónica desde el índice de la librería.
 * =================================================================
 */

"use client";

import React, { useState, useEffect } from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Toaster } from "sonner";
import { ThemeProvider } from "@/components/providers/theme-provider";
import { useNeuralHandshake } from "@/hooks/use-neural-handshake";
// ✅ CORRECCIÓN: Importación limpia desde la interfaz pública
import { configureHeimdallUplink } from "@prospector/heimdall-ts";

interface ProvidersProps {
  children: React.ReactNode;
}

export default function Providers({ children }: ProvidersProps): React.JSX.Element {

  useNeuralHandshake();

  // Configuración del Uplink de Telemetría (C2 Beacon)
  useEffect(() => {
    const uplink = process.env.NEXT_PUBLIC_API_URL;
    // Nota: El token público es solo para el handshake inicial de logs anónimos
    const token = process.env.NEXT_PUBLIC_API_TOKEN || "observer";

    if (uplink) {
      configureHeimdallUplink(uplink, token);
    }
  }, []);

  const [queryClient] = useState(
    () =>
      new QueryClient({
        defaultOptions: {
          queries: {
            retry: process.env.NODE_ENV === "production" ? 3 : 1,
            refetchOnWindowFocus: false,
            staleTime: 5000,
          },
        },
      }),
  );

  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider
        attribute="class"
        defaultTheme="dark"
        enableSystem={true}
        disableTransitionOnChange={true}
      >
        {children}
        <Toaster
          position="bottom-right"
          theme="dark"
          closeButton={true}
          className="font-mono"
          toastOptions={{
            style: {
              background: "#050505",
              border: "1px solid #18181b",
              color: "#e4e4e7",
            },
            classNames: {
              toast: "group bg-[#0a0a0a] border-zinc-800 text-zinc-200 shadow-2xl",
              title: "text-emerald-500 font-black uppercase tracking-widest text-[10px]",
              description: "text-zinc-500 text-[9px] font-mono",
              actionButton: "bg-emerald-600 text-white font-bold",
              cancelButton: "bg-zinc-800 text-zinc-400",
              error: "border-red-900/50 bg-red-950/10 text-red-400",
              success: "border-emerald-900/50 bg-emerald-950/10",
            },
          }}
        />
      </ThemeProvider>
    </QueryClientProvider>
  );
}
// FIN DEL ARCHIVO [apps/web-dashboard/app/providers.tsx]

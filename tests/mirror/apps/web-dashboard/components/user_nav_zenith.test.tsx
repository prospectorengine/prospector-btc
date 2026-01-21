/**
 * =================================================================
 * APARATO: USER NAV ZENITH TEST (V1.1 - HYGIENE FIXED)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L5-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE INTEGRACIÓN NEXUS/BILLING
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. LINT COMPLIANCE: Resuelve el error 'no-console' utilizando el canal
 *    'info' autorizado para bitácoras de éxito en la suite de pruebas.
 * 2. ASYNC SYNC: Refuerza la espera de elementos reactivos (LVL 5, 75.5)
 *    para evitar falsos negativos por latencia de renderizado.
 * 3. ZERO REGRESSIONS: Mantiene paridad absoluta con los contratos L7
 *    del Orquestador.
 * =================================================================
 */

import React from "react";
import { render, screen, fireEvent } from "@testing-library/react";
import { UserNav } from "../../../../../apps/web-dashboard/components/layout/user-nav";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

/**
 * MOCK DE SERVICIOS L7
 * Simula la respuesta de los estratos de Prestigio (Nexus) y Energía (Billing).
 */
jest.mock("@prospector/api-client", () => ({
  nexusApi: { getPrestige: jest.fn().mockResolvedValue({ level: 5, title: "Elite Archaeologist" }) },
  billingApi: { getQuota: jest.fn().mockResolvedValue({ remaining_compute_credits_balance: 75.5 }) },
  useHeimdall: () => ({ info: jest.fn() })
}));

// Mock de traducciones para evitar dependencias de archivos locales
jest.mock("next-intl", () => ({
  useTranslations: () => (key: string) => key,
}));

describe("L5: User Navigation Zenith Integration Audit V1.1", () => {
  const createTestWrapper = () => {
    const queryClient = new QueryClient({
      defaultOptions: { queries: { retry: false } },
    });
    return ({ children }: { children: React.ReactNode }) => (
      <QueryClientProvider client={queryClient}>
        {children}
      </QueryClientProvider>
    );
  };

  it("should display operator rank and energy credits from strata L2/L4 after trigger", async () => {
    const Wrapper = createTestWrapper();
    const mock_user_artifact = { name: "Raz Podesta", email: "raz@metashark.tech" };

    render(<UserNav user_identity={mock_user_artifact} />, { wrapper: Wrapper });

    // 1. EXECUTION: Disparar la apertura del menú de mando
    const menu_trigger = screen.getByLabelText(/Open sovereign menu/i);
    fireEvent.click(menu_trigger);

    // 2. VALIDATION: Verificación de sincronía L7 con el Ledger Táctico
    // Esperamos la resolución de las promesas de los mocks
    expect(await screen.findByText(/LVL 5/i)).toBeInTheDocument();
    expect(await screen.findByText(/75.5/i)).toBeInTheDocument();
    expect(screen.getByText(/Elite Archaeologist/i)).toBeInTheDocument();

    // ✅ RESOLUCIÓN LINT: Uso de console.info autorizado por el Codex
    console.info("   ✅ UserNav Zenith: L7 Data synchronization certified bit-perfect.");
  });
});

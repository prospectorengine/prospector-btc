/**
 * =================================================================
 * APARATO: USER NAV ZENITH TEST (V1.2 - INTERFACE ALIGNED)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L5-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE INTEGRACIÓN NEXUS/BILLING
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. INTERFACE ALIGNMENT: Resuelve el error TS2322 mediante la sincronización
 *    nominal de la prop 'operator_identity' con el componente L5 V2.6.
 * 2. LINT COMPLIANCE: Mantiene el canal 'console.info' para trazado forense
 *    de éxito en la suite de pruebas.
 * 3. ASYNC DETERMINISM: Refuerza la captura de elementos asíncronos para
 *    validar la reactividad de los estratos L7.
 * 4. ZERO REGRESSIONS: Preserva los mocks de API de prestigio y energía.
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
  nexusApi: {
    getPrestige: jest.fn().mockResolvedValue({
      level: 5,
      title: "Elite Archaeologist"
    })
  },
  billingApi: {
    getQuota: jest.fn().mockResolvedValue({
      remaining_compute_credits_balance: 75.5
    })
  },
  useHeimdall: () => ({
    info: jest.fn(),
    debug: jest.fn()
  })
}));

// Mock de internacionalización para entorno de pruebas
jest.mock("next-intl", () => ({
  useTranslations: () => (key: string) => key,
}));

describe("L5: User Navigation Zenith Integration Audit V1.2", () => {
  /**
   * Proveedor de contexto para aislar el estado de las consultas.
   */
  const createTestWrapper = () => {
    const queryClient = new QueryClient({
      defaultOptions: {
        queries: {
          retry: false,
          gcTime: 0
        }
      },
    });

    return ({ children }: { children: React.ReactNode }) => (
      <QueryClientProvider client={queryClient}>
        {children}
      </QueryClientProvider>
    );
  };

  it("should display operator rank and energy credits from strata L2/L4 after trigger", async () => {
    const Wrapper = createTestWrapper();

    // ARTEFACTO DE IDENTIDAD SOBERANA
    const mock_operator_artifact = {
      name: "Raz Podesta",
      email: "raz@metashark.tech"
    };

    // ✅ RESOLUCIÓN TS2322: Sincronización nominal de la propiedad 'operator_identity'
    render(
      <UserNav operator_identity={mock_operator_artifact} />,
      { wrapper: Wrapper }
    );

    // 1. EXECUTION: Disparar la apertura del menú de mando Zenith
    const menu_trigger = screen.getByLabelText(/Open sovereign command menu/i);
    fireEvent.click(menu_trigger);

    // 2. VALIDATION: Verificación de sincronía L7 con el Ledger Táctico
    // Utilizamos findBy para manejar la naturaleza asíncrona de los mocks de TanStack Query
    const level_indicator = await screen.findByText(/LVL 5/i);
    const energy_indicator = await screen.findByText(/75.5/i);
    const title_indicator = screen.getByText(/Elite Archaeologist/i);

    expect(level_indicator).toBeInTheDocument();
    expect(energy_indicator).toBeInTheDocument();
    expect(title_indicator).toBeInTheDocument();

    // ✅ VEREDICTO: Registro de éxito en la bitácora de pruebas
    console.info("   ✅ UserNav Zenith: L7 Data synchronization certified bit-perfect.");
  });

  it("should handle correctly the initials derivation for the fallback avatar", () => {
    const Wrapper = createTestWrapper();
    const mock_operator_artifact = { name: "Satoshi Nakamoto", email: "genesis@btc.org" };

    render(
      <UserNav operator_identity={mock_operator_artifact} />,
      { wrapper: Wrapper }
    );

    // Verificamos que las iniciales se extraen correctamente (SN)
    expect(screen.getByText("SN")).toBeInTheDocument();
  });
});

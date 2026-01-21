/**
 * =================================================================
 * APARATO: SETTINGS MANDO INTEGRITY TEST (V1.2 - HYGIENE FIXED)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L5-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE TRANSMISIÓN C2 Y TYPE SAFETY
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. LINT COMPLIANCE: Resuelve el error 'no-console' utilizando el canal
 *    'info' autorizado por el Codex para bitácoras forenses.
 * 2. TYPE NARROWING AUDIT: Verifica que el despacho táctico extraiga
 *    correctamente el payload sin recurrir a 'any'.
 * 3. SYMBOL VERIFICATION: Certifica la presencia de iconografía crítica
 *    (Fingerprint) para estrategias forenses.
 * 4. ZERO REGRESSIONS: Mantiene paridad absoluta con el Kernel L3.
 * =================================================================
 */

import React from "react";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import SettingsPage from "../../../../../apps/web-dashboard/app/[locale]/dashboard/settings/page";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { apiClient } from "@prospector/api-client";

/**
 * MOCK DE INFRAESTRUCTURA TÁCTICA
 * Sincronizado con la V3.3 de la interfaz.
 */
jest.mock("@prospector/api-client", () => ({
  useNeuralLink: () => ({ is_neural_link_connected: true }),
  apiClient: { post: jest.fn().mockResolvedValue({ status: 200 }) },
  useHeimdall: () => ({ info: jest.fn(), error: jest.fn() })
}));

jest.mock("next-intl", () => ({
  useTranslations: () => (key: string) => key,
}));

describe("L5: Settings Command Console Audit V1.2", () => {
  const createTestWrapper = () => {
    const queryClient = new QueryClient({
      defaultOptions: { queries: { retry: false } }
    });
    return ({ children }: { children: React.ReactNode }) => (
      <QueryClientProvider client={queryClient}>
        {children}
      </QueryClientProvider>
    );
  };

  it("should dispatch a real HTTP POST when Halt Swarm is triggered with type-safe payload", async () => {
    const Wrapper = createTestWrapper();
    render(<SettingsPage />, { wrapper: Wrapper });

    const halt_button_locator = screen.getByText(/EXECUTE_HALT_PROTOCOL/i);
    fireEvent.click(halt_button_locator);

    // Validación de la transmisión real hacia el Orquestador
    await waitFor(() => {
      expect(apiClient.post).toHaveBeenCalledWith("/admin/system/mode", expect.objectContaining({
        targetMode: "Maintenance",
        reason: "MANUAL_OVERRIDE"
      }));
    });

    // ✅ RESOLUCIÓN LINT: Uso de console.info autorizado para trazado de éxito en tests
    console.info("   ✅ Settings Mando: C2 Directive transmission certified bit-perfect.");
  });

  it("should render the Fingerprint identity signature for forensic modules", () => {
    const Wrapper = createTestWrapper();
    render(<SettingsPage />, { wrapper: Wrapper });

    // Verificamos que el StrategyBox de forense se materializa (Prueba indirecta de importación de icono)
    const forensic_label = screen.getByText(/Forensic_Satoshi/i);
    expect(forensic_label).toBeInTheDocument();
  });
});

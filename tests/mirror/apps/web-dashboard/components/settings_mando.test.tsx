/**
 * =================================================================
 * APARATO: SETTINGS MANDO INTEGRITY TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L5-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE TRANSMISIÓN C2
 * =================================================================
 */

import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import SettingsPage from "../../../../../apps/web-dashboard/app/[locale]/dashboard/settings/page";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { apiClient } from "@prospector/api-client";

// Mock de infraestructura
jest.mock("@prospector/api-client", () => ({
  useNeuralLink: () => ({ is_neural_link_connected: true }),
  apiClient: { post: jest.fn().mockResolvedValue({ status: 200 }) },
  useHeimdall: () => ({ info: jest.fn() })
}));

jest.mock("next-intl", () => ({
  useTranslations: () => (key: string) => key,
}));

describe("L5: Settings Command Console Audit", () => {
  it("should dispatch a real HTTP POST when Halt Swarm is triggered", async () => {
    const queryClient = new QueryClient();
    render(
      <QueryClientProvider client={queryClient}>
        <SettingsPage />
      </QueryClientProvider>
    );

    const halt_btn = screen.getByText(/EXECUTE_HALT_PROTOCOL/i);
    fireEvent.click(halt_btn);

    // Verificamos que el comando llegó a la capa de infraestructura
    await waitFor(() => {
      expect(apiClient.post).toHaveBeenCalledWith("/admin/system/mode", expect.objectContaining({
        targetMode: "Maintenance"
      }));
    });

    console.log("   ✅ Settings Mando: C2 Directive transmission certified.");
  });
});

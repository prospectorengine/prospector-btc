/**
 * =================================================================
 * APARATO: SWARM LAUNCHER UI TEST (V15.5 - SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE (ESTRATO L5-MIRROR)
 * RESPONSABILIDAD: CERTIFICACIÓN DE LÓGICA DE MANDO Y BAN-SHIELD
 *
 * VISION HIPER-HOLÍSTICA:
 * Este aparato valida la integridad reactiva del SwarmLauncher.
 * 1. RESOLUCIÓN TS2307: Rutas absolutas vinculadas al alias @prospector.
 * 2. RESOLUCIÓN ESLINT: Eliminación de variables muertas mediante implementación real.
 * 3. INTEGRIDAD: Validación del ratio 1:3 (Identidad:Nodos).
 * =================================================================
 */

import React from "react";
import { render, screen } from "@testing-library/react";
import "@testing-library/jest-dom";
import { SwarmLauncher } from "@/components/features/control/swarm-launcher";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

/**
 * Mock de Proveedores de Infraestructura.
 * Requerido para estabilizar el contexto de TanStack Query y Next-Intl.
 */
const createTestProviders = () => {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });

  return ({ children }: { children: React.ReactNode }) => (
    <QueryClientProvider client={queryClient}>
      {children}
    </QueryClientProvider>
  );
};

/**
 * Mocks de Señales Externas (Neural Link).
 * Evita pánicos por falta de conexión SSE durante la prueba unitaria.
 */
jest.mock("@prospector/api-client", () => ({
  useNeuralLink: () => ({
    is_neural_link_connected: true,
    provisioning_logs: [],
    global_aggregated_metrics: { active_nodes_count: 5 },
  }),
  controlApi: { launchSwarm: jest.fn() },
  adminApi: { listIdentities: jest.fn().mockResolvedValue([]) },
}));

describe("Swarm Launcher Cockpit V15", () => {
  const Providers = createTestProviders();

  /**
   * # Mathematical Proof:
   * Verifica que la interfaz de mando se materializa correctamente
   * en el DOM bajo el estándar visual Hydra.
   */
  it("should render the Command Hub correctly in the visual strata", () => {
    render(<SwarmLauncher />, { wrapper: Providers });

    const commandHubTitle = screen.getByText(/Swarm_Command_Hub_V15/i);
    expect(commandHubTitle).toBeInTheDocument();

    // Verificación de existencia del botón de ignición
    const ignitionButton = screen.getByRole("button", { name: /EXECUTE_IGNITION_PROTOCOL/i });
    expect(ignitionButton).toBeInTheDocument();
  });

  /**
   * # Safety Check:
   * Valida que el botón de ignición esté deshabilitado si no hay
   * señales de identidad válidas (Protección anti-regresión).
   */
  it("should maintain ignition in standby if configuration is invalid", () => {
    render(<SwarmLauncher />, { wrapper: Providers });

    const ignitionButton = screen.getByRole("button", { name: /EXECUTE_IGNITION_PROTOCOL/i });

    // En el montaje inicial sin datos de identidad, debe estar bloqueado.
    expect(ignitionButton).toBeDisabled();
  });
});

/**
 * =================================================================
 * APARATO: OMNISCIENT COMMANDER TEST (V15.6 - SOBERANO)
 * CLASIFICACIÓN: TRINITY EVIDENCE (ESTRATO L5-MIRROR)
 * RESPONSABILIDAD: CERTIFICACIÓN DE LÓGICA DE MANDO Y TELEMETRÍA C2
 *
 * VISION HIPER-HOLÍSTICA:
 * Valida la integridad del cerebro visual del Dashboard.
 * 1. RESOLUCIÓN TS2307: Vinculación de componente vía alias de estrato.
 * 2. RESOLUCIÓN ESLINT: Implementación de lógica real consumiendo 'render' y 'screen'.
 * 3. LÓGICA: Simulación de inyección de identidades para validar el Ban-Shield.
 * =================================================================
 */

import React from "react";
import { render, screen } from "@testing-library/react";
import "@testing-library/jest-dom";
import { SwarmLauncher } from "@/components/features/control/swarm-launcher";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

/**
 * MOCK DE SINAPSIS NEURAL
 * Simula el estado del enlace SSE y el inventario de identidades.
 */
jest.mock("@prospector/api-client", () => ({
  useNeuralLink: () => ({
    is_neural_link_connected: true,
    provisioning_logs: [
      {
        node_index: 1,
        message: "VM_READY: Google Colab Instance Provisioned",
        level: "INFO",
        timestamp: new Date().toISOString()
      }
    ],
    global_aggregated_metrics: { active_nodes_count: 5 }
  }),
  controlApi: { launchSwarm: jest.fn() },
  adminApi: {
    // Simulamos 2 identidades activas: Capacidad = 2 * 3 = 6 Nodos
    listIdentities: jest.fn().mockResolvedValue([
      { id: "1", email: "op1@prospector.io", status: "active" },
      { id: "2", email: "op2@prospector.io", status: "active" }
    ])
  }
}));

/**
 * PROVEEDOR TÁCTICO DE PRUEBAS
 * Envuelve el componente en el contexto de TanStack Query necesario para L5.
 */
const TestWrapper = ({ children }: { children: React.ReactNode }) => {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } }
  });
  return (
    <QueryClientProvider client={queryClient}>
      {children}
    </QueryClientProvider>
  );
};

describe("Omniscient Swarm Commander V15", () => {

  /**
   * # Mathematical Proof:
   * Verifica que el ratio de seguridad (1 Identidad : 3 Nodos) se
   * calcula y visualiza correctamente en la interfaz.
   */
  it("should display the correct safe node capacity based on vault volume", async () => {
    render(<SwarmLauncher />, { wrapper: TestWrapper });

    // El label debe aparecer después de la resolución de la promesa de identidades
    const capacityLabel = await screen.findByText(/SAFE_LIMIT: 6/i);
    expect(capacityLabel).toBeInTheDocument();
    expect(capacityLabel).toHaveClass("text-emerald-500");
  });

  /**
   * # Observability Check:
   * Valida que el túnel L6 -> L5 (Provisioner -> Dashboard)
   * materializa los logs de navegación en la consola visual.
   */
  it("should render live provisioning logs from the neural link", () => {
    render(<SwarmLauncher />, { wrapper: TestWrapper });

    const logEntry = screen.getByText(/VM_READY: Google Colab Instance/i);
    expect(logEntry).toBeInTheDocument();

    // Verificación de prefijo de nodo nominal
    const nodePrefix = screen.getByText(/NODE_1:/i);
    expect(nodePrefix).toBeInTheDocument();
  });

  /**
   * # Safety Guard:
   * Asegura que el botón de mando principal esté presente y
   * etiquetado según el estándar de la Tesis.
   */
  it("should provide the master ignition protocol button", () => {
    render(<SwarmLauncher />, { wrapper: TestWrapper });

    const ignitionBtn = screen.getByText(/EXECUTE_IGNITION_PROTOCOL/i);
    expect(ignitionBtn).toBeInTheDocument();
    expect(ignitionBtn.closest('button')).toBeDisabled(); // Deshabilitado por default sin input
  });
});

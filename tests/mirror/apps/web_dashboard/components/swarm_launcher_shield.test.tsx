/**
 * =================================================================
 * APARATO: SWARM LAUNCHER SHIELD TEST (V16.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L5-MIRROR
 * OBJETIVO: Certificar que el Ban-Shield bloquea físicamente la ignición.
 * =================================================================
 */

import { render, screen } from "@testing-library/react";
import { SwarmLauncher } from "@/components/features/control/swarm-launcher";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

// Mock del Neural Link simulando un BanShield activo
jest.mock("@prospector/api-client", () => ({
  useNeuralLink: () => ({
    is_neural_link_connected: true,
    provisioning_logs: [],
    global_aggregated_metrics: { active_nodes_count: 5 },
    ban_shield_status: {
      is_ignition_authorized: false,
      restriction_reason: "IP_DENSITY_CRITICAL",
      safe_node_capacity: 0
    }
  }),
  controlApi: { launchSwarm: jest.fn() }
}));

describe("L5: Swarm Launcher Omniscient Authority Audit", () => {
  it("should physically disable the ignition button when Ban-Shield is active", () => {
    const client = new QueryClient();
    render(
      <QueryClientProvider client={client}>
        <SwarmLauncher />
      </QueryClientProvider>
    );

    const ignitionBtn = screen.getByText(/EXECUTE_IGNITION_SEQUENCE/i).closest('button');
    expect(ignitionBtn).toBeDisabled();

    expect(screen.getByText(/IP_DENSITY_CRITICAL/i)).toBeInTheDocument();
  });
});

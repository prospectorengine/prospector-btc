/**
 * =================================================================
 * APARATO: USER NAV ZENITH TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L5-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE INTEGRACIÓN NEXUS/BILLING
 * =================================================================
 */


import { render, screen } from "@testing-library/react";
import { UserNav } from "../../../../../apps/web-dashboard/components/layout/user-nav";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

// Mocks de servicios L7
jest.mock("@prospector/api-client", () => ({
  nexusApi: { getPrestige: jest.fn().mockResolvedValue({ level: 5, title: "Elite Archaeologist" }) },
  billingApi: { getQuota: jest.fn().mockResolvedValue({ remaining_compute_credits_balance: 75.5 }) },
  useHeimdall: () => ({ info: jest.fn() })
}));

describe("L5: User Navigation Zenith Integration Audit", () => {
  it("should display operator rank and energy credits from strata L2/L4", async () => {
    const queryClient = new QueryClient();
    const mock_user = { name: "Raz Podesta", email: "raz@metashark.tech" };

    render(
      <QueryClientProvider client={queryClient}>
        <UserNav user_identity={mock_user} />
      </QueryClientProvider>
    );

    // Abrir el menú (Trigger)
    const trigger = screen.getByLabelText(/Open sovereign menu/i);
    trigger.click();

    // Verificación de sincronía L7
    expect(await screen.findByText(/LVL 5/i)).toBeInTheDocument();
    expect(await screen.findByText(/75.5/i)).toBeInTheDocument();
    expect(screen.getByText(/Elite Archaeologist/i)).toBeInTheDocument();

    console.log("   ✅ UserNav Zenith: L7 Data synchronization certified.");
  });
});

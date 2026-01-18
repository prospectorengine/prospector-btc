import { render, screen } from "@testing-library/react";
import AcademyPage from "../../../../apps/web-dashboard/app/[locale]/dashboard/academy/page";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

// Mock del cliente de traducción
jest.mock("next-intl", () => ({
  useTranslations: () => (key: string) => key,
}));

// Mock del Oracle Neural
jest.mock("@prospector/api-client", () => ({
  neuralOracle: {
    query: jest.fn().mockResolvedValue({
      getAdaptiveCurriculum: [
        {
          identifier: "ECC-01",
          i18nTitleKey: "The Elliptic Enigma",
          visualIconSignature: "network",
          currentStatus: "UNLOCKED"
        }
      ],
      getOperatorMastery: {
        certifiedModulesCount: 5,
        totalMiningTimeMinutes: 150,
        masterStratumLevel: 2
      }
    })
  }
}));

describe("Academy Zenith UI V2 Certification", () => {
  it("should materialize the mastery HUD with real-time metrics", async () => {
    const queryClient = new QueryClient();
    render(
      <QueryClientProvider client={queryClient}>
        <AcademyPage />
      </QueryClientProvider>
    );

    // Verificamos que el HUD de maestría se renderiza tras la hidratación
    const certifiedValue = await screen.findByText("5");
    expect(certifiedValue).toBeInTheDocument();
    expect(screen.getByText(/CERTIFIED_UNITS/i)).toBeInTheDocument();
  });
});

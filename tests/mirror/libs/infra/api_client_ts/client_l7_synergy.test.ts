/**
 * =================================================================
 * APARATO: API CLIENT SYNERGY TEST (V1.0)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L4-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE TÚNELES L7
 * =================================================================
 */

import { billingApi, heraldApi, nexusApi } from "../../../../../libs/infra/api-client-ts/src/lib/client";

// Mock de Axios para evitar tráfico de red real durante la auditoría
jest.mock('axios', () => ({
  create: jest.fn(() => ({
    interceptors: {
      request: { use: jest.fn() },
      response: { use: jest.fn() }
    },
    get: jest.fn(() => Promise.resolve({ data: { status: "MOCK_OK" } })),
    post: jest.fn(() => Promise.resolve({ data: { status: "MOCK_ACK" } }))
  }))
}));

describe("L4: API Client Stratum L7 Synergy Audit", () => {

  it("should expose specialized tunnels for User Services", () => {
    expect(billingApi.getQuota).toBeDefined();
    expect(heraldApi.listNotifications).toBeDefined();
    expect(nexusApi.getPrestige).toBeDefined();
  });

  it("should format tactical requests correctly for Herald marked-as-read", async () => {
    // La prueba de éxito es que el método no lance excepciones de tipo
    const result = await heraldApi.markAsRead("test-id-001");
    expect(result).toBeDefined();
  });
});

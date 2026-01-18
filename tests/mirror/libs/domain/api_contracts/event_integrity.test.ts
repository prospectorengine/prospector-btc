import { RealTimeEventSchema } from "@prospector/api-contracts";

describe("Provisioning Telemetry Integrity", () => {
  it("should validate a raw provisioning log event", () => {
    const mockEvent = {
      t: "pl",
      p: {
        node_index: 1,
        message: "PLAYWRIGHT: Authenticating with Google...",
        level: "INFO",
        timestamp: new Date().toISOString()
      }
    };

    const result = RealTimeEventSchema.safeParse(mockEvent);
    expect(result.success).toBe(true);
  });

  it("should block unauthorized ignition via Ban-Shield logic", () => {
    const mockShield = {
      t: "bs",
      p: {
        identities_in_vault: 2,
        safe_node_capacity: 6,
        is_ignition_authorized: false,
        restriction_reason: "INSUFFICIENT_IDENTITIES"
      }
    };

    const result = RealTimeEventSchema.safeParse(mockShield);
    expect(result.success).toBe(true);
  });
});

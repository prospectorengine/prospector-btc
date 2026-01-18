import { RealTimeEventSchema } from "./schema";

describe("Sovereign Schema V80 Integrity", () => {
  it("should validate all legacy events from V79.0 (No Regressions)", () => {
    const legacyPulse = { t: "sp", p: { active_nodes_count: 5, cumulative_global_hashrate: 100, active_missions_in_flight: 2, system_timestamp_milliseconds: 12345 } };
    expect(RealTimeEventSchema.safeParse(legacyPulse).success).toBe(true);
  });

  it("should validate new Provisioning Trace (Incremental)", () => {
    const newTrace = { t: "pl", p: { node_index: 1, message: "VM_READY", level: "INFO", timestamp: new Date().toISOString() } };
    expect(RealTimeEventSchema.safeParse(newTrace).success).toBe(true);
  });

  it("should validate new Ban-Shield signal (Incremental)", () => {
    const shield = { t: "bs", p: { identities_in_vault: 1, safe_node_capacity: 3, is_ignition_authorized: true } };
    expect(RealTimeEventSchema.safeParse(shield).success).toBe(true);
  });
});

import { NextRequest } from "next/server";
import { POST } from "../../../../../apps/web-dashboard/app/api/github/runs/sync/route";

describe("Neural Handshake Sync V2: Ballistic Audit", () => {

  it("should reject unencrypted/raw JSON signals for security", async () => {
    const raw_req = new NextRequest("http://localhost/api/sync", {
      method: "POST",
      body: JSON.stringify({ invalid: "data" })
    });

    const response = await POST(raw_req);
    const data = await response.json();

    // Debe retornar PROCESSED (Fail-silent) pero no dar éxito de estrato
    expect(data.status).toBe("PROCESSED");
    expect(data.code).toBe("ACK_SILENT");
  });

  it("should decode and validate a legitimate Base64 signal", async () => {
    const valid_payload = {
      identity_token: "UNIT_TEST_FINGERPRINT_001",
      stratum_layer: "L6_TEST",
      dispatch_timestamp: new Date().toISOString()
    };

    const encoded = btoa(JSON.stringify(valid_payload));

    const req = new NextRequest("http://localhost/api/sync", {
      method: "POST",
      body: JSON.stringify({ _signal_data: encoded })
    });

    const response = await POST(req);
    const data = await response.json();

    // En entorno de test sin MONGO_URI, debe responder DEGRADED_OPS_ACK
    expect(response.status).toBe(200);
    expect(data.status).toMatch(/SYNC_OK|DEGRADED_OPS_ACK/);
  });
});

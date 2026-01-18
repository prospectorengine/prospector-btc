/**
 * =================================================================
 * APARATO: TELEMETRY INSTALL SINK (V1.2 - SOBERANO)
 * CLASIFICACIÓN: API ROUTE (ESTRATO L4)
 * RESPONSABILIDAD: REGISTRO DE INSTALACIÓN EN EL MOTOR C
 * =================================================================
 */

import { NextResponse } from "next/server";
import { MongoClient } from "mongodb";

const MONGODB_TELEMETRY_URI = process.env.MONGO_TELEMETRY_URI;

export async function POST(execution_request: Request): Promise<NextResponse> {
  try {
    const installation_data_payload = await execution_request.json();

    if (!MONGODB_TELEMETRY_URI) {
      return NextResponse.json({ status: "DEGRADED_MODE" }, { status: 200 });
    }

    const database_connection_client = await MongoClient.connect(MONGODB_TELEMETRY_URI);
    const primary_database = database_connection_client.db("prospector_analytics");

    await primary_database.collection("deployment_logs").insertOne({
      ...installation_data_payload,
      server_crystallized_at: new Date()
    });

    await database_connection_client.close();
    return NextResponse.json({ status: "ACKNOWLEDGED" });

  } catch {
    /**
     * RESOLUCIÓN LINT: Fallo silencioso por diseño.
     * La telemetría de instalación no debe bloquear el runtime principal.
     */
    return NextResponse.json({ status: "INGESTION_BYPASSED" }, { status: 200 });
  }
}

/**
 * =================================================================
 * APARATO: NEURAL HANDSHAKE SYNC (V2.0 - ZENITH SECURED)
 * CLASIFICACIÓN: API STRATUM // ESTRATO L4-API
 * RESPONSABILIDAD: AUDITORÍA SILENCIOSA E INGESTA EN MOTOR C
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la validación defensiva de señales de instalación.
 * 1. ZOD SHIELD: Validación estructural post-decodificación.
 * 2. OBSERVABILIDAD: Registro de trazas vía Heimdall-TS.
 * 3. RESILIENCIA: Gestión de conexiones MongoDB con Fail-Silent optimizado.
 * =================================================================
 */

import { NextRequest, NextResponse } from "next/server";
import { MongoClient } from "mongodb";
import { z } from "zod";
import { visitorHandler } from "@/lib/handlers/visitor";
import { createLogger } from "@prospector/heimdall-ts";

const logger = createLogger("Uplink:HandshakeSync");

/**
 * Esquema de Validación SSS (Sovereign Signal Schema).
 * Garantiza que el payload decodificado sea íntegro.
 */
const HandshakePayloadSchema = z.object({
  identity_token: z.string().min(16).max(128),
  stratum_layer: z.string().default("L6_INIT"),
  dispatch_timestamp: z.string().datetime(),
});

type HandshakePayload = z.infer<typeof HandshakePayloadSchema>;

const ACADEMIC_SINK_URI = process.env.MONGODB_AUDIT_URI;

/**
 * Endpoint de Sincronización Táctica.
 */
export async function POST(execution_request: NextRequest): Promise<NextResponse> {
  const audit_trace = logger.track("Academic_Installation_Sync");

  try {
    // 1. ADQUISICIÓN Y DECODIFICACIÓN SEGURA
    const body = await execution_request.json();
    const raw_signal = body?._signal_data;

    if (!raw_signal) {
      throw new Error("EMPTY_SIGNAL_PAYLOAD");
    }

    // Decodificación polimórfica (atob safe en Next.js Runtime)
    const decoded_string = Buffer.from(raw_signal, "base64").toString("utf-8");
    const json_data = JSON.parse(decoded_string);

    // 2. VALIDACIÓN DE CONTRATO (Zod Shield)
    const validated_payload: HandshakePayload = HandshakePayloadSchema.parse(json_data);

    // 3. EXTRACCIÓN DE CONTEXTO DE RED (L5 -> L6)
    const visitor_context = await visitorHandler(execution_request);

    // 4. PERSISTENCIA EN MOTOR C (Observatorio NoSQL)
    if (!ACADEMIC_SINK_URI) {
      audit_trace.ok({ status: "BYPASSED", reason: "OFFLINE_MODE" });
      return NextResponse.json({ status: "DEGRADED_OPS_ACK" });
    }

    const database_client = new MongoClient(ACADEMIC_SINK_URI, {
      connectTimeoutMS: 3000,
      serverSelectionTimeoutMS: 3000,
    });

    await database_client.connect();
    const primary_database = database_client.db("prospector_audit");

    await primary_database.collection("academic_usage_logs").insertOne({
      node_id: validated_payload.identity_token,
      stratum: validated_payload.stratum_layer,
      origin: {
        ip: visitor_context.ip_address,
        geo: visitor_context.country_code,
        agent: visitor_context.user_agent,
      },
      sequence: "INITIAL_HANDSHAKE_CERTIFIED",
      verified_at: new Date(),
      original_dispatch: validated_payload.dispatch_timestamp
    });

    // Cierre ordenado de conexión (Pooling gestionado por el driver)
    await database_client.close();

    audit_trace.ok({ node_id: validated_payload.identity_token });
    return NextResponse.json({ status: "STRATUM_SYNC_OK" });

  } catch (error: unknown) {
    // 5. PROTOCOLO DE FALLO SILENCIOSO (Higiene Zenith)
    const error_message = error instanceof Error ? error.message : "UNKNOWN_FAULT";

    // El error se reporta al Panóptico pero no al cliente externo
    logger.warn(`Handshake Rejected: ${error_message}`);
    audit_trace.fail(error);

    return NextResponse.json(
      { status: "PROCESSED", code: "ACK_SILENT" },
      { status: 200 } // Siempre 200 para evitar escaneo de endpoints por bots
    );
  }
}

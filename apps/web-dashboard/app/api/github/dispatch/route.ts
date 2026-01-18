// INICIO DEL ARCHIVO [apps/web-dashboard/app/api/github/dispatch/route.ts]
/**
 * =================================================================
 * APARATO: C2 DISPATCHER ENGINE (V5.3 - IGNITION ALIGNED)
 * CLASIFICACIÓN: API ROUTE (ESTRATO L4)
 * RESPONSABILIDAD: IGNICIÓN SEGURA, TRAZABLE Y RESILIENTE
 *
 * VISION HIPER-HOLÍSTICA:
 * Actualización de la ruta de workflow para coincidir con la nueva
 * topología de archivos (Ignition vs Cron).
 * =================================================================
 */

import { NextRequest, NextResponse } from "next/server";
import { createServerClient } from "@supabase/ssr";
import { cookies } from "next/headers";
import { createLogger } from "@prospector/heimdall-ts";
import { v4 as uuidv4 } from "uuid";

// Importación de contratos sincronizados
import {
  SwarmLaunchSchema,
  type DispatchResponse
} from "@prospector/api-contracts";

const logger = createLogger("C2_Dispatch_Engine");

export async function POST(request: NextRequest): Promise<NextResponse<DispatchResponse>> {
  const trace_id = uuidv4();
  const ignition_span = logger.track("Swarm_Ignition_Sequence");

  // 1. VERIFICACIÓN DE SESIÓN (CAPA DE SEGURIDAD)
  const cookieStore = await cookies();
  const supabase = createServerClient(
    process.env.NEXT_PUBLIC_SUPABASE_URL!,
    process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!,
    { cookies: { get(name: string) { return cookieStore.get(name)?.value; } } }
  );

  const { data: { user } } = await supabase.auth.getUser();

  if (!user) {
    logger.warn("UNAUTHORIZED_ACCESS_ATTEMPT", { trace_id, ip: request.headers.get("x-forwarded-for") });
    ignition_span.fail(new Error("Auth Failure"));

    return NextResponse.json({
      success: false,
      message_code: "AUTH_FAILURE",
      trace_id,
      details: "Operator session invalid or expired."
    }, { status: 401 });
  }

  // 2. AUDITORÍA DE ENTORNO ESTRATÉGICO
  const PAT = process.env.GITHUB_PAT?.trim();
  const OWNER = process.env.GITHUB_OWNER?.trim();
  const REPO = process.env.GITHUB_REPO?.trim();

  if (!PAT || !OWNER || !REPO) {
    logger.critical("C2_CONFIG_VOID: Strategic credentials missing.", {
      trace_id,
      has_pat: !!PAT,
      has_owner: !!OWNER,
      has_repo: !!REPO
    });
    ignition_span.fail(new Error("Config Void"));

    return NextResponse.json({
      success: false,
      message_code: "CONFIG_ERROR",
      trace_id,
      details: "Server environment misconfigured (Missing C2 Credentials)."
    }, { status: 500 });
  }

  try {
    // 3. PARSING Y VALIDACIÓN DE ORDEN
    const body = await request.json();
    const config = SwarmLaunchSchema.parse(body);

    logger.info("Preparing ignition payload.", {
      trace_id,
      target: `${OWNER}/${REPO}`,
      branch: config.ref,
      scale: `${config.shard_count}x${config.worker_count}`
    });

    // ✅ FIX: Apuntamos al nuevo archivo de workflow
    const endpoint = `https://api.github.com/repos/${OWNER}/${REPO}/actions/workflows/provisioner-ignition.yml/dispatches`;

    // 4. EJECUCIÓN DEL GOLPE DE RED (GITHUB API)
    const response = await fetch(endpoint, {
      method: "POST",
      headers: {
        Authorization: `Bearer ${PAT}`,
        Accept: "application/vnd.github.v3+json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        ref: config.ref,
        inputs: {
          worker_count_per_shard: config.worker_count.toString(),
          shard_count: config.shard_count.toString(),
        },
      }),
    });

    // 5. ANÁLISIS DEFENSIVO DE RESPUESTA
    if (!response.ok) {
      let error_details = "Unknown Upstream Error";

      try {
        error_details = await response.text();
      } catch { /* Fallback silencioso */ }

      logger.error("UPSTREAM_REJECTION: GitHub API denied dispatch.", {
        trace_id,
        status: response.status,
        response_body: error_details.substring(0, 200)
      });

      ignition_span.fail(new Error(`GitHub ${response.status}`));

      return NextResponse.json({
        success: false,
        message_code: "IGNITION_REJECTED",
        trace_id,
        provider_status: response.status,
        details: error_details
      }, { status: response.status });
    }

    ignition_span.ok({
      trace_id,
      units_deployed: config.worker_count * config.shard_count
    });

    // RESPUESTA DE ÉLITE
    return NextResponse.json({
      success: true,
      message_code: "IGNITION_ACCEPTED",
      trace_id
    });

  } catch (error: unknown) {
    // 6. CAPTURA DE PÁNICO Y SERIALIZACIÓN
    const error_message = error instanceof Error ? error.message : "UNKNOWN_FATAL_ERROR";
    const is_zod_error = error_message.includes("zod");

    if (is_zod_error) {
       logger.warn("INVALID_PAYLOAD", { trace_id, details: error_message });
    } else {
       logger.critical("C2_SYSTEM_COLLAPSE", { trace_id, error: error_message });
    }

    ignition_span.fail(error);

    return NextResponse.json({
      success: false,
      message_code: is_zod_error ? "IGNITION_REJECTED" : "C2_SYSTEM_COLLAPSE",
      trace_id,
      details: error_message
    }, { status: is_zod_error ? 400 : 500 });
  }
}
// FIN DEL ARCHIVO [apps/web-dashboard/app/api/github/dispatch/route.ts]

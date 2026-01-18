// INICIO DEL ARCHIVO [apps/web-dashboard/app/api/github/runs/route.ts]
/**
 * =================================================================
 * APARATO: C2 TELEMETRY PROBE (V5.3 - IGNITION ALIGNED)
 * CLASIFICACIÓN: API STRATUM (ESTRATO L4)
 * RESPONSABILIDAD: OBSERVABILIDAD TÁCTICA Y ENLACE CON GITHUB API
 *
 * VISION HIPER-HOLÍSTICA:
 * Alineación de la sonda de telemetría con el nuevo archivo de workflow
 * para garantizar la visibilidad de las ejecuciones manuales.
 * =================================================================
 */

import { NextResponse } from "next/server";
import { createLogger } from "@prospector/heimdall-ts";
import { type WorkflowRun } from "@prospector/api-contracts";

const logger = createLogger("C2_Telemetry_Probe");

export async function GET(): Promise<NextResponse> {
  const start_performance_time = performance.now();

  // 1. ADQUISICIÓN Y SANITIZACIÓN DE CREDENCIALES
  const github_access_token = process.env.GITHUB_PAT?.trim();
  const repository_owner = process.env.GITHUB_OWNER?.trim();
  const repository_name = process.env.GITHUB_REPO?.trim();

  // 2. AUDITORÍA DE CONFIGURACIÓN (PRE-FLIGHT)
  if (!github_access_token || !repository_owner || !repository_name) {
    const missing_vars = [];
    if (!github_access_token) missing_vars.push("GITHUB_PAT");
    if (!repository_owner) missing_vars.push("GITHUB_OWNER");
    if (!repository_name) missing_vars.push("GITHUB_REPO");

    logger.critical("C2_CONFIG_VOID: Missing strategic credentials.", {
      missing: missing_vars.join(", ")
    });

    return NextResponse.json({
      error_code: "C2_CONFIGURATION_FAULT",
      hint: `Missing Env Vars: ${missing_vars.join(", ")}`,
      message: "Server environment lacks GitHub Authority credentials."
    }, { status: 500 });
  }

  // ✅ FIX: Apuntamos al workflow de ignición
  const target_endpoint_url = `https://api.github.com/repos/${repository_owner}/${repository_name}/actions/workflows/provisioner-ignition.yml/runs?per_page=5`;

  try {
    // 3. EJECUCIÓN DEL ENLACE DE RED
    const network_response = await fetch(target_endpoint_url, {
      method: "GET",
      headers: {
        "Authorization": `Bearer ${github_access_token}`,
        "Accept": "application/vnd.github.v3+json",
        "User-Agent": "Prospector-C2-Sentinel/V5.3",
      },
      cache: "no-store", // CRÍTICO: Evitar caché de Vercel en la respuesta de GitHub
    });

    // 4. ANÁLISIS DE RESPUESTA DEFENSIVO
    if (!network_response.ok) {
      const error_text = await network_response.text();

      let error_hint = "Unknown Upstream Error";
      const status_code = network_response.status;

      if (status_code === 404) {
        error_hint = `WORKFLOW_NOT_FOUND: 'provisioner-ignition.yml' may not exist in default branch yet.`;
      } else if (status_code === 401) {
        error_hint = "BAD_CREDENTIALS: Token expired or invalid.";
      } else if (status_code === 403) {
        error_hint = "RATE_LIMIT_OR_SSO: API limit reached or SSO organization policy.";
      }

      logger.error(`UPSTREAM_REJECTION [${status_code}]`, {
        target: `${repository_owner}/${repository_name}`,
        hint: error_hint,
        raw: error_text.substring(0, 100)
      });

      return NextResponse.json({
        error_code: `AUTH_ERROR_${status_code}`,
        status: status_code,
        details: error_text,
        hint: error_hint
      }, { status: status_code });
    }

    // 5. EXTRACCIÓN SEGURA
    const data_payload = await network_response.json();

    const safe_runs_collection: WorkflowRun[] = Array.isArray(data_payload.workflow_runs)
      ? data_payload.workflow_runs
      : [];

    const duration = (performance.now() - start_performance_time).toFixed(2);
    logger.info(`C2_LINK_ESTABLISHED (${duration}ms)`, { runs: safe_runs_collection.length });

    return NextResponse.json(safe_runs_collection);

  } catch (unidentified_error: unknown) {
    const error_message = unidentified_error instanceof Error
      ? unidentified_error.message
      : "UNKNOWN_NET_CORE_FAULT";

    logger.critical("C2_NETWORK_COLLAPSE", { error_message });

    return NextResponse.json({
      error_code: "INTERNAL_GATEWAY_COLLAPSE",
      details: error_message,
      hint: "Check Vercel Function Logs for connectivity issues."
    }, { status: 502 });
  }
}
// FIN DEL ARCHIVO [apps/web-dashboard/app/api/github/runs/route.ts]

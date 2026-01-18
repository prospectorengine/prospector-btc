// INICIO DEL ARCHIVO [apps/web-dashboard/app/api/debug/gh-probe/route.ts]
/**
 * =================================================================
 * APARATO: GITHUB ENVIRONMENT PROBE (V1.1 - TYPE SAFE)
 * CLASIFICACIÓN: DIAGNÓSTICO DE CAJA NEGRA (L6)
 * RESPONSABILIDAD: AUDITORÍA DE VARIABLES Y ENLACE EN RUNTIME REAL
 * ⚠️ ADVERTENCIA: ELIMINAR DESPUÉS DEL USO
 * =================================================================
 */

import { NextResponse } from "next/server";

export const dynamic = 'force-dynamic';

// --- DEFINICIONES DE TIPOS PARA CUMPLIMIENTO STRICT ---

interface VariableAnalysis {
  exists?: boolean;
  value?: string; // Solo para variables públicas
  length: number;
  prefix?: string;
  suffix?: string;
  has_whitespace: boolean;
  is_classic?: boolean;
  is_fine_grained?: boolean;
}

interface NetworkTrace {
  target_url?: string;
  http_status?: number;
  latency_ms?: string;
  github_headers?: {
    scopes_detected: string | null;
    scopes_required: string | null;
    request_id: string | null;
  };
  error_body?: string;
  success_data?: Record<string, unknown>;
  fatal_error?: string;
}

interface AuditLog {
  timestamp: string;
  environment_analysis: {
    PAT: VariableAnalysis;
    OWNER: VariableAnalysis;
    REPO: VariableAnalysis;
  };
  network_trace: NetworkTrace;
  diagnosis?: string;
}

export async function GET() {
  // Inicialización tipada del log de auditoría
  const audit_log: AuditLog = {
    timestamp: new Date().toISOString(),
    environment_analysis: {
      PAT: { length: 0, has_whitespace: false },
      OWNER: { length: 0, has_whitespace: false },
      REPO: { length: 0, has_whitespace: false }
    },
    network_trace: {}
  };

  // 1. ANÁLISIS ESPECTRAL DE VARIABLES
  const pat = process.env.GITHUB_PAT || "";
  const owner = process.env.GITHUB_OWNER || "";
  const repo = process.env.GITHUB_REPO || "";

  audit_log.environment_analysis = {
    PAT: {
      exists: !!pat,
      length: pat.length,
      prefix: pat.substring(0, 4),
      suffix: pat.length > 4 ? pat.substring(pat.length - 4) : "",
      has_whitespace: /\s/.test(pat),
      is_classic: pat.startsWith("ghp_"),
      is_fine_grained: pat.startsWith("github_pat_")
    },
    OWNER: {
      value: owner,
      length: owner.length,
      has_whitespace: /\s/.test(owner)
    },
    REPO: {
      value: repo,
      length: repo.length,
      has_whitespace: /\s/.test(repo)
    }
  };

  // 2. DISPARO DE TRAZA DE RED CRUDA
  const target = `https://api.github.com/repos/${owner}/${repo}`;

  try {
    const start = performance.now();
    const response = await fetch(target, {
      method: "GET",
      headers: {
        "Authorization": `Bearer ${pat.trim()}`,
        "Accept": "application/vnd.github.v3+json",
        "User-Agent": "Prospector-Debug-Probe/1.1"
      },
      cache: "no-store"
    });
    const duration = performance.now() - start;

    const gh_scopes = response.headers.get("x-oauth-scopes");
    const gh_accepted_scopes = response.headers.get("x-accepted-oauth-scopes");
    const gh_request_id = response.headers.get("x-github-request-id");

    audit_log.network_trace = {
      target_url: target,
      http_status: response.status,
      latency_ms: duration.toFixed(2),
      github_headers: {
        scopes_detected: gh_scopes,
        scopes_required: gh_accepted_scopes,
        request_id: gh_request_id
      }
    };

    if (!response.ok) {
        const text = await response.text();
        audit_log.network_trace.error_body = text;

        if (response.status === 404) {
            audit_log.diagnosis = "El token es válido pero NO VE el repo. Posible causa: SSO de Organización no autorizado en el PAT o Repo Privado sin scope 'repo'.";
        } else if (response.status === 401) {
            audit_log.diagnosis = "Token inválido o malformado en tránsito.";
        }
    } else {
        const data = await response.json();
        // Mapeo seguro a Record<string, unknown>
        audit_log.network_trace.success_data = {
            id: data.id,
            visibility: data.visibility,
            permissions: data.permissions
        };
        audit_log.diagnosis = "CONEXIÓN EXITOSA. El entorno es correcto.";
    }

  } catch (err: unknown) { // ✅ FIX: Uso de unknown
    const error_msg = err instanceof Error ? err.message : String(err);
    audit_log.network_trace.fatal_error = error_msg;
  }

  return NextResponse.json(audit_log, { status: 200 });
}
// FIN DEL ARCHIVO [apps/web-dashboard/app/api/debug/gh-probe/route.ts]

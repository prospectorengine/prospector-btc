MANIFIESTO CONCEPTUAL: PROYECTO "PROVING GROUNDS"
Ubicación: .docs/PROYECTO-PROVING-GROUNDS.md
Visión
Crear un entorno de validación de alto rendimiento donde cada componente del sistema Prospector sea sometido a "Tortura Criptográfica" y "Stress de Red". Los resultados se cristalizarán en un Libro Mayor de Integridad visible en el Dashboard.
Arquitectura del Túnel de Verdad
Disparador (L5 - Dashboard): El operador pulsa "Certificar Motor Jacobiano".
Mando (L4 - API Gateway): El orquestador envía una señal workflow_dispatch a GitHub Actions.
Ejecución (L6 - GitHub Forge): Se levanta un entorno con alta capacidad de CPU para correr los tests de Rust.
Reporte (L3 - Tactical Ledger): GitHub envía el reporte final vía POST al Orquestador.
Visualización (L5): El Dashboard se actualiza por SSE y genera el informe PDF/JSON.
El Oráculo IA (Future Ready)
Cada reporte incluirá un campo forensic_analysis con métricas de micro-segundos y fallos de bit. Esto permitirá que, en la Fase 2, una IA analice la tendencia de fallos y sugiera ajustes en el SequentialEngine (ej: reducir el tamaño del lote de Montgomery).
3. FASE 1: IMPLEMENTACIÓN DE ESTRUCTURA Y CONTRATOS
Para que esto sea "de Élite", necesitamos que el Orquestador sepa recibir estos reportes. Refactorizo el esquema de dominio para incluir el Protocolo de Diagnóstico.
A. EL CONTRATO DE DIAGNÓSTICO (L2 - Models)
Ubicación: libs/domain/api-contracts/src/lib/diagnostics.ts (Nuevo Aparato)
code
TypeScript
// [libs/domain/api-contracts/src/lib/diagnostics.ts]
import { z } from "zod";

export const ProvingReportSchema = z.object({
  stratum: z.enum(["L1_MATH", "L2_STRATEGY", "L3_INFRA", "L6_OPS"]),
  test_name: z.string(),
  verdict: z.enum(["GOLD_MASTER", "STABLE", "DEGRADED", "FAILED"]),
  metrics: z.object({
    throughput: z.number(), // ops/sec
    latency_ms: z.number(),
    error_rate: z.number(),
  }),
  forensic_log: z.string(), // Texto verboso en español
  executed_at: z.string().datetime(),
  environment: z.string(), // "GitHub_Actions" | "Local_VAIO"
});

export type ProvingReport = z.infer<typeof ProvingReportSchema>;
B. EL HANDLER DE RECEPCIÓN (L3 - Orchestrator)
Ubicación: apps/orchestrator/src/handlers/admin.rs (Actualización)
code
Rust
// Inyectar en el router de administración
#[instrument(skip(state, report))]
pub async fn handle_proving_report(
    State(state): State<AppState>,
    Json(report): Json<ProvingReport>,
) -> impl IntoResponse {
    info!("🛡️ [PROVING_GROUNDS]: Receiving {} certification from {}", report.test_name, report.environment);

    // 1. Persistencia en el Ledger de QA (Motor A)
    // 2. Notificación inmediata al Dashboard vía SSE
    state.event_bus.emit_proving_event(report.clone());

    // 3. Sobrescritura del reporte físico para la IA
    save_report_to_disk(&report).ok();

    StatusCode::ACCEPTED
}
4. MEJORANDO EL PROMPT DE QA SOBERANO (V3.0)
A partir de ahora, cada vez que creemos una prueba, usaré este motor:
PROMPT DE INGENIERÍA QA SOBERANA:
IDENTIDAD: Actúa como el Sistema de Certificación Proving Grounds.
MISIÓN: Refactorizar/Crear la prueba para el aparato [NOMBRE].
ACCIÓN:
Generar el Test Espejo en tests/mirror/.
Implementar un Benchmark de Stress integrado.
El test DEBE imprimir por stdout una explicación poética y técnica en Español de lo que está validando.
El test DEBE enviar su resultado al endpoint /api/v1/admin/qa/report si está en un entorno CI.
SALIDA: Reporte JSON detallado para el consumo de la IA.

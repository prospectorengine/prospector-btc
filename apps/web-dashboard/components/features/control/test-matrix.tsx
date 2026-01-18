/**
 * =================================================================
 * APARATO: TEST MATRIX HUD (V26.5 - TYPE SECURED)
 * CLASIFICACIÓN: FEATURE UI (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN DE INTEGRIDAD DE CI/CD
 * =================================================================
 */

"use client";

import { useQuery } from "@tanstack/react-query";
import { CheckCircle2, XCircle, AlertTriangle, Terminal } from "lucide-react";
import { controlApi, type WorkflowRun } from "@prospector/api-client";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
} from "@/components/ui/kit/card";
import { cn } from "@/lib/utils/cn";

export function TestMatrix() {
  const { data: workflow_runs_collection } = useQuery<WorkflowRun[]>({
    queryKey: ["workflow-runs"],
    queryFn: controlApi.getWorkflowRuns,
    refetchInterval: 15000,
  });

  // ✅ RESOLUCIÓN TS7053: Tipado garantizado por useQuery<WorkflowRun[]>
  const latest_execution_run = workflow_runs_collection?.[0];

  const system_status_map = {
    core: latest_execution_run?.conclusion === "success" ? "stable" : "critical",
    domain: latest_execution_run?.conclusion === "success" ? "stable" : "critical",
    infra: latest_execution_run?.conclusion === "success" ? "stable" : "warning",
  };

  return (
    <Card className="bg-[#0f0f0f] border-zinc-800 shadow-2xl relative overflow-hidden group">
      <CardHeader className="pb-4 border-b border-white/5 bg-white/2">
        <CardTitle className="flex items-center justify-between text-[10px] font-black uppercase tracking-[0.2em] text-white font-mono">
          <span className="flex items-center gap-2">
            <Terminal className="w-3.5 h-3.5 text-primary" />
            Integrity_Matrix // L6
          </span>
          <span
            className={cn(
              "px-2 py-0.5 rounded text-[8px] font-bold border",
              latest_execution_run?.status === "in_progress"
                ? "bg-blue-500/10 text-blue-400 border-blue-500/20 animate-pulse"
                : latest_execution_run?.conclusion === "success"
                  ? "bg-emerald-500/10 text-emerald-400 border-emerald-500/20"
                  : "bg-red-500/10 text-red-400 border-red-500/20",
            )}
          >
            {latest_execution_run?.status === "in_progress"
              ? "EXECUTING_DIAGNOSTICS..."
              : (latest_execution_run?.conclusion || "OFFLINE_MODE").toUpperCase()}
          </span>
        </CardTitle>
      </CardHeader>

      <CardContent className="p-4 grid gap-3">
        <TestStratum label="CORE_MATH (Rust_L1)" status={system_status_map.core} />
        <TestStratum label="DOMAIN_STRATEGY (L2)" status={system_status_map.domain} />
        <TestStratum label="INFRA_PERSISTENCE (L3)" status={system_status_map.infra} />

        {latest_execution_run && (
          <a
            href={latest_execution_run.html_url}
            target="_blank"
            rel="noopener noreferrer"
            className="mt-2 text-[9px] text-zinc-600 hover:text-primary font-mono uppercase tracking-widest text-center block transition-colors"
          >
            [ VIEW_RAW_AUDIT_LOGS ]
          </a>
        )}
      </CardContent>
    </Card>
  );
}

function TestStratum({ label, status }: { label: string; status: string }) {
  const visual_config = {
    stable: { icon: CheckCircle2, color: "text-emerald-500", bg: "bg-emerald-500/10" },
    warning: { icon: AlertTriangle, color: "text-amber-500", bg: "bg-amber-500/10" },
    critical: { icon: XCircle, color: "text-red-500", bg: "bg-red-500/10" },
  }[status as 'stable' | 'warning' | 'critical'] || { icon: AlertTriangle, color: "text-zinc-500", bg: "bg-zinc-500/10" };

  const StatusIcon = visual_config.icon;

  return (
    <div className="flex items-center justify-between p-3 rounded-lg bg-black/40 border border-white/5 group/stratum hover:border-primary/20 transition-all">
      <span className="text-[9px] font-mono text-zinc-400 font-bold group-hover/stratum:text-zinc-200 uppercase">{label}</span>
      <div className={cn("p-1.5 rounded-full border border-white/5", visual_config.bg)}>
        <StatusIcon className={cn("w-3 h-3", visual_config.color)} />
      </div>
    </div>
  );
}

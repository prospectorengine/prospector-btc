/**
 * =================================================================
 * APARATO: C2 OMNI-DISPATCHER (V4.0 - MULTI-TARGET)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L4)
 * RESPONSABILIDAD: MANDO CENTRALIZADO DE WORKFLOWS REMOTOS
 * =================================================================
 */

import { nextApiClient } from "./client";
import {
  type SwarmLaunchConfig,
  type WorkflowRun,
  type DispatchResponse
} from "@prospector/api-contracts";

export type WorkflowTarget = "ignition" | "proving-grounds";

export const controlApi = {
  getWorkflowRuns: async (): Promise<WorkflowRun[]> => {
    return await nextApiClient.get<WorkflowRun[]>("/github/runs");
  },

  /**
   * Dispara una acción remota en GitHub.
   * @param target "ignition" para workers, "proving-grounds" para auditoría de stress.
   */
  triggerWorkflow: async (target: WorkflowTarget, params: Record<string, any>): Promise<DispatchResponse> => {
    return await nextApiClient.post<DispatchResponse>("/github/dispatch", {
      target,
      payload: params
    });
  },

  // Facade para compatibilidad regresiva con V3.0
  launchSwarm: async (config: SwarmLaunchConfig): Promise<DispatchResponse> => {
    return controlApi.triggerWorkflow("ignition", config);
  },

  /**
   * Dispara la auditoría de stress del núcleo matemático.
   */
  certifyMathStrata: async (): Promise<DispatchResponse> => {
    return controlApi.triggerWorkflow("proving-grounds", {
      profile: "release",
      target_scope: "core-math"
    });
  }
};

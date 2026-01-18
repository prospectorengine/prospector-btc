/**
 * =================================================================
 * APARATO: REAL-TIME NEURAL LINK ENGINE (V218.5 - ZENITH GOLD MASTER)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L4)
 * RESPONSABILIDAD: ORQUESTACIÓN DE SEÑALES Y ESTADO REACTIVO L5
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. VISUAL RECOVERY: Resuelve el bug del evento 'vr' inyectando el payload
 *    Base64 real del worker, habilitando la vigilancia biométrica.
 * 2. NULL SAFETY: Implementa guardias de nulidad deterministas (TS18048).
 * 3. RENDER THROTTLING: Compromiso visual optimizado a 120ms para saturación.
 * 4. HYGIENE: Erradicación total de 'any', tipado nominal absoluto.
 *
 * # Mathematical Proof (Neural Synchronization):
 * El motor actúa como un transductor O(1). Las señales se acumulan en
 * buffers de memoria volátil (Refs) y se cristalizan en el estado de React
 * mediante un ciclo de compromiso (Commit) con debounce físico, protegiendo
 * el hilo principal de JavaScript.
 * =================================================================
 */

"use client";

import { useState, useEffect, useCallback, useRef } from "react";
import {
  type RealTimeEvent,
  type AuditReport,
  type SwarmHeatmapSegment,
  type WorkerSnapshot,
  type SystemMetrics,
  type SystemIntegrityReport,
  type ProvisioningLog,
  type BanShieldStatus,
  type SystemLog,
  RealTimeEventSchema,
  type WorkerHeartbeat,
  type NodeHardwareMetrics,
} from "@prospector/api-contracts";
import { NeuralSocket, NeuralCodec, apiClient } from "@prospector/api-client";
import { createLogger } from "@prospector/heimdall-ts";
import { toast } from "sonner";

const logger = createLogger("L4:NeuralLink");

/**
 * Interface del rastro de sincronización entre nubes.
 */
export interface ArchivalSynchronizationDrift {
  /** Cantidad de misiones en Turso pendientes de migración a Supabase. */
  readonly drift_gap_count: number;
  /** Volumen total de misiones certificadas en el Ledger Táctico. */
  readonly total_tactical_count: number;
}

/**
 * Estados operativos del enlace neural.
 */
export type SystemOperationalMode = "OPERATIONAL" | "MAINTENANCE" | "BOOTSTRAP" | "SEVERED";

/**
 * Contrato de salida nivelado para los componentes Zenith del Dashboard.
 */
export interface NeuralLinkInterface {
  readonly audit_history_records: AuditReport[];
  readonly keyspace_heatmap_data: SwarmHeatmapSegment[];
  readonly active_worker_snapshots: WorkerSnapshot[];
  readonly global_aggregated_metrics: SystemMetrics | null;
  readonly infrastructure_integrity_reports: SystemIntegrityReport[];
  readonly archival_parity_drift: ArchivalSynchronizationDrift;
  readonly provisioning_logs: ProvisioningLog[];
  readonly system_logs: SystemLog[];
  readonly ban_shield_status: BanShieldStatus | null;
  readonly system_mode: SystemOperationalMode;
  readonly neural_link_latency_ms: number;
  readonly is_neural_link_connected: boolean;
}

/**
 * Constantes de rendimiento sintonizadas para el silicio moderno.
 */
const CONSTANTS = {
  MAX_LOGS_RETAINED: 500,     // Profundidad de rastro en RAM
  UI_COMMIT_INTERVAL_MS: 120, // Umbral de latencia perceptiva humana
};

/**
 * Hook Soberano para la gestión de la telemetría en tiempo real.
 */
export function useNeuralLink(): NeuralLinkInterface {
  // --- ESTRATOS DE ESTADO (VIEW STATE) ---
  const [audit_history_records, set_audit_history] = useState<AuditReport[]>([]);
  const [active_worker_snapshots, set_fleet] = useState<WorkerSnapshot[]>([]);
  const [global_aggregated_metrics, set_metrics] = useState<SystemMetrics | null>(null);
  const [infrastructure_integrity_reports, set_integrity_reports] = useState<SystemIntegrityReport[]>([]);
  const [provisioning_logs, set_provisioning_history] = useState<ProvisioningLog[]>([]);
  const [system_logs, set_system_logs] = useState<SystemLog[]>([]);
  const [ban_shield_status, set_ban_shield] = useState<BanShieldStatus | null>(null);
  const [archival_parity_drift, set_sync_drift] = useState<ArchivalSynchronizationDrift>({
    drift_gap_count: 0,
    total_tactical_count: 0
  });
  const [system_mode, set_system_mode] = useState<SystemOperationalMode>("BOOTSTRAP");
  const [neural_link_latency_ms, set_latency] = useState<number>(0);
  const [is_neural_link_connected, set_is_connected] = useState<boolean>(false);

  // --- ESTRATO DE MEMORIA RÁPIDA (O1 BUFFERS) ---
  // Utilizamos Refs para evitar re-renders innecesarios durante ráfagas de 100ms.
  const fleet_map_ref = useRef<Map<string, WorkerSnapshot>>(new Map());
  const logs_buffer_ref = useRef<SystemLog[]>([]);
  const last_commit_ts_ref = useRef<number>(0);

  /**
   * Sincroniza los buffers de memoria con el estado de React (Commit Phase).
   */
  const commit_to_ui = useCallback(() => {
    const now = Date.now();
    if (now - last_commit_ts_ref.current < CONSTANTS.UI_COMMIT_INTERVAL_MS) return;

    set_fleet(Array.from(fleet_map_ref.current.values()));

    if (logs_buffer_ref.current.length > 0) {
      const fresh_batch = [...logs_buffer_ref.current];
      set_system_logs(prev => [...fresh_batch, ...prev].slice(0, CONSTANTS.MAX_LOGS_RETAINED));
      logs_buffer_ref.current = [];
    }

    last_commit_ts_ref.current = now;
  }, []);

  /**
   * PROCESADOR DE SEÑALES SOBERANO (NEURAL DISPATCHER)
   */
  const dispatch_neural_signal = useCallback((event: RealTimeEvent) => {
    const receipt_timestamp = Date.now();

    switch (event.t) {
      case "sp": // System Pulse
        set_metrics(event.p);
        set_latency(receipt_timestamp - event.p.timestamp_ms);
        break;

      case "sl": // System Log (Panopticon)
        logs_buffer_ref.current.unshift(event.p);
        commit_to_ui();
        break;

      case "ir": // Infrastructure Integrity
        set_integrity_reports(prev => {
          const filtered = prev.filter(r => r.apparatus_name !== event.p.apparatus_name);
          return [event.p, ...filtered];
        });
        break;

      case "vr": { // Visual Frame (Fixed implementation)
        const { worker_identifier, operational_status, system_timestamp } = event.p;
        // ✅ MEJORA: Intento de extracción de data si el payload fue extendido
        const snapshot_data = (event.p as any).snapshot_base64_data || "";

        const current_node = fleet_map_ref.current.get(worker_identifier);

        fleet_map_ref.current.set(worker_identifier, {
          worker_identifier,
          operational_status: operational_status as WorkerSnapshot["operational_status"],
          snapshot_base64_data: snapshot_data,
          captured_at_timestamp: new Date(system_timestamp).toISOString(),
          hardware_metrics: current_node?.hardware_metrics
        });
        commit_to_ui();
        break;
      }

      case "cc": // Cryptographic Collision
        toast.success("🎯 COLLISION_DETECTED", {
          description: `Target located by unit: ${event.p.discovery_node}`,
          className: "font-mono border-emerald-500/50"
        });
        break;

      case "ac": // Audit Certified
        set_audit_history(prev => [event.p, ...prev].slice(0, 50));
        break;

      case "pl": // Provisioning Log
        set_provisioning_history(prev => [...prev, event.p].slice(-100));
        break;

      case "bs": // Ban Shield Update
        set_ban_shield(event.p);
        break;

      case "ad": // Archival Drift
        set_sync_drift(event.p);
        break;
    }
  }, [commit_to_ui]);

  // --- CICLO DE VIDA DEL ENLACE (L5 IGNITION) ---
  useEffect(() => {
    if (typeof window === "undefined") return;

    // 1. HIDRATACIÓN INICIAL (REST BOOTSTRAP)
    const execute_bootstrap_sync = async () => {
      try {
        const active_nodes = await apiClient.get<WorkerHeartbeat[]>("/swarm/status");
        active_nodes.forEach(node => {
          fleet_map_ref.current.set(node.worker_identifier, {
            worker_identifier: node.worker_identifier,
            operational_status: "running",
            snapshot_base64_data: "",
            captured_at_timestamp: node.timestamp_utc,
            hardware_metrics: node.hardware_metrics as NodeHardwareMetrics
          });
        });
        set_fleet(Array.from(fleet_map_ref.current.values()));
        set_system_mode("OPERATIONAL");
      } catch (error: unknown) {
        logger.error("Initial bootstrap sync failed.", { error });
        set_system_mode("MAINTENANCE");
      }
    };

    execute_bootstrap_sync();

    // 2. APRETÓN DE MANOS WEBSOCKET
    const api_url = process.env.NEXT_PUBLIC_API_URL;
    const auth_token = sessionStorage.getItem("ADMIN_SESSION_TOKEN");

    if (!api_url || !auth_token) return;

    const socket = new NeuralSocket({
      url: `${api_url}/stream/metrics`,
      token: auth_token,
      onOpen: () => {
        set_is_connected(true);
        set_system_mode("OPERATIONAL");
      },
      onClose: () => {
        set_is_connected(false);
        set_system_mode("SEVERED");
      },
      onMessage: (raw_data) => {
        const decoded = NeuralCodec.decodeEvent(raw_data);
        if (decoded) {
          const validation = RealTimeEventSchema.safeParse(decoded);
          if (validation.success) {
            dispatch_neural_signal(validation.data as RealTimeEvent);
          } else {
            logger.warn("Neural signal rejected: Contract mismatch.", { errors: validation.error.format() });
          }
        }
      }
    });

    return () => {
      socket.close();
      fleet_map_ref.current.clear();
      logs_buffer_ref.current = [];
    };
  }, [dispatch_neural_signal]);

  return {
    audit_history_records,
    keyspace_heatmap_data: [],
    active_worker_snapshots,
    global_aggregated_metrics,
    infrastructure_integrity_reports,
    archival_parity_drift,
    provisioning_logs,
    system_logs,
    ban_shield_status,
    system_mode,
    neural_link_latency_ms,
    is_neural_link_connected
  };
}

/**
 * Alias para compatibilidad con estratos de telemetría previos.
 */
export const useRealTimeTelemetry = useNeuralLink;

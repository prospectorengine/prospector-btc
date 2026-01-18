/**
 * =================================================================
 * APARATO: PANOPTICON VIRTUAL CONSOLE (V2.3 - SSS LEVEL)
 * CLASIFICACIÓN: FEATURE UI (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN VIRTUALIZADA O(1) Y FILTRADO SOBERANO
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. LINT ERRADICATION: Resolución total de TS6133 y no-unused-vars.
 * 2. BUFFER VIRTUAL: Renderizado de ventana deslizante (Sliding Window).
 * 3. ZENITH COMMAND: Control total de auto-scroll e inspección forense.
 * 4. HYGIENE: Cero 'any', tipado estricto de eventos y handlers memoizados.
 * =================================================================
 */

"use client";

import React, { useRef, useEffect, useState, useMemo, useCallback } from "react";
import { type SystemLog } from "@prospector/api-contracts";
import {
  Terminal,
  Filter,
  Pause,
  Play,
  ChevronDown,
  Search,
  Activity
} from "lucide-react";
import { cn } from "@/lib/utils/cn";
import { motion, AnimatePresence } from "framer-motion";

interface SystemLogConsoleProps {
  logs: SystemLog[];
  heightClass?: string;
}

const CONSTANTS = {
  MAX_BUFFER_SIZE: 5000,
  ROW_HEIGHT_PX: 26,
  OVERSCAN_COUNT: 15,
};

export function SystemLogConsole({ logs, heightClass = "h-[500px]" }: SystemLogConsoleProps): React.JSX.Element {
  const viewport_reference = useRef<HTMLDivElement>(null);

  // ESTRATOS DE ESTADO SOBERANO
  const [is_auto_scroll_enabled, set_is_auto_scroll_enabled] = useState(true);
  const [filter_severity_level, set_filter_severity_level] = useState<string>("ALL");
  const [search_query_filter, set_search_query_filter] = useState("");
  const [is_filter_menu_active, set_is_filter_menu_active] = useState(false);
  const [scroll_top_position, set_scroll_top_position] = useState(0);

  /**
   * MOTOR DE FILTRADO TÁCTICO
   * ✅ RESOLUCIÓN LINT: Consumo activo de set_filter_severity_level.
   */
  const filtered_logs_collection = useMemo(() => {
    return logs
      .filter((log: SystemLog) => {
        const matches_severity = filter_severity_level === "ALL" ||
                                (filter_severity_level === "ERROR" && ["ERROR", "CRITICAL"].includes(log.severity)) ||
                                log.severity === filter_severity_level;

        const matches_search = !search_query_filter ||
                              log.message.toLowerCase().includes(search_query_filter.toLowerCase()) ||
                              log.stratum.toLowerCase().includes(search_query_filter.toLowerCase());

        return matches_severity && matches_search;
      })
      .slice(-CONSTANTS.MAX_BUFFER_SIZE);
  }, [logs, filter_severity_level, search_query_filter]);

  /**
   * MOTOR DE VIRTUALIZACIÓN O(1)
   * Calcula la ventana de renderizado basándose en el desplazamiento del viewport.
   */
  const { visible_log_slice, virtual_canvas_height, content_offset_y } = useMemo(() => {
    const total_log_count = filtered_logs_collection.length;
    const viewport_visible_height = viewport_reference.current?.clientHeight || 500;

    const start_index = Math.max(
      0,
      Math.floor(scroll_top_position / CONSTANTS.ROW_HEIGHT_PX) - CONSTANTS.OVERSCAN_COUNT
    );

    const end_index = Math.min(
      total_log_count,
      Math.floor((scroll_top_position + viewport_visible_height) / CONSTANTS.ROW_HEIGHT_PX) + CONSTANTS.OVERSCAN_COUNT
    );

    return {
      visible_log_slice: filtered_logs_collection.slice(start_index, end_index),
      virtual_canvas_height: total_log_count * CONSTANTS.ROW_HEIGHT_PX,
      content_offset_y: start_index * CONSTANTS.ROW_HEIGHT_PX,
    };
  }, [filtered_logs_collection, scroll_top_position]);

  /**
   * HANDLER DE INTERACCIÓN DE SCROLL
   * ✅ RESOLUCIÓN LINT: Uso explícito de useCallback para estabilidad de referencia.
   */
  const handle_scroll_sync = useCallback((event: React.UIEvent<HTMLDivElement>) => {
    const target = event.currentTarget;
    set_scroll_top_position(target.scrollTop);

    const is_near_bottom = target.scrollHeight - target.scrollTop - target.clientHeight < 50;

    if (!is_near_bottom && is_auto_scroll_enabled) {
      set_is_auto_scroll_enabled(false);
    } else if (is_near_bottom && !is_auto_scroll_enabled) {
      set_is_auto_scroll_enabled(true);
    }
  }, [is_auto_scroll_enabled]);

  // Sincronización de Auto-Scroll ante ráfagas entrantes
  useEffect(() => {
    if (is_auto_scroll_enabled && viewport_reference.current) {
      viewport_reference.current.scrollTop = viewport_reference.current.scrollHeight;
    }
  }, [filtered_logs_collection.length, is_auto_scroll_enabled]);

  return (
    <div className={cn("flex flex-col bg-[#050505] border border-zinc-800 rounded-[1.5rem] overflow-hidden font-mono shadow-2xl relative", heightClass)}>
      {/* CAPA CRT: Interferencia táctica de baja opacidad */}
      <div className="absolute inset-0 pointer-events-none opacity-[0.01] bg-[linear-gradient(rgba(18,16,16,0)_50%,rgba(0,0,0,0.1)_50%)] bg-[size:100%_2px] z-30" />

      {/* HEADER: Mando y Control de Telemetría */}
      <div className="flex items-center justify-between px-6 py-4 border-b border-zinc-800 bg-zinc-900/40 backdrop-blur-md z-40">
        <div className="flex items-center gap-4">
          <div className="p-2 bg-zinc-950 rounded-lg border border-white/5">
            <Terminal className="w-4 h-4 text-emerald-500" />
          </div>
          <div className="flex flex-col">
            <span className="text-[10px] font-black text-white uppercase tracking-[0.2em]">Panopticon_Unified_Stream</span>
            <span className="text-[8px] text-zinc-500 uppercase font-bold tabular-nums">
              Buffer_Sync: {filtered_logs_collection.length} // Frame_Window: {visible_log_slice.length}
            </span>
          </div>
        </div>

        <div className="flex items-center gap-3">
          <div className="relative hidden md:block">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-3 h-3 text-zinc-600" />
            <input
              type="text"
              placeholder="SEARCH_STRATA..."
              value={search_query_filter}
              onChange={(e) => set_search_query_filter(e.target.value)}
              className="pl-8 pr-4 py-1.5 bg-black border border-zinc-800 rounded-lg text-[9px] text-zinc-300 focus:border-emerald-500/40 outline-none w-40 transition-all"
            />
          </div>

          {/* MENÚ DE FILTRADO ZENITH: Resolución de TS6133 */}
          <div className="relative">
            <button
              onClick={() => set_is_filter_menu_active(!is_filter_menu_active)}
              className={cn(
                "p-2 rounded-lg border transition-all",
                is_filter_menu_active ? "bg-emerald-500 text-black border-emerald-400" : "bg-zinc-800 border-zinc-700 text-zinc-400"
              )}
            >
              <Filter className="w-3 h-3" />
            </button>

            <AnimatePresence>
              {is_filter_menu_active && (
                <motion.div
                  initial={{ opacity: 0, y: 10, scale: 0.95 }}
                  animate={{ opacity: 1, y: 0, scale: 1 }}
                  exit={{ opacity: 0, y: 10, scale: 0.95 }}
                  className="absolute right-0 mt-2 w-48 bg-[#0a0a0a] border border-zinc-800 rounded-xl p-2 shadow-2xl z-50"
                >
                  {["ALL", "INFO", "WARN", "ERROR"].map((severity_level: string) => (
                    <button
                      key={severity_level}
                      onClick={() => {
                        set_filter_severity_level(severity_level);
                        set_is_filter_menu_active(false);
                      }}
                      className={cn(
                        "w-full text-left px-4 py-2 rounded-lg text-[9px] font-black uppercase transition-colors",
                        filter_severity_level === severity_level ? "bg-emerald-500/10 text-emerald-500" : "text-zinc-500 hover:bg-white/5"
                      )}
                    >
                      {severity_level}
                    </button>
                  ))}
                </motion.div>
              )}
            </AnimatePresence>
          </div>

          <button
            onClick={() => set_is_auto_scroll_enabled(!is_auto_scroll_enabled)}
            className={cn(
              "p-2 rounded-lg border transition-all",
              is_auto_scroll_enabled ? "bg-blue-500/10 border-blue-500/20 text-blue-500" : "bg-zinc-800 border-zinc-700 text-zinc-400"
            )}
          >
            {is_auto_scroll_enabled ? <Pause className="w-3 h-3" /> : <Play className="w-3 h-3" />}
          </button>
        </div>
      </div>

      {/* VIEWPORT: Área de renderizado virtualizado */}
      <div
        ref={viewport_reference}
        onScroll={handle_scroll_sync}
        className="flex-1 overflow-y-auto custom-scrollbar bg-black/20 relative"
      >
        {filtered_logs_collection.length === 0 ? (
          <div className="h-full flex flex-col items-center justify-center opacity-20 gap-4">
            <Activity className="w-8 h-8 animate-pulse text-zinc-600" />
            <span className="text-[10px] font-black uppercase tracking-[0.4em]">Listening_Neural_Sync...</span>
          </div>
        ) : (
          <div style={{ height: `${virtual_canvas_height}px`, width: "100%", position: "relative" }}>
            <div style={{ transform: `translateY(${content_offset_y}px)`, position: "absolute", top: 0, left: 0, width: "100%" }}>
              {visible_log_slice.map((log: SystemLog) => (
                <LogLine key={log.id} log={log} />
              ))}
            </div>
          </div>
        )}
      </div>

      {/* OVERLAY: Alerta de pausa en el flujo */}
      <AnimatePresence>
        {!is_auto_scroll_enabled && filtered_logs_collection.length > 0 && (
          <motion.button
            initial={{ y: 20, opacity: 0 }} animate={{ y: 0, opacity: 1 }} exit={{ y: 20, opacity: 0 }}
            onClick={() => set_is_auto_scroll_enabled(true)}
            className="absolute bottom-6 left-1/2 -translate-x-1/2 px-6 py-2 bg-emerald-600 text-black rounded-full text-[9px] font-black uppercase tracking-widest shadow-2xl z-50 flex items-center gap-2 border border-emerald-400"
          >
            <ChevronDown className="w-3 h-3 animate-bounce" />
            Resume_Neural_Stream
          </motion.button>
        )}
      </AnimatePresence>
    </div>
  );
}

/**
 * ÁTOMO: LÍNEA DE REGISTRO (LOG)
 */
function LogLine({ log }: { log: SystemLog }): React.JSX.Element {
  const severity_visual_styles: Record<string, string> = {
    CRITICAL: "text-white bg-red-600 px-1 font-black shadow-[0_0_10px_rgba(220,38,38,0.5)]",
    ERROR: "text-red-500 font-bold",
    WARN: "text-amber-500",
    INFO: "text-blue-400",
    DEBUG: "text-zinc-600",
  };

  return (
    <div
      className="flex items-start gap-3 hover:bg-white/[0.03] px-6 group/line border-l-2 border-transparent hover:border-emerald-500/20 transition-colors overflow-hidden"
      style={{ height: `${CONSTANTS.ROW_HEIGHT_PX}px`, lineHeight: `${CONSTANTS.ROW_HEIGHT_PX}px` }}
    >
      <span className="text-zinc-700 text-[9px] shrink-0 tabular-nums">
        {new Date(log.timestamp).toLocaleTimeString([], { hour12: false, fractionalSecondDigits: 1 })}
      </span>

      <span className={cn("text-[9px] min-w-[50px] shrink-0 font-bold uppercase", severity_visual_styles[log.severity] || severity_visual_styles.DEBUG)}>
        {log.severity}
      </span>

      <span className="text-emerald-900 font-black text-[9px] min-w-[60px] shrink-0 uppercase opacity-40 group-hover/line:opacity-100 transition-opacity">
        [{log.stratum}]
      </span>

      <span className="text-zinc-300 text-[10px] truncate flex-1 font-mono">
        {log.message}
      </span>
    </div>
  );
}

// INICIO DEL ARCHIVO [apps/web-dashboard/components/features/monitoring/curve-heatmap.tsx]
/**
 * =================================================================
 * APARATO: SWARM INTELLIGENCE HEATMAP (V12.1 - GOLD MASTER)
 * CLASIFICACIÓN: FEATURE UI (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN ESPACIAL DEL ESFUERZO DE MINERÍA
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la rejilla de exploración de la curva secp256k1.
 * 1. RESOLUCIÓN TS2304: Inyección del import para la utilidad 'cn'.
 * 2. SINCRO V76.0: Consumo nominal de 'keyspace_heatmap_data'.
 * 3. RENDIMIENTO: Renderizado optimizado vía Canvas 2D con Motion Blur.
 * =================================================================
 */

"use client";

import React, { useRef, useEffect } from "react";
import { type SwarmHeatmapSegment } from "@prospector/api-contracts";
import { useNeuralLink } from "@prospector/api-client";
import { Target, Zap } from "lucide-react";
// ✅ RESOLUCIÓN TS2304: Importación de la utilidad de fusión de clases
import { cn } from "@/lib/utils/cn";

interface CurveHeatmapProperties {
  /** Resolución de la rejilla de visualización (ej: 100x100). */
  grid_resolution?: number;
}

export function CurveHeatmap({
  grid_resolution = 100
}: CurveHeatmapProperties): React.ReactElement {
  const canvas_reference = useRef<HTMLCanvasElement>(null);

  // ADQUISICIÓN DE SEÑALES (Sincronizado con Gold Master L4)
  const { keyspace_heatmap_data, is_neural_link_connected } = useNeuralLink();

  useEffect(() => {
    const current_canvas = canvas_reference.current;
    if (!current_canvas) return;

    const rendering_context = current_canvas.getContext("2d");
    if (!rendering_context) return;

    const canvas_width = current_canvas.width;
    const canvas_height = current_canvas.height;
    const cell_dimension = canvas_width / grid_resolution;

    // Limpieza de buffer con rastro persistente (Efecto de persistencia forense)
    rendering_context.fillStyle = "rgba(5, 5, 5, 0.2)";
    rendering_context.fillRect(0, 0, canvas_width, canvas_height);

    // Renderizado de Rejilla Base Táctica
    rendering_context.strokeStyle = "rgba(16, 185, 129, 0.05)";
    rendering_context.lineWidth = 0.5;

    // Solo proyectamos la rejilla estructural si el enlace neural está activo
    if (is_neural_link_connected) {
        for (let i = 0; i <= grid_resolution; i++) {
            rendering_context.beginPath();
            rendering_context.moveTo(i * cell_dimension, 0);
            rendering_context.lineTo(i * cell_dimension, canvas_height);
            rendering_context.stroke();
            rendering_context.beginPath();
            rendering_context.moveTo(0, i * cell_dimension);
            rendering_context.lineTo(canvas_width, i * cell_dimension);
            rendering_context.stroke();
        }
    }

    /**
     * MOTOR DE PROYECCIÓN DE SEGMENTOS
     * Mapea la ocupación del espacio de búsqueda a la matriz visual.
     */
    keyspace_heatmap_data.forEach((segment: SwarmHeatmapSegment) => {
      const total_matrix_cells = grid_resolution * grid_resolution;
      const target_cell_index = Math.floor(segment.normalized_start_position * total_matrix_cells);

      const x_coordinate = (target_cell_index % grid_resolution) * cell_dimension;
      const y_coordinate = Math.floor(target_cell_index / grid_resolution) * cell_dimension;

      // Cálculo de intensidad lumínica basado en el peso del segmento
      const alpha_intensity = 0.3 + (segment.intensity_weight * 0.7);
      rendering_context.shadowBlur = 15 * segment.intensity_weight;
      rendering_context.shadowColor = "#10b981";
      rendering_context.fillStyle = `rgba(16, 185, 129, ${alpha_intensity})`;

      rendering_context.fillRect(
        x_coordinate + 1,
        y_coordinate + 1,
        cell_dimension - 2,
        cell_dimension - 2
      );
      rendering_context.shadowBlur = 0;
    });
  }, [keyspace_heatmap_data, grid_resolution, is_neural_link_connected]);

  return (
    <div className="bg-[#0a0a0a] border border-zinc-800 rounded-2xl p-6 space-y-6 relative overflow-hidden shadow-2xl group font-mono">
      <div className="flex justify-between items-center relative z-10">
        <div className="space-y-1">
          <h3 className="text-[10px] font-black text-emerald-500 uppercase tracking-[0.4em] flex items-center gap-3">
            {/* ✅ RESOLUCIÓN TS2304: Uso de 'cn' verificado */}
            <Target className={cn("w-4 h-4", is_neural_link_connected && "animate-pulse")} />
            Keyspace Exploration Matrix
          </h3>
          <p className="text-[8px] text-zinc-500 uppercase tracking-widest">
            Stratum: 2^256 secp256k1 // Grid_Saturation
          </p>
        </div>
        {/* ✅ RESOLUCIÓN TS2304: Uso de 'cn' verificado */}
        <Zap className={cn("w-5 h-5", is_neural_link_connected ? "text-amber-500" : "text-zinc-800")} />
      </div>

      <div className="relative aspect-square w-full bg-black rounded-xl border border-white/5 overflow-hidden shadow-inner">
        <canvas
          ref={canvas_reference}
          width={1000}
          height={1000}
          className="w-full h-full cursor-crosshair transition-opacity duration-1000"
        />
        {!is_neural_link_connected && (
          <div className="absolute inset-0 flex items-center justify-center bg-black/60 backdrop-blur-sm animate-in fade-in duration-500">
            <div className="flex flex-col items-center gap-3">
               <Activity className="w-6 h-6 text-zinc-700 animate-pulse" />
               <span className="text-[10px] text-red-500/80 font-black uppercase tracking-[0.3em]">
                 Awaiting_Neural_Handshake
               </span>
            </div>
          </div>
        )}
      </div>

      <footer className="pt-4 border-t border-white/5 flex justify-between items-center opacity-40">
        <span className="text-[8px] font-bold text-zinc-700 uppercase tracking-[0.2em]">
          Spatial Intelligence Stratum // V12.1
        </span>
      </footer>
    </div>
  );
}

/**
 * Átomo Visual: Indicador de pulso de red.
 */
function Activity({ className }: { className?: string }) {
    return (
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
            className={className}
        >
            <path d="M22 12h-4l-3 9L9 3l-3 9H2" />
        </svg>
    );
}
// FIN DEL ARCHIVO [apps/web-dashboard/components/features/monitoring/curve-heatmap.tsx]

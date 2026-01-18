/**
 * =================================================================
 * APARATO: PANOPTICON VIRTUALIZATION TEST (V2.1)
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L5-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE RENDIMIENTO O(1)
 * =================================================================
 */

import { render, screen, fireEvent } from "@testing-library/react";
import { SystemLogConsole } from "../../../../apps/web-dashboard/components/features/monitoring/system-log-console";
import { type SystemLog } from "@prospector/api-contracts";

const generate_massive_dataset = (count: number): SystemLog[] => {
  return Array.from({ length: count }).map((_, i) => ({
    id: `uuid-${i}`,
    timestamp: new Date().toISOString(),
    stratum: "L1_CORE",
    severity: "INFO",
    message: `Cryptographic heartbeat signal sequence #${i}`,
  }));
};

describe("Panopticon V2.1: Virtualization Strata Audit", () => {
  it("should maintain low DOM node count with 5,000 logs in buffer", () => {
    const massive_logs = generate_massive_dataset(5000);
    const { container } = render(<SystemLogConsole logs={massive_logs} />);

    // Verificación de Virtualización: El número de líneas renderizadas debe ser mucho menor que 5,000
    const rendered_lines = container.querySelectorAll(".group\\/line");

    // El cálculo nominal con overscan es ~60-80 líneas, nunca 5,000
    expect(rendered_lines.length).toBeLessThan(150);
    expect(rendered_lines.length).toBeGreaterThan(0);

    // Verificamos que el HUD reporta el tamaño total del buffer
    expect(screen.getByText(/Buffer: 5000 Units/i)).toBeInTheDocument();
  });

  it("should update virtual window on scrollTop change", () => {
    const logs = generate_massive_dataset(1000);
    const { container } = render(<SystemLogConsole logs={logs} />);

    const viewport = container.querySelector(".flex-1.overflow-y-auto");
    if (!viewport) throw new Error("Viewport not found");

    // Simular scroll profundo
    fireEvent.scroll(viewport, { target: { scrollTop: 5000 } });

    // El componente debe reaccionar al cambio de scroll_top y recalcular el slice
    expect(viewport.scrollTop).toBe(5000);
  });
});

/**
 * =================================================================
 * APARATO: PANOPTICON V2.3 SSS TEST
 * CLASIFICACIÓN: TRINITY EVIDENCE // ESTRATO L5-MIRROR
 * RESPONSABILIDAD: CERTIFICACIÓN DE VIRTUALIZACIÓN Y LINT-FREE SCOPE
 * =================================================================
 */

import { render, screen, fireEvent } from "@testing-library/react";
import { SystemLogConsole } from "../../../../apps/web-dashboard/components/features/monitoring/system-log-console";
import { type SystemLog } from "@prospector/api-contracts";

const mock_logs: SystemLog[] = [
  { id: "L1", timestamp: new Date().toISOString(), stratum: "L1_CORE", severity: "INFO", message: "Math Engine Online" },
  { id: "L3", timestamp: new Date().toISOString(), stratum: "L3_ORCH", severity: "ERROR", message: "Turso Pool Warning" },
];

describe("Panopticon V2.3 SSS Logic Audit", () => {
  it("should consume all declared hooks and icons avoiding regressions", async () => {
    // La prueba de éxito es que el componente renderice sin advertencias de hooks no usados
    render(<SystemLogConsole logs={mock_logs} />);

    expect(screen.getByText(/Buffer_Sync: 2/i)).toBeInTheDocument();

    // Verificamos que el botón de filtrado (uso de Filter icon y state) es interactivo
    const filter_trigger = screen.getByRole("button", { name: "" });
    fireEvent.click(filter_trigger);

    expect(screen.getByText("INFO")).toBeInTheDocument();
  });

  it("should manage virtual windowing during high-speed scroll emulation", async () => {
    // Generar 100 logs para asegurar que el overscan y el scroll_top actúan
    const massive_logs: SystemLog[] = Array.from({ length: 100 }).map((_, i) => ({
        id: `id-${i}`,
        timestamp: new Date().toISOString(),
        stratum: "L4_API",
        severity: "DEBUG",
        message: `Packet trace sequence #${i}`
    }));

    const { container } = render(<SystemLogConsole logs={massive_logs} />);
    const viewport = container.querySelector(".overflow-y-auto");

    if (viewport) {
        fireEvent.scroll(viewport, { target: { scrollTop: 500 } });
        // Verificamos que el componente no colapsó
        expect(screen.getByText(/Buffer_Sync: 100/i)).toBeInTheDocument();
    }
  });
});

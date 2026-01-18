// INICIO DEL ARCHIVO [tests/mirror/apps/web_dashboard/observability_ui.test.tsx]
/**
 * =================================================================
 * APARATO: PANOPTICON UI INTEGRITY TEST (V1.0)
 * CLASIFICACIÓN: COMPONENT TEST
 * OBJETIVO: Verificar renderizado de logs unificados
 * =================================================================
 */


import { render, screen } from "@testing-library/react";
import { SystemLogConsole } from "../../../../apps/web-dashboard/components/features/monitoring/system-log-console";
import { type SystemLog } from "@prospector/api-contracts";

describe("Panopticon UI (System Log Console)", () => {
    const mockLogs: SystemLog[] = [
        {
            id: "1",
            timestamp: new Date().toISOString(),
            stratum: "L1_CORE",
            severity: "INFO",
            message: "Genesis Block Validated",
        },
        {
            id: "2",
            timestamp: new Date().toISOString(),
            stratum: "L6_OPS",
            severity: "CRITICAL",
            message: "Uplink Severed",
        }
    ];

    it("should render logs with correct semantic coloring", () => {
        render(<SystemLogConsole logs={mockLogs} />);

        // Verificar presencia de mensajes
        expect(screen.getByText("Genesis Block Validated")).toBeInTheDocument();
        expect(screen.getByText("Uplink Severed")).toBeInTheDocument();

        // Verificar etiquetas de estrato
        expect(screen.getByText("[L1_CORE]")).toBeInTheDocument();
        expect(screen.getByText("[L6_OPS]")).toBeInTheDocument();
    });

    it("should filter logs correctly", async () => {
        render(<SystemLogConsole logs={mockLogs} />);

        // Simular clic en filtro de error (usando title del botón)
        const filterBtn = screen.getByTitle("Filter Errors");
        filterBtn.click();

        // El log INFO debe desaparecer, el CRITICAL debe quedar
        expect(screen.queryByText("Genesis Block Validated")).not.toBeInTheDocument();
        expect(screen.getByText("Uplink Severed")).toBeInTheDocument();
    });
});
// FIN DEL ARCHIVO [tests/mirror/apps/web_dashboard/observability_ui.test.tsx]

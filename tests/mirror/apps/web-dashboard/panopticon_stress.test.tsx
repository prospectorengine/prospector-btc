import { render, screen, fireEvent } from "@testing-library/react";
import { SystemLogConsole } from "../../../../apps/web-dashboard/components/features/monitoring/system-log-console";
import { type SystemLog } from "@prospector/api-contracts";

const generate_massive_logs = (count: number): SystemLog[] => {
  return Array.from({ length: count }).map((_, i) => ({
    id: `id-${i}`,
    timestamp: new Date().toISOString(),
    stratum: "L1_CORE",
    severity: "INFO",
    message: `Stress signal sequence packet #${i}`,
  }));
};

describe("Panopticon V2: High-Density Stress Audit", () => {
  it("should handle a burst of 5000 logs without DOM collapse", () => {
    const massive_logs = generate_massive_logs(5000);
    const { container } = render(<SystemLogConsole logs={massive_logs} />);

    // Verificamos que el buffer circular de la UI limitó el renderizado o mantiene el rastro
    const log_elements = container.querySelectorAll(".group\\/line");
    expect(log_elements.length).toBeLessThanOrEqual(5000);
    expect(screen.getByText(/Buffer: 5000 \/ 5000/i)).toBeInTheDocument();
  });

  it("should pause auto-scroll when user interacts with the viewport", () => {
    const logs = generate_massive_logs(10);
    render(<SystemLogConsole logs={logs} />);

    const pauseBtn = screen.getByTitle(/Pause Auto-scroll/i);
    fireEvent.click(pauseBtn);

    expect(screen.getByTitle(/Resume Auto-scroll/i)).toBeInTheDocument();
  });
});

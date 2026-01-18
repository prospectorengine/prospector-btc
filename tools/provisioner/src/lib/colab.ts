// INICIO DEL ARCHIVO [tools/provisioner/src/lib/colab.ts]
/**
 * =================================================================
 * APARATO: COLAB CONTROLLER (V43.0 - PHOENIX INTEGRATED)
 * CLASIFICACIÓN: COMPOSITE CONTROLLER (ESTRATO L6)
 * RESPONSABILIDAD: ORQUESTACIÓN DE DESPLIEGUE CON FEEDBACK C2
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el flujo de ignición completo.
 * 1. SINCRO: Integra 'SessionHarvester' para cerrar el bucle de identidad.
 * 2. RESILIENCIA: Gestión de limpieza de intervalos en 'teardown'.
 * 3. OBSERVABILIDAD: Trazabilidad total de la inyección del payload.
 * =================================================================
 */

import { Page } from "playwright";
import { createCursor, GhostCursor } from "ghost-cursor-playwright";
import chalk from "chalk";

import { SELECTORS } from "./selectors";
import { ColabNavigator } from "./mechanics/navigator";
import { Sentinel } from "./mechanics/sentinel";
import { SessionHarvester } from "./mechanics/harvester"; // ✅ RECUPERADO
import { generate_mission_payload } from "./payload";

export class ColabController {
  private worker_node_identifier: string;
  private terminal_prefix: string;

  // Subsistemas Mecánicos
  private sentinel: Sentinel;
  private navigator: ColabNavigator | null = null;
  private harvester: SessionHarvester | null = null;

  // Estado de Interacción
  private interaction_cursor: GhostCursor | null = null;
  private harvest_interval_handle: NodeJS.Timeout | null = null;

  constructor(
    private playwright_page: Page,
    node_sequence_index: number,
    private identity_email_label: string | null, // ✅ ACTIVO: Necesario para el Harvester
  ) {
    this.worker_node_identifier = `hydra-node-${node_sequence_index}`;
    this.terminal_prefix = chalk.cyan(`[${this.worker_node_identifier}]`);

    this.sentinel = new Sentinel(
      playwright_page,
      this.worker_node_identifier,
      node_sequence_index,
      this.terminal_prefix,
    );
  }

  /**
   * Ejecuta la secuencia de despliegue informando cada hito al Dashboard.
   * @param master_vault_key Llave maestra para el re-cifrado de cookies (Phoenix).
   */
  public async deploy(master_vault_key: string): Promise<void> {
    try {
      await this.sentinel.emitTrace("Initializing neuro-navigation sequence...", "INFO");

      this.interaction_cursor = await createCursor(this.playwright_page);
      this.navigator = new ColabNavigator(
        this.playwright_page,
        this.interaction_cursor,
        this.sentinel
      );

      // 1. APROXIMACIÓN AL OBJETIVO
      await this.navigator.approachTarget();

      // 2. VALIDACIÓN DE MURO DE AUTENTICACIÓN
      const is_auth_wall_present = await this.navigator.detectAuthWall();
      if (is_auth_wall_present) {
        await this.sentinel.triggerKillSwitch("AUTH_WALL_DETECTED");
        throw new Error("RECOIL: Authentication required manually.");
      }

      // 3. ADQUISICIÓN DE RUNTIME (GPU/CPU)
      await this.navigator.acquireRuntime();

      // 4. PROTOCOLO PHOENIX: COSECHA Y RENOVACIÓN (Si hay identidad)
      if (this.identity_email_label && master_vault_key) {
        this.harvester = new SessionHarvester(
            this.playwright_page.context(),
            this.sentinel,
            this.identity_email_label,
            master_vault_key,
            this.worker_node_identifier
        );

        await this.sentinel.emitTrace("♻️ PHOENIX: Initiating post-login harvest...", "INFO");

        // Cosecha inmediata (Bootstrap de sesión)
        await this.harvester.harvestAndRotate();

        // Programación de ciclo de mantenimiento (Cada 15 minutos)
        // Esto mantiene la sesión viva en la DB indefinidamente mientras el nodo viva
        this.harvest_interval_handle = setInterval(() => {
            this.harvester?.harvestAndRotate().catch(e =>
                console.error(`${this.terminal_prefix} Harvest cycle failed:`, e)
            );
        }, 15 * 60 * 1000);
      } else {
        await this.sentinel.emitTrace("⚠️ PHOENIX_SKIP: Running in Anonymous/Degraded Mode.", "WARN");
      }

      // 5. INYECCIÓN DE PAYLOAD CRIPTOGRÁFICO
      await this.inject_and_execute_payload(master_vault_key);

      // 6. ACTIVACIÓN DE SUPERVIVENCIA
      this.sentinel.startHeartbeat();
      await this.sentinel.emitTrace("IGNITION_SUCCESS: Unit is now mining.", "INFO");

    } catch (unidentified_error: unknown) {
      const error_message = unidentified_error instanceof Error
        ? unidentified_error.message
        : String(unidentified_error);

      await this.sentinel.emitTrace(`DEPLOYMENT_COLLAPSE: ${error_message}`, "CRITICAL");
      await this.sentinel.captureFrame("error");

      this.teardown_operations();
      throw unidentified_error;
    }
  }

  /**
   * Limpieza de recursos y parada de emergencia.
   */
  private teardown_operations(): void {
    this.sentinel.stop();
    if (this.harvest_interval_handle) {
        clearInterval(this.harvest_interval_handle);
        this.harvest_interval_handle = null;
    }
  }

  /**
   * Realiza la inyección del código híbrido en el editor Monaco.
   */
  private async inject_and_execute_payload(master_vault_key: string): Promise<void> {
    await this.sentinel.emitTrace("Locating execution strata (Monaco Editor)...");

    const editor_locator = this.playwright_page.locator(SELECTORS.EDITOR.LINE).first();
    await editor_locator.waitFor({ state: "visible", timeout: 15000 });

    if (this.interaction_cursor) {
      await this.interaction_cursor.click(editor_locator);
    } else {
      await editor_locator.click();
    }

    await this.sentinel.emitTrace("Purging cell memory...");
    await this.playwright_page.keyboard.press("Control+A");
    await this.playwright_page.keyboard.press("Backspace");

    await this.sentinel.emitTrace("Crystallizing mission payload with ZK Keys...");
    const mission_code = generate_mission_payload(
      this.worker_node_identifier,
      master_vault_key
    );

    // Inyección vía Clipboard para velocidad y evasión de detección de tecleo mecánico
    await this.playwright_page.evaluate(
      (text) => window.navigator.clipboard.writeText(text),
      mission_code,
    );

    await this.playwright_page.keyboard.press("Control+V");
    // Pausa táctica para permitir renderizado del editor
    await new Promise(resolve => setTimeout(resolve, 500));

    await this.sentinel.emitTrace("🚀 FIRING KERNEL...", "INFO");
    await this.playwright_page.keyboard.press("Control+Enter");
  }
}
// FIN DEL ARCHIVO [tools/provisioner/src/lib/colab.ts]

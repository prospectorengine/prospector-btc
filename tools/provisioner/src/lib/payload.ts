/**
 * =================================================================
 * APARATO: HYDRA PAYLOAD CRYSTALLIZER (V47.0 - ZK_KEY_SECURED)
 * CLASIFICACIÓN: OPS CONTROL (ESTRATO L6)
 * RESPONSABILIDAD: GENERACIÓN DE CÓDIGO CON INYECCIÓN DE LLAVE MAESTRA
 * =================================================================
 */

import * as fs from "fs";
import * as path from "path";
import chalk from "chalk";
import { config } from "../config";

/**
 * Transforma el template de Python en un script de misión activo.
 *
 * # Mathematical Proof:
 * Realiza una sustitución determinista de 7 puntos de control,
 * garantizando que la MASTER_VAULT_KEY llegue al entorno del worker
 * sin persistencia en disco (Memory-Only).
 *
 * @param worker_node_identifier - ID de la unidad.
 * @param master_vault_key - Llave para descifrar identidades ZK.
 */
export function generate_mission_payload(
  worker_node_identifier: string,
  master_vault_key: string
): string {
  try {
    const template_file_path = path.resolve(__dirname, "../assets/miner_template.py");

    if (!fs.existsSync(template_file_path)) {
      throw new Error(`CRITICAL_MISSING_ASSET: Template not found at ${template_file_path}`);
    }

    let python_source = fs.readFileSync(template_file_path, "utf-8");

    // Matriz de Inyección Estratégica
    const injection_map: Record<string, string> = {
      "{{MINER_BINARY_URL}}": config.MINER_BINARY_URL,
      "{{ORCHESTRATOR_URL}}": config.ORCHESTRATOR_URL,
      "{{WORKER_AUTH_TOKEN}}": config.WORKER_AUTH_TOKEN,
      "{{MASTER_VAULT_KEY}}": master_vault_key, // <-- Sincronizado
      "{{FILTER_BASE_URL}}": config.FILTER_BASE_URL,
      "{{FILTER_SHARDS}}": config.FILTER_SHARDS.toString(),
      "{{WORKER_ID}}": worker_node_identifier,
    };

    for (const [placeholder, value] of Object.entries(injection_map)) {
      python_source = python_source.split(placeholder).join(value);
    }

    return [
      `# PROSPECTOR BTC // IGNITION_SEQ_${Date.now().toString(16).toUpperCase()}`,
      python_source
    ].join("\n");

  } catch (error: unknown) {
    const msg = error instanceof Error ? error.message : String(error);
    console.error(chalk.bgRed.white("\n🔥 [PAYLOAD_FAULT]:"), msg);
    throw new Error("CRYSTALLIZATION_FAILURE");
  }
}

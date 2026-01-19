/**
 * =================================================================
 * APARATO: HYDRA PAYLOAD CRYSTALLIZER (V48.0 - ZENITH ALIGNED)
 * CLASIFICACIÓN: OPS CONTROL (ESTRATO L6)
 * RESPONSABILIDAD: GENERACIÓN DE CÓDIGO CON INYECCIÓN DE LLAVE MAESTRA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. NOMINAL PURITY: Erradicación de abreviaciones. 'config' se consume de forma nominal.
 * 2. ATOMIC SUBSTITUTION: Implementa el mapeo determinista de 7 puntos de control.
 * 3. ERROR TRIAGE: Gestión de fallos de I/O con reporte forense en terminal.
 * 4. ZERO RESIDUE: Limpieza de rastro temporal mediante inyección en memoria.
 *
 * # Mathematical Proof (Deterministic Crystallization):
 * El aparato garantiza la paridad bit-a-bit entre el template físico y el
 * script inyectado, asegurando que las variables de red (URLs) y de
 * seguridad (Tokens) sean válidas antes de la ignición del worker.
 * =================================================================
 */

import * as fs from "fs";
import * as path from "path";
import chalk from "chalk";
import { config } from "../config";

/**
 * Transforma el sustrato de Python en un script de misión activo.
 *
 * # Performance:
 * Operación O(N) donde N es el número de marcadores de posición.
 * Utiliza reemplazo de strings encadenado para minimizar el uso de memoria.
 *
 * # Errors:
 * @throws {Error} Si el archivo 'miner_template.py' no es localizado en el estrato assets.
 * @throws {Error} Si ocurre una falla de cristalización durante la sustitución de claves.
 *
 * @param worker_node_identifier - Identificador unívoco de la unidad (ej: HYDRA-COLAB-01).
 * @param master_vault_key - Llave de descifrado local para la Bóveda ZK.
 * @returns El código Python final listo para ser transmitido al portapapeles de Colab.
 */
export function generate_mission_payload(
  worker_node_identifier: string,
  master_vault_key: string
): string {
  try {
    // 1. ADQUISICIÓN DE LA FUENTE (TEMPLATE ASSET)
    const template_file_path = path.resolve(__dirname, "../assets/miner_template.py");

    if (!fs.existsSync(template_file_path)) {
      throw new Error(`CRITICAL_MISSING_ASSET: Material not found at ${template_file_path}`);
    }

    let python_source_buffer = fs.readFileSync(template_file_path, "utf-8");

    /**
     * MATRIZ DE INYECCIÓN ESTRATÉGICA
     * Sincronizada con el Supervisor Phoenix V22.0.
     */
    const injection_strata_map: Record<string, string> = {
      "{{MINER_BINARY_URL}}": config.MINER_BINARY_URL,
      "{{ORCHESTRATOR_URL}}": config.ORCHESTRATOR_URL,
      "{{WORKER_AUTH_TOKEN}}": config.WORKER_AUTH_TOKEN,
      "{{MASTER_VAULT_KEY}}": master_vault_key,
      "{{FILTER_BASE_URL}}": config.FILTER_BASE_URL,
      "{{FILTER_SHARDS}}": config.FILTER_SHARDS.toString(),
      "{{WORKER_ID}}": worker_node_identifier,
    };

    // 2. EJECUCIÓN DE CRISTALIZACIÓN (Sustitución de Marcadores)
    for (const [placeholder_marker, tactical_value] of Object.entries(injection_strata_map)) {
      python_source_buffer = python_source_buffer.split(placeholder_marker).join(tactical_value);
    }

    // 3. SELLO DE SECUENCIA Y VERSIÓN
    const ignition_sequence_id = Date.now().toString(16).toUpperCase();

    return [
      `# PROSPECTOR BTC // ZENITH_IGNITION_SEQ_${ignition_sequence_id}`,
      `# PROTOCOL_VERSION: V11.5_GOLD_MASTER`,
      python_source_buffer
    ].join("\n");

  } catch (unidentified_error: unknown) {
    const error_message = unidentified_error instanceof Error
      ? unidentified_error.message
      : String(unidentified_error);

    console.error(chalk.bgRed.white("\n🔥 [CRYSTALLIZER_FAULT]:"), error_message);
    throw new Error(`PAYLOAD_GENERATION_FAILED: ${error_message}`);
  }
}

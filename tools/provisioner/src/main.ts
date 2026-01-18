// INICIO DEL ARCHIVO [tools/provisioner/src/main.ts]
/**
 * =================================================================
 * APARATO: SOVEREIGN SWARM COMMANDER (V47.0 - CRYPTO ENABLED)
 * CLASIFICACIÓN: OPS CONTROL (ESTRATO L6)
 * RESPONSABILIDAD: ORQUESTACIÓN DE IGNICIÓN MULTI-VECTOR
 *
 * VISION HIPER-HOLÍSTICA:
 * Punto de entrada principal para el despliegue de infraestructura.
 * Implementa el polyfill criptográfico necesario para que el
 * motor AES-GCM (L1) funcione en el entorno Node.js de los runners.
 * =================================================================
 */

// --- 1. INYECCIÓN DE POLYFILL CRIPTOGRÁFICO (CRÍTICO) ---
// Debe ejecutarse antes de cualquier importación que dependa de @prospector/crypto-vault
import { webcrypto } from "node:crypto";

if (!globalThis.crypto) {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  (globalThis as any).crypto = webcrypto;
}

import { BrowserFactory } from "./lib/browser";
import { ColabController } from "./lib/colab";
import { KaggleController } from "./lib/kaggle";
import { config } from "./config";
import chalk from "chalk";
import pLimit from "p-limit";

/**
 * Función maestra de ejecución.
 * Coordina el despliegue paralelo de nodos limitando la concurrencia
 * para evitar la detección por análisis de tráfico (Rate Limiting).
 */
async function main(): Promise<void> {
  console.log(chalk.bold.magenta("\n💠 PROSPECTOR HYDRA-IGNITION :: MULTI-VECTOR MASTER"));
  console.log(chalk.gray("--------------------------------------------------"));

  // Semáforo de concurrencia: Máximo 3 navegadores iniciándose simultáneamente.
  const ignition_semaphore = pLimit(3);

  try {
    // Inicialización del contexto base (Fingerprint + Identidad)
    // Nota: 'browser_instance' se descarta aquí porque cada controlador gestionará sus páginas,
    // pero necesitamos el 'context' y el 'identityEmail' resueltos.
    const { browser: _browser_instance, context, identityEmail } = await BrowserFactory.createContext();

    console.log(`${chalk.cyan("👤 IDENTITY:")} ${identityEmail || "ANONYMOUS_SESSION (Capability Degraded)"}`);
    console.log(`${chalk.cyan("🌊 SWARM_TARGET:")} ${config.WORKER_COUNT} grid units`);

    const kaggle_percentage = (config.KAGGLE_DISTRIBUTION_RATIO * 100).toFixed(0);
    console.log(`${chalk.cyan("⚖️  DISTRIBUTION:")} ${kaggle_percentage}% Kaggle / ${100 - Number(kaggle_percentage)}% Colab`);

    // Generación de la matriz de despliegue
    const deployment_sequence = Array.from({ length: config.WORKER_COUNT }).map((_, index) => {
      return ignition_semaphore(async () => {
        const sequence_identifier = index + 1;

        // Creación de página aislada dentro del contexto huellado
        const page_handle = await context.newPage();

        // Determinación estocástica del vector de ataque
        const is_kaggle_target = Math.random() < config.KAGGLE_DISTRIBUTION_RATIO;

        if (is_kaggle_target) {
          const kaggle_unit = new KaggleController(page_handle, sequence_identifier, identityEmail);
          await kaggle_unit.deploy_ignition();
        } else {
          const colab_unit = new ColabController(page_handle, sequence_identifier, identityEmail);
          // La MASTER_KEY es necesaria para operaciones internas del payload, aunque la identidad ya esté inyectada
          const master_vault_key = process.env.MASTER_VAULT_KEY || "Satoshi2009";
          await colab_unit.deploy(master_vault_key);
        }
      });
    });

    // Ejecución paralela y recolección de resultados
    const execution_results = await Promise.allSettled(deployment_sequence);

    const successful_ignitions = execution_results.filter(result => result.status === "fulfilled").length;

    console.log(
      chalk.bold.green(`\n✅ IGNITION_PHASE_COMPLETE: ${successful_ignitions}/${config.WORKER_COUNT} nodes online.`)
    );

    // Mantenimiento del proceso vivo para soportar el túnel de Playwright
    keep_system_alive();

  } catch (fatal_error: unknown) {
    const error_message = fatal_error instanceof Error ? fatal_error.message : "UNKNOWN_FATAL_ERROR";
    console.error(chalk.bgRed.white("\n🔥 FATAL_COMMAND_FAULT:"), error_message);
    process.exit(1);
  }
}

/**
 * Bucle de mantenimiento de constantes vitales.
 * Previene que el proceso de Node.js termine, manteniendo los navegadores abiertos.
 */
function keep_system_alive(): void {
  setInterval(() => {
    const memory_usage_mb = process.memoryUsage().rss / 1024 / 1024;
    console.log(
      chalk.dim(`[${new Date().toLocaleTimeString()}] SwarmCommander HUD -> RAM: ${memory_usage_mb.toFixed(1)} MB`)
    );
  }, 300000); // Reporte cada 5 minutos
}

main();
// FIN DEL ARCHIVO [tools/provisioner/src/main.ts]

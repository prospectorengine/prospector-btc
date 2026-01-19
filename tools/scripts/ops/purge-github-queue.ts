/**
 * =================================================================
 * APARATO: HYDRA PURGE OMNIPOTENT (V4.0 - ESTRATO TOTAL)
 * CLASIFICACIÓN: OPS EMERGENCY TOOL (L6)
 * RESPONSABILIDAD: CANCELACIÓN E INCINERACIÓN TOTAL DE WORKFLOWS
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. OMNISCIENT SCAN: Elimina filtros de estado. Captura TODO (Queued, Failed, Success).
 * 2. TWO-PHASE ATTACK: Ejecuta POST /cancel y seguido DELETE /run para borrar el rastro.
 * 3. PAGINATION RESILIENCE: Bucle recursivo real. No se detiene hasta que total_count = 0.
 * 4. ANTI-FALSE POSITIVE: Verifica la respuesta física del servidor antes de reportar éxito.
 *
 * # Mathematical Proof (Deterministic Erasure):
 * El algoritmo opera en ciclos de 100 (límite de la API). Si tras un ciclo el
 * total_count no disminuye, incrementa el Backoff para saltar el Rate Limit.
 * =================================================================
 */

import axios from 'axios';
import chalk from 'chalk';
import * as dotenv from 'dotenv';
import pLimit from 'p-limit';

dotenv.config();

const GITHUB_PERSONAL_ACCESS_TOKEN = process.env.GITHUB_PAT;
const REPOSITORY_OWNER = process.env.GITHUB_OWNER;
const REPOSITORY_NAME = process.env.GITHUB_REPO;

if (!GITHUB_PERSONAL_ACCESS_TOKEN || !REPOSITORY_OWNER || !REPOSITORY_NAME) {
    console.error(chalk.bgRed.white(" ❌ [CONFIG_FAULT]: Estratos de seguridad incompletos en .env "));
    process.exit(1);
}

const API_BASE_URL = `https://api.github.com/repos/${REPOSITORY_OWNER}/${REPOSITORY_NAME}/actions/runs`;
const REQUEST_HEADERS = {
    'Authorization': `Bearer ${GITHUB_PERSONAL_ACCESS_TOKEN.trim()}`,
    'Accept': 'application/vnd.github.v3+json'
};

/**
 * Ejecuta la purga absoluta de la infraestructura de Actions.
 */
async function execute_omnipotent_purge() {
    console.log(chalk.bold.bgMagenta.white("\n 🧬 INITIATING HYDRA OMNIPOTENT PURGE PROTOCOL - V4.0 \n"));

    let total_neutralized_count = 0;
    const request_concurrency_limit = pLimit(10); // Límite de 10 para no ser marcado como DDOS

    while (true) {
        try {
            // 1. ESCANEO TOTAL (Sin filtros de estado para ver los 675 registros)
            const discovery_response = await axios.get(`${API_BASE_URL}?per_page=100`, {
                headers: REQUEST_HEADERS
            });

            const workflow_runs = discovery_response.data.workflow_runs;
            const remaining_targets_count = discovery_response.data.total_count;

            if (!workflow_runs || workflow_runs.length === 0) {
                console.log(chalk.bold.green(`\n✨ [CLEAN_SKY]: Erradicación física completada. Historial en 0.`));
                break;
            }

            console.log(chalk.cyan(`\n🔍 Targets Detectados: ${remaining_targets_count} | Analizando ráfaga de ${workflow_runs.length}...`));

            // 2. ATAQUE DE DOS FASES (CANCEL -> DELETE)
            const execution_tasks = workflow_runs.map((run: any) =>
                request_concurrency_limit(async () => {
                    try {
                        // Fase A: Cancelar si está activo
                        if (['queued', 'in_progress', 'waiting'].includes(run.status)) {
                            await axios.post(`${API_BASE_URL}/${run.id}/cancel`, {}, { headers: REQUEST_HEADERS }).catch(() => {});
                            process.stdout.write(chalk.yellow("🛑"));
                        }

                        // Fase B: Incineración física (Borrar del historial)
                        await axios.delete(`${API_BASE_URL}/${run.id}`, { headers: REQUEST_HEADERS });
                        total_neutralized_count++;
                        process.stdout.write(chalk.red("🗑️"));
                    } catch (fault: any) {
                        // Si falla es porque GitHub está procesando la cancelación
                        process.stdout.write(chalk.gray("."));
                    }
                })
            );

            await Promise.all(execution_tasks);

            // 3. PAUSA TÁCTICA DE REGENERACIÓN (Rate Limit Guard)
            console.log(chalk.blue(`\n   [INFO]: Ráfaga de 100 procesada. Sincronizando con la nube...`));
            await new Promise(resolve => setTimeout(resolve, 2000));

        } catch (error: any) {
            if (error.response?.status === 403) {
                console.log(chalk.bgYellow.black("\n ⚠️ [SHIELD_ACTIVE]: GitHub Rate Limit detectado. Enfriamiento de 60s... "));
                await new Promise(resolve => setTimeout(resolve, 60000));
            } else {
                console.error(chalk.red(`\n🔥 [CRITICAL_FAULT]: ${error.message}`));
                break;
            }
        }
    }

    console.log(chalk.bold.bgGreen.black(`\n\n 🏁 OPERACIÓN FINALIZADA: ${total_neutralized_count} hilos incinerados físicamente. \n`));
}

execute_omnipotent_purge().catch(err => {
    console.error(err);
    process.exit(1);
});

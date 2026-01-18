/**
 * =================================================================
 * APARATO: HYDRA ANNIHILATOR (V3.0 - RECURSIVE PURGE)
 * CLASIFICACIÓN: OPS EMERGENCY TOOL (L6)
 * RESPONSABILIDAD: ERRADICACIÓN TOTAL DE WORKFLOWS EN BUCLE
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa un motor de búsqueda y destrucción que agota la cola
 * de GitHub Actions mediante recursión controlada.
 * =================================================================
 */

import axios from 'axios';
import chalk from 'chalk';
import * as dotenv from 'dotenv';
import pLimit from 'p-limit';

dotenv.config();

const GITHUB_PAT = process.env.GITHUB_PAT;
const OWNER = process.env.GITHUB_OWNER;
const REPO = process.env.GITHUB_REPO;

if (!GITHUB_PAT || !OWNER || !REPO) {
    console.error(chalk.red("❌ [CONFIG_FAULT]: Strategic credentials missing in .env"));
    process.exit(1);
}

const API_BASE = `https://api.github.com/repos/${OWNER}/${REPO}`;
const HEADERS = {
    'Authorization': `Bearer ${GITHUB_PAT}`,
    'Accept': 'application/vnd.github.v3+json'
};

async function execute_annihilation_sequence() {
    console.log(chalk.bold.bgRed.white("\n 🔥 INITIATING HYDRA ANNIHILATOR PROTOCOL - V3.0 \n"));

    let total_neutralized = 0;
    let cycle_count = 1;

    while (true) {
        try {
            // 1. ADQUISICIÓN DE OBJETIVOS (Sonda de vanguardia)
            // Filtramos por 'queued' e 'in_progress' para atacar solo lo que consume recursos
            const runs_res = await axios.get(
                `${API_BASE}/actions/runs?per_page=100&status=queued&status=in_progress`,
                { headers: HEADERS }
            );

            const runs_to_kill = runs_res.data.workflow_runs;
            const remaining_in_cloud = runs_res.data.total_count;

            if (runs_to_kill.length === 0) {
                console.log(chalk.bold.green(`\n✨ [CLEAN_SKY]: Erradicación completada. Cielo despejado.`));
                console.log(chalk.cyan(`   Total de hilos neutralizados: ${total_neutralized}\n`));
                break;
            }

            console.log(chalk.magenta(`\n🚀 [CYCLE_${cycle_count}]: Targets detected: ${remaining_in_cloud}`));
            console.log(chalk.gray(`   Neutralizing batch of ${runs_to_kill.length} units...`));

            // 2. MOTOR DE ANULACIÓN PARALELA (Backpressure de 15 peticiones)
            const limit = pLimit(15);

            const tasks = runs_to_kill.map((run: any) =>
                limit(async () => {
                    try {
                        await axios.post(`${API_BASE}/actions/runs/${run.id}/cancel`, {}, { headers: HEADERS });
                        total_neutralized++;
                        process.stdout.write(chalk.red("✘"));
                    } catch (e: any) {
                        // 409: Conflict (Ya cancelado), 404: Not Found
                        process.stdout.write(chalk.gray("."));
                    }
                })
            );

            await Promise.all(tasks);

            cycle_count++;

            // 3. PAUSA TÁCTICA (Evitar Rate Limit secundario de GitHub)
            process.stdout.write(chalk.yellow(`\n   [WAIT]: Cool-down pulse...`));
            await new Promise(resolve => setTimeout(resolve, 1500));

        } catch (error: any) {
            console.error(chalk.bgRed.white("\n🔥 [FATAL_ANNIHILATOR_ERROR]:"), error.message);
            // Si hay un error de rate limit real (403), esperamos más tiempo
            if (error.response?.status === 403) {
                console.log(chalk.yellow("\n⚠️ [RATE_LIMIT]: Cloud shielding active. Waiting 30s..."));
                await new Promise(resolve => setTimeout(resolve, 30000));
            } else {
                process.exit(1);
            }
        }
    }
}

execute_annihilation_sequence();

/**
 * =================================================================
 * APARATO: HYDRA JANITOR (V1.0 - DEEP CLEAN)
 * CLASIFICACIÓN: OPS HYGIENE TOOL (L6)
 * RESPONSABILIDAD: ELIMINACIÓN MASIVA DE HISTORIAL DE WORKFLOWS
 * =================================================================
 */

import axios from 'axios';
import chalk from 'chalk';
import * as dotenv from 'dotenv';
import pLimit from 'p-limit';

dotenv.config();

const { GITHUB_PAT, GITHUB_OWNER, GITHUB_REPO } = process.env;

if (!GITHUB_PAT || !GITHUB_OWNER || !GITHUB_REPO) {
    console.error(chalk.red("❌ [CONFIG_FAULT]: Missing credentials in .env"));
    process.exit(1);
}

const API_BASE = `https://api.github.com/repos/${GITHUB_OWNER}/${GITHUB_REPO}`;
const HEADERS = {
    'Authorization': `Bearer ${GITHUB_PAT}`,
    'Accept': 'application/vnd.github.v3+json'
};

async function execute_deep_clean() {
    console.log(chalk.bold.bgBlue.white("\n 🧹 INITIATING HYDRA JANITOR PROTOCOL - V1.0 \n"));

    let total_deleted = 0;

    while (true) {
        try {
            // 1. ESCANEO DE RESIDUOS (Cualquier status: completed, failure, cancelled)
            const runs_res = await axios.get(`${API_BASE}/actions/runs?per_page=100`, { headers: HEADERS });
            const runs_to_delete = runs_res.data.workflow_runs;

            if (!runs_to_delete || runs_to_delete.length === 0) {
                console.log(chalk.bold.green(`\n✨ [STRATA_CLEAN]: Todo el historial ha sido incinerado.`));
                break;
            }

            console.log(chalk.cyan(`\n🔍 Found ${runs_res.data.total_count} records. Purging batch of ${runs_to_delete.length}...`));

            // 2. MOTOR DE INCINERACIÓN PARALELA (15 hilos)
            const limit = pLimit(15);
            const tasks = runs_to_delete.map((run: any) =>
                limit(async () => {
                    try {
                        await axios.delete(`${API_BASE}/actions/runs/${run.id}`, { headers: HEADERS });
                        total_deleted++;
                        process.stdout.write(chalk.red("🗑️"));
                    } catch (e: any) {
                        process.stdout.write(chalk.gray("."));
                    }
                })
            );

            await Promise.all(tasks);

            // Pausa de cortesía para la API
            await new Promise(resolve => setTimeout(resolve, 1000));

        } catch (error: any) {
            console.error(chalk.red("\n🔥 [JANITOR_ERROR]:"), error.message);
            if (error.response?.status === 403) {
                console.log(chalk.yellow("⚠️ Rate limit detected. Cooling down for 30s..."));
                await new Promise(resolve => setTimeout(resolve, 30000));
            } else {
                break;
            }
        }
    }

    console.log(chalk.bold.green(`\n\n🏁 TOTAL RECOVERY: ${total_deleted} runs deleted from history.\n`));
}

execute_deep_clean();

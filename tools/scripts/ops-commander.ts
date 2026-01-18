// INICIO DEL ARCHIVO [tools/scripts/ops-commander.ts]
/**
 * =================================================================
 * APARATO: OPS COMMANDER (V1.0 - AUTONOMIC SYSTEM)
 * CLASIFICACIÓN: OPS ORCHESTRATOR (ESTRATO L6)
 * RESPONSABILIDAD: AUTOMATIZACIÓN E2E (CODE -> SWARM)
 *
 * FLUJO DE EJECUCIÓN:
 * 1. GIT SYNC: Asegura que el código local esté en el repositorio.
 * 2. PROVING GROUNDS: Dispara y espera los tests en CI.
 * 3. SEEDING: Si los tests pasan, hidrata la DB (Campaña Forense).
 * 4. IGNITION: Si la DB está lista, lanza el Enjambre Fantasma.
 * =================================================================
 */

import axios from 'axios';
import { execSync } from 'child_process';
import chalk from 'chalk';
import * as dotenv from 'dotenv';
import { setTimeout } from 'timers/promises';

dotenv.config();

// CONFIGURACIÓN ESTRATÉGICA
const GITHUB_PAT = process.env.GITHUB_PAT;
const OWNER = process.env.GITHUB_OWNER;
const REPO = process.env.GITHUB_REPO;
const BRANCH = "main"; // Rama objetivo

if (!GITHUB_PAT || !OWNER || !REPO) {
    console.error(chalk.red("❌ [CONFIG_VOID]: Credenciales de GitHub faltantes en .env"));
    process.exit(1);
}

const API_BASE = `https://api.github.com/repos/${OWNER}/${REPO}`;
const HEADERS = {
    'Authorization': `Bearer ${GITHUB_PAT}`,
    'Accept': 'application/vnd.github.v3+json'
};

// --- MOTORES DE UTILIDAD ---

async function triggerWorkflow(filename: string, inputs: any = {}) {
    console.log(chalk.yellow(`⚡ Disparando Workflow: [${filename}]...`));
    try {
        await axios.post(
            `${API_BASE}/actions/workflows/${filename}/dispatches`,
            { ref: BRANCH, inputs },
            { headers: HEADERS }
        );
        console.log(chalk.green(`   ✅ Señal enviada.`));
    } catch (e: any) {
        console.error(chalk.red(`   ❌ FALLO DE DISPARO: ${e.response?.data?.message || e.message}`));
        process.exit(1);
    }
}

async function waitForWorkflowCompletion(filename: string): Promise<'success' | 'failure'> {
    console.log(chalk.cyan(`⏳ Esperando resultados de [${filename}]...`));

    // Esperar inicialización
    await setTimeout(5000);

    let runId = null;

    // 1. Obtener el ID del Run más reciente
    try {
        const res = await axios.get(
            `${API_BASE}/actions/workflows/${filename}/runs?per_page=1&branch=${BRANCH}`,
            { headers: HEADERS }
        );
        if (res.data.workflow_runs.length > 0) {
            runId = res.data.workflow_runs[0].id;
            console.log(chalk.gray(`   🆔 Run ID: ${runId}`));
        } else {
            throw new Error("No runs found");
        }
    } catch (e: any) {
        console.error(chalk.red(`   ❌ No se pudo rastrear el workflow: ${e.message}`));
        process.exit(1);
    }

    // 2. Polling de estado
    while (true) {
        try {
            const res = await axios.get(`${API_BASE}/actions/runs/${runId}`, { headers: HEADERS });
            const status = res.data.status;
            const conclusion = res.data.conclusion;

            if (status === 'completed') {
                if (conclusion === 'success') {
                    console.log(chalk.bold.green(`   ✅ [${filename}] COMPLETADO CON ÉXITO.`));
                    return 'success';
                } else {
                    console.log(chalk.bold.red(`   ❌ [${filename}] FALLÓ (Conclusion: ${conclusion}).`));
                    return 'failure';
                }
            }

            process.stdout.write(chalk.gray(`   ... Estado: ${status} (Esperando 10s)\r`));
            await setTimeout(10000);

        } catch (e) {
            console.error(chalk.red("   ❌ Error de red durante polling."));
            process.exit(1);
        }
    }
}

async function syncGit() {
    console.log(chalk.bold.magenta("\n📦 [FASE 1]: SINCRONIZACIÓN DE CÓDIGO"));
    try {
        console.log(chalk.gray("   Ejecutando git add/commit/push..."));

        // Comprobamos si hay cambios
        const status = execSync('git status --porcelain').toString();
        if (status) {
            execSync('git add .');
            // Commit automático táctico
            try {
                execSync('git commit -m "ops(auto): sync before massive deployment"', { stdio: 'ignore' });
            } catch { /* Empty commit ignored */ }
            execSync('git push');
            console.log(chalk.green("   ✅ Código sincronizado con el repositorio remoto."));
        } else {
            console.log(chalk.white("   ℹ️  No hay cambios locales pendientes."));
            // Aseguramos que lo local esté arriba igual
            execSync('git push');
        }
    } catch (e) {
        console.error(chalk.red("   ❌ FALLO EN GIT SYNC."));
        process.exit(1);
    }
}

// --- SECUENCIA MAESTRA ---

async function executeGlobalDeployment() {
    console.log(chalk.bold.bgBlue.white("\n 🌐 PROSPECTOR OPS COMMANDER - GLOBAL DEPLOYMENT SEQUENCE \n"));

    // 1. GIT SYNC
    await syncGit();

    // 2. PROVING GROUNDS (TESTS)
    console.log(chalk.bold.magenta("\n🧪 [FASE 2]: CERTIFICACIÓN REMOTA (Proving Grounds)"));
    // Asumimos que proving-grounds.yml existe y se dispara con push, pero lo forzamos manualmente
    // para trazar su ID exacto y esperar.
    await triggerWorkflow('proving-grounds.yml');
    const testResult = await waitForWorkflowCompletion('proving-grounds.yml');

    if (testResult === 'failure') {
        console.log(chalk.bold.bgRed.white("\n 🛑 ABORTANDO: Los tests fallaron. Revise GitHub Actions. \n"));
        process.exit(1);
    }

    // 3. SEEDING (CAMPAÑA FORENSE)
    console.log(chalk.bold.magenta("\n🧬 [FASE 3]: HIDRATACIÓN DE DATOS (Campaign Seeding)"));
    // Trigger remoto del seeder para no depender de la red local
    await triggerWorkflow('seed-campaign.yml');
    const seedResult = await waitForWorkflowCompletion('seed-campaign.yml');

    if (seedResult === 'failure') {
        console.log(chalk.bold.bgRed.white("\n 🛑 ABORTANDO: Fallo al sembrar la base de datos. \n"));
        process.exit(1);
    }

    // 4. IGNITION (ENJAMBRE FANTASMA)
    console.log(chalk.bold.magenta("\n🚀 [FASE 4]: IGNICIÓN DEL ENJAMBRE (Phantom Swarm)"));
    await triggerWorkflow('forensic-grid.yml', { grid_size: '20' });

    // No esperamos a que termine el enjambre (dura 6 horas), solo confirmamos el arranque
    console.log(chalk.green("   ✅ Señal de ignición confirmada."));
    console.log(chalk.gray("   Los nodos están arrancando en segundo plano. Monitoree el Dashboard."));

    console.log(chalk.bold.bgGreen.black("\n 🏁 OPERACIÓN EXITOSA: EL SISTEMA ESTÁ EN GUERRA. \n"));
}

executeGlobalDeployment();
// FIN DEL ARCHIVO [tools/scripts/ops-commander.ts]

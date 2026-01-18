// INICIO DEL ARCHIVO [tools/scripts/health-checks/verify-github-pat.ts]
/**
 * =================================================================
 * APARATO: GITHUB PAT BALLISTIC TESTER (V3.1 - IGNITION ALIGNED)
 * CLASIFICACIÓN: OPS DIAGNOSTIC (ESTRATO L6)
 * RESPONSABILIDAD: DISECCIÓN DE PERMISOS C2 Y VISIBILIDAD
 * =================================================================
 */

import axios from 'axios';
import chalk from 'chalk';
import * as dotenv from 'dotenv';

dotenv.config();

async function execute_deep_audit() {
    console.log(chalk.bold.magenta("\n🕵️ [C2_AUDIT]: Initiating Deep GitHub Authority Scan..."));

    const PAT = process.env.GITHUB_PAT?.trim();
    const OWNER = process.env.GITHUB_OWNER?.trim();
    const REPO = process.env.GITHUB_REPO?.trim();

    // 1. VALIDACIÓN DE ENTORNO
    if (!PAT || !OWNER || !REPO) {
        console.error(chalk.red("❌ [CONFIG_VOID]: Missing credentials in .env"));
        console.log(chalk.yellow("   Required: GITHUB_PAT, GITHUB_OWNER, GITHUB_REPO"));
        process.exit(1);
    }

    console.log(chalk.gray(`   Target Coordinates: ${chalk.white(OWNER)} / ${chalk.white(REPO)}`));
    console.log(chalk.gray(`   Token Fingerprint:  ${chalk.cyan(PAT.substring(0, 4) + "..." + PAT.slice(-4))}`));

    const headers = {
        'Authorization': `Bearer ${PAT}`,
        'Accept': 'application/vnd.github.v3+json'
    };

    try {
        // 2. NIVEL 1: IDENTIDAD Y SCOPES
        console.log(chalk.bold.blue("\n🧪 PHASE 1: IDENTITY HANDSHAKE"));
        const user_res = await axios.get('https://api.github.com/user', { headers });

        const scopes_header = user_res.headers['x-oauth-scopes'] || "";
        const active_scopes = scopes_header.split(',').map((s: string) => s.trim());

        console.log(chalk.green(`   ✅ AUTHENTICATED AS: ${user_res.data.login}`));
        console.log(chalk.cyan(`   🔑 DETECTED SCOPES: [${active_scopes.join(', ')}]`));

        // Validación Estricta de Scopes
        const required_scopes = ['repo', 'workflow'];
        const missing_scopes = required_scopes.filter(s => !active_scopes.includes(s));

        if (missing_scopes.length > 0) {
            console.error(chalk.bold.red("\n❌ [PERMISSION_DENIED]: Token lacks critical capabilities."));
            console.error(chalk.yellow(`   MISSING SCOPES: ${missing_scopes.join(', ')}`));
            console.error(chalk.gray("   ACTION: Generate a new PAT with 'repo' and 'workflow' checked."));
            process.exit(1);
        } else {
            console.log(chalk.green("   ✅ SCOPE INTEGRITY: OPTIMAL"));
        }

        // 3. NIVEL 2: VISIBILIDAD DE OBJETIVO
        console.log(chalk.bold.blue("\n🧪 PHASE 2: TARGET ACQUISITION"));

        try {
            await axios.get(`https://api.github.com/repos/${OWNER}/${REPO}`, { headers });
            console.log(chalk.green(`   ✅ REPOSITORY FOUND: Access Confirmed.`));
        } catch (repo_error: any) {
            if (repo_error.response?.status === 404) {
                console.error(chalk.bold.red("❌ [TARGET_INVISIBLE_404]:"));
                console.error(chalk.red("   El token es válido, pero NO PUEDE VER el repositorio."));
            } else {
                throw repo_error;
            }
            process.exit(1);
        }

        // 4. NIVEL 3: CAPACIDAD DE DISPARO
        console.log(chalk.bold.blue("\n🧪 PHASE 3: WORKFLOW INSPECTION"));
        const workflow_url = `https://api.github.com/repos/${OWNER}/${REPO}/actions/workflows`;
        const wf_res = await axios.get(workflow_url, { headers });

        const workflows = wf_res.data.workflows || [];
        // ✅ FIX: Validación contra el nuevo nombre de archivo
        const target_wf = workflows.find((w: any) => w.path.includes('provisioner-ignition.yml'));

        if (target_wf) {
            console.log(chalk.green(`   ✅ WORKFLOW FOUND: ${target_wf.name} (ID: ${target_wf.id})`));
            console.log(chalk.green(`   ✅ STATE: ${target_wf.state}`));
        } else {
            console.warn(chalk.bold.yellow("⚠️ [WORKFLOW_MISSING]: 'provisioner-ignition.yml' not found in default branch."));
            console.warn(chalk.gray("   Ignition requests will fail until the workflow file is pushed to main."));
        }

        console.log(chalk.bold.bgGreen.black("\n 🏁 SYSTEM C2 READY: AUTHORITY CERTIFIED. \n"));

    } catch (error: any) {
        console.error(chalk.bgRed.white("\n🔥 [FATAL_AUDIT_ERROR]:"));
        const status = error.response?.status;
        const msg = error.response?.data?.message || error.message;
        console.error(chalk.red(`   Status: ${status}`));
        console.error(chalk.red(`   Error: ${msg}`));
        process.exit(1);
    }
}

execute_deep_audit();
// FIN DEL ARCHIVO [tools/scripts/health-checks/verify-github-pat.ts]

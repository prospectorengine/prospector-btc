/**
 * =================================================================
 * APARATO: GITHUB PAT BALLISTIC TESTER (V3.2 - AUTO-CLEAN ENABLED)
 * CLASIFICACIÓN: OPS DIAGNOSTIC (ESTRATO L6)
 * RESPONSABILIDAD: DISECCIÓN Y PURGA DE CREDENCIALES C2
 * =================================================================
 */

import axios from 'axios';
import chalk from 'chalk';
import * as dotenv from 'dotenv';

dotenv.config();

async function execute_deep_audit() {
    console.log(chalk.bold.magenta("\n🕵️ [C2_AUDIT]: Initiating Deep GitHub Authority Scan V3.2..."));

    // ✅ REPARACIÓN SOBERANA: Purgar comillas y espacios accidentales del .env
    const raw_pat = process.env.GITHUB_PAT || "";
    const PAT = raw_pat.trim().replace(/^["'](.+(?=["']$))["']$/, '$1').replace(/['"]/g, '');

    const OWNER = process.env.GITHUB_OWNER?.trim();
    const REPO = process.env.GITHUB_REPO?.trim();

    if (!PAT || !OWNER || !REPO) {
        console.error(chalk.red("❌ [CONFIG_VOID]: Missing credentials in .env"));
        process.exit(1);
    }

    // Visualizamos el fingerprint LIMPIO para confirmar la purga
    console.log(chalk.gray(`   Target: ${OWNER}/${REPO}`));
    console.log(chalk.gray(`   Clean Token Fingerprint: ${chalk.cyan(PAT.substring(0, 4) + "..." + PAT.slice(-4))}`));

    const headers = {
        'Authorization': `Bearer ${PAT}`,
        'Accept': 'application/vnd.github.v3+json',
        'User-Agent': 'Prospector-C2-V3.2'
    };

    try {
        console.log(chalk.bold.blue("\n🧪 PHASE 1: IDENTITY HANDSHAKE"));
        const user_res = await axios.get('https://api.github.com/user', { headers });
        console.log(chalk.green(`   ✅ AUTHENTICATED AS: ${user_res.data.login}`));

        const scopes = (user_res.headers['x-oauth-scopes'] as string || "").split(',').map(s => s.trim());
        console.log(chalk.cyan(`   🔑 SCOPES: [${scopes.join(', ')}]`));

        if (!scopes.includes('repo') || !scopes.includes('workflow')) {
            console.error(chalk.red("   ❌ ERROR: Missing 'repo' or 'workflow' scopes."));
            process.exit(1);
        }

        console.log(chalk.bold.green("\n🏁 SYSTEM C2 READY: AUTHORITY CERTIFIED.\n"));

    } catch (error: any) {
        const status = error.response?.status;
        const msg = error.response?.data?.message || error.message;
        console.error(chalk.bgRed.white(`\n🔥 FATAL_AUDIT_ERROR [${status}]: ${msg}`));
        console.log(chalk.yellow("👉 TIP: Asegúrese de que el PAT no tenga comillas ni espacios en el archivo .env"));
        process.exit(1);
    }
}

execute_deep_audit();

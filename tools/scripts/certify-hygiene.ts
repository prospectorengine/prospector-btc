/**
 * =================================================================
 * APARATO: HYGIENE CERTIFIER (V1.0)
 * RESPONSABILIDAD: Validación de Linting en librerías críticas
 * =================================================================
 */

import { execSync } from 'child_process';
import chalk from 'chalk';

const TARGETS = ['infra-supabase', 'api-client-ts'];

function runLintAudit() {
    console.log(chalk.bold.magenta("\n🕵️ AUDITORÍA DE HIGIENE (LINT): Prospector Frontend\n"));

    for (const target of TARGETS) {
        process.stdout.write(chalk.white(`  🚀 Auditando lints para [${target}]... `));
        try {
            execSync(`npx nx lint ${target}`, { stdio: 'pipe' });
            console.log(chalk.green("✅ CLEAN"));
        } catch (error: any) {
            console.log(chalk.red("❌ FAIL"));
            console.error(chalk.yellow(`\n     DETALLE DEL FALLO [${target}]:\n`));
            console.error(error.stdout.toString());
            process.exit(1);
        }
    }

    console.log(chalk.bold.bgGreen.black("\n🏁 CERTIFICACIÓN DE HIGIENE COMPLETADA: Sin residuos detectados. \n"));
}

runLintAudit();

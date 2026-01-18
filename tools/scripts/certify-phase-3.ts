/**
 * =================================================================
 * APARATO: PHASE 3 HYGIENE CERTIFIER
 * RESPONSABILIDAD: Validación de Linting en Vault y Provisioner
 * =================================================================
 */

import { execSync } from 'child_process';
import chalk from 'chalk';

const NEW_TARGETS = ['core-client-vault', 'provisioner'];

function runAudit() {
    console.log(chalk.bold.blue("\n🧪 AUDITORÍA FASE 3: Vault & Provisioner\n"));

    for (const target of NEW_TARGETS) {
        process.stdout.write(chalk.white(`  🚀 Ejecutando linter para [${target}]... `));
        try {
            execSync(`npx nx lint ${target}`, { stdio: 'pipe' });
            console.log(chalk.green("✅ CLEAN"));
        } catch (error: any) {
            console.log(chalk.red("❌ FAIL"));
            console.error(chalk.yellow(`\n     LOG DE ERROR [${target}]:\n`));
            console.error(error.stdout.toString());
            process.exit(1);
        }
    }

    console.log(chalk.bold.bgBlue.white("\n🏁 FASE 3: Certificación Parcial Exitosa. \n"));
}

runAudit();

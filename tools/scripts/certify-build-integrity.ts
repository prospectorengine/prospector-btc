/**
 * =================================================================
 * APARATO: BUILD INTEGRITY CERTIFIER (V3.0)
 * RESPONSABILIDAD: Ejecución semántica del compilador de TS
 * =================================================================
 */

import { execSync } from 'child_process';
import chalk from 'chalk';

const TARGET_CONFIG = 'libs/infra/api-client-ts/tsconfig.lib.json';

function runRealCompilerTest() {
    console.log(chalk.bold.magenta(`\n🕵️ AUDITORÍA SEMÁNTICA: Probando tsc en ${TARGET_CONFIG}\n`));

    try {
        console.log(chalk.cyan(`  🚀 Ejecutando: npx tsc -p ${TARGET_CONFIG} --noEmit`));

        // Ejecutamos el comando real. Si hay errores de rootDir, lanzará excepción.
        execSync(`npx tsc -p ${TARGET_CONFIG} --noEmit`, { stdio: 'pipe' });

        console.log(chalk.bold.green(`\n✅ CERTIFICACIÓN EXITOSA: El compilador valida la estructura.`));
        console.log(chalk.green(`   Las fronteras del aparato están selladas y aisladas.`));

    } catch (error: any) {
        console.log(chalk.bold.red(`\n❌ FALLO DE COMPILACIÓN REAL:`));
        const output = error.stdout?.toString() || error.stderr?.toString() || error.message;

        // Filtrado de ruido para mostrar solo los errores de TS
        const lines = output.split('\n').filter((l: string) => l.includes('error TS'));
        console.error(chalk.yellow(lines.join('\n')));

        process.exit(1);
    }
}

runRealCompilerTest();

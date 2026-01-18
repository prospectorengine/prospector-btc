import { execSync } from 'child_process';
import chalk from 'chalk';

function certifyNextConfig() {
    console.log(chalk.bold.cyan("\n🧪 AUDITORÍA DE CONFIGURACIÓN: web-dashboard\n"));

    try {
        process.stdout.write(chalk.white("  🛰️  Verificando sintaxis de next.config.js... "));
        // Intentamos cargar la configuración vía Node.js
        execSync(`node -e "require('./apps/web-dashboard/next.config.js')"`, { stdio: 'pipe' });
        console.log(chalk.green("OK"));

        process.stdout.write(chalk.white("  🛰️  Ejecutando validación de tipos local... "));
        execSync(`npx tsc -p apps/web-dashboard/tsconfig.json --noEmit`, { stdio: 'pipe' });
        console.log(chalk.green("OK"));

        console.log(chalk.bold.bgGreen.black("\n🚀 DASHBOARD NIVELADO: El aparato es estable para Vercel. \n"));
    } catch (error: any) {
        console.log(chalk.red("\n❌ FALLO DE ESTABILIDAD:"));
        console.error(chalk.yellow(error.stdout?.toString() || error.message));
        process.exit(1);
    }
}

certifyNextConfig();

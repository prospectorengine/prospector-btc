import { execSync } from 'child_process';
import chalk from 'chalk';

function certifyWorkspaceGraph() {
    console.log(chalk.bold.cyan("\n🕵️ AUDITORÍA DE INTEGRIDAD DE GRAFO NX\n"));

    try {
        process.stdout.write(chalk.white("  🛰️  Validando grafo de dependencias... "));
        // Esto detecta dependencias circulares o rutas mal mapeadas
        execSync(`npx nx graph --file=dist/graph.json`, { stdio: 'pipe' });
        console.log(chalk.green("OK"));

        process.stdout.write(chalk.white("  🛰️  Simulando build local de Next.js... "));
        // Ejecutamos solo el build de la app para ver si Turbopack protesta
        execSync(`npx nx build web-dashboard --skip-nx-cache`, { stdio: 'pipe' });
        console.log(chalk.green("OK"));

        console.log(chalk.bold.bgGreen.black("\n🚀 GRAFO CERTIFICADO: El sistema es estable para el despliegue. \n"));
    } catch (error: any) {
        console.log(chalk.red("\n❌ FALLO DE INTEGRIDAD EN EL GRAFO:"));
        console.error(chalk.yellow(error.stdout?.toString() || error.message));
        process.exit(1);
    }
}

certifyWorkspaceGraph();

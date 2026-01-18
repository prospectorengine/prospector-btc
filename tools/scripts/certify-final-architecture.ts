import * as fs from 'fs';
import * as path from 'path';
import chalk from 'chalk';

const CHECKLIST = [
    { name: 'infra-supabase', path: 'libs/infra/supabase/tsconfig.lib.json', expectedRoot: '.' },
    { name: 'api-client-ts', path: 'libs/infra/api-client-ts/tsconfig.lib.json', expectedRoot: '../../..' },
    { name: 'web-dashboard', path: 'apps/web-dashboard/tsconfig.json', expectedRoot: '../..' }
];

function finalAudit() {
    console.log(chalk.bold.magenta("\n🕵️ CERTIFICACIÓN FINAL DE ARQUITECTURA SOBERANA\n"));
    let allPassed = true;

    CHECKLIST.forEach(item => {
        const fullPath = path.join(process.cwd(), item.path);
        if (!fs.existsSync(fullPath)) {
            console.log(chalk.red(`  ❌ ERROR: No se encuentra ${item.name}`));
            allPassed = false;
            return;
        }

        const config = JSON.parse(fs.readFileSync(fullPath, 'utf-8'));
        const root = config.compilerOptions.rootDir;

        if (root === item.expectedRoot) {
            console.log(chalk.green(`  🟢 ${item.name.padEnd(15)}: rootDir [${root}] OK`));
        } else {
            console.log(chalk.red(`  🔴 ${item.name.padEnd(15)}: rootDir INCORRECTO (Esperado: ${item.expectedRoot}, Recibido: ${root})`));
            allPassed = false;
        }
    });

    if (allPassed) {
        console.log(chalk.bold.bgGreen.black("\n🚀 SISTEMA LISTO PARA EL DESPLIEGUE DEFINITIVO \n"));
    } else {
        console.log(chalk.bold.bgRed.white("\n⚠️ FALLO DETECTADO: El sistema no está nivelado. \n"));
        process.exit(1);
    }
}

finalAudit();

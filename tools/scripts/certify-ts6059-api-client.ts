/**
 * =================================================================
 * APARATO: TS6059 API-CLIENT VERIFIER (V1.0)
 * RESPONSABILIDAD: Certificación de Soberanía de Root para el Cliente API
 * =================================================================
 */

import * as fs from 'fs';
import * as path from 'path';
import chalk from 'chalk';

const TARGET_LIB = 'libs/infra/api-client-ts';
const TARGET_CONFIG = path.join(process.cwd(), TARGET_LIB, 'tsconfig.lib.json');

function runAudit() {
    console.log(chalk.bold.blue("\n🧪 AUDITORÍA DE FRONTERA TS6059: api-client-ts\n"));

    if (!fs.existsSync(TARGET_CONFIG)) {
        console.error(chalk.red("❌ ERROR: El aparato de configuración no existe en la ruta esperada."));
        process.exit(1);
    }

    const config = JSON.parse(fs.readFileSync(TARGET_CONFIG, 'utf-8'));
    const rootDir = config.compilerOptions.rootDir;

    console.log(chalk.white(`  1. Verificando expansión de rootDir...`));
    // El valor debe ser "../../.." para cubrir la raíz del monorepo
    if (rootDir === "../../..") {
        console.log(chalk.green("     ✅ CORRECTO: rootDir elevado a la soberanía del monorepo."));
    } else {
        console.error(chalk.red(`     ❌ FALLO: rootDir restrictivo detectado: "${rootDir}".`));
        process.exit(1);
    }

    console.log(chalk.white(`  2. Verificando inclusiones de estratos externos...`));
    const includes = config.include || [];
    const hasContracts = includes.some((i: string) => i.includes('api-contracts'));

    if (hasContracts) {
        console.log(chalk.green("     ✅ CORRECTO: Estratos de contratos incluidos en el programa."));
    } else {
        console.error(chalk.red("     ❌ FALLO: No se detectan inclusiones de dependencias externas."));
        process.exit(1);
    }

    console.log(chalk.bold.bgGreen.black("\n🚀 PRUEBA SUPERADA: El Cliente API está blindado contra TS6059. \n"));
}

runAudit();

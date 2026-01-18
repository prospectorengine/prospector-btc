/**
 * =================================================================
 * APARATO: TS6059 BOUNDARY VERIFIER (V1.0)
 * RESPONSABILIDAD: Certificación de paridad de RootDir e Inclusiones
 * =================================================================
 */

import * as fs from 'fs';
import * as path from 'path';
import chalk from 'chalk';

const TARGET_LIB = 'libs/infra/supabase';
const TARGET_CONFIG = path.join(process.cwd(), TARGET_LIB, 'tsconfig.lib.json');

function runAudit() {
    console.log(chalk.bold.cyan("\n🔍 AUDITORÍA DE FRONTERA TS6059: infra-supabase\n"));

    if (!fs.existsSync(TARGET_CONFIG)) {
        console.error(chalk.red("❌ ERROR: El aparato de configuración no existe."));
        process.exit(1);
    }

    const config = JSON.parse(fs.readFileSync(TARGET_CONFIG, 'utf-8'));
    const rootDir = config.compilerOptions.rootDir;

    console.log(chalk.white(`  1. Verificando rootDir...`));
    if (rootDir === ".") {
        console.log(chalk.green("     ✅ CORRECTO: rootDir nivelado a la raíz de la librería."));
    } else {
        console.error(chalk.red(`     ❌ FALLO: rootDir sigue en "${rootDir}". Bloqueo TS6059 inminente.`));
        process.exit(1);
    }

    console.log(chalk.white(`  2. Verificando rastro de dependencias...`));
    const rawMainConfig = fs.readFileSync(path.join(process.cwd(), TARGET_LIB, 'tsconfig.json'), 'utf-8');
    if (rawMainConfig.includes('"path": "../../domain/api-contracts"')) {
        console.log(chalk.green("     ✅ CORRECTO: Referencia a api-contracts detectada."));
    } else {
        console.error(chalk.red("     ❌ FALLO: No hay enlace formal hacia api-contracts."));
        process.exit(1);
    }

    console.log(chalk.bold.bgGreen.black("\n🚀 PRUEBA SUPERADA: La frontera estructural es segura para el deploy. \n"));
}

runAudit();

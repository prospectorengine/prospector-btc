import { execSync } from 'child_process';
import chalk from 'chalk';
import * as fs from 'fs';

function runDeepAudit() {
    console.log(chalk.bold.magenta("\n🕵️ INICIANDO AUDITORÍA SOBERANA DE COMPILACIÓN\n"));

    try {
        // Limpieza de artefactos previos para evitar falsos positivos
        process.stdout.write(chalk.white("  🧹 Limpiando dist/out-tsc... "));
        if (fs.existsSync('dist/out-tsc')) fs.rmSync('dist/out-tsc', { recursive: true });
        console.log(chalk.green("OK"));

        // 1. Compilar Hoja (Contratos)
        process.stdout.write(chalk.white("  🛰️  Fase 1: Compilando api-contracts... "));
        execSync(`npx tsc -p libs/domain/api-contracts/tsconfig.lib.json`, { stdio: 'pipe' });
        console.log(chalk.green("OK"));

        // 2. Compilar Intermedio (Supabase)
        process.stdout.write(chalk.white("  🛰️  Fase 2: Compilando infra-supabase... "));
        execSync(`npx tsc -p libs/infra/supabase/tsconfig.lib.json`, { stdio: 'pipe' });
        console.log(chalk.green("OK"));

        // 3. Compilar Target (API Client)
        process.stdout.write(chalk.white("  🛰️  Fase 3: Compilando api-client-ts... "));
        execSync(`npx tsc -p libs/infra/api-client-ts/tsconfig.lib.json`, { stdio: 'pipe' });
        console.log(chalk.green("OK"));

        console.log(chalk.bold.bgGreen.black("\n🚀 CERTIFICACIÓN TOTAL: El sistema respeta las leyes de TypeScript. \n"));

    } catch (error: any) {
        console.log(chalk.bold.red("\n❌ FALLO ESTRUCTURAL DETECTADO:"));
        const output = error.stdout?.toString() || error.message;
        console.error(chalk.yellow(output));
        process.exit(1);
    }
}

runDeepAudit();

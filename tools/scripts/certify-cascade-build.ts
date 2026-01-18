// INICIO DEL ARCHIVO [tools/scripts/certify-cascade-build.ts]
/**
 * =================================================================
 * APARATO: CASCADE BUILD CERTIFIER (V1.0)
 * RESPONSABILIDAD: Verificación de la cadena de compilación incremental
 * =================================================================
 */

import { execSync } from 'child_process';
import * as fs from 'fs';
import * as path from 'path';
import chalk from 'chalk';

const DIST_BASE = path.join(process.cwd(), 'dist/out-tsc');

function verifyArtifact(libPath: string) {
    const artifact = path.join(DIST_BASE, libPath, 'index.d.ts');
    if (!fs.existsSync(artifact)) {
        throw new Error(`Artefacto no encontrado: ${artifact}`);
    }
    return true;
}

function runCascadeAudit() {
    console.log(chalk.bold.magenta("\n🌊 INICIANDO AUDITORÍA DE BUILD EN CASCADA\n"));

    try {
        // 1. Limpieza
        console.log(chalk.white("  🧹 Limpiando artefactos previos..."));
        if (fs.existsSync('dist/out-tsc')) fs.rmSync('dist/out-tsc', { recursive: true, force: true });

        // 2. Compilar Nivel 0: Contracts
        console.log(chalk.cyan("  🏗️  Compilando [api-contracts]..."));
        execSync(`npx nx build api-contracts`, { stdio: 'inherit' });
        verifyArtifact('libs/domain/api-contracts/src');
        console.log(chalk.green("     ✅ Contracts cristalizados."));

        // 3. Compilar Nivel 1: Supabase (Debe usar d.ts de contracts)
        console.log(chalk.cyan("  🏗️  Compilando [infra-supabase]..."));
        execSync(`npx nx build infra-supabase`, { stdio: 'inherit' });
        verifyArtifact('libs/infra/supabase/src');
        console.log(chalk.green("     ✅ Supabase enlazado correctamente."));

        // 4. Compilar Nivel 2: API Client (Debe usar d.ts de contracts y supabase)
        console.log(chalk.cyan("  🏗️  Compilando [api-client-ts]..."));
        execSync(`npx nx build api-client-ts`, { stdio: 'inherit' });
        verifyArtifact('libs/infra/api-client-ts/src');
        console.log(chalk.green("     ✅ API Client ensamblado."));

        console.log(chalk.bold.bgGreen.black("\n🚀 SISTEMA DE CASCADA FUNCIONAL: Despliegue autorizado. \n"));

    } catch (error: any) {
        console.error(chalk.bold.red("\n❌ FALLO EN LA CADENA DE MONTAJE:"));
        console.error(chalk.red(error.message));
        process.exit(1);
    }
}

runCascadeAudit();
// FIN DEL ARCHIVO [tools/scripts/certify-cascade-build.ts]

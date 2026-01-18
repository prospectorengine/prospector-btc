/**
 * =================================================================
 * APARATO: V16 IGNITION CERTIFIER (V1.0)
 * RESPONSABILIDAD: Verificación de paridad de configuración Next 16
 * =================================================================
 */

import * as fs from 'fs';
import * as path from 'path';
import chalk from 'chalk';

function runIgnitionAudit() {
    console.log(chalk.bold.magenta("\n🕵️ AUDITORÍA DE PRE-IGNICIÓN: Protocolo 2026\n"));

    // 1. Verificar tsconfig dashboard
    const dashTsPath = path.join(process.cwd(), 'apps/web-dashboard/tsconfig.json');
    const dashTs = JSON.parse(fs.readFileSync(dashTsPath, 'utf-8'));
    if (dashTs.references) {
        console.error(chalk.red("❌ FALLO: El Dashboard aún tiene 'references'. Turbopack fallará."));
        process.exit(1);
    } else {
        console.log(chalk.green("✅ DASHBOARD TSCONFIG: Aislamiento confirmado."));
    }

    // 2. Verificar next.config raíz
    const nextConfig = fs.readFileSync(path.join(process.cwd(), 'apps/web-dashboard/next.config.js'), 'utf-8');
    if (nextConfig.includes('reactCompiler: true') && !nextConfig.includes('experimental: { reactCompiler')) {
        console.log(chalk.green("✅ NEXT_CONFIG: Estructura nativa V16 detectada."));
    } else {
        console.error(chalk.red("❌ FALLO: next.config.js no está nivelado a la V16."));
        process.exit(1);
    }

    console.log(chalk.bold.bgGreen.black("\n🚀 SISTEMA LISTO: Fronteras selladas para Vercel. \n"));
}

runIgnitionAudit();

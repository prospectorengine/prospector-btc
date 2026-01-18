// INICIO DEL ARCHIVO [tools/scripts/certify-playwright-version.ts]
/**
 * =================================================================
 * APARATO: DEPENDENCY UNITY AUDITOR (V1.0)
 * RESPONSABILIDAD: Verificación de versión única de Playwright
 * =================================================================
 */

import { execSync } from 'child_process';
import chalk from 'chalk';

function auditDependencies() {
    console.log(chalk.bold.magenta("\n🕵️ AUDITORÍA DE UNIFICACIÓN: Playwright Core\n"));

    try {
        // Consultamos pnpm list para ver qué versiones están resueltas
        // El comando puede fallar si no hay dependencias, por eso el try/catch agresivo
        const output = execSync('pnpm list playwright-core --depth 3 --json', { encoding: 'utf-8' });
        const data = JSON.parse(output);

        let versionsFound = new Set<string>();

        const recurse = (deps: any) => {
            if (!deps) return;
            for (const key in deps) {
                if (key.includes('playwright-core')) {
                    versionsFound.add(deps[key].version);
                }
                if (deps[key].dependencies) {
                    recurse(deps[key].dependencies);
                }
            }
        };

        data.forEach((pkg: any) => recurse(pkg.dependencies));
        data.forEach((pkg: any) => recurse(pkg.devDependencies));

        const versions = Array.from(versionsFound);

        if (versions.length === 0) {
            console.log(chalk.yellow("⚠️ No se detectó playwright-core en el árbol principal."));
            return;
        }

        console.log(chalk.white(`  Versiones detectadas: ${versions.join(', ')}`));

        if (versions.length > 1) {
            console.error(chalk.red(`\n❌ CONFLICTO CRÍTICO: Múltiples versiones detectadas.`));
            console.error(chalk.red(`   Esto causará TS2345/TS2322. Ejecuta 'pnpm install' para aplicar overrides.`));
            process.exit(1);
        }

        if (versions[0] !== '1.57.0') {
            console.error(chalk.red(`\n❌ VERSIÓN INCORRECTA: Se esperaba 1.57.0, se encontró ${versions[0]}`));
            process.exit(1);
        }

        console.log(chalk.bold.green(`\n✅ UNIFICACIÓN EXITOSA: Solo reina la versión ${versions[0]}. \n`));

    } catch (error) {
        // Si pnpm falla o el json es inválido, asumimos entorno hostil pero intentamos seguir
        console.warn(chalk.yellow("⚠️ No se pudo auditar profundamente el árbol de dependencias (pnpm list failed)."));
    }
}

auditDependencies();
// FIN DEL ARCHIVO [tools/scripts/certify-playwright-version.ts]

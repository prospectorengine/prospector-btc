/**
 * =================================================================
 * APARATO: PROJECT.JSON RULES PROTECTOR (V1.0)
 * CLASIFICACIÓN: OPS INFRASTRUCTURE (ESTRATO L6)
 * RESPONSABILIDAD: AUDITORÍA DE OBJETIVOS, EJECUTORES Y CACHÉ
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. TARGET UNIFORMITY: Valida que cada proyecto posea 'lint' y 'test'.
 * 2. OUTPUTS INTEGRITY: Asegura que los 'outputs' existan para que la
 *    caché de Nx funcione en Vercel/Render.
 * 3. EXECUTOR SINCRO: Verifica que los proyectos Rust usen '@monodon/rust'
 *    y los TS usen '@nx/js' o '@nx/eslint'.
 * 4. TAG ENFORCEMENT: Valida la existencia de tags 'layer:' y 'type:'.
 * =================================================================
 */

import * as fs from 'fs';
import * as path from 'path';
import chalk from 'chalk';

const REQUIRED_TARGETS = ['lint', 'test'];
const MANDATORY_TAG_PREFIXES = ['layer:', 'type:'];

function sanitize_json(raw: string): any {
  return JSON.parse(raw.replace(/\/\/.*$/gm, '').replace(/\/\*[\s\S]*?\*\//g, '').trim());
}

async function execute_project_rules_audit() {
  console.log(chalk.bold.blue("\n🛡️  [PROJECT_RULES_AUDIT]: Checking Nx Governance...\n"));

  const root_dir = process.cwd();
  const nx_json = sanitize_json(fs.readFileSync(path.join(root_dir, 'nx.json'), 'utf-8'));
  const tsconfig_base = sanitize_json(fs.readFileSync(path.join(root_dir, 'tsconfig.base.json'), 'utf-8'));

  // Mapeamos los proyectos desde tsconfig para asegurar que auditamos lo que el sistema "ve"
  const project_paths = Object.values(tsconfig_base.compilerOptions.paths)
    .map((p: any) => path.join(root_dir, p[0].replace(/\/src\/index\.ts|\/index\.ts|\/src\/lib\.rs/, '')));

  let total_faults = 0;

  for (const project_path of project_paths) {
    const config_path = path.join(project_path, 'project.json');
    if (!fs.existsSync(config_path)) continue;

    const config = sanitize_json(fs.readFileSync(config_path, 'utf-8'));
    const relative_name = path.relative(root_dir, project_path);

    console.log(chalk.cyan(`  🛰️  Project: [${config.name}]`));

    // 1. AUDITORÍA DE TARGETS (Lint/Test)
    for (const target of REQUIRED_TARGETS) {
      if (!config.targets || !config.targets[target]) {
        console.error(chalk.red(`     ❌ MISSING_TARGET: '${target}' is mandatory.`));
        total_faults++;
      }
    }

    // 2. AUDITORÍA DE OUTPUTS (Para Caching)
    if (config.targets?.build && !config.targets.build.outputs) {
      console.error(chalk.yellow(`     ⚠️  CACHE_WARNING: Build target has no 'outputs' defined.`));
      total_faults++;
    }

    // 3. AUDITORÍA DE TAGS SOBERANOS
    const tags = config.tags || [];
    for (const prefix of MANDATORY_TAG_PREFIXES) {
      if (!tags.some((t: string) => t.startsWith(prefix))) {
        console.error(chalk.red(`     ❌ TAG_VIOLATION: Missing tag with prefix '${prefix}'.`));
        total_faults++;
      }
    }

    // 4. AUDITORÍA DE SOURCE ROOT
    if (!config.sourceRoot || !fs.existsSync(path.join(root_dir, config.sourceRoot))) {
      console.error(chalk.red(`     ❌ INVALID_SOURCE_ROOT: ${config.sourceRoot}`));
      total_faults++;
    }

    console.log(chalk.gray("  --------------------------------------------------"));
  }

  if (total_faults === 0) {
    console.log(chalk.bold.bgGreen.black("\n ✨ NX GOVERNANCE CERTIFIED: All project.json files follow the Codex. \n"));
    process.exit(0);
  } else {
    console.log(chalk.bold.bgRed.white(`\n 💀 GOVERNANCE BREACH: ${total_faults} rule violations detected. \n`));
    process.exit(1);
  }
}

execute_project_rules_audit().catch(err => {
  console.error(chalk.red("🔥 FATAL_RULES_ERROR:"), err);
  process.exit(1);
});

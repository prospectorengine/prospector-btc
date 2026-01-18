/**
 * =================================================================
 * APARATO: BOUNDARY & PACKAGE AUDITOR (V1.1 - ALIGNED)
 * CLASIFICACIÓN: OPS INFRASTRUCTURE (ESTRATO L6)
 * RESPONSABILIDAD: VALIDACIÓN DE ESTRATOS, TAGS Y NOMENCLATURA NPM
 * =================================================================
 */

import * as fs from 'fs';
import * as path from 'path';
import chalk from 'chalk';

function sanitize_json(raw: string): any {
  return JSON.parse(raw.replace(/\/\/.*$/gm, '').replace(/\/\*[\s\S]*?\*\//g, '').trim());
}

async function execute_boundary_audit() {
  console.log(chalk.bold.yellow("\n🛡️  [BOUNDARY_AUDIT]: Enforcing Layer Sovereignty (V1.1)...\n"));

  const root_dir = process.cwd();
  const tsconfig_base_path = path.join(root_dir, 'tsconfig.base.json');

  if (!fs.existsSync(tsconfig_base_path)) {
    console.error(chalk.red("❌ FATAL: tsconfig.base.json not found."));
    process.exit(1);
  }

  const tsconfig_base = sanitize_json(fs.readFileSync(tsconfig_base_path, 'utf-8'));
  const root_package = sanitize_json(fs.readFileSync(path.join(root_dir, 'package.json'), 'utf-8'));

  let total_faults = 0;

  for (const [alias, paths] of Object.entries(tsconfig_base.compilerOptions.paths)) {
    const project_rel_path = (paths as string[])[0].replace(/\/src\/index\.ts|\/index\.ts|\/src\/lib\.rs|\/src\/lib\.rs/, '');
    const project_path = path.join(root_dir, project_rel_path);

    const is_rust = fs.existsSync(path.join(project_path, 'Cargo.toml'));
    if (is_rust) continue;

    console.log(chalk.cyan(`  🛰️  Boundary Check: [${alias}]`));

    const package_json_path = path.join(project_path, 'package.json');
    if (!fs.existsSync(package_json_path)) {
      console.error(chalk.red("     ❌ MISSING_PACKAGE: TS Library requires package.json."));
      total_faults++;
    } else {
      const pkg = sanitize_json(fs.readFileSync(package_json_path, 'utf-8'));
      if (pkg.name !== alias) {
        console.error(chalk.red(`     ❌ NAME_MISMATCH: Package [${pkg.name}] vs Alias [${alias}].`));
        total_faults++;
      }
    }

    const project_json_path = path.join(project_path, 'project.json');
    if (fs.existsSync(project_json_path)) {
      const project = sanitize_json(fs.readFileSync(project_json_path, 'utf-8'));
      const tags = project.tags || [];
      const has_layer = tags.some((t: string) => t.startsWith('layer:'));
      const has_type = tags.some((t: string) => t.startsWith('type:'));

      if (!has_layer || !has_type) {
        console.error(chalk.red("     ❌ TAG_VOID: Missing 'layer:' or 'type:' tags."));
        total_faults++;
      }
    }
    console.log(chalk.gray("  --------------------------------------------------"));
  }

  if (total_faults === 0) {
    console.log(chalk.bold.bgGreen.black("\n ✨ BOUNDARIES CERTIFIED: Layer isolation is absolute. \n"));
    process.exit(0);
  } else {
    console.log(chalk.bold.bgRed.white(`\n 💀 SECURITY BREACH: ${total_faults} violations detected. \n`));
    process.exit(1);
  }
}

execute_boundary_audit();

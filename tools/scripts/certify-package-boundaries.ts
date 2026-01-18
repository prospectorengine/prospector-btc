/**
 * =================================================================
 * APARATO: BOUNDARY & PACKAGE AUDITOR (V1.2 - SANEADO)
 * CLASIFICACIÓN: OPS INFRASTRUCTURE (ESTRATO L6)
 * RESPONSABILIDAD: VALIDACIÓN DE ESTRATOS, TAGS Y NOMENCLATURA NPM
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. PARSING RESILIENCE: Motor de limpieza de comentarios para archivos JSON.
 * 2. PACKAGE ENFORCEMENT: Todo workspace TS debe tener un package.json coherente.
 * 3. LAYER SEGREGATION: Valida tags 'layer:' y 'type:' para el linter de Nx.
 * 4. NOMINAL PARITY: Asegura que el nombre en package.json coincida con el alias.
 * =================================================================
 */

import * as fs from 'fs';
import * as path from 'path';
import chalk from 'chalk';

/**
 * Motor de limpieza de impurezas para JSON.
 * Elimina comentarios de una sola línea (//) y de bloque (/* *\/).
 */
function sanitize_json_content(raw_content_string: string): any {
  const sanitized_string = raw_content_string
    .replace(/\/\/.*$/gm, '')
    .replace(/\/\*[\s\S]*?\*\//g, '')
    .trim();

  try {
    return JSON.parse(sanitized_string);
  } catch (error: unknown) {
    const message = error instanceof Error ? error.message : String(error);
    throw new Error(`JSON_PARSE_FAULT: ${message}`);
  }
}

async function execute_boundary_audit() {
  console.log(chalk.bold.yellow("\n🛡️  [BOUNDARY_AUDIT]: Enforcing Layer Sovereignty (V1.2)...\n"));

  const root_directory_path = process.cwd();
  const tsconfig_base_path = path.join(root_directory_path, 'tsconfig.base.json');

  if (!fs.existsSync(tsconfig_base_path)) {
    console.error(chalk.red("❌ FATAL: tsconfig.base.json not found in root."));
    process.exit(1);
  }

  const tsconfig_base = sanitize_json_content(fs.readFileSync(tsconfig_base_path, 'utf-8'));
  const paths_matrix = tsconfig_base.compilerOptions.paths;

  let global_fault_accumulator = 0;

  for (const [nominal_alias, target_locations] of Object.entries(paths_matrix)) {
    const locations_array = target_locations as string[];
    const relative_project_path = locations_array[0]
        .replace(/\/src\/index\.ts|\/index\.ts|\/src\/lib\.rs|\/src\/lib\.rs/, '');

    const absolute_project_path = path.join(root_directory_path, relative_project_path);

    // Ignoramos estratos Rust (gobernados por Cargo.toml)
    const is_rust_stratum = fs.existsSync(path.join(absolute_project_path, 'Cargo.toml'));
    if (is_rust_stratum) continue;

    console.log(chalk.cyan(`  🛰️  Boundary Check: [${nominal_alias}]`));

    let current_apparatus_faults = 0;

    // 1. AUDITORÍA DE PACKAGE.JSON
    const package_json_path = path.join(absolute_project_path, 'package.json');
    if (!fs.existsSync(package_json_path)) {
      console.error(chalk.red("     ❌ MISSING_PACKAGE: TS Library requires package.json for module resolution."));
      current_apparatus_faults++;
    } else {
      const package_json = sanitize_json_content(fs.readFileSync(package_json_path, 'utf-8'));
      if (package_json.name !== nominal_alias) {
        console.error(chalk.red(`     ❌ NAME_MISMATCH: Package name [${package_json.name}] must match alias [${nominal_alias}].`));
        current_apparatus_faults++;
      }
    }

    // 2. AUDITORÍA DE TAGS EN PROJECT.JSON (Crucial para Nx Boundaries)
    const project_json_path = path.join(absolute_project_path, 'project.json');
    if (fs.existsSync(project_json_path)) {
      const project_json = sanitize_json_content(fs.readFileSync(project_json_path, 'utf-8'));
      const tags_collection = project_json.tags || [];

      const has_layer_tag = tags_collection.some((tag: string) => tag.startsWith('layer:'));
      const has_type_tag = tags_collection.some((tag: string) => tag.startsWith('type:'));

      if (!has_layer_tag || !has_type_tag) {
        console.error(chalk.red("     ❌ TAG_VOID: Project missing 'layer:' or 'type:' tags for boundary enforcement."));
        current_apparatus_faults++;
      }
    }

    if (current_apparatus_faults === 0) {
      console.log(chalk.green("     ✅ BOUNDARY_STATUS: SECURE"));
    } else {
      global_fault_accumulator += current_apparatus_faults;
    }
    console.log(chalk.gray("  --------------------------------------------------"));
  }

  // --- REPORTE DE SENTENCIA FINAL ---
  if (global_fault_accumulator === 0) {
    console.log(chalk.bold.bgGreen.black("\n ✨ BOUNDARIES CERTIFIED: Layer isolation is absolute. \n"));
    process.exit(0);
  } else {
    console.log(chalk.bold.bgRed.white(`\n 💀 SECURITY BREACH: ${global_fault_accumulator} violations detected. \n`));
    process.exit(1);
  }
}

execute_boundary_audit().catch((fatal_error: unknown) => {
    const error_message = fatal_error instanceof Error ? fatal_error.message : String(fatal_error);
    console.error(chalk.bgRed.white("🔥 [FATAL_BOUNDARY_AUDIT_COLLAPSE]:"), error_message);
    process.exit(1);
});

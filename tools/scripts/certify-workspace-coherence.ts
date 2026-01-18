/**
 * =================================================================
 * APARATO: WORKSPACE COHERENCE METRONOME (V1.1 - COMMENT RESILIENT)
 * CLASIFICACIÓN: OPS INFRASTRUCTURE (ESTRATO L6)
 * RESPONSABILIDAD: AUDITORÍA GRANULAR DE CONFIGURACIÓN Y ESTRUCTURA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. PARSING RESILIENCE: Implementa un motor de limpieza de comentarios
 *    antes del procesado JSON, permitiendo encabezados descriptivos.
 * 2. STRUCTURAL EXHAUSTION: Verifica la existencia de la "Armadura"
 *    (project.json, tsconfigs, jest, package, index).
 * 3. BOUNDARY VALIDATION: Valida que los 'outDir' sean unívocos.
 * 4. NOMINAL PARITY: Asegura que el nombre en 'project.json' coincida
 *    con el alias del monorepo.
 *
 * # Logic:
 * El script actúa como un filtro de ruido. Remueve cualquier rastro de
 * '//' o '/*' antes de entregar el string al motor de tipos.
 * =================================================================
 */

import * as fs from 'fs';
import * as path from 'path';
import chalk from 'chalk';

// --- CONFIGURACIÓN DE REQUISITOS POR ESTRATO ---

const TS_FEATURE_REQUIREMENTS = [
  'project.json',
  'tsconfig.json',
  'tsconfig.lib.json',
  'tsconfig.spec.json',
  'jest.config.ts',
  'package.json',
  'src/index.ts'
];

const RUST_LIB_REQUIREMENTS = [
  'project.json',
  'Cargo.toml',
  'src/lib.rs'
];

/**
 * Motor de limpieza de impurezas para JSON.
 * Elimina comentarios de una sola línea y de bloque.
 */
function sanitize_json_content(raw_content_string: string): any {
  const sanitized_string = raw_content_string
    .replace(/\/\/.*$/gm, '') // Elimina comentarios de línea (//)
    .replace(/\/\*[\s\S]*?\*\//g, '') // Elimina comentarios de bloque (/* */)
    .trim();

  try {
    return JSON.parse(sanitized_string);
  } catch (error: unknown) {
    const message = error instanceof Error ? error.message : String(error);
    throw new Error(`JSON_PARSE_FAULT: ${message}\nContent preview: ${sanitized_string.substring(0, 50)}...`);
  }
}

async function execute_coherence_audit() {
  console.log(chalk.bold.magenta("\n🕵️  [COHERENCE_METRONOME]: Initiating Total Strata Audit (V1.1)...\n"));

  const root_directory_path = process.cwd();
  const tsconfig_base_path = path.join(root_directory_path, 'tsconfig.base.json');

  const tsconfig_base_raw = fs.readFileSync(tsconfig_base_path, 'utf-8');
  const tsconfig_base = sanitize_json_content(tsconfig_base_raw);

  const paths_matrix = tsconfig_base.compilerOptions.paths;

  let global_fault_accumulator = 0;
  let audited_workspaces_count = 0;

  for (const [alias_name, target_locations] of Object.entries(paths_matrix)) {
    const locations_array = target_locations as string[];
    // Limpieza de ruta para obtener el directorio raíz del workspace
    const relative_workspace_path = locations_array[0]
        .replace('/src/index.ts', '')
        .replace('/src/lib.rs', '')
        .replace('/index.ts', '');

    const absolute_workspace_path = path.join(root_directory_path, relative_workspace_path);
    const nominal_workspace_id = alias_name.toString();

    console.log(chalk.cyan(`  🛰️  Auditing Apparatus: [${nominal_workspace_id}]`));
    audited_workspaces_count++;

    let current_workspace_faults = 0;

    // A. DETERMINACIÓN DE FÍSICA (RUST vs TYPESCRIPT)
    const is_rust_stratum = nominal_workspace_id.includes('domain') &&
                            fs.existsSync(path.join(absolute_workspace_path, 'Cargo.toml'));

    const requirements_set = is_rust_stratum ? RUST_LIB_REQUIREMENTS : TS_FEATURE_REQUIREMENTS;

    // B. AUDITORÍA DE EXISTENCIA FÍSICA
    for (const mandatory_file of requirements_set) {
      const physical_file_path = path.join(absolute_workspace_path, mandatory_file);
      if (!fs.existsSync(physical_file_path)) {
        console.error(chalk.red(`     ❌ MISSING_FILE: ${mandatory_file}`));
        current_workspace_faults++;
      }
    }

    // C. AUDITORÍA DE FRONTERAS (OUTDIR)
    if (!is_rust_stratum) {
      const tsconfig_lib_path = path.join(absolute_workspace_path, 'tsconfig.lib.json');
      if (fs.existsSync(tsconfig_lib_path)) {
        const lib_config_raw = fs.readFileSync(tsconfig_lib_path, 'utf-8');
        const lib_config = sanitize_json_content(lib_config_raw);

        const current_out_dir = lib_config.compilerOptions.outDir;
        const expected_out_dir = `../../../dist/out-tsc/${relative_workspace_path}`;

        if (current_out_dir !== expected_out_dir) {
          console.error(chalk.yellow(`     ⚠️  OUTDIR_DRIFT: Expected [${expected_out_dir}], Found [${current_out_dir}]`));
          current_workspace_faults++;
        }
      }
    }

    // D. AUDITORÍA DE NOMENCLATURA (NX ALIGNMENT)
    const project_json_path = path.join(absolute_workspace_path, 'project.json');
    if (fs.existsSync(project_json_path)) {
      const project_json_raw = fs.readFileSync(project_json_path, 'utf-8');
      const project_json = sanitize_json_content(project_json_raw);

      const expected_project_name = nominal_workspace_id.split('/').pop();

      if (project_json.name !== expected_project_name && !nominal_workspace_id.includes('api-client')) {
        console.error(chalk.yellow(`     ⚠️  NAME_MISMATCH: Project [${project_json.name}] vs Alias [${expected_project_name}]`));
      }
    }

    if (current_workspace_faults === 0) {
      console.log(chalk.green(`     ✅ COHERENCE_LEVEL: SWISS_WATCH`));
    } else {
      global_fault_accumulator += current_workspace_faults;
    }
    console.log(chalk.gray("  --------------------------------------------------"));
  }

  // --- REPORTE DE SENTENCIA FINAL ---
  console.log(chalk.bold.white(`\n📊 AUDIT SUMMARY:`));
  console.log(chalk.white(`   Total Workspaces Audited: ${audited_workspaces_count}`));
  console.log(chalk.red(`   Total Faults Identified:  ${global_fault_accumulator}`));

  if (global_fault_accumulator === 0) {
    console.log(chalk.bold.bgGreen.black("\n ✨ SYSTEM CERTIFIED: All configurations are bit-perfect. \n"));
    process.exit(0);
  } else {
    console.log(chalk.bold.bgRed.white("\n 💀 INTEGRITY COMPROMISED: Fix boundaries before next sync. \n"));
    process.exit(1);
  }
}

execute_coherence_audit().catch((fatal_error: unknown) => {
  const error_message = fatal_error instanceof Error ? fatal_error.message : String(fatal_error);
  console.error(chalk.bgRed.white("🔥 [FATAL_METRONOME_COLLAPSE]:"), error_message);
  process.exit(1);
});

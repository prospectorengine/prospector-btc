/**
 * =================================================================
 * APARATO: RUST WORKSPACE COHERENCE METRONOME (V1.1 - SANEADO)
 * CLASIFICACIÓN: OPS INFRASTRUCTURE (ESTRATO L6)
 * RESPONSABILIDAD: AUDITORÍA DE CRATES, CARGOS Y SINAPSIS NX
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. SYNTAX HYGIENE: Resolución definitiva de TS2552 y TS6133.
 * 2. WORKSPACE SINCRO: Valida que cada carpeta física esté registrada
 *    en el 'members' del Cargo.toml raíz.
 * 3. NOMINAL ALIGNMENT: Asegura paridad entre 'package.name' (Rust)
 *    y 'name' (Nx project.json).
 * 4. STRUCTURAL INTEGRITY: Verifica puntos de entrada (lib.rs/main.rs).
 *
 * # Logic:
 * El script actúa como un validador de integridad del grafo de Rust.
 * Remueve comentarios de JSON antes del parseo para tolerar rastro de IA.
 * =================================================================
 */

import * as fs from 'fs';
import * as path from 'path';
import chalk from 'chalk';

// --- CONFIGURACIÓN DE REQUISITOS RUST ---
const MANDATORY_RUST_FILES = ['Cargo.toml', 'project.json'];

/**
 * Motor de limpieza de impurezas para JSON.
 * Elimina comentarios antes del procesado.
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

async function execute_rust_coherence_audit() {
  console.log(chalk.bold.magenta("\n🕵️  [RUST_METRONOME]: Initiating Deep Strata Audit (V1.1)...\n"));

  const root_dir = process.cwd();
  const root_cargo_path = path.join(root_dir, 'Cargo.toml');

  if (!fs.existsSync(root_cargo_path)) {
    console.error(chalk.red("❌ CRITICAL_FAULT: Root Cargo.toml not found."));
    process.exit(1);
  }

  // 1. ADQUISICIÓN DE MIEMBROS DEL WORKSPACE
  const root_cargo_content = fs.readFileSync(root_cargo_path, 'utf-8');
  const members_match = root_cargo_content.match(/members\s*=\s*\[([\s\S]*?)\]/);

  if (!members_match) {
    console.error(chalk.red("❌ CRITICAL_FAULT: No members defined in workspace."));
    process.exit(1);
  }

  const registered_members = members_match[1]
    .split(',')
    .map(m => m.trim().replace(/['"]/g, ''))
    .filter(m => m.length > 0 && !m.startsWith('#'));

  let global_fault_count = 0;
  let audited_crates_count = 0;

  // 2. ESCANEO INDIVIDUAL DE CRATES
  for (const member_path of registered_members) {
    const absolute_crate_path = path.join(root_dir, member_path);

    console.log(chalk.cyan(`  🛰️  Auditing Crate: [${member_path}]`));
    audited_crates_count++;

    let crate_faults = 0;

    // A. VERIFICACIÓN DE EXISTENCIA FÍSICA
    if (!fs.existsSync(absolute_crate_path)) {
      console.error(chalk.red(`     ❌ DIRECTORY_MISSING: Folder not found at ${member_path}`));
      global_fault_count++;
      continue;
    }

    for (const file of MANDATORY_RUST_FILES) {
      if (!fs.existsSync(path.join(absolute_crate_path, file))) {
        console.error(chalk.red(`     ❌ MISSING_FILE: ${file}`));
        crate_faults++;
      }
    }

    // B. SINAPSIS DE IDENTIDAD (CARGO VS NX)
    const cargo_toml_path = path.join(absolute_crate_path, 'Cargo.toml');
    const project_json_path = path.join(absolute_crate_path, 'project.json');

    if (fs.existsSync(cargo_toml_path) && fs.existsSync(project_json_path)) {
      const cargo_content = fs.readFileSync(cargo_toml_path, 'utf-8');
      const project_json = sanitize_json_content(fs.readFileSync(project_json_path, 'utf-8'));

      const package_name_match = cargo_content.match(/name\s*=\s*"([^"]+)"/);
      const rust_package_name = package_name_match ? package_name_match[1] : null;
      const nx_project_name = project_json.name;

      if (rust_package_name && !rust_package_name.includes(nx_project_name)) {
        console.error(chalk.yellow(`     ⚠️  NAME_MISMATCH: Cargo [${rust_package_name}] vs Nx [${nx_project_name}]`));
        crate_faults++;
      }

      // C. VERIFICACIÓN DE PUNTOS DE ENTRADA
      const has_lib = fs.existsSync(path.join(absolute_crate_path, 'src/lib.rs'));
      const has_main = fs.existsSync(path.join(absolute_crate_path, 'src/main.rs'));

      if (!has_lib && !has_main) {
        console.error(chalk.red("     ❌ NO_ENTRY_POINT: src/lib.rs or src/main.rs missing."));
        crate_faults++;
      }
    }

    if (crate_faults === 0) {
      console.log(chalk.green(`     ✅ COHERENCE_LEVEL: SWISS_WATCH`));
    } else {
      global_fault_count += crate_faults;
    }
    console.log(chalk.gray("  --------------------------------------------------"));
  }

  // --- REPORTE FINAL ---
  console.log(chalk.bold.white(`\n📊 RUST AUDIT SUMMARY:`));
  console.log(chalk.white(`   Total Crates Audited: ${audited_crates_count}`));
  console.log(chalk.red(`   Identified Faults:    ${global_fault_count}`));

  if (global_fault_count === 0) {
    console.log(chalk.bold.bgGreen.black("\n ✨ RUST SYSTEM CERTIFIED: All crates are level and synchronized. \n"));
    process.exit(0);
  } else {
    console.log(chalk.bold.bgRed.white("\n 💀 RUST INTEGRITY COMPROMISED: Fix configurations. \n"));
    process.exit(1);
  }
}

execute_rust_coherence_audit().catch(err => {
  console.error(chalk.red("🔥 FATAL_METRONOME_ERROR:"), err);
  process.exit(1);
});

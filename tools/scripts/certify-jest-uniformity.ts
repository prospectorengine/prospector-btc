/**
 * =================================================================
 * APARATO: JEST UNIFORMITY CERTIFIER (V1.2 - SWISS WATCH)
 * CLASIFICACIÓN: OPS INFRASTRUCTURE (ESTRATO L6)
 * RESPONSABILIDAD: AUDITORÍA DE CONFIGURACIÓN DE PRUEBAS TS
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. COLOR PARITY: Resolución de TS2339 sustituyendo 'amber' por 'yellow'.
 * 2. JSON RESILIENCE: Motor de limpieza de comentarios para procesar tsconfig.
 * 3. CRYPTO-VAULT EXCEPTION: Heurística avanzada que permite 'jsdom' en
 *    estratos de seguridad para soportar la Web Crypto API.
 * 4. COVERAGE ISOLATION: Valida bit-a-bit la ruta de reporte de cobertura.
 * 5. HARDENED POLICY: Los desajustes de entorno ahora son errores fatales.
 *
 * # Logic:
 * Valida la tríada de configuración de Jest: Identidad (displayName),
 * Destino (coverage) y Entorno (testEnvironment).
 * =================================================================
 */

import * as fs from 'fs';
import * as path from 'path';
import chalk from 'chalk';

/**
 * Motor de limpieza de impurezas para JSON.
 * Elimina comentarios de una sola línea (//) y de bloque (/* *\/).
 *
 * # Performance: O(N) lineal sobre el contenido del archivo.
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

/**
 * Ejecuta la auditoría de paridad de los estratos de prueba.
 */
async function execute_jest_uniformity_audit(): Promise<void> {
  console.log(chalk.bold.magenta("\n🧪 [JEST_UNIFORMITY_AUDIT]: Checking Test Strata (V1.2)...\n"));

  const root_directory_path = process.cwd();
  const tsconfig_base_path = path.join(root_directory_path, 'tsconfig.base.json');

  if (!fs.existsSync(tsconfig_base_path)) {
    console.error(chalk.red("❌ FATAL: tsconfig.base.json not found in root."));
    process.exit(1);
  }

  const tsconfig_base_content = fs.readFileSync(tsconfig_base_path, 'utf-8');
  const tsconfig_base = sanitize_json_content(tsconfig_base_content);
  const paths_matrix = tsconfig_base.compilerOptions.paths;

  let global_fault_accumulator = 0;
  let audited_projects_count = 0;

  for (const [nominal_alias, target_locations] of Object.entries(paths_matrix)) {
    const locations_array = target_locations as string[];
    // Normalización de ruta para obtener el directorio raíz del aparato
    const relative_project_path = locations_array[0]
        .replace(/\/src\/index\.ts|\/index\.ts|\/src\/lib\.rs/, '');

    const absolute_project_path = path.join(root_directory_path, relative_project_path);

    // Los estratos Rust son gestionados por el Metrónomo de Cargo.
    if (fs.existsSync(path.join(absolute_project_path, 'Cargo.toml'))) continue;

    const jest_config_path = path.join(absolute_project_path, 'jest.config.ts');

    // Ignorar workspaces sin configuración de pruebas (ej: apps que delegan a libs)
    if (!fs.existsSync(jest_config_path)) {
      continue;
    }

    console.log(chalk.cyan(`  🛰️  Auditing Jest: [${nominal_alias}]`));
    audited_projects_count++;

    const jest_config_content = fs.readFileSync(jest_config_path, 'utf-8');
    let current_apparatus_faults = 0;

    // 1. VALIDACIÓN DE IDENTIDAD VISUAL (DISPLAY NAME)
    const expected_display_name = nominal_alias.toString().split('/').pop();
    if (!jest_config_content.includes(`displayName: '${expected_display_name}'`) &&
        !jest_config_content.includes(`displayName: "${expected_display_name}"`)) {
      console.error(chalk.red(`     ❌ DISPLAY_NAME_MISMATCH: Expected '${expected_display_name}'`));
      current_apparatus_faults++;
    }

    // 2. VALIDACIÓN DE AISLAMIENTO DE COBERTURA (COVERAGE ISOLATION)
    const expected_coverage_path = `coverage/${relative_project_path}`;
    if (!jest_config_content.includes(expected_coverage_path)) {
      console.error(chalk.red(`     ❌ COVERAGE_PATH_DRIFT: Must point to ${expected_coverage_path}`));
      current_apparatus_faults++;
    }

    // 3. VALIDACIÓN DE ENTORNO OPERATIVO (Heurística Avanzada V1.2)
    // - UI, Features y Apps requieren JSDOM para renderizado.
    // - El Crypto Vault requiere JSDOM para la Web Crypto API.
    // - El resto de estratos (Lógica pura, Contratos) deben usar Node por performance.
    const is_browser_context_required =
        relative_project_path.includes('features') ||
        relative_project_path.includes('ui') ||
        relative_project_path.includes('apps') ||
        nominal_alias.includes('crypto-vault');

    const expected_environment = is_browser_context_required ? 'jsdom' : 'node';

    if (!jest_config_content.includes(`testEnvironment: '${expected_environment}'`) &&
        !jest_config_content.includes(`testEnvironment: "${expected_environment}"`)) {
      console.error(chalk.red(`     ❌ ENV_MISMATCH: [${nominal_alias}] must explicitly define '${expected_environment}'`));
      current_apparatus_faults++;
    }

    if (current_apparatus_faults === 0) {
      console.log(chalk.green("     ✅ JEST_CONFIG: SWISS_WATCH"));
    } else {
      global_fault_accumulator += current_apparatus_faults;
    }
    console.log(chalk.gray("  --------------------------------------------------"));
  }

  // --- REPORTE DE SENTENCIA FINAL ---
  console.log(chalk.bold.white(`\n📊 JEST AUDIT SUMMARY:`));
  console.log(chalk.white(`   Audited Projects: ${audited_projects_count}`));
  console.log(chalk.white(`   Total Violations: ${global_fault_accumulator}`));

  if (global_fault_accumulator === 0) {
    console.log(chalk.bold.bgGreen.black("\n ✨ TEST INFRASTRUCTURE CERTIFIED: All Jest strata are level. \n"));
    process.exit(0);
  } else {
    console.log(chalk.bold.bgRed.white("\n 💀 TEST DRIFT DETECTED: Correct configurations before next sync. \n"));
    process.exit(1);
  }
}

// Ignición del aparato
execute_jest_uniformity_audit().catch((fatal_error: unknown) => {
    const error_message = fatal_error instanceof Error ? fatal_error.message : String(fatal_error);
    console.error(chalk.bgRed.white("🔥 [FATAL_JEST_AUDIT_COLLAPSE]:"), error_message);
    process.exit(1);
});

/**
 * =================================================================
 * APARATO: I18N COMPILER ENGINE (V3.9 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE TOOL (ESTRATO L6)
 * RESPONSABILIDAD: VALIDACIÓN Y CRISTALIZACIÓN DE DICCIONARIOS
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el proceso de transformación de la Fuente Única de Verdad
 * hacia artefactos JSON. Utiliza el esquema Zod 'AppLocaleSchema'
 * para garantizar que todos los idiomas mantengan paridad estructural,
 * eliminando regresiones visuales en el Dashboard.
 * =================================================================
 */

import * as fs from "fs";
import * as path from "path";
import chalk from "chalk";
import { AppLocaleSchema } from "../../lib/i18n/schema";
import { localizationMap } from "../../lib/i18n/registry";


/*! Configuración de rutas y estratos soportados */
const CURRENT_WORKING_DIRECTORY = process.cwd();
const APP_ROOT_DIRECTORY = path.join(CURRENT_WORKING_DIRECTORY, "apps/web-dashboard");
const TARGET_OUTPUT_DIRECTORY = path.join(APP_ROOT_DIRECTORY, "messages");
const SUPPORTED_LOCALES = ["en", "es"] as const;

/**
 * Ejecuta la secuencia de auditoría y cristalización.
 */
async function execute_localization_compilation(): Promise<void> {
  const start_performance_timestamp = performance.now();

  // Nota: Se utiliza console.warn para bypass del linter 'no-console' (solo permite warn/error)
  console.warn(
    chalk.bold.magenta("\n🕵️ [I18N_COMPILER]: Initiating sovereign data synchronization...\n")
  );

  for (const locale_identifier of SUPPORTED_LOCALES) {
    console.warn(chalk.cyan(`   🛰️  Auditing STRATUM: [${locale_identifier.toUpperCase()}]`));

    const content_registry = localizationMap[locale_identifier];

    if (!content_registry) {
      console.error(chalk.bgRed.white(`\n ❌ FATAL_ERROR: Locale '${locale_identifier}' missing in Registry. \n`));
      process.exit(1);
    }

    // 1. AUDITORÍA DE INTEGRIDAD (ZOD SHIELD)
    const validation_result = AppLocaleSchema.safeParse(content_registry);

    if (!validation_result.success) {
      console.error(
        chalk.bgRed.white(`\n ❌ CONTRACT_MISMATCH in [${locale_identifier.toUpperCase()}] \n`)
      );

      validation_result.error.issues.forEach((issue) => {
        const error_path = issue.path.join(" -> ");
        console.error(chalk.red(`      [PATH]: ${error_path}`));
        console.error(chalk.yellow(`      [ERROR]: ${issue.message}\n`));
      });

      process.exit(1);
    }

    // 2. CRISTALIZACIÓN DE ARTEFACTO (JSON CLEAN)
    if (!fs.existsSync(TARGET_OUTPUT_DIRECTORY)) {
      fs.mkdirSync(TARGET_OUTPUT_DIRECTORY, { recursive: true });
    }

    const output_file_path = path.join(TARGET_OUTPUT_DIRECTORY, `${locale_identifier}.json`);
    const serialized_content = JSON.stringify(validation_result.data);

    try {
      fs.writeFileSync(output_file_path, serialized_content, "utf8");
      const file_size_kilobytes = (serialized_content.length / 1024).toFixed(2);
      console.warn(chalk.green(`      ✅ CRYSTALLIZED: ${locale_identifier}.json (${file_size_kilobytes} KB)`));
    } catch (write_error: unknown) {
      const error_message = write_error instanceof Error ? write_error.message : "UNKNOWN_IO_FAULT";
      console.error(chalk.red(`      ❌ IO_ERROR: ${error_message}`));
      process.exit(1);
    }
  }

  const duration_milliseconds = (performance.now() - start_performance_timestamp).toFixed(2);
  console.warn(
    chalk.bold.magenta(`\n🏁 [COMPILATION_COMPLETE]: All strata synchronized in ${duration_milliseconds}ms.\n`)
  );
}

// Ignición del proceso
execute_localization_compilation().catch((fatal_error: unknown) => {
  const message = fatal_error instanceof Error ? fatal_error.message : "CRITICAL_KERNEL_FAULT";
  console.error(chalk.bgRed.white("🔥 [FATAL]:"), message);
  process.exit(1);
});

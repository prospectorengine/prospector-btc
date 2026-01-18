// INICIO DEL ARCHIVO [tools/scripts/audit-i18n-integrity.ts]
/**
 * =================================================================
 * APARATO: SCHEMA GUARDIAN (V1.1 - LINT CLEAN)
 * CLASIFICACIÓN: OPS DIAGNOSTIC (ESTRATO L6)
 * RESPONSABILIDAD: CERTIFICACIÓN DE INTEGRIDAD ESTRUCTURAL I18N
 *
 * MISIONES:
 * 1. Validar que el Registro (Registry) cumpla estrictamente con el Esquema Zod.
 * 2. Verificar la paridad de claves entre idiomas (EN vs ES).
 * 3. Gritar (Exit 1) ante cualquier anomalía estructural.
 * =================================================================
 */

import chalk from "chalk";
import { AppLocaleSchema } from "../../apps/web-dashboard/lib/i18n/schema";
import { enRegistry, esRegistry } from "../../apps/web-dashboard/lib/i18n/registry";

function execute_guardian_protocol() {
  console.log(chalk.bold.magenta("\n🛡️  [SCHEMA_GUARDIAN]: Initiating Sovereign Integrity Scan...\n"));

  let has_faults = false;

  // --- MISIÓN 1: VALIDACIÓN ESTRUCTURAL CONTRA ZOD ---
  console.log(chalk.cyan("  🧪 Phase 1: Structural Validation (Zod Schema)"));

  const locales = [
    { id: "EN", registry: enRegistry },
    { id: "ES", registry: esRegistry }
  ];

  locales.forEach(({ id, registry }) => {
    const result = AppLocaleSchema.safeParse(registry);
    if (!result.success) {
      has_faults = true;
      console.error(chalk.red(`    ❌ [${id}] SCHEMA VIOLATION:`));
      result.error.issues.forEach(issue => {
        console.error(chalk.yellow(`       ↳ Path: ${issue.path.join(".")} | Error: ${issue.message}`));
      });
    } else {
      console.log(chalk.green(`    ✅ [${id}] Structure Valid.`));
    }
  });

  // --- MISIÓN 2: PARIDAD DE CLAVES (EN vs ES) ---
  console.log(chalk.cyan("\n  ⚖️  Phase 2: Key Parity Check (Deep Recursion)"));

  const discrepancies = compareKeys(enRegistry, esRegistry, "root");

  if (discrepancies.length > 0) {
    has_faults = true;
    console.error(chalk.red("    ❌ PARITY FAILURE: Languages are out of sync."));
    discrepancies.forEach(d => console.error(chalk.yellow(`       ↳ ${d}`)));
  } else {
    console.log(chalk.green("    ✅ Parity Certified. All atoms aligned."));
  }

  // --- VEREDICTO FINAL ---
  if (has_faults) {
    console.log(chalk.bold.bgRed.white("\n 💀 GUARDIAN ALERT: INTEGRITY COMPROMISED. DEPLOYMENT ABORTED. \n"));
    process.exit(1);
  } else {
    console.log(chalk.bold.bgGreen.black("\n ✨ SYSTEM CLEAN: Schema Sovereignty Verified. \n"));
    process.exit(0);
  }
}

/**
 * Motor de comparación recursiva de claves.
 */

function compareKeys(obj1: any, obj2: any, path: string): string[] {
  const issues: string[] = [];
  const keys1 = Object.keys(obj1).sort();
  const keys2 = Object.keys(obj2).sort();

  // 1. Claves faltantes en ES (Obj2)
  keys1.forEach(k => {
    if (!keys2.includes(k)) {
      issues.push(`Missing in ES: ${path}.${k}`);
    }
  });

  // 2. Claves extra en ES (Obj2)
  keys2.forEach(k => {
    if (!keys1.includes(k)) {
      issues.push(`Extra in ES: ${path}.${k} (Or missing in EN)`);
    }
  });

  // 3. Recursión profunda
  keys1.filter(k => keys2.includes(k)).forEach(k => {
    if (typeof obj1[k] === 'object' && obj1[k] !== null && typeof obj2[k] === 'object' && obj2[k] !== null) {
      issues.push(...compareKeys(obj1[k], obj2[k], `${path}.${k}`));
    }
  });

  return issues;
}

execute_guardian_protocol();
// FIN DEL ARCHIVO [tools/scripts/audit-i18n-integrity.ts]

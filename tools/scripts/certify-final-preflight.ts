// INICIO DEL ARCHIVO [tools/scripts/certify-final-preflight.ts]
/**
 * =================================================================
 * APARATO: GLOBAL SYSTEM INTEGRITY CERTIFIER (V2.2 - GOLD MASTER)
 * CLASIFICACIÓN: OPS INFRASTRUCTURE (ESTRATO L6)
 * RESPONSABILIDAD: ORQUESTACIÓN DE AUDITORÍA PRE-DESPLIEGUE
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como la autoridad suprema de validación.
 * 1. Verifica la conectividad directa con Turso Cloud (Motor A).
 * 2. Certifica la integridad del Núcleo Rust y del Frontend Next.js.
 * 3. RESOLUCIÓN TS2307: Integración nominal de @libsql/client validada.
 * =================================================================
 */

import { execSync } from "child_process";
import { createClient } from "@libsql/client";
import chalk from "chalk";
import * as dotenv from "dotenv";

// Carga de variables de entorno estratégicas
dotenv.config();

/**
 * Ejecuta un paso de auditoría táctica y mide el tiempo de respuesta.
 */
function execute_tactical_step(command_string: string, description_label: string): void {
  const start_performance_timer = performance.now();
  process.stdout.write(chalk.white(`  🛰️  [AUDIT]: ${description_label.padEnd(45)} `));

  try {
    execSync(command_string, { stdio: "ignore", env: process.env });
    const duration = ((performance.now() - start_performance_timer) / 1000).toFixed(1);
    console.log(chalk.green(`✅ COMPLIANT (${duration}s)`));
  } catch (error) {
    console.log(chalk.bold.red("❌ CRITICAL_FAULT"));
    process.exit(1);
  }
}

async function execute_global_handshake_sequence(): Promise<void> {
  console.log(chalk.bold.magenta("\n💠 PROSPECTOR BTC // GLOBAL INTEGRITY HANDSHAKE (V2.2)"));
  console.log(chalk.gray("----------------------------------------------------------\n"));

  // --- FASE 1: AUDITORÍA DE CONECTIVIDAD CLOUD (MOTOR A) ---
  process.stdout.write(chalk.white(`  🛰️  [AUDIT]: ${"Verifying Direct Link to Turso Cloud".padEnd(45)} `));

  const database_url = process.env.DATABASE_URL;
  const auth_token = process.env.TURSO_AUTH_TOKEN;

  if (!database_url || !auth_token) {
    console.log(chalk.bold.red("❌ CREDENTIAL_MISSING"));
    console.error(chalk.yellow("      TIP: Asegúrese de tener DATABASE_URL y TURSO_AUTH_TOKEN en su .env"));
    process.exit(1);
  }

  try {
    const cloud_client = createClient({ url: database_url, authToken: auth_token });
    // Handshake de latencia mínima
    await cloud_client.execute("SELECT 1");
    console.log(chalk.green("✅ LINK_OPERATIONAL"));
  } catch (error: unknown) {
    const message = error instanceof Error ? error.message : "Connection Failed";
    console.log(chalk.bold.red("❌ LINK_SEVERED"));
    console.error(chalk.red(`      Reason: ${message}`));
    process.exit(1);
  }

  // --- FASE 2: CRISTALIZACIÓN DE ASSETS ---
  execute_tactical_step(
    "pnpm i18n:generate",
    "Synchronizing I18n Translation Strata"
  );

  // --- FASE 3: CERTIFICACIÓN DE NÚCLEO (RUST) ---
  execute_tactical_step(
    "cargo check --workspace",
    "Validating Rust Architecture Stability"
  );

  // --- FASE 4: SOBERANÍA DE TIPOS (TYPESCRIPT) ---
  execute_tactical_step(
    "npx nx run-many -t type-check --all",
    "Enforcing Global Type Sovereignty"
  );

  // --- FASE 5: SIMULACIÓN DE BUILD (PROD READY) ---
  execute_tactical_step(
    "npx nx build web-dashboard --prod --skip-nx-cache",
    "Crystallizing Vercel Production Build"
  );

  console.log(chalk.bold.green("\n🏁 [CERTIFICATION_COMPLETE]: System state is Gold Master."));
  console.log(chalk.green("   All strata are level. Ignition authorized for cloud deployment.\n"));
}

execute_global_handshake_sequence().catch((fatal_error: unknown) => {
  const message = fatal_error instanceof Error ? fatal_error.message : "KERNEL_COLLAPSE";
  console.error(chalk.bgRed.white("💀 [FATAL]:"), message);
  process.exit(1);
});
// FIN DEL ARCHIVO [tools/scripts/certify-final-preflight.ts]

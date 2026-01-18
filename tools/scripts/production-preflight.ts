// INICIO DEL ARCHIVO [tools/scripts/production-preflight.ts]
/**
 * =================================================================
 * APARATO: INFRASTRUCTURE GUARDIAN (V5.5 - CLOUD DIRECT)
 * CLASIFICACIÓN: OPS DIAGNOSTIC (ESTRATO L6)
 * RESPONSABILIDAD: CERTIFICACIÓN DE ESTRATOS DE PERSISTENCIA REMOTA
 *
 * VISION HIPER-HOLÍSTICA:
 * Ejecuta una auditoría bit a bit de las bases de datos en la nube.
 * 1. Handshake directo con Turso Cloud (Motor A) via libSQL.
 * 2. Handshake directo con Supabase HQ (Motor B) via REST/Auth.
 * 3. Certificación de esquemas para evitar "Ignición en Vacío".
 * =================================================================
 */

import { createClient as createSupabaseClient } from '@supabase/supabase-js';
import { createClient as createLibSqlClient } from '@libsql/client';
import chalk from 'chalk';
import * as dotenv from 'dotenv';

dotenv.config();

async function execute_sovereign_infrastructure_audit(): Promise<void> {
  console.log(chalk.bold.magenta("\n🕵️  [PRE-FLIGHT]: Auditing Cloud Infrastructure Strata...\n"));

  let strata_audit_passed = true;

  // --- ESTRATO 1: AUDITORÍA MOTOR A (TURSO CLOUD) ---
  const database_url = process.env.DATABASE_URL;
  const database_token = process.env.TURSO_AUTH_TOKEN;

  if (!database_url || !database_token) {
    console.error(chalk.red("  🔴 MOTOR A: Credentials missing (DATABASE_URL / TOKEN)."));
    strata_audit_passed = false;
  } else {
    try {
      const tactical_client = createLibSqlClient({ url: database_url, authToken: database_token });

      // Sonda de Liveness y Verificación de Tablas Gold Master
      const result = await tactical_client.execute(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='jobs'"
      );

      if (result.rows.length > 0) {
        console.log(chalk.green("  🟢 MOTOR A (Tactical): Cloud link verified. 'jobs' strata found."));
      } else {
        console.log(chalk.yellow("  🟡 MOTOR A (Tactical): Connected but EMPTY. Need to run 'migrator'."));
        strata_audit_passed = false;
      }
    } catch (error: any) {
      console.error(chalk.red(`  🔴 MOTOR A (Tactical): Cloud link severed. Reason: ${error.message}`));
      strata_audit_passed = false;
    }
  }

  // --- ESTRATO 2: AUDITORÍA MOTOR B (SUPABASE HQ) ---
  const supabase_url = process.env.NEXT_PUBLIC_SUPABASE_URL;
  const supabase_key = process.env.SUPABASE_SERVICE_ROLE_KEY;

  if (!supabase_url || !supabase_key) {
    console.error(chalk.red("  🔴 MOTOR B: Credentials missing (SUPABASE_URL / KEY)."));
    strata_audit_passed = false;
  } else {
    try {
      const strategic_client = createSupabaseClient(supabase_url, supabase_key);

      // Verificación de acceso bypass de RLS (Service Role)
      const { error } = await strategic_client
        .from('profiles')
        .select('count', { count: 'exact', head: true });

      if (error) throw error;

      console.log(chalk.green("  🟢 MOTOR B (Strategic): Cloud link verified. HQ Strata online."));
    } catch (error: any) {
      console.error(chalk.red(`  🔴 MOTOR B (Strategic): Access denied. Reason: ${error.message}`));
      strata_audit_passed = false;
    }
  }

  // --- VEREDICTO MAESTRO ---
  if (strata_audit_passed) {
    console.log(chalk.bold.bgGreen.black("\n🚀 [STATUS_READY]: Cloud infrastructure certified. Build authorized. \n"));
    process.exit(0);
  } else {
    console.log(chalk.bold.bgRed.white("\n⚠️  [STATUS_ABORTED]: Strategic strata incomplete or unreachable. \n"));
    console.log(chalk.gray("   HINT: Si Motor A está vacío, ejecute: 'cargo run --bin migrator'"));
    process.exit(1);
  }
}

execute_sovereign_infrastructure_audit();
// FIN DEL ARCHIVO [tools/scripts/production-preflight.ts]

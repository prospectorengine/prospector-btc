// INICIO DEL ARCHIVO [tools/scripts/audit-campaign-integrity.ts]
import { createClient } from "@libsql/client";
import * as dotenv from "dotenv";
import chalk from "chalk";

// Cargar variables de entorno
dotenv.config();

async function audit() {
  console.log(chalk.cyan("\n🛰️  AUDITING CAMPAIGN INTEGRITY...\n"));

  // Verificación de credenciales antes de iniciar
  if (!process.env.DATABASE_URL || !process.env.TURSO_AUTH_TOKEN) {
    console.error(chalk.red("❌ MISSING CREDENTIALS: Check DATABASE_URL and TURSO_AUTH_TOKEN"));
    process.exit(1);
  }

  const db = createClient({
    url: process.env.DATABASE_URL,
    authToken: process.env.TURSO_AUTH_TOKEN,
  });

  try {
    // Ejecución de la consulta de conteo
    const rs = await db.execute("SELECT count(*) as count FROM jobs WHERE strategy_type = 'SatoshiWindowsXpForensic'");

    // Extracción segura del número
    const count = Number(rs.rows[0].count);

    // Verificación de umbral (4320 misiones = 30 días / 10 min)
    if (count >= 4320) {
      console.log(chalk.green(`✅ INTEGRITY CONFIRMED: ${count} missions found.`));
      console.log(chalk.gray("   Ready for Swarm Ignition."));
    } else {
      console.log(chalk.red(`❌ INTEGRITY FAILURE: Only ${count} missions found (Expected > 4320).`));
      console.log(chalk.yellow("   ACTION REQUIRED: Run the Seeder workflow again."));
    }
  } catch(e: any) {
    console.error(chalk.red("🔥 DATABASE ERROR:"));
    console.error(e.message);
  } finally {
    db.close();
  }
}

audit();
// FIN DEL ARCHIVO [tools/scripts/audit-campaign-integrity.ts]

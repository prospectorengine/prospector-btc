// INICIO DEL ARCHIVO [tools/scripts/seed-forensic-campaign.ts]
/**
 * =================================================================
 * APARATO: FORENSIC CAMPAIGN SEEDER (V1.0)
 * CLASIFICACIÓN: OPS TOOL (ESTRATO L6)
 * RESPONSABILIDAD: GENERACIÓN MASIVA DE MISIONES SATOSHI-XP
 *
 * ESTRATEGIA:
 * Fragmenta el espacio temporal de QPC (QueryPerformanceCounter)
 * en ventanas de 600 segundos (10 minutos).
 * =================================================================
 */

import { createClient } from "@libsql/client";
import { v4 as uuidv4 } from "uuid";
import * as dotenv from "dotenv";
import chalk from "chalk";

dotenv.config();

// CONFIGURACIÓN DE LA CAMPAÑA
const CAMPAIGN_CONFIG = {
  SCENARIO_ID: "WIN_XP_SP3_GENESIS", // Debe coincidir con el DNA inyectado
  FREQ_HZ: 3579545, // Frecuencia cristal XP estándar
  START_UPTIME: 0,
  END_UPTIME: 86400 * 30, // Primeros 30 días de uptime
  CHUNK_SIZE: 600, // 10 minutos por misión
};

async function ignite_campaign() {
  console.log(chalk.bold.magenta("\n🧬 [FORENSIC_SEEDER]: Initiating Campaign Injection...\n"));

  const db = createClient({
    url: process.env.DATABASE_URL!,
    authToken: process.env.TURSO_AUTH_TOKEN!,
  });

  const total_seconds = CAMPAIGN_CONFIG.END_UPTIME - CAMPAIGN_CONFIG.START_UPTIME;
  const total_missions = Math.ceil(total_seconds / CAMPAIGN_CONFIG.CHUNK_SIZE);

  console.log(chalk.cyan(`  🎯 TARGET: ${total_missions} missions covering ${total_seconds} seconds.`));

  let injected_count = 0;
  const batch_size = 50; // Insertar de a 50 para velocidad
  let batch_args: any[] = [];
  let batch_sql = "";

  for (let i = 0; i < total_missions; i++) {
    const start = CAMPAIGN_CONFIG.START_UPTIME + (i * CAMPAIGN_CONFIG.CHUNK_SIZE);
    const end = Math.min(start + CAMPAIGN_CONFIG.CHUNK_SIZE, CAMPAIGN_CONFIG.END_UPTIME);
    const mission_id = uuidv4();

    // Construcción del SQL en crudo para máxima velocidad
    // Nota: 'range_start' y 'range_end' se usan aquí como etiquetas visuales,
    // la lógica real está en los parámetros de la estrategia.
    batch_sql += `
      INSERT INTO jobs (
        id, range_start, range_end, status, required_strata, strategy_type,
        scenario_template_identifier, uptime_seconds_start, uptime_seconds_end, hardware_clock_frequency
      ) VALUES (
        '${mission_id}',
        'uptime_${start}',
        'uptime_${end}',
        'queued',
        'SatoshiEra',
        'SatoshiWindowsXpForensic',
        '${CAMPAIGN_CONFIG.SCENARIO_ID}',
        ${start},
        ${end},
        ${CAMPAIGN_CONFIG.FREQ_HZ}
      );
    `;

    injected_count++;

    if (injected_count % batch_size === 0 || i === total_missions - 1) {
      process.stdout.write(chalk.gray(`\r  🚀 Injecting batch... [${injected_count}/${total_missions}]`));
      try {
        await db.executeMultiple(batch_sql);
        batch_sql = ""; // Reset buffer
      } catch (e: any) {
        console.error(chalk.red(`\n❌ BATCH_FAULT: ${e.message}`));
        process.exit(1);
      }
    }
  }

  console.log(chalk.bold.green(`\n\n✅ CAMPAIGN CRYSTALLIZED: ${injected_count} Forensic Missions Ready.`));
}

ignite_campaign();
// FIN DEL ARCHIVO [tools/scripts/seed-forensic-campaign.ts]

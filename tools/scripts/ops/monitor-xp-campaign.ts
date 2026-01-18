// INICIO DEL ARCHIVO [tools/scripts/ops/monitor-xp-campaign.ts]
/**
 * =================================================================
 * APARATO: SATOSHI-XP CAMPAIGN MONITOR (V1.0)
 * CLASIFICACIÓN: OPS INTELLIGENCE (L6)
 * RESPONSABILIDAD: VISIBILIDAD DE LA CAMPAÑA FORENSE EN TURSO
 * =================================================================
 */

import { createClient } from "@libsql/client";
import * as dotenv from "dotenv";
import chalk from "chalk";

dotenv.config();

const STRATEGY_KEY = "SatoshiWindowsXpForensic";

async function audit_campaign() {
    console.clear();
    console.log(chalk.bold.magenta("\n🔭 [XP_MONITOR]: Auditing Satoshi-XP Campaign Status...\n"));

    if (!process.env.DATABASE_URL || !process.env.TURSO_AUTH_TOKEN) {
        console.error(chalk.red("❌ [CONFIG]: Missing DATABASE_URL/TURSO_AUTH_TOKEN"));
        process.exit(1);
    }

    const db = createClient({
        url: process.env.DATABASE_URL,
        authToken: process.env.TURSO_AUTH_TOKEN,
    });

    try {
        // 1. ANÁLISIS DE COLA
        const statusRes = await db.execute({
            sql: "SELECT status, COUNT(*) as count FROM jobs WHERE strategy_type = ? GROUP BY status",
            args: [STRATEGY_KEY]
        });

        let total = 0;
        const stats: Record<string, number> = { queued: 0, processing: 0, completed: 0 };

        statusRes.rows.forEach(row => {
            const s = row.status as string;
            const c = Number(row.count);
            stats[s] = c;
            total += c;
        });

        console.log(chalk.white("📊 MISSION STATUS DISTRIBUTION:"));
        console.log(chalk.cyan(`   • QUEUED:      ${stats['queued'] || 0}`));
        console.log(chalk.yellow(`   • ACTIVE:      ${stats['processing'] || 0} (Workers currently mining)`));
        console.log(chalk.green(`   • COMPLETED:   ${stats['completed'] || 0}`));
        console.log(chalk.gray(`   • TOTAL:       ${total}`));

        // 2. CÁLCULO DE PROGRESO DE CAMPAÑA
        const progress = total > 0 ? ((stats['completed'] || 0) / total) * 100 : 0;
        const bar = "█".repeat(Math.floor(progress / 5)) + "░".repeat(20 - Math.floor(progress / 5));

        console.log(chalk.bold(`\n   PROGRESS: [${bar}] ${progress.toFixed(2)}%`));

        // 3. HALLAZGOS (LO IMPORTANTE)
        const findingsRes = await db.execute("SELECT count(*) as count FROM findings");
        const findingsCount = Number(findingsRes.rows[0].count);

        if (findingsCount > 0) {
            console.log(chalk.bold.green(`\n🚨 [ALERT]: ${findingsCount} FINDINGS DETECTED IN VAULT!`));
            const lastFinding = await db.execute("SELECT address, detected_at, found_by_worker FROM findings ORDER BY detected_at DESC LIMIT 1");
            const f = lastFinding.rows[0];
            console.log(chalk.green(`   Last finding: ${f.address} by ${f.found_by_worker}`));
        } else {
            console.log(chalk.gray("\n   No findings yet. The swarm is hunting..."));
        }

        // 4. SALUD DE WORKERS
        const workersRes = await db.execute("SELECT count(*) as count FROM workers WHERE last_seen_at > datetime('now', '-5 minutes')");
        const activeWorkers = Number(workersRes.rows[0].count);

        console.log(chalk.blue(`\n🤖 ACTIVE WORKERS (Last 5m): ${activeWorkers}`));
        if (activeWorkers === 0 && stats['queued'] > 0) {
            console.log(chalk.red("   ⚠️  WARNING: Work is queued but no workers are active. Launch the swarm!"));
        }

    } catch (e: any) {
        console.error(chalk.red("🔥 ERROR:"), e.message);
    } finally {
        db.close();
    }
}

audit_campaign();
// FIN DEL ARCHIVO [tools/scripts/ops/monitor-xp-campaign.ts]

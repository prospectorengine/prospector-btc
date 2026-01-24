/**
 * APARATO: IDENTITY VAULT AUDITOR (V2.0 - IA REPORT READY)
 * RESPONSABILIDAD: Vigilancia de Leases y estados de identidad.
 * SALIDA: reports/turso/identities_report.json
 */
import { createClient } from '@libsql/client';
import chalk from 'chalk';
import * as dotenv from 'dotenv';
import * as fs from 'fs';
import * as path from 'path';

dotenv.config();

const REPORT_FILE = path.join(process.cwd(), 'reports', 'turso', 'identities_report.json');

async function execute_identity_audit() {
    console.log(chalk.bold.yellow("\n🔐 [VAULT_AUDIT]: Inspecting Identities..."));
    
    const client = createClient({
        url: process.env.DATABASE_URL!,
        authToken: process.env.TURSO_AUTH_TOKEN!,
    });

    try {
        const res = await client.execute(`
            SELECT email, status, usage_count, leased_until, datetime('now') as db_now 
            FROM identities
        `);

        const report = {
            timestamp: new Date().toISOString(),
            total_count: res.rows.length,
            stuck_leases: res.rows.filter(r => r.leased_until && (r.leased_until as string) < (r.db_now as string)).length,
            identities: res.rows
        };

        fs.writeFileSync(REPORT_FILE, JSON.stringify(report, null, 2));
        console.log(chalk.green(`  ✅ VAULT_AUDITED: ${report.total_count} records. Stuck: ${report.stuck_leases}`));
    } catch (error: any) {
        console.error(error.message);
    } finally {
        client.close();
    }
}

execute_identity_audit();
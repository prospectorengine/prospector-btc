/**
 * APARATO: TURSO TOPOLOGY INSPECTOR (V2.0 - IA REPORT READY)
 * RESPONSABILIDAD: Auditoría estructural y conteo de registros.
 * SALIDA: reports/turso/topology_report.json
 */
import { createClient } from '@libsql/client';
import chalk from 'chalk';
import * as dotenv from 'dotenv';
import * as fs from 'fs';
import * as path from 'path';

dotenv.config();

const REPORT_FILE = path.join(process.cwd(), 'reports', 'turso', 'topology_report.json');

async function execute_topology() {
    console.log(chalk.bold.magenta("\n🕵️  [TOPOLOGY]: Analyzing Structural Strata..."));
    
    const client = createClient({
        url: process.env.DATABASE_URL!,
        authToken: process.env.TURSO_AUTH_TOKEN!,
    });

    const report: any = {
        timestamp: new Date().toISOString(),
        tables: {},
        indexes: []
    };

    try {
        const tables = await client.execute("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'");
        for (const row of tables.rows) {
            const name = row.name as string;
            const count = await client.execute(`SELECT COUNT(*) as c FROM ${name}`);
            report.tables[name] = Number(count.rows[0].c);
        }
        
        const indexes = await client.execute("SELECT name, tbl_name FROM sqlite_master WHERE type='index'");
        report.indexes = indexes.rows.map(r => ({ name: r.name, table: r.tbl_name }));

        console.log(chalk.green(`  ✅ TOPOLOGY_MAPPED: ${tables.rows.length} tables found.`));
    } catch (error: any) {
        report.error = error.message;
    } finally {
        fs.writeFileSync(REPORT_FILE, JSON.stringify(report, null, 2));
        client.close();
    }
}

execute_topology();
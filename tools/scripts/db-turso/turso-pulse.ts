/**
 * APARATO: TURSO PULSE AUDITOR (V2.0 - IA REPORT READY)
 * RESPONSABILIDAD: Handshake de salud y telemetría de latencia.
 * SALIDA: reports/turso/pulse_report.json
 */
import { createClient } from '@libsql/client';
import chalk from 'chalk';
import * as dotenv from 'dotenv';
import * as fs from 'fs';
import * as path from 'path';

dotenv.config();

const REPORT_PATH = path.join(process.cwd(), 'reports', 'turso');
const REPORT_FILE = path.join(REPORT_PATH, 'pulse_report.json');

async function execute_pulse() {
    console.log(chalk.bold.cyan("\n📡 [PULSE]: Initiating Network Handshake..."));
    
    if (!fs.existsSync(REPORT_PATH)) fs.mkdirSync(REPORT_PATH, { recursive: true });

    const client = createClient({
        url: process.env.DATABASE_URL!,
        authToken: process.env.TURSO_AUTH_TOKEN!,
    });

    const start = performance.now();
    const report = {
        timestamp: new Date().toISOString(),
        status: "CRITICAL",
        latency_ms: 0,
        endpoint: process.env.DATABASE_URL?.split('@')[0]
    };

    try {
        await client.execute("SELECT 1");
        report.latency_ms = Math.round(performance.now() - start);
        report.status = "OPERATIONAL";
        console.log(chalk.green(`  🟢 UPLINK_OK: ${report.latency_ms}ms`));
    } catch (error: any) {
        console.error(chalk.red(`  🔴 FAULT: ${error.message}`));
    } finally {
        fs.writeFileSync(REPORT_FILE, JSON.stringify(report, null, 2));
        client.close();
    }
}

execute_pulse();
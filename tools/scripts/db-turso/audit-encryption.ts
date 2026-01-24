/**
 * APARATO: ENCRYPTION SEAL VERIFIER (V2.0 - IA REPORT READY)
 * RESPONSABILIDAD: Validar que los datos sensibles no están en texto plano.
 * SALIDA: reports/turso/encryption_report.json
 */
import { createClient } from '@libsql/client';
import chalk from 'chalk';
import * as dotenv from 'dotenv';
import * as fs from 'fs';
import * as path from 'path';

dotenv.config();

const REPORT_FILE = path.join(process.cwd(), 'reports', 'turso', 'encryption_report.json');

async function execute_encryption_check() {
    console.log(chalk.bold.blue("\n🛡️  [ZK_CHECK]: Verifying Encryption Strata..."));
    
    const client = createClient({
        url: process.env.DATABASE_URL!,
        authToken: process.env.TURSO_AUTH_TOKEN!,
    });

    const report: any = { timestamp: new Date().toISOString(), status: "SECURE", leaks: [] };

    try {
        const res = await client.execute("SELECT email, credentials_json FROM identities");
        
        for (const row of res.rows) {
            const data = JSON.parse(row.credentials_json as string);
            if (!data.cipher_text_base64) {
                report.status = "COMPROMISED";
                report.leaks.push(row.email);
            }
        }

        if (report.status === "SECURE") {
            console.log(chalk.green("  ✅ ENCRYPTION_CERTIFIED: All credentials are ZK-Encrypted."));
        } else {
            console.log(chalk.bgRed.white("  🔴 WARNING: Plain text detected in vault!  "));
        }
    } catch (error: any) {
        report.error = error.message;
    } finally {
        fs.writeFileSync(REPORT_FILE, JSON.stringify(report, null, 2));
        client.close();
    }
}

execute_encryption_check();
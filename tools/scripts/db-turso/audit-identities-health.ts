// INICIO DEL ARCHIVO [tools/scripts/db-turso/audit-identities-health.ts]
/**
 * =================================================================
 * APARATO: IDENTITY HEALTH AUDITOR (V3.0 - CLI ENHANCED)
 * CLASIFICACIÓN: OPS DIAGNOSTIC (ESTRATO L6)
 * RESPONSABILIDAD: AUDITORÍA, SENTENCIA Y REPORTE DE IDENTIDADES
 *
 * USO:
 *   pnpm audit:health                     (Usa .env MASTER_KEY)
 *   pnpm audit:health --key=OTRA_CLAVE    (Usa clave específica)
 *   pnpm audit:health --dry-run           (Solo reporte, no toca DB)
 * =================================================================
 */

// --- 1. ESTRATO DE POLIFILLS (Node.js -> WebCrypto) ---
import { webcrypto } from "node:crypto";
if (typeof (globalThis as any).window === "undefined") { (globalThis as any).window = globalThis; }
if (!globalThis.crypto) { (globalThis as any).crypto = webcrypto; }
if (!(globalThis as any).window.crypto) { (globalThis as any).window.crypto = webcrypto; }

import { createClient } from "@libsql/client";
import { VaultCryptoEngine, type EncryptedVaultPayload } from "../../../libs/core/client-vault/src/lib/aes-gcm";
import * as dotenv from "dotenv";
import * as fs from "fs";
import * as path from "path";
import chalk from "chalk";

dotenv.config();

// --- 2. PARSEO DE ARGUMENTOS ---
const args = process.argv.slice(2);
const argKey = args.find(a => a.startsWith("--key="))?.split("=")[1];
const isDryRun = args.includes("--dry-run");

const MASTER_KEY = argKey || process.env.NEXT_PUBLIC_ADMIN_PASSWORD;
const CRITICAL_COOKIES = ["__Secure-1PSID", "__Secure-3PSID", "SID", "HSID", "SSID"];
const REPORT_DIR = path.join(process.cwd(), "reports/identity");

async function execute_audit_and_report() {
    console.log(chalk.bold.magenta("\n🩺 [IDENTITY_JUDGE V3.0]: Initiating Forensic Audit & Sentencing...\n"));

    if (isDryRun) {
        console.log(chalk.bgYellow.black(" ⚠️  DRY RUN MODE ACTIVE: Database will NOT be modified. \n"));
    }

    if (!process.env.DATABASE_URL || !process.env.TURSO_AUTH_TOKEN) {
        console.error(chalk.red("❌ [CONFIG_FAULT]: DB Credentials missing."));
        process.exit(1);
    }

    if (!MASTER_KEY) {
        console.error(chalk.red("❌ [CONFIG_FAULT]: MASTER_KEY missing. Use --key=... or set .env"));
        process.exit(1);
    }

    console.log(chalk.gray(`   🔑 Using Master Key: ${MASTER_KEY.substring(0, 3)}***`));

    const db = createClient({
        url: process.env.DATABASE_URL,
        authToken: process.env.TURSO_AUTH_TOKEN,
    });

    const report = {
        generated_at: new Date().toISOString(),
        total_identities: 0,
        stats: { healthy: 0, expired: 0, corrupt: 0, decrypt_fail: 0 },
        identities: [] as any[]
    };

    try {
        const result = await db.execute("SELECT email, credentials_json, status, updated_at FROM identities");
        report.total_identities = result.rows.length;

        for (const row of result.rows) {
            const email = row.email as string;
            const current_status_db = row.status as string;
            const rawJson = row.credentials_json as string;

            let audit_status: "HEALTHY" | "EXPIRED" | "CORRUPT" | "DECRYPT_FAIL" = "HEALTHY";
            let days_remaining = null;
            let issues: string[] = [];

            process.stdout.write(chalk.white(`  👤 Analyzing: ${email.padEnd(35)} `));

            try {
                // 1. DESCIFRADO
                const payload = JSON.parse(rawJson);
                let cookies: any[] = [];

                if (payload.cipher_text_base64) {
                    const decryptedString = await VaultCryptoEngine.decryptPortable(
                        payload as EncryptedVaultPayload,
                        MASTER_KEY,
                        email
                    );
                    cookies = JSON.parse(decryptedString);
                } else {
                    cookies = payload;
                    issues.push("PLAIN_TEXT_STORAGE_DETECTED");
                }

                // 2. VALIDACIÓN ESTRUCTURAL
                if (!Array.isArray(cookies) || cookies.length === 0) {
                    audit_status = "CORRUPT";
                    issues.push("INVALID_JSON_STRUCTURE");
                } else {
                    // 3. VALIDACIÓN DE CLAVES
                    const foundKeys = cookies.map(c => c.name);
                    const missing = CRITICAL_COOKIES.filter(k => !foundKeys.includes(k));
                    if (missing.length > 0) {
                        audit_status = "CORRUPT";
                        issues.push(`MISSING_KEYS: ${missing.join(', ')}`);
                    }

                    // 4. VALIDACIÓN TEMPORAL
                    const now = Date.now() / 1000;
                    let minExpiration = Infinity;
                    let hasExpired = false;

                    cookies.forEach(c => {
                        const exp = c.expirationDate || c.expires;
                        if (exp) {
                            if (exp < now) hasExpired = true;
                            if (exp < minExpiration) minExpiration = exp;
                        }
                    });

                    if (minExpiration !== Infinity) {
                        days_remaining = Number(((minExpiration - now) / 86400).toFixed(1));
                    }

                    if (hasExpired) {
                        audit_status = "EXPIRED";
                        issues.push("COOKIES_EXPIRED");
                    }
                }

            } catch (e: any) {
                audit_status = "DECRYPT_FAIL";
                issues.push(e.message);
            }

            // 5. SENTENCIA Y ACTUALIZACIÓN DB
            if (audit_status === "HEALTHY") {
                const daysStr = days_remaining ? `${days_remaining} days` : "Unknown expiry";
                console.log(chalk.green(`[OK] (${daysStr})`));
                report.stats.healthy++;

                if (current_status_db !== 'active' && !isDryRun) {
                    await db.execute({
                        sql: "UPDATE identities SET status = 'active', updated_at = CURRENT_TIMESTAMP WHERE email = ?",
                        args: [email]
                    });
                    console.log(chalk.cyan(`     ✨ RESTORED: Status set to ACTIVE.`));
                }

            } else {
                const color = audit_status === "DECRYPT_FAIL" ? chalk.magenta : chalk.red;
                console.log(color(`[${audit_status}] -> ${issues.join(', ')}`));

                if (audit_status === "DECRYPT_FAIL") report.stats.decrypt_fail++;
                else if (audit_status === "CORRUPT") report.stats.corrupt++;
                else report.stats.expired++;

                const target_db_status = audit_status === "DECRYPT_FAIL" ? "revoked" : "expired";

                if (current_status_db !== target_db_status && !isDryRun) {
                    await db.execute({
                        sql: "UPDATE identities SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE email = ?",
                        args: [target_db_status, email]
                    });
                    console.log(chalk.yellow(`     ⚖️  SENTENCED: Status changed to ${target_db_status.toUpperCase()}`));
                }
            }

            report.identities.push({
                email,
                status_db: current_status_db,
                status_audit: audit_status,
                days_remaining,
                issues,
                last_updated: row.updated_at as string
            });
        }

        // 6. REPORTE FINAL
        if (!fs.existsSync(REPORT_DIR)) fs.mkdirSync(REPORT_DIR, { recursive: true });
        const filename = path.join(REPORT_DIR, "identity_health_report.json");
        fs.writeFileSync(filename, JSON.stringify(report, null, 2));

        console.log(chalk.bold.white("\n📊 SUMMARY:"));
        console.log(chalk.green(`   Healthy:      ${report.stats.healthy}`));
        console.log(chalk.yellow(`   Expired:      ${report.stats.expired}`));
        console.log(chalk.red(`   Corrupt:      ${report.stats.corrupt}`));
        console.log(chalk.magenta(`   Decrypt Fail: ${report.stats.decrypt_fail}`));
        console.log(chalk.bold.cyan(`\n💾 REPORT CRYSTALLIZED: ${filename}\n`));

    } catch (err: any) {
        console.error(chalk.red("\n🔥 FATAL SCAN ERROR:"), err.message);
    } finally {
        db.close();
    }
}

execute_audit_and_report();
// FIN DEL ARCHIVO [tools/scripts/db-turso/audit-identities-health.ts]

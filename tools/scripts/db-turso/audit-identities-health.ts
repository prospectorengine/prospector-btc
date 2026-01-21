/**
 * =================================================================
 * APARATO: IDENTITY HEALTH JUDGE (V4.0 - LIVE PROBE)
 * CLASIFICACIÓN: OPS DIAGNOSTIC (ESTRATO L6)
 * RESPONSABILIDAD: AUDITORÍA LOCAL, SONDA DE RED ANTI-BAN Y SENTENCIA
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. ACTIVE VALIDATION: Implementa 'verify_live_session' para detectar
 *    identidades revocadas por Google antes de que causen errores en el worker.
 * 2. REDIRECT SHIELD: Utiliza 'maxRedirects: 0' para detectar el muro de login
 *    sin interactuar con los sistemas de detección de bots de Google.
 * 3. ZERO REGRESSIONS: Mantiene el soporte para '--key' y '--dry-run' de la V3.0.
 * 4. HYGIENE: Nomenclatura nominal absoluta y documentación técnica MIT.
 * =================================================================
 */

// --- 1. ESTRATO DE POLIFILLS (Node.js -> WebCrypto) ---
import { webcrypto } from "node:crypto";
if (typeof (globalThis as any).window === "undefined") { (globalThis as any).window = globalThis; }
if (!globalThis.crypto) { (globalThis as any).crypto = webcrypto; }
if (!(globalThis as any).window.crypto) { (globalThis as any).window.crypto = webcrypto; }

import { createClient, type Client } from "@libsql/client";
import { VaultCryptoEngine, type EncryptedVaultPayload } from "../../../libs/core/client-vault/src/lib/aes-gcm";
import axios from "axios";
import * as dotenv from "dotenv";
import * as fs from "fs";
import * as path from "path";
import chalk from "chalk";

dotenv.config();

// --- 2. PARSEO DE ARGUMENTOS ESTRATÉGICOS ---
const command_line_arguments = process.argv.slice(2);
const argument_master_key = command_line_arguments.find(a => a.startsWith("--key="))?.split("=")[1];
const is_dry_run_active = command_line_arguments.includes("--dry-run");

const MASTER_KEY_AUTHORITY = argument_master_key || process.env.NEXT_PUBLIC_ADMIN_PASSWORD;
const CRITICAL_COOKIES_WHITELIST = ["__Secure-1PSID", "__Secure-3PSID", "SID", "HSID", "SSID"];
const REPORT_STORAGE_DIRECTORY = path.join(process.cwd(), "reports/identity");

/**
 * Realiza una sonda de red hacia el Motor de Identidad de Google.
 *
 * # Mathematical Proof (Binary Liveness):
 * Si el servidor retorna 200 OK en un endpoint privado, la sesión es válida.
 * Si intenta una redirección (302), la sesión ha sido invalidada en el servidor.
 */
async function verify_google_session_liveness(
    cookies_collection: any[],
    user_agent_signature: string
): Promise<boolean> {
    const cookie_payload_string = cookies_collection
        .map(cookie => `${cookie.name}=${cookie.value}`)
        .join('; ');

    try {
        const network_response = await axios.get('https://myaccount.google.com/notifications', {
            headers: {
                'Cookie': cookie_payload_string,
                'User-Agent': user_agent_signature,
                'Accept': 'text/html',
                'Cache-Control': 'no-cache'
            },
            maxRedirects: 0, // No seguir el rastro a la página de login
            timeout: 8000
        });

        return network_response.status === 200;
    } catch (unidentified_fault: any) {
        // Un error 302 o 401 confirma que la identidad ha sido revocada
        return false;
    }
}

/**
 * Ejecuta la secuencia de auditoría forense y reporte.
 */
async function execute_sovereign_health_audit() {
    console.log(chalk.bold.magenta("\n🕵️ [HYDRA_JUDGE V4.0]: Initiating Forensic Audit & Sentencing...\n"));

    if (is_dry_run_active) {
        console.log(chalk.bgYellow.black(" ⚠️  DRY RUN MODE: Strata observation only. No DB mutations. \n"));
    }

    if (!process.env.DATABASE_URL || !process.env.TURSO_AUTH_TOKEN || !MASTER_KEY_AUTHORITY) {
        console.error(chalk.red("❌ [CONFIG_FAULT]: Mandatory credentials missing. Check .env"));
        process.exit(1);
    }

    const database_client: Client = createClient({
        url: process.env.DATABASE_URL,
        authToken: process.env.TURSO_AUTH_TOKEN,
    });

    const audit_final_report = {
        generated_at: new Date().toISOString(),
        total_identities_analyzed: 0,
        stats: { healthy: 0, expired: 0, corrupt: 0, decrypt_fail: 0, revoked_by_server: 0 },
        identities: [] as any[]
    };

    try {
        const query_result = await database_client.execute(
            "SELECT email, credentials_json, status, user_agent, updated_at FROM identities"
        );

        audit_final_report.total_identities_analyzed = query_result.rows.length;

        for (const data_row of query_result.rows) {
            const operator_email = data_row.email as string;
            const current_db_status = data_row.status as string;
            const raw_credentials_json = data_row.credentials_json as string;
            const user_agent_signature = (data_row.user_agent as string) || "Mozilla/5.0 (Windows NT 10.0; Win64; x64)";

            let audit_verdict: "HEALTHY" | "EXPIRED" | "CORRUPT" | "DECRYPT_FAIL" | "REVOKED_BY_SERVER" = "HEALTHY";
            let days_remaining_projection = null;
            let identified_issues: string[] = [];
            let cookies_array: any[] = [];

            process.stdout.write(chalk.white(`  👤 Analyzing: ${operator_email.padEnd(35)} `));

            try {
                // 1. DESCIFRADO DE BÓVEDA
                const encrypted_payload = JSON.parse(raw_credentials_json);
                if (encrypted_payload.cipher_text_base64) {
                    const decrypted_string = await VaultCryptoEngine.decryptPortable(
                        encrypted_payload as EncryptedVaultPayload,
                        MASTER_KEY_AUTHORITY,
                        operator_email
                    );
                    cookies_array = JSON.parse(decrypted_string);
                } else {
                    cookies_array = encrypted_payload;
                    identified_issues.push("PLAIN_TEXT_STORAGE_DETECTED");
                }

                // 2. VALIDACIÓN ESTRUCTURAL
                if (!Array.isArray(cookies_array) || cookies_array.length === 0) {
                    audit_verdict = "CORRUPT";
                    identified_issues.push("INVALID_JSON_STRUCTURE");
                } else {
                    const found_cookie_keys = cookies_array.map(c => c.name);
                    const missing_critical = CRITICAL_COOKIES_WHITELIST.filter(k => !found_cookie_keys.includes(k));

                    if (missing_critical.length > 0) {
                        audit_verdict = "CORRUPT";
                        identified_issues.push(`MISSING_KEYS: ${missing_critical.join(', ')}`);
                    }

                    // 3. VALIDACIÓN TEMPORAL (Local)
                    const current_time_unix = Date.now() / 1000;
                    let minimum_expiration_ts = Infinity;

                    cookies_array.forEach(cookie => {
                        const exp = cookie.expirationDate || cookie.expires;
                        if (exp && exp < minimum_expiration_ts) minimum_expiration_ts = exp;
                    });

                    if (minimum_expiration_ts !== Infinity) {
                        days_remaining_projection = Number(((minimum_expiration_ts - current_time_unix) / 86400).toFixed(1));
                        if (minimum_expiration_ts < current_time_unix) {
                            audit_verdict = "EXPIRED";
                            identified_issues.push("COOKIES_EXPIRED_BY_TIMESTAMP");
                        }
                    }
                }

                // 4. SONDA DE RED ACTIVA (Anti-Ban Probing)
                if (audit_verdict === "HEALTHY") {
                    const is_actually_live = await verify_google_session_liveness(cookies_array, user_agent_signature);
                    if (!is_actually_live) {
                        audit_verdict = "REVOKED_BY_SERVER";
                        identified_issues.push("SERVER_SIDE_TERMINATION_DETECTED");
                    }
                }

            } catch (decryption_fault: any) {
                audit_verdict = "DECRYPT_FAIL";
                identified_issues.push(decryption_fault.message);
            }

            // 5. SENTENCIA Y ACTUALIZACIÓN DEL LEDGER TÁCTICO
            if (audit_verdict === "HEALTHY") {
                const days_label = days_remaining_projection ? `${days_remaining_projection} days` : "Indefinite";
                console.log(chalk.green(`[LIVE] (${days_label})`));
                audit_final_report.stats.healthy++;

                if (current_db_status !== 'active' && !is_dry_run_active) {
                    await database_client.execute({
                        sql: "UPDATE identities SET status = 'active', updated_at = CURRENT_TIMESTAMP WHERE email = ?",
                        args: [operator_email]
                    });
                }
            } else {
                const color_theme = audit_verdict === "DECRYPT_FAIL" ? chalk.magenta : chalk.red;
                console.log(color_theme(`[${audit_verdict}] -> ${identified_issues.join(', ')}`));

                switch (audit_verdict) {
                    case "DECRYPT_FAIL": audit_final_report.stats.decrypt_fail++; break;
                    case "CORRUPT": audit_final_report.stats.corrupt++; break;
                    case "EXPIRED": audit_final_report.stats.expired++; break;
                    case "REVOKED_BY_SERVER": audit_final_report.stats.revoked_by_server++; break;
                }

                const target_db_status = (audit_verdict === "DECRYPT_FAIL" || audit_verdict === "REVOKED_BY_SERVER") ? "revoked" : "expired";

                if (current_db_status !== target_db_status && !is_dry_run_active) {
                    await database_client.execute({
                        sql: "UPDATE identities SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE email = ?",
                        args: [target_db_status, operator_email]
                    });
                }
            }

            audit_final_report.identities.push({
                email: operator_email,
                verdict: audit_verdict,
                days_remaning: days_remaining_projection,
                issues: identified_issues
            });
        }

        // 6. CRISTALIZACIÓN DE LA EVIDENCIA
        if (!fs.existsSync(REPORT_STORAGE_DIRECTORY)) fs.mkdirSync(REPORT_STORAGE_DIRECTORY, { recursive: true });
        const final_report_path = path.join(REPORT_STORAGE_DIRECTORY, "identity_health_report.json");
        fs.writeFileSync(final_report_path, JSON.stringify(audit_final_report, null, 2));

        console.log(chalk.bold.white("\n📊 AUDIT SUMMARY:"));
        console.log(chalk.green(`   Healthy:      ${audit_final_report.stats.healthy}`));
        console.log(chalk.red(`   Revoked/Fault: ${audit_final_report.stats.revoked_by_server + audit_final_report.stats.decrypt_fail + audit_final_report.stats.corrupt}`));
        console.log(chalk.bold.cyan(`\n💾 REPORT CRYSTALLIZED: ${final_report_path}\n`));

    } catch (critical_fault: any) {
        console.error(chalk.red("\n🔥 FATAL_AUDIT_COLLAPSE:"), critical_fault.message);
    } finally {
        database_client.close();
    }
}

execute_sovereign_health_audit();

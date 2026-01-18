// INICIO DEL ARCHIVO [tools/scripts/db-turso/audit-encryption.ts]
/**
 * =================================================================
 * APARATO: VAULT ENCRYPTION AUDITOR (V1.0)
 * CLASIFICACIÓN: OPS DIAGNOSTIC (L6)
 * RESPONSABILIDAD: VERIFICACIÓN DEL FORMATO DE DATOS EN 'IDENTITIES'
 * =================================================================
 */

import { createClient } from "@libsql/client";
import * as dotenv from "dotenv";
import chalk from "chalk";

dotenv.config();

async function audit_encryption_status() {
  console.log(chalk.bold.cyan("\n🔐 [VAULT_AUDIT]: Inspecting Credentials Format...\n"));

  const client = createClient({
    url: process.env.DATABASE_URL!,
    authToken: process.env.TURSO_AUTH_TOKEN!,
  });

  try {
    const result = await client.execute("SELECT email, credentials_json FROM identities LIMIT 1");

    if (result.rows.length === 0) {
      console.log(chalk.yellow("⚠️  VAULT IS EMPTY. No identities to audit."));
      return;
    }

    const row = result.rows[0];
    const email = row.email;
    const rawData = row.credentials_json as string;

    console.log(`👤 Identity: ${chalk.white(email)}`);
    console.log(chalk.gray("--------------------------------------------------"));

    // Análisis Heurístico del Payload
    let parsed;
    try {
        parsed = JSON.parse(rawData);
    } catch {
        console.log(chalk.red("❌ DATA CORRUPTION: Invalid JSON format."));
        return;
    }

    if (parsed.cipher_text_base64 && parsed.initialization_vector_base64) {
        console.log(chalk.green("✅ ENCRYPTION CONFIRMED (AES-GCM Payload Detected)"));
        console.log(`   Cipher Length: ${parsed.cipher_text_base64.length} chars`);
        console.log(`   IV Present:    YES`);
        console.log(`   Salt Present:  ${parsed.salt_base64 ? "YES" : "NO"}`);
    } else if (Array.isArray(parsed)) {
        console.log(chalk.red("❌ WARNING: PLAIN TEXT COOKIES DETECTED (Legacy Format)"));
        console.log(`   Cookie Count: ${parsed.length}`);
        console.log(chalk.yellow("   ADVISORY: Re-inject via Dashboard for Zero-Knowledge protection."));
    } else {
        console.log(chalk.red("❓ UNKNOWN FORMAT"));
        console.log(parsed);
    }

    console.log(chalk.gray("--------------------------------------------------\n"));

  } catch (err: any) {
    console.error(chalk.red(`🔥 AUDIT ERROR: ${err.message}`));
  } finally {
    client.close();
  }
}

audit_encryption_status();
// FIN DEL ARCHIVO [tools/scripts/db-turso/audit-encryption.ts]

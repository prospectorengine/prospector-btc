// INICIO DEL ARCHIVO [tools/scripts/inject-sovereign-identity.ts]
/**
 * =================================================================
 * APARATO: IDENTITY SECURE INJECTOR (V2.2 - SCHEMA ALIGNED)
 * CLASIFICACIÓN: OPS UTILITY (ESTRATO L6)
 * RESPONSABILIDAD: CIFRADO AES-GCM E INYECCIÓN IDEMPOTENTE
 *
 * VISION HIPER-HOLÍSTICA:
 * - Polyfill de WebCrypto para Node.js.
 * - Corrección de restricción UNIQUE (platform, email) para UPSERT.
 * =================================================================
 */

// 1. INYECCIÓN DE ENTORNOS SIMULADOS (MOCKS)
import { webcrypto } from "node:crypto";

if (typeof (globalThis as any).window === "undefined") {
    (globalThis as any).window = globalThis;
}

if (!globalThis.crypto) {
    (globalThis as any).crypto = webcrypto;
}
if (!(globalThis as any).window.crypto) {
    (globalThis as any).window.crypto = webcrypto;
}

// 2. IMPORTS DEL SISTEMA
import { createClient } from "@libsql/client";
import { VaultCryptoEngine } from "../../libs/core/client-vault/src/lib/aes-gcm";
import * as dotenv from "dotenv";
import chalk from "chalk";
import { v4 as uuidv4 } from "uuid";

dotenv.config();

// --- CONFIGURACIÓN ESTRATÉGICA ---
const OPERATOR_EMAIL = process.env.OPERATOR_EMAIL || "razpodesta@gmail.com";
const MASTER_KEY = process.env.NEXT_PUBLIC_ADMIN_PASSWORD || "Netflix69";

// --- MATERIAL BINARIO (COOKIES REALES) ---
const RAW_COOKIES = [
  { "domain": ".google.com", "name": "SAPISID", "value": "3CyPUDLUd8Ms0Q1-/AvzxaKONbfPdSt0oD", "path": "/", "secure": true },
  { "domain": ".google.com", "name": "__Secure-1PSID", "value": "g.a0005QhqOeI0_4_rupcs_oDy8L87ddzbdSkkUvGYLk0ieKl3NbaCD6xvxOvTilyyQZSeiBNtmgACgYKAdkSARcSFQHGX2MiI7IfePYXuqhHsOKc7eF63hoVAUF8yKq-qwNcctSrkP5DmtrOOOcL0076", "path": "/", "secure": true },
  { "domain": ".google.com", "name": "SSID", "value": "AF2pb47Wls3mXAB1N", "path": "/", "secure": true }
];

async function execute_secure_injection_sequence(): Promise<void> {
    console.log(chalk.bold.magenta("\n🔐 [SECURITY_INJECTOR]: Initiating Zero-Knowledge Encryption Sequence..."));

    const database_url = process.env.DATABASE_URL;
    const auth_token = process.env.TURSO_AUTH_TOKEN;

    if (!database_url || !auth_token) {
        console.error(chalk.red("❌ [FAULT]: Missing Turso credentials in .env"));
        return;
    }

    if (!MASTER_KEY) {
        console.error(chalk.red("❌ [FAULT]: MASTER_KEY is missing."));
        return;
    }

    const client = createClient({ url: database_url, authToken: auth_token });

    try {
        console.log(chalk.cyan(`  📦 Encrypting payload for: ${OPERATOR_EMAIL}...`));

        const encrypted_payload = await VaultCryptoEngine.encryptPortable(
            JSON.stringify(RAW_COOKIES),
            MASTER_KEY,
            OPERATOR_EMAIL
        );

        const serialized_payload = JSON.stringify(encrypted_payload);
        const identity_uuid = uuidv4();

        console.log(chalk.gray(`     Ciphertext generated (${serialized_payload.length} bytes). Injecting...`));

        // ✅ FIX: Corrección de cláusula ON CONFLICT para coincidir con UNIQUE(platform, email)
        await client.execute({
            sql: `INSERT INTO identities (id, platform, email, credentials_json, user_agent, status, usage_count, created_at, updated_at)
                  VALUES (?, 'google_colab', ?, ?, 'Mozilla/5.0 (Windows NT 10.0; Win64; x64)', 'active', 0, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
                  ON CONFLICT(platform, email) DO UPDATE SET
                  credentials_json = excluded.credentials_json,
                  status = 'active',
                  updated_at = CURRENT_TIMESTAMP`,
            args: [identity_uuid, OPERATOR_EMAIL, serialized_payload]
        });

        console.log(chalk.bold.green(`\n✅ SUCCESS: Identity crystallized in Turso Cloud (AES-256-GCM).`));

    } catch (error: any) {
        console.error(chalk.red(`\n🔥 [INJECTION_CRITICAL_FAULT]: ${error.message}`));
        if (error.stack) console.error(chalk.dim(error.stack));
    } finally {
        client.close();
    }
}

execute_secure_injection_sequence();
// FIN DEL ARCHIVO [tools/scripts/inject-sovereign-identity.ts]

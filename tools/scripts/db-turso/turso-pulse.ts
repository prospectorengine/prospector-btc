/**
 * =================================================================
 * APARATO: TURSO PULSE AUDITOR (V1.2 - REMOTE ONLY)
 * CLASIFICACIÓN: INFRASTRUCTURE DIAGNOSTIC (ESTRATO L6)
 * RESPONSABILIDAD: Handshake de salud y telemetría de latencia remota.
 *
 * # Mathematical Proof:
 * Mide el RTT (Round Trip Time) entre la terminal del operador y el
 * nodo de Turso más cercano utilizando una consulta de costo cero.
 * =================================================================
 */

import { createClient, type Client } from '@libsql/client';
import chalk from 'chalk';
import * as dotenv from 'dotenv';

dotenv.config();

async function executeRemotePulseSequence(): Promise<void> {
    const databaseUrl = process.env.DATABASE_URL;
    const authToken = process.env.TURSO_AUTH_TOKEN;

    console.log(chalk.bold.cyan("\n📡 [TURSO_PULSE]: Initiating Remote Handshake...\n"));

    if (!databaseUrl || !authToken) {
        console.error(chalk.red("❌ [CONFIG_FAULT]: Missing DATABASE_URL or TURSO_AUTH_TOKEN in .env"));
        process.exit(1);
    }

    // Configuración del enlace táctico remoto
    const client: Client = createClient({
        url: databaseUrl,
        authToken: authToken,
    });

    const executionStartTimestamp = performance.now();

    try {
        // Ejecución de sonda atómica
        const result = await client.execute("SELECT 1 as liveness_signal");
        const roundTripLatency = (performance.now() - executionStartTimestamp).toFixed(2);

        if (result.rows.length > 0) {
            console.log(chalk.green(`  🟢 UPLINK_STATUS: SECURE_CONNECTION`));
            console.log(chalk.green(`  ⏱️  RTT_LATENCY:  ${roundTripLatency}ms`));
            console.log(chalk.gray(`  🔗 TARGET:        ${databaseUrl.split('@')[0]}`)); // Ofuscación de seguridad
        }
    } catch (error: any) {
        console.error(chalk.bgRed.white("\n🔥 [PULSE_CRITICAL_FAULT]: Remote Link Severed"));
        console.error(chalk.red(`  Reason: ${error.message}`));
        process.exit(1);
    }
}

executeRemotePulseSequence();

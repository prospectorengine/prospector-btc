/**
 * =================================================================
 * APARATO: TURSO SNAPSHOT DUMPER (V1.2 - REMOTE ONLY)
 * CLASIFICACIÓN: INFRASTRUCTURE DIAGNOSTIC (ESTRATO L6)
 * RESPONSABILIDAD: Volcado de datos remotos para auditoría forense.
 * =================================================================
 */

import { createClient, type Client } from '@libsql/client';
import chalk from 'chalk';
import * as dotenv from 'dotenv';
import * as fs from 'fs';
import * as path from 'path';

dotenv.config();

async function executeRemoteDataCapture(): Promise<void> {
    const databaseUrl = process.env.DATABASE_URL;
    const authToken = process.env.TURSO_AUTH_TOKEN;

    console.log(chalk.bold.yellow("\n📸 [TURSO_SNAPSHOT]: Capturing Remote Ledger Content...\n"));

    if (!databaseUrl || !authToken) {
        console.error(chalk.red("❌ [CONFIG_FAULT]: Strategic credentials missing."));
        process.exit(1);
    }

    const client: Client = createClient({ url: databaseUrl, authToken });

    const snapshotArtifact: any = {
        capture_timestamp: new Date().toISOString(),
        remote_endpoint: databaseUrl.split('@')[0],
        strata_content: {}
    };

    try {
        const tacticalTables = ['jobs', 'findings', 'identities'];

        for (const tableName of tacticalTables) {
            console.log(chalk.gray(`  Dumping strata: ${tableName}...`));
            const queryResult = await client.execute(`SELECT * FROM ${tableName} ORDER BY rowid DESC LIMIT 50`);
            snapshotArtifact.strata_content[tableName] = queryResult.rows;
        }

        const artifactFileName = `turso_remote_dump_${Date.now()}.json`;
        const artifactPath = path.join(process.cwd(), artifactFileName);

        fs.writeFileSync(artifactPath, JSON.stringify(snapshotArtifact, null, 2));

        console.log(chalk.green(`\n✅ SNAPSHOT_SECURED: ${artifactFileName}`));

        if (snapshotArtifact.strata_content.findings.length > 0) {
            console.log(chalk.cyan(`  🎯 LATEST_DISCOVERY: ${snapshotArtifact.strata_content.findings[0].address}`));
        }

    } catch (error: any) {
        console.error(chalk.red(`\n❌ [SNAPSHOT_FAULT]: ${error.message}`));
    }
}

executeRemoteDataCapture();

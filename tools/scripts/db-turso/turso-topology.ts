/**
 * =================================================================
 * APARATO: TURSO TOPOLOGY INSPECTOR (V1.2 - REMOTE ONLY)
 * CLASIFICACIÓN: INFRASTRUCTURE DIAGNOSTIC (ESTRATO L6)
 * RESPONSABILIDAD: Auditoría estructural y conteo de registros remotos.
 * =================================================================
 */

import { createClient, type Client } from '@libsql/client';
import chalk from 'chalk';
import * as dotenv from 'dotenv';

dotenv.config();

async function executeTopologyAudit(): Promise<void> {
    const databaseUrl = process.env.DATABASE_URL;
    const authToken = process.env.TURSO_AUTH_TOKEN;

    console.log(chalk.bold.magenta("\n🕵️  [TURSO_TOPOLOGY]: Analyzing Remote Structural Strata...\n"));

    if (!databaseUrl || !authToken) {
        console.error(chalk.red("❌ [CONFIG_FAULT]: Credentials missing in environment."));
        process.exit(1);
    }

    const client: Client = createClient({ url: databaseUrl, authToken });

    try {
        // 1. Auditoría de Inventario de Tablas
        console.log(chalk.yellow("📋 TABLE_INVENTORY:"));
        const masterScan = await client.execute(
            "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'"
        );

        for (const row of masterScan.rows) {
            const tableName = row.name as string;
            const countResult = await client.execute(`SELECT COUNT(*) as total FROM ${tableName}`);
            const totalRecords = countResult.rows[0].total;
            console.log(chalk.white(`  - ${tableName.padEnd(20)} [${totalRecords} records]`));
        }

        // 2. Auditoría de Motores de Aceleración (Índices)
        console.log(chalk.yellow("\n🚀 ACCELERATION_INDEXES:"));
        const indexScan = await client.execute(
            "SELECT name, tbl_name FROM sqlite_master WHERE type='index' AND name NOT LIKE 'sqlite_%'"
        );
        for (const row of indexScan.rows) {
            console.log(chalk.gray(`  - ${String(row.name).padEnd(25)} on table [${row.tbl_name}]`));
        }

    } catch (error: any) {
        console.error(chalk.red(`\n❌ [TOPOLOGY_FAULT]: ${error.message}`));
    }
}

executeTopologyAudit();

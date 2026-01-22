/**
 * =================================================================
 * APARATO: STRATEGIC ARCHAEOLOGY EXTRACTOR (V1.1 - STRICT TYPE FIXED)
 * CLASIFICACIÓN: OPS DIAGNOSTIC (ESTRATO L6)
 * RESPONSABILIDAD: EXTRACCIÓN MOLECULAR DE ESQUEMA POSTGRESQL
 * =================================================================
 */

import { Client, QueryResult } from 'pg';
import * as fs from 'fs';
import * as path from 'path';
import chalk from 'chalk';
import { createLogger } from '../../../libs/shared/heimdall-ts/src/index';

const logger = createLogger("Ops:StrategicExtractor");

// Definición de interfaces para erradicar el error 'implicit any'
interface EnumRow {
    name: string;
    values: string;
}

interface ColumnRow {
    column_name: string;
    data_type: string;
    column_default: string | null;
    is_nullable: string;
}

interface PolicyRow {
    policyname: string;
    cmd: string;
    roles: string[];
    qual: string;
}

async function execute_archaeology_extraction(): Promise<void> {
  const source_url_argument = process.argv.find(arg => arg.startsWith('--source='))?.split('=')[1];

  if (!source_url_argument) {
    console.error(chalk.bgRed.white("\n ❌ FATAL_FAULT: Missing --source argument. \n"));
    process.exit(1);
  }

  const postgres_client = new Client({
    connectionString: source_url_argument,
    ssl: { rejectUnauthorized: false }
  });

  const output_directory = path.join(process.cwd(), 'reports', 'archaeology');
  if (!fs.existsSync(output_directory)) fs.mkdirSync(output_directory, { recursive: true });

  const output_file = path.join(output_directory, `supabase_old_schema_${Date.now()}.sql`);

  try {
    const trace = logger.track("Schema_Extraction_Pulse");
    await postgres_client.connect();

    let consolidated_sql_dump = `-- 💠 PROSPECTOR BTC // STRATEGIC SCHEMA DUMP (V1.1)\n\n`;

    // 1. EXTRACCIÓN DE ENUMS
    console.warn(chalk.cyan("  🧪 Phase 1: Extracting Custom Types (Enums)..."));
    const enum_query = `
      SELECT n.nspname as schema, t.typname as name, string_agg(e.enumlabel, ', ' ORDER BY e.enumsortorder) as values
      FROM pg_type t
      JOIN pg_enum e ON t.oid = e.enumtypid
      JOIN pg_namespace n ON n.oid = t.typnamespace
      WHERE n.nspname = 'public'
      GROUP BY n.nspname, t.typname;
    `;
    const enums: QueryResult<EnumRow> = await postgres_client.query(enum_query);
    enums.rows.forEach((r: EnumRow) => {
        consolidated_sql_dump += `-- Type: ${r.name}\nCREATE TYPE public.${r.name} AS ENUM (${r.values.split(', ').map((v:string) => `'${v}'`).join(', ')});\n\n`;
    });

    // 2. EXTRACCIÓN DE TABLAS
    console.warn(chalk.cyan("  🧪 Phase 2: Mapping Table Topology..."));
    const tables_query = `SELECT tablename FROM pg_tables WHERE schemaname = 'public'`;
    const tables = await postgres_client.query(tables_query);

    for (const table of tables.rows) {
        const table_name = table.tablename;
        console.warn(chalk.gray(`     ↳ Auditing table: ${table_name}`));

        const cols_query = `
            SELECT column_name, data_type, column_default, is_nullable
            FROM information_schema.columns
            WHERE table_name = '${table_name}' AND table_schema = 'public'
        `;
        const columns: QueryResult<ColumnRow> = await postgres_client.query(cols_query);

        consolidated_sql_dump += `CREATE TABLE public.${table_name} (\n`;
        const col_definitions = columns.rows.map((c: ColumnRow) => {
            return `  ${c.column_name} ${c.data_type}${c.is_nullable === 'NO' ? ' NOT NULL' : ''}${c.column_default ? ' DEFAULT ' + c.column_default : ''}`;
        });
        consolidated_sql_dump += col_definitions.join(',\n') + `\n);\n\n`;

        // 3. EXTRACCIÓN DE POLÍTICAS RLS
        const rls_query = `SELECT policyname, cmd, roles, qual FROM pg_policies WHERE tablename = '${table_name}'`;
        const policies: QueryResult<PolicyRow> = await postgres_client.query(rls_query);
        policies.rows.forEach((p: PolicyRow) => {
            consolidated_sql_dump += `ALTER TABLE public.${table_name} ENABLE ROW LEVEL SECURITY;\n`;
            consolidated_sql_dump += `CREATE POLICY "${p.policyname}" ON public.${table_name} FOR ${p.cmd} TO ${p.roles.join(', ')} USING (${p.qual});\n\n`;
        });
    }

    fs.writeFileSync(output_file, consolidated_sql_dump);
    trace.ok({ file: output_file });

    console.warn(chalk.bold.green(`\n✅ EXTRACTION SUCCESSFUL: ${output_file}\n`));

  } catch (error: any) {
    console.error(chalk.red(`\n🔥 FATAL_ERROR: ${error.message}`));
  } finally {
    await postgres_client.end();
  }
}

execute_archaeology_extraction();

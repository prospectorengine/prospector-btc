// INICIO DEL ARCHIVO [tools/scripts/supabase/check-schema-status.ts]
/**
 * =================================================================
 * APARATO: STRATEGIC SCHEMA VERIFIER (V1.0)
 * CLASIFICACIÓN: OPS TOOL
 * RESPONSABILIDAD: Verificación de existencia de tablas críticas
 * =================================================================
 */

import { createClient } from '@supabase/supabase-js';
import * as dotenv from 'dotenv';
import chalk from 'chalk';

dotenv.config();

const TABLES = ['archived_jobs', 'vault_items', 'profiles', 'workspaces'];

async function verify() {
    console.log(chalk.cyan("\n🛰️  VERIFICANDO ESTRUCTURA MOTOR B (SUPABASE)...\n"));

    const url = process.env.NEXT_PUBLIC_SUPABASE_URL!;
    const key = process.env.SUPABASE_SERVICE_ROLE_KEY!;

    if(!url || !key) {
        console.error(chalk.red("❌ Faltan credenciales de Supabase en .env"));
        process.exit(1);
    }

    const client = createClient(url, key);

    for (const table of TABLES) {
        // Intentamos un SELECT HEAD (limit 0) para ver si la tabla existe
        const { error } = await client.from(table).select('id').limit(1);

        if (error && error.code === '42P01') { // undefined_table
            console.log(chalk.red(`  ❌ TABLA FALTANTE: [public.${table}]`));
        } else if (error) {
             // PGRST205 también indica tabla no encontrada en la cache de esquema
             if(error.code === 'PGRST205') {
                 console.log(chalk.red(`  ❌ TABLA NO EXPUESTA/FALTANTE: [public.${table}]`));
             } else {
                 console.log(chalk.yellow(`  ⚠️  ERROR EN [${table}]: ${error.message}`));
             }
        } else {
            console.log(chalk.green(`  ✅ TABLA OK: [public.${table}]`));
        }
    }

    console.log(chalk.gray("\n  Si faltan tablas, debe ejecutar el SQL de 'tools/supabase/schema.sql'"));
    console.log(chalk.gray("  en el SQL Editor de su proyecto Supabase."));
}

verify();
// FIN DEL ARCHIVO [tools/scripts/supabase/check-schema-status.ts]

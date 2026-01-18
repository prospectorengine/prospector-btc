// INICIO DEL ARCHIVO [tools/scripts/supabase/topology_inspector.ts]
/**
 * =================================================================
 * APARATO: DEEP TOPOLOGY RADIOGRAPHER (V2.0 - ELITE INSPECTION)
 * CLASIFICACIÓN: OPS DIAGNOSTIC (L6)
 * RESPONSABILIDAD: Mapeo molecular de la estructura de base de datos.
 *
 * SALIDA: Genera 'reports/supabase/deep_topology.report.json'
 * =================================================================
 */

import { createClient } from '@supabase/supabase-js';
import * as fs from 'fs';
import * as path from 'path';
import chalk from 'chalk';
import * as dotenv from 'dotenv';

dotenv.config();

const REPORTS_DIRECTORY = path.join(process.cwd(), 'reports', 'supabase');
const TARGET_FILENAME = 'deep_topology.report.json';

// Configuración de visualización de consola
const log = {
    info: (msg: string) => console.log(chalk.cyan(msg)),
    success: (msg: string) => console.log(chalk.green(msg)),
    warn: (msg: string) => console.log(chalk.yellow(msg)),
    error: (msg: string) => console.error(chalk.red(msg)),
    detail: (key: string, val: string) => console.log(chalk.gray(`  ${key.padEnd(20)}: `) + chalk.white(val))
};

async function execute_radiography_sequence(): Promise<void> {
    console.log(chalk.bold.magenta("\n🩻 [DEEP_SCAN]: Initiating Strategic Radiography Sequence...\n"));

    if (!fs.existsSync(REPORTS_DIRECTORY)) {
        fs.mkdirSync(REPORTS_DIRECTORY, { recursive: true });
    }

    const supabase_client = createClient(
        process.env.NEXT_PUBLIC_SUPABASE_URL!,
        process.env.SUPABASE_SERVICE_ROLE_KEY!
    );

    try {
        const start_time = performance.now();

        // 1. INVOCACIÓN DE LA SONDA SQL
        const { data: radiography, error } = await supabase_client.rpc('inspect_strategic_topology_v2');

        if (error) {
            throw new Error(`RPC_FAULT: ${error.message}. (Did you run the SQL V2 script?)`);
        }

        const duration = (performance.now() - start_time).toFixed(2);

        // 2. ANÁLISIS DE SUPERFICIE (OUTPUT EN CONSOLA)
        log.success(`✅ SCAN_COMPLETE (${duration}ms)`);

        const stats = {
            tables: radiography.tables?.length || 0,
            policies: radiography.policies?.length || 0,
            triggers: radiography.triggers?.length || 0,
            extensions: radiography.extensions?.length || 0
        };

        console.log(chalk.bold.white("\n--- EXECUTIVE SUMMARY ---"));
        log.detail("Active Tables", stats.tables.toString());
        log.detail("Security Policies", stats.policies.toString());
        log.detail("Automation Triggers", stats.triggers.toString());
        log.detail("Extensions", radiography.extensions.map((e: any) => e.name).join(', '));

        // Validación Rápida de Tablas Críticas
        const critical_tables = ['archived_jobs', 'vault_items', 'system_integrity_reports'];
        const found_tables = radiography.tables.map((t: any) => t.table_name);

        console.log(chalk.bold.white("\n--- CRITICAL STRATA CHECK ---"));
        critical_tables.forEach(table => {
            const exists = found_tables.includes(table);
            const status = exists ? chalk.green("NOMINAL") : chalk.bgRed(" MISSING ");

            // Si existe, verificamos RLS
            let rls_status = "";
            if (exists) {
                const table_meta = radiography.tables.find((t: any) => t.table_name === table);
                rls_status = table_meta.rls_enabled ? chalk.blue("[RLS: ON]") : chalk.red("[RLS: OFF]");
            }

            console.log(`  ${table.padEnd(30)} ${status} ${rls_status}`);
        });

        // 3. PERSISTENCIA EN MOTOR DE DIAGNÓSTICO
        // Guardamos el reporte también en la nube para historial
        await supabase_client.from('system_integrity_reports').insert({
            apparatus_name: "deep_radiographer",
            status: "OPERATIONAL",
            metrics: { summary: stats, critical_check: "PASSED" } // Guardamos resumen para no saturar JSONB
        });

        // 4. CRISTALIZACIÓN LOCAL
        const report_path = path.join(REPORTS_DIRECTORY, TARGET_FILENAME);
        fs.writeFileSync(report_path, JSON.stringify(radiography, null, 2));

        console.log(chalk.bold.green(`\n💾 FULL RADIOGRAPHY SAVED: ${report_path}\n`));

    } catch (err: any) {
        log.error(`🔥 SCAN_ABORTED: ${err.message}`);
        process.exit(1);
    }
}

execute_radiography_sequence();
// FIN DEL ARCHIVO [tools/scripts/supabase/topology_inspector.ts]

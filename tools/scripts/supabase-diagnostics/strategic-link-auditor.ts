/**
 * =================================================================
 * APARATO: STRATEGIC LINK AUDITOR (V11.0 - SOBERANO)
 * CLASIFICACIÓN: OPS DIAGNOSTIC (L6)
 * RESPONSABILIDAD: CERTIFICACIÓN DE CONECTIVIDAD Y PULSO DE RED
 * =================================================================
 */

import { createClient } from '@supabase/supabase-js';
import * as fs from 'fs';
import * as path from 'path';
import chalk from 'chalk';
import * as dotenv from 'dotenv';

dotenv.config();

async function execute_connection_audit() {
    const start_timestamp = performance.now();
    const supabase_url = process.env.NEXT_PUBLIC_SUPABASE_URL!;
    const service_key = process.env.SUPABASE_SERVICE_ROLE_KEY!;

    const report = {
        apparatus: "strategic-link-auditor",
        version: "11.0",
        timestamp: new Date().toISOString(),
        connectivity: { status: "unknown", latency_ms: 0 },
        authentication: { role: "service_role", authorized: false }
    };

    const client = createClient(supabase_url, service_key);

    try {
        // Ejecutamos una query minimalista para medir latencia real
        const { error } = await client.from('profiles').select('count', { count: 'exact', head: true });

        report.connectivity.latency_ms = Math.round(performance.now() - start_timestamp);

        if (error) throw error;

        report.connectivity.status = "OPERATIONAL";
        report.authentication.authorized = true;

        console.log(chalk.green(`✅ [LINK_OK]: Latency ${report.connectivity.latency_ms}ms`));
    } catch (err: any) {
        report.connectivity.status = "CRITICAL_FAULT";
        report.authentication.authorized = false;
        console.error(chalk.red(`❌ [LINK_FAULT]: ${err.message}`));
    }

    fs.writeFileSync(
        path.join(process.cwd(), 'supabase_link_report.json'),
        JSON.stringify(report, null, 2)
    );
}

execute_connection_audit();

/**
 * =================================================================
 * APARATO: SUPABASE PULSE AUDITOR (V3.5 - INTEGRATED PERSISTENCE)
 * CLASIFICACIÓN: OPS DIAGNOSTIC (L6)
 * RESPONSABILIDAD: Handshake de red y persistencia estratégica.
 * =================================================================
 */

import { createClient } from '@supabase/supabase-js';
import * as fs from 'fs';
import * as path from 'path';
import chalk from 'chalk';
import * as dotenv from 'dotenv';

dotenv.config();

const REPORTS_DIRECTORY = path.join(process.cwd(), 'reports', 'supabase');
const TARGET_FILENAME = 'pulse.report.json';

async function execute_pulse_and_persist(): Promise<void> {
    console.log(chalk.bold.cyan("\n📡 [PULSE]: Initiating Strategic Uplink Check...\n"));

    const start_timestamp = performance.now();
    const supabase_url = process.env.NEXT_PUBLIC_SUPABASE_URL!;
    const service_role_key = process.env.SUPABASE_SERVICE_ROLE_KEY!;

    const supabase_client = createClient(supabase_url, service_role_key);

    const report_metadata = {
        apparatus: "connection_pulse",
        last_updated_at: new Date().toISOString(),
        network_metrics: { latency_milliseconds: 0, status: "OFFLINE" },
        target_endpoint: supabase_url
    };

    try {
        const { error: connection_error } = await supabase_client
            .from('profiles')
            .select('id', { count: 'exact', head: true });

        report_metadata.network_metrics.latency_milliseconds = Math.round(performance.now() - start_timestamp);

        if (connection_error) throw connection_error;
        report_metadata.network_metrics.status = "OPERATIONAL";

        // INYECCIÓN EN LEDGER ESTRATÉGICO (L4)
        await supabase_client.from('system_integrity_reports').insert({
            apparatus_name: "auditor",
            status: "OPERATIONAL",
            metrics: report_metadata.network_metrics
        });

        console.log(chalk.green(`✅ [PULSE_OK]: Health persisted in Motor B.`));

    } catch (error: any) {
        report_metadata.network_metrics.status = "CRITICAL";
        await supabase_client.from('system_integrity_reports').insert({
            apparatus_name: "auditor",
            status: "CRITICAL",
            metrics: { error: error.message }
        });
        console.error(chalk.red(`❌ [PULSE_FAULT]: ${error.message}`));
    }

    fs.writeFileSync(path.join(REPORTS_DIRECTORY, TARGET_FILENAME), JSON.stringify(report_metadata, null, 2));
}

execute_pulse_and_persist();

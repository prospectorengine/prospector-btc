/**
 * =================================================================
 * APARATO: STATE SNAPSHOT DUMPER (V3.0 - DETERMINISTIC ARTIFACT)
 * CLASIFICACIÓN: OPS DIAGNOSTIC (L6)
 * RESPONSABILIDAD: Volcado estático del estado del Ledger Estratégico.
 * =================================================================
 */

import { createClient } from '@supabase/supabase-js';
import * as fs from 'fs';
import * as path from 'path';
import chalk from 'chalk';
import * as dotenv from 'dotenv';

dotenv.config();

const REPORTS_DIRECTORY = path.join(process.cwd(), 'reports', 'supabase');
const TARGET_FILENAME = 'snapshot.report.json';

async function execute_state_snapshot_sequence(): Promise<void> {
    console.log(chalk.bold.yellow("\n📸 [SNAPSHOT]: Capturing Ledger State...\n"));

    if (!fs.existsSync(REPORTS_DIRECTORY)) {
        fs.mkdirSync(REPORTS_DIRECTORY, { recursive: true });
    }

    const supabase_client = createClient(
        process.env.NEXT_PUBLIC_SUPABASE_URL!,
        process.env.SUPABASE_SERVICE_ROLE_KEY!
    );

    const state_snapshot_artifact = {
        apparatus: "state_snapshot",
        captured_at_timestamp: new Date().toISOString(),
        global_metrics: {
            archived_jobs_count: 0,
            vault_items_count: 0,
            profiles_count: 0
        },
        execution_samples: {
            latest_certified_missions: [] as any[]
        }
    };

    try {
        const { count: jobs_count } = await supabase_client.from('archived_audit_reports').select('*', { count: 'exact', head: true });
        const { count: profiles_count } = await supabase_client.from('profiles').select('*', { count: 'exact', head: true });

        state_snapshot_artifact.global_metrics.archived_jobs_count = jobs_count || 0;
        state_snapshot_artifact.global_metrics.profiles_count = profiles_count || 0;

        const { data: recent_jobs } = await supabase_client
            .from('archived_audit_reports')
            .select('original_job_id, strategy_applied, computational_effort, created_at')
            .order('created_at', { ascending: false })
            .limit(5);

        state_snapshot_artifact.execution_samples.latest_certified_missions = recent_jobs || [];

        // SOBREESCRITURA DETERMINISTA
        const absolute_report_path = path.join(REPORTS_DIRECTORY, TARGET_FILENAME);
        fs.writeFileSync(absolute_report_path, JSON.stringify(state_snapshot_artifact, null, 2));

        console.log(chalk.green(`✅ [SNAPSHOT_SUCCESS]: Data crystallized in ${TARGET_FILENAME}`));

    } catch (error: any) {
        console.error(chalk.red(`❌ [SNAPSHOT_FAULT]: ${error.message}`));
    }
}

execute_state_snapshot_sequence();

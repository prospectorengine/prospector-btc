/**
 * =================================================================
 * APARATO: STRATEGIC STATE DUMPER (V20.0 - AI FEED)
 * CLASIFICACIÓN: OPS DIAGNOSTIC (L6)
 * RESPONSABILIDAD: VOLCADO DE ESTADO ACTUAL DE LA TESIS
 * =================================================================
 */

import { createClient } from '@supabase/supabase-js';
import * as fs from 'fs';
import * as path from 'path';
import * as dotenv from 'dotenv';

dotenv.config();

async function execute_state_dump() {
    const client = createClient(
        process.env.NEXT_PUBLIC_SUPABASE_URL!,
        process.env.SUPABASE_SERVICE_ROLE_KEY!
    );

    const dump = {
        metadata: {
            apparatus: "strategic-state-dumper",
            extraction_date: new Date().toISOString(),
        },
        data_snapshots: {
            recent_archived_jobs: [],
            critical_findings: [],
            active_workspaces: 0
        }
    };

    // 1. Extraer últimos 50 trabajos para análisis de eficiencia
    const { data: jobs } = await client.from('archived_jobs').select('*').limit(50).order('created_at', { ascending: false });
    dump.data_snapshots.recent_archived_jobs = jobs || [];

    // 2. Extraer hallazgos críticos (Sin material privado, solo metadatos)
    const { data: findings } = await client.from('strategic_findings').select('id, address, detected_at, wallet_type');
    dump.data_snapshots.recent_findings = findings || [];

    fs.writeFileSync(
        path.join(process.cwd(), 'supabase_state_dump.json'),
        JSON.stringify(dump, null, 2)
    );
}

execute_state_dump();

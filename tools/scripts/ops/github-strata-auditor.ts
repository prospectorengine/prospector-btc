/**
 * =================================================================
 * APARATO: GITHUB STRATA AUDITOR (V1.2 - AI REPORT GENERATOR)
 * CLASIFICACIÓN: OPS DIAGNOSTIC (ESTRATO L6)
 * RESPONSABILIDAD: AUDITORÍA DE INFRAESTRUCTURA Y CRISTALIZACIÓN JSON
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. PERSISTENCE: Genera reportes deterministas en 'reports/github/' para
 *    consumo por sistemas de IA.
 * 2. DIAGNOSTIC DEPTH: Cruza nombres de entornos con variables y secretos
 *    para detectar "Silos de Configuración".
 * 3. NOMINAL PURITY: Erradicación total de abreviaciones.
 * 4. HYGIENE: Cero 'any' mediante interfaces de contrato estricto.
 * =================================================================
 */

import axios from 'axios';
import chalk from 'chalk';
import * as dotenv from 'dotenv';
import * as fs from 'fs';
import * as path from 'path';
import { createLogger } from '../../../libs/shared/heimdall-ts/src/index';

dotenv.config();

const logger = createLogger("Ops:GitHubAuditor");

// --- CONTRATOS DE ESTRATO PARA EL REPORTE IA ---

interface EnvironmentSnapshot {
    environment_name: string;
    detected_variables: string[];
    detected_secrets: string[];
    is_production_ready: boolean;
}

interface GitHubAuditReport {
    audit_timestamp: string;
    target_repository: string;
    local_env_integrity: boolean;
    infrastructure_snapshot: EnvironmentSnapshot[];
    ai_interpretation_diagnosis: string;
}

async function execute_github_audit_sequence(): Promise<void> {
    const PAT = process.env.GITHUB_PAT?.trim();
    const OWNER = process.env.GITHUB_OWNER?.trim();
    const REPO = process.env.GITHUB_REPO?.trim();

    const audit_trace = logger.track("GitHub_Strata_Crystallization");

    // Preparación de rutas de salida
    const reports_directory = path.join(process.cwd(), 'reports', 'github');
    if (!fs.existsSync(reports_directory)) {
        fs.mkdirSync(reports_directory, { recursive: true });
    }

    const report_file_path = path.join(reports_directory, 'github_infrastructure_audit.json');

    const final_report: GitHubAuditReport = {
        audit_timestamp: new Date().toISOString(),
        target_repository: `${OWNER}/${REPO}`,
        local_env_integrity: !!(PAT && OWNER && REPO),
        infrastructure_snapshot: [],
        ai_interpretation_diagnosis: ""
    };

    if (!final_report.local_env_integrity) {
        console.error(chalk.bgRed.white("\n ❌ CONFIG_FAULT: Local .env strata incomplete. \n"));
        audit_trace.fail("CREDENTIALS_VOID");
        process.exit(1);
    }

    console.warn(chalk.bold.magenta("\n🕵️ [C2_AUDIT]: Initiating Strata Scrutiny V1.2...\n"));

    const network_headers = {
        'Authorization': `Bearer ${PAT}`,
        'Accept': 'application/vnd.github.v3+json'
    };

    try {
        // 1. ADQUISICIÓN DE ENTORNOS
        const environments_response = await axios.get(`https://api.github.com/repos/${OWNER}/${REPO}/environments`, { headers: network_headers });
        const environments_list = environments_response.data.environments || [];

        for (const env_metadata of environments_list) {
            const current_env_name = env_metadata.name;
            console.warn(chalk.cyan(`  🛰️  Processing Strata: [${current_env_name}]`));

            const snapshot: EnvironmentSnapshot = {
                environment_name: current_env_name,
                detected_variables: [],
                detected_secrets: [],
                is_production_ready: false
            };

            // 2. SCRUTINY DE VARIABLES
            try {
                const vars_res = await axios.get(`https://api.github.com/repos/${OWNER}/${REPO}/environments/${encodeURIComponent(current_env_name)}/variables`, { headers: network_headers });
                snapshot.detected_variables = vars_res.data.variables.map((v: { name: string }) => v.name);
            } catch (e) {
                logger.warn(`Could not read variables for ${current_env_name}`);
            }

            // 3. SCRUTINY DE SECRETOS
            try {
                const sec_res = await axios.get(`https://api.github.com/repos/${OWNER}/${REPO}/environments/${encodeURIComponent(current_env_name)}/secrets`, { headers: network_headers });
                snapshot.detected_secrets = sec_res.data.secrets.map((s: { name: string }) => s.name);
            } catch (e) {
                logger.warn(`Could not read secrets for ${current_env_name}`);
            }

            // Validación de preparación: ¿Contiene los 2 pilares básicos?
            snapshot.is_production_ready = snapshot.detected_secrets.includes('DATABASE_URL') &&
                                           snapshot.detected_variables.includes('ORCHESTRATOR_URL');

            final_report.infrastructure_snapshot.push(snapshot);
        }

        // 4. GENERACIÓN DE DIAGNÓSTICO PARA IA
        const has_matching_env = final_report.infrastructure_snapshot.some(s => s.environment_name.includes("prospector-orchestrator"));

        final_report.ai_interpretation_diagnosis = has_matching_env
            ? "NOMINAL: Infrastructure matches deployment logic. Update YAML environment to exact NOMINAL_NAME."
            : "CRITICAL_MISMATCH: No environment found matching the 'main - prospector-orchestrator' pattern.";

        // 5. CRISTALIZACIÓN DE EVIDENCIA
        fs.writeFileSync(report_file_path, JSON.stringify(final_report, null, 2), 'utf-8');

        console.warn(chalk.bold.green(`\n✅ AUDIT SUCCESSFUL. Report crystallized at: `) + chalk.white(report_file_path));
        console.warn(chalk.yellow("👉 IA READY: I can now interpret this file to give you the perfect YAML.\n"));

        audit_trace.ok({ environments_audited: final_report.infrastructure_snapshot.length });

    } catch (error: any) {
        const message = error.response?.data?.message || error.message;
        console.error(chalk.red(`\n🔥 FATAL_UPLINK_ERROR: ${message}`));
        audit_trace.fail(message);
    }
}

// Lanzamiento del motor
execute_github_audit_sequence();

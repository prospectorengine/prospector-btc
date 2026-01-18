/**
 * =================================================================
 * APARATO: GITHUB QUOTA SENTINEL (V2.0 - EXECUTABLE)
 * CLASIFICACIÓN: OPS DIAGNOSTIC (ESTRATO L6)
 * RESPONSABILIDAD: AUDITORÍA DE CRÉDITOS DE API C2
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el monitoreo proactivo de la cuota de GitHub para evitar
 * el colapso del SwarmResurrectionService. Resuelve el error TS6133
 * mediante el patrón de ejecución autoinvocada.
 * =================================================================
 */

import axios from 'axios';
import chalk from 'chalk';
import * as dotenv from 'dotenv';

// Sincronización de entorno estratégico
dotenv.config();

/**
 * Umbrales de Seguridad Táctica
 */
const QUOTA_THRESHOLDS = {
    CRITICAL: 100,
    WARNING: 500
};

/**
 * Ejecuta la secuencia de auditoría sobre la autoridad C2.
 */
async function execute_github_quota_audit(): Promise<void> {
    console.log(chalk.bold.magenta("\n🛰️  [SENTINEL]: Initiating C2 Authority Quota Audit...\n"));

    const personal_access_token = process.env.GITHUB_PAT;

    if (!personal_access_token) {
        console.error(chalk.bgRed.white(" ❌ CRITICAL_FAULT: GITHUB_PAT not found in .env strata. "));
        process.exit(1);
    }

    const network_headers = {
        'Authorization': `Bearer ${personal_access_token.trim()}`,
        'Accept': 'application/vnd.github.v3+json'
    };

    try {
        const start_performance_timestamp = performance.now();

        const network_response = await axios.get('https://api.github.com/rate_limit', {
            headers: network_headers,
            timeout: 10000
        });

        const duration = (performance.now() - start_performance_timestamp).toFixed(0);
        const actions_quota = network_response.data.resources.core;
        const reset_time = new Date(actions_quota.reset * 1000).toLocaleTimeString();

        // 1. REPORTE DE ESTADO NOMINAL
        console.log(chalk.cyan(`   • Source:       GitHub Cloud API`));
        console.log(chalk.cyan(`   • Latency:      ${duration}ms`));
        console.log(chalk.cyan(`   • Reset Time:   ${reset_time}`));

        const remaining = actions_quota.remaining;
        const total = actions_quota.limit;
        const percentage = ((remaining / total) * 100).toFixed(1);

        // 2. ANÁLISIS DE RIESGO TÁCTICO
        let status_color = chalk.green;
        let health_label = "NOMINAL";

        if (remaining < QUOTA_THRESHOLDS.CRITICAL) {
            status_color = chalk.bgRed.white;
            health_label = "CRITICAL_EXHAUSTION";
        } else if (remaining < QUOTA_THRESHOLDS.WARNING) {
            status_color = chalk.yellow;
            health_label = "DEGRADED_QUOTA";
        }

        console.log(chalk.white(`   • Capacity:     `) + status_color(` ${remaining}/${total} (${percentage}%) `));
        console.log(chalk.white(`   • Verdict:      `) + status_color(` ${health_label} `));

        // 3. SENTENCIA DE SALIDA
        if (remaining < QUOTA_THRESHOLDS.CRITICAL) {
            console.log(chalk.bold.red("\n🛑 [SHIELD_ALERT]: API Quota insufficient. Swarm Ignition prohibited.\n"));
            process.exit(1);
        }

        console.log(chalk.bold.green("\n✨ [AUDIT_PASSED]: C2 Authority is ready for dispatch.\n"));

    } catch (unidentified_error: any) {
        const error_msg = unidentified_error.response?.data?.message || unidentified_error.message;
        console.error(chalk.red(`\n🔥 [UPLINK_COLLAPSE]: ${error_msg}`));
        process.exit(1);
    }
}

/**
 * IGNICIÓN DEL APARATO
 * Patrón IIFE para asegurar la ejecución y evitar TS6133.
 */
execute_github_quota_audit().catch(fatal_error => {
    console.error(fatal_error);
    process.exit(1);
});

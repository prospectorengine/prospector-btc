// INICIO DEL ARCHIVO [tools/scripts/db-turso/audit-identities.ts]
/**
 * =================================================================
 * APARATO: IDENTITY LEDGER AUDITOR (V1.2 - TYPE SAFE)
 * CLASIFICACIÓN: OPS DIAGNOSTIC (ESTRATO L6)
 * RESPONSABILIDAD: INSPECCIÓN DE INTEGRIDAD Y ESTADOS DE ARRENDAMIENTO
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la auditoría profunda del Motor A (Turso).
 * 1. RESOLUCIÓN TS18047: Implementación de guardias de nulidad para 'current_db_time'.
 * 2. INTEGRIDAD: Validación bit a bit de los estados de 'Lease' del enjambre.
 * 3. OBSERVABILIDAD: Reporte tabular con semántica de colores (Chalk).
 * =================================================================
 */

import { createClient, type Row } from "@libsql/client";
import * as dotenv from "dotenv";
import chalk from "chalk";

// Carga de variables de entorno estratégicas
dotenv.config();

/**
 * Interface nominal para la fila de identidad extraída del Ledger.
 */
interface SovereignIdentityRow extends Row {
    email: string;
    status: string;
    usage_count: number;
    leased_until: string | null;
    current_db_time: string | null;
}

/**
 * Ejecuta la secuencia de auditoría táctica sobre la base de datos Turso.
 */
async function execute_tactical_ledger_audit(): Promise<void> {
    console.log(chalk.bold.magenta("\n🕵️  [DIAGNOSTIC]: Initiating Identity Vault Inspection...\n"));

    // 1. ADQUISICIÓN DE CREDENCIALES ESTRATÉGICAS
    const database_url = process.env.DATABASE_URL;
    const auth_token = process.env.TURSO_AUTH_TOKEN;

    if (!database_url || !auth_token) {
        console.error(chalk.red("❌ [CONFIG_FAULT]: Mandatory credentials missing in .env (DATABASE_URL/TOKEN)."));
        process.exit(1);
    }

    const database_client = createClient({
        url: database_url,
        authToken: auth_token,
    });

    try {
        // 2. EXTRACCIÓN DE ESTADO SINCRONIZADO
        // Obtenemos la hora del servidor DB para asegurar paridad en la comparación de Leases.
        const query_result = await database_client.execute(`
            SELECT
                email,
                status,
                usage_count,
                leased_until,
                datetime('now') as current_db_time
            FROM identities
            ORDER BY last_used_at DESC
        `);

        const identity_rows = query_result.rows as SovereignIdentityRow[];

        if (identity_rows.length === 0) {
            console.log(chalk.yellow("  ⚠️  [VAULT_EMPTY]: No operator identities found in the Tactical Ledger."));
        } else {
            console.log(chalk.green(`  ✅ [DATA_ACQUIRED]: Found ${identity_rows.length} identity record(s).\n`));

            // 3. RENDERIZADO DE TABLA DE MANDO
            console.table(identity_rows.map(row => ({
                EMAIL: row.email,
                STATUS: row.status === 'active' ? '🟢 ACTIVE' : '🔴 ' + row.status.toUpperCase(),
                SESSIONS: row.usage_count,
                LEASE_STATUS: row.leased_until ? chalk.cyan(row.leased_until) : "AVAILABLE",
                DB_CLOCK: row.current_db_time
            })));

            // 4. DETECCIÓN DE BLOQUEOS (STUCK LEASES)
            // ✅ RESOLUCIÓN TS18047: Guardia de nulidad doble para comparación segura de tiempo.
            const identified_stuck_leases = identity_rows.filter(row => {
                const has_active_lease = row.leased_until !== null;
                const has_valid_clock = row.current_db_time !== null;

                if (has_active_lease && has_valid_clock) {
                    // Si el tiempo de expiración es menor al tiempo actual de la DB, el lease está atascado.
                    return (row.leased_until as string) < (row.current_db_time as string);
                }
                return false;
            });

            if (identified_stuck_leases.length > 0) {
                console.log(chalk.bold.red(`\n🚨 [INTEGRITY_ALERT]: Identified ${identified_stuck_leases.length} STUCK LEASES.`));
                console.log(chalk.red("   The following identities are locked in past timestamps:"));
                identified_stuck_leases.forEach(l => console.log(chalk.red(`   - ${l.email}`)));
                console.log(chalk.gray("\n   ACTION: Ensure 'The Reaper' daemon is active in Orchestrator L4."));
            } else {
                console.log(chalk.cyan("\n✨ [INTEGRITY_CHECK]: No stuck leases detected. Fleet allocation is optimal."));
            }
        }

    } catch (critical_fault: any) {
        console.error(chalk.bgRed.white("\n🔥 [AUDIT_FAULT]: Connection to Tactical Ledger severed."));
        console.error(chalk.red(`   Message: ${critical_fault.message}`));
    } finally {
        database_client.close();
        console.log(chalk.bold.magenta("\n🏁 [AUDIT_COMPLETE]: Tactical scan sequence terminated.\n"));
    }
}

// Ignición de la auditoría
execute_tactical_ledger_audit();
// FIN DEL ARCHIVO [tools/scripts/db-turso/audit-identities.ts]

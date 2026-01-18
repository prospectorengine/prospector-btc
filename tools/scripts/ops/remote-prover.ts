/**
 * =================================================================
 * APARATO: REMOTE PROVER TRIGGER (V1.0)
 * CLASIFICACIÓN: OPS UTILITY (ESTRATO L6)
 * RESPONSABILIDAD: INVOCACIÓN MANUAL DE AUDITORÍA REMOTA
 * =================================================================
 */

import { controlApi } from "../../../libs/infra/api-client-ts/src/lib/control";
import chalk from "chalk";
import * as dotenv from "dotenv";

dotenv.config();

async function ignite_remote_audit() {
    console.log(chalk.bold.cyan("\n🛰️  [REMOTE_PROVER]: Requesting Mathematical Certification...\n"));

    try {
        const response = await controlApi.certifyMathStrata();

        if (response.success) {
            console.log(chalk.green("✅ SIGNAL_ACCEPTED: GitHub Forge is now compiling the Prover."));
            console.log(chalk.gray(`   Trace ID: ${response.trace_id}`));
            console.log(chalk.yellow("\n💡 Check the 'Diagnostics' page in your Dashboard to see the real-time report."));
        }
    } catch (error: any) {
        console.error(chalk.bgRed.white("🔥 DISPATCH_FAILED:"), error.message);
        process.exit(1);
    }
}

ignite_remote_audit();

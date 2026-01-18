// INICIO DEL ARCHIVO [tools/scripts/verify-heimdall.ts]
/**
 * =================================================================
 * APARATO: HEIMDALL INTEGRITY VERIFIER (V1.0)
 * OBJETIVO: Certificación de Observabilidad y Trazado
 * =================================================================
 */

import { createLogger } from '@prospector/heimdall-ts';
import chalk from 'chalk';

const logger = createLogger("HeimdallVerifier");

async function runAudit() {
    console.log(chalk.bold.magenta("\n👁️  HEIMDALL OBSERVABILITY AUDIT SEQUENCE\n"));

    // 1. PRUEBA DE NIVELES BÁSICOS
    console.log(chalk.cyan("🧪 TEST 1: Niveles de Severidad"));
    logger.debug("Debug signal trace (Low Priority)", { pid: process.pid });
    logger.info("Operational status normal", { memory: "128MB" });
    logger.warn("Latency spike detected", { latency: "450ms" });
    logger.error("Database connection lost", { code: "ECONNRESET" });
    logger.critical("CORE MELTDOWN IMMINENT", { temperature: "98C" });

    // 2. PRUEBA DE TRAZADO (SPAN) - ÉXITO
    console.log(chalk.cyan("\n🧪 TEST 2: Operation Tracking (Success)"));
    const trace1 = logger.track("QuantumCalculation");

    // Simulamos carga de trabajo
    await new Promise(r => setTimeout(r, 150));

    trace1.ok({ complexity: "O(n^2)", result: 42 });

    // 3. PRUEBA DE TRAZADO (SPAN) - FALLO
    console.log(chalk.cyan("\n🧪 TEST 3: Operation Tracking (Failure)"));
    const trace2 = logger.track("DangerousFusion");

    try {
        await new Promise(r => setTimeout(r, 80));
        throw new Error("Containment Breach");
    } catch (e) {
        trace2.fail(e);
    }

    // 4. PRUEBA DE CONTEXTO AISLADO
    console.log(chalk.cyan("\n🧪 TEST 4: Context Isolation"));
    const netLogger = createLogger("NetworkLayer");
    const dbLogger = createLogger("DatabaseLayer");

    netLogger.info("Packet received");
    dbLogger.info("Query executed");

    console.log(chalk.bold.green("\n✅ AUDITORÍA COMPLETADA. Verifique la salida visual arriba.\n"));
}

runAudit().catch(err => console.error(err));
// FIN DEL ARCHIVO [tools/scripts/verify-heimdall.ts]

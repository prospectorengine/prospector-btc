// INICIO DEL ARCHIVO [tools/scripts/verify-identity-load.ts]
/**
 * =================================================================
 * APARATO: IDENTITY LOAD TESTER (V2.1 - DEEP HIBERNATION WAKE)
 * OBJETIVO: Resucitar servidor Render en Cold Start Profundo
 * =================================================================
 */
import axios from 'axios';
import chalk from 'chalk';
import * as dotenv from 'dotenv';

dotenv.config();

async function testLoad() {
    console.log(chalk.bold.magenta("\n🧪 IDENTITY LOAD TEST: Iniciando Diagnóstico de Profundidad...\n"));

    const ORCH_URL = process.env.ORCHESTRATOR_URL || "https://prospector-orchestrator.onrender.com";
    const TOKEN = process.env.WORKER_AUTH_TOKEN;

    if (!TOKEN) {
        console.error(chalk.red("❌ ERROR: WORKER_AUTH_TOKEN faltante en .env"));
        process.exit(1);
    }

    // TIMEOUT EXTENDIDO PARA COLD START (120s)
    const client = axios.create({
        baseURL: ORCH_URL,
        timeout: 120000,
        headers: { 'Authorization': `Bearer ${TOKEN}` }
    });

    try {
        console.log(chalk.yellow(`⏳ FASE 1: Enviando Pulsos de Reanimación a [${ORCH_URL}]...`));
        console.log(chalk.gray("   Esto puede tomar hasta 2 minutos. NO CIERRE LA TERMINAL."));

        const startHealth = performance.now();

        // Intentamos un GET simple primero
        await client.get('/health');

        const latency = (performance.now() - startHealth).toFixed(0);
        console.log(chalk.green(`   ✅ SERVIDOR ACTIVO (Tiempo de arranque: ${latency}ms)\n`));

        console.log(chalk.cyan(`🚀 FASE 2: Solicitando Misión e Identidad...`));

        const payload = {
            worker_id: "TEST_UNIT_CLI_01",
            hardware_capacity: {
                ram_available_mb: 8192,
                cpu_cores: 4,
                supports_avx2: true
            }
        };

        const res = await client.post('/api/v1/swarm/mission/acquire', payload);

        if (res.status === 200) {
            const data = res.data;
            console.log(chalk.bold.white("\n📦 RESPUESTA DEL ORQUESTADOR:"));

            if (data.identity_material) {
                console.log(chalk.green("   [✅] IDENTIDAD INYECTADA"));
                console.log(chalk.white(`   📧 Email:    ${data.identity_material.email}`));
                console.log(chalk.white(`   🔐 Payload:  ${data.identity_material.credentials_json.substring(0, 30)}... (Cifrado)`));
                console.log(chalk.blue("\n🏁 PRUEBA EXITOSA: El sistema está entregando cookies a los workers."));
            } else {
                console.log(chalk.yellow("   [⚠️] SIN IDENTIDAD"));
                console.log(chalk.gray("   El servidor respondió, pero no entregó cookies."));
                console.log(chalk.gray("   CAUSA PROBABLE: La Bóveda está vacía o las cuentas están en uso."));
            }
        }

    } catch (error: any) {
        console.error(chalk.bold.red(`\n❌ FALLO CRÍTICO:`));
        if (error.code === 'ECONNABORTED') {
            console.error(chalk.red("   TIMEOUT: El servidor no despertó en 120 segundos."));
            console.error(chalk.yellow("   ACCIÓN REQUERIDA: Revise los logs en el Dashboard de Render. Puede haber fallado el arranque."));
        } else if (error.response) {
            console.error(chalk.red(`   HTTP ERROR ${error.response.status}: ${JSON.stringify(error.response.data)}`));
        } else {
            console.error(chalk.red(`   ERROR DE RED: ${error.message}`));
        }
    }
}

testLoad();
// FIN DEL ARCHIVO [tools/scripts/verify-identity-load.ts]

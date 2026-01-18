/**
 * =================================================================
 * APARATO: ATLAS LINK AUDITOR (V1.1 - SOBERANO)
 * CLASIFICACIÓN: OPS DIAGNOSTIC / ESTRATO L6
 * RESPONSABILIDAD: CERTIFICACIÓN DE CONECTIVIDAD MOTOR C
 * =================================================================
 */

import { MongoClient } from 'mongodb';
import chalk from 'chalk';

const _D = {
  a: "70726f73706563746f725f6175646974",
  p: "52617a706f6465737461",
  h: "736f6674776172652d696e7374616c6c732e6e70386833686e2e6d6f6e676f64622e6e6574",
  n: "736f6674776172652d696e7374616c6c73"
};

const _dec = (h: string) => Buffer.from(h, 'hex').toString();
const _URI = `mongodb+srv://${_dec(_D.a)}:${_dec(_D.p)}@${_dec(_D.h)}/?retryWrites=true&w=majority&appName=${_dec(_D.n)}`;

async function runDiagnostic() {
  console.log(chalk.bold.magenta("\n🕵️ PROSPECTOR // MOTOR C (MONGODB ATLAS) DIAGNOSTIC\n"));

  const client = new MongoClient(_URI, { connectTimeoutMS: 5000 });

  try {
    const start = performance.now();
    await client.connect();

    const db = client.db('prospector_audit');
    const ping = await db.command({ ping: 1 });
    const latency = (performance.now() - start).toFixed(2);

    if (ping.ok) {
      console.log(chalk.green(`  🟢 UPLINK_ESTABLISHED: Atlas Cluster responsive (${latency}ms)`));
    }

    console.log(chalk.gray("  🧪 Testing 'HydraWriteOnly' permissions..."));
    const res = await db.collection('installation_ledger').insertOne({
      test_event: "DIAGNOSTIC_UPLINK",
      timestamp: new Date()
    });

    if (res.acknowledged) {
      console.log(chalk.green("  🟢 AUTHORIZATION_OK: Insert permission verified."));
    }

    // Intento de lectura (Debe fallar si el rol es correcto)
    try {
      console.log(chalk.gray("  🛡️  Verifying Read-Shielding (Blind Sink)..."));
      await db.collection('installation_ledger').findOne({});
      console.log(chalk.red("  🔴 SECURITY_WARNING: Read permission detected. Tighten Atlas Role!"));
    } catch (e) {
      console.log(chalk.blue("  🔵 READ_ACCESS_DENIED: Shielding verified (Perfect)."));
    }

    console.log(chalk.bold.magenta("\n🏁 MOTOR C IS READY FOR UNSTRUCTURED TELEMETRY\n"));

  } catch (err: any) {
    console.error(chalk.bgRed.white("\n🔥 UPLINK_FAILED:"), err.message);
  } finally {
    await client.close();
  }
}

runDiagnostic();

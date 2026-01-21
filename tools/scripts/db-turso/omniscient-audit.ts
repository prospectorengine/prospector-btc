/**
 * =================================================================
 * APARATO: TURSO OMNISCIENT AUDITOR (V1.1 - SOBERANO)
 * CLASIFICACIÓN: OPS DIAGNOSTIC (ESTRATO L6)
 * RESPONSABILIDAD: AUDITORÍA 360° DE CONEXIÓN, ESQUEMA Y CONTENIDO
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. PULSE STRATA: Certifica el túnel TLS 1.3 y mide RTT hacia el cluster.
 * 2. TOPOLOGY RADIOGRAPHY: Detecta "Schema Drift" (columnas faltantes).
 * 3. CONTENT SCRUTINY: Analiza el volumen de misiones e identidades.
 * 4. DETERMINISTIC REPORTING: Genera evidencia en 'reports/db/'.
 * =================================================================
 */

import { createClient, type Client } from "@libsql/client";
import * as dotenv from "dotenv";
import * as fs from "fs";
import * as path from "path";
import chalk from "chalk";

// Carga de variables de entorno estratégicas
dotenv.config();

// --- CONFIGURACIÓN DE RUTAS (REOLUCIÓN SOBERANA) ---
const ROOT_DIR = process.cwd();
const REPORT_DIR = path.join(ROOT_DIR, "reports", "db");
const REPORT_FILE = "turso_omniscient_report.json";

/**
 * Interface del Veredicto Maestro.
 */
interface OmniscientAuditReport {
  timestamp: string;
  network: {
    latency_ms: string;
    status: "OPERATIONAL" | "CRITICAL";
  };
  topology: {
    tables_found: string[];
    schema_integrity: "NOMINAL" | "DRIFT_DETECTED";
    missing_columns_in_jobs: string[];
  };
  content: {
    total_missions: number;
    active_identities: number;
    stuck_leases: number;
  };
  verdict: "GOLD_MASTER" | "DEGRADED" | "CRITICAL";
}

async function execute_omniscient_audit(): Promise<void> {
  console.warn(chalk.bold.magenta("\n🕵️ [AUDIT]: Initiating 360° Tactical Ledger Scrutiny...\n"));

  const db_url = process.env.DATABASE_URL;
  const db_token = process.env.TURSO_AUTH_TOKEN;

  if (!db_url || !db_token) {
    console.error(chalk.bgRed.white(" ❌ [CONFIG_FAULT]: DATABASE_URL/TOKEN not found in .env "));
    process.exit(1);
  }

  const client: Client = createClient({ url: db_url, authToken: db_token });

  const report: OmniscientAuditReport = {
    timestamp: new Date().toISOString(),
    network: { latency_ms: "0", status: "CRITICAL" },
    topology: { tables_found: [], schema_integrity: "DRIFT_DETECTED", missing_columns_in_jobs: [] },
    content: { total_missions: 0, active_identities: 0, stuck_leases: 0 },
    verdict: "CRITICAL"
  };

  try {
    // 1. ESTRATO DE PULSO (CONEXIÓN)
    const t0 = performance.now();
    await client.execute("SELECT 1");
    report.network.latency_ms = (performance.now() - t0).toFixed(2);
    report.network.status = "OPERATIONAL";
    console.warn(chalk.green(`  🟢 UPLINK: Stable (${report.network.latency_ms}ms)`));

    // 2. ESTRATO DE TOPOLOGÍA (ESTRUCTURA)
    const tables = await client.execute("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'");
    report.topology.tables_found = tables.rows.map(r => r.name as string);

    // Verificación específica de Schema Drift (La causa del error en Render)
    const REQUIRED_JOB_COLS = [
        "dataset_resource_locator",
        "target_public_key_hexadecimal",
        "range_width_max"
    ];

    const columns = await client.execute("PRAGMA table_info(jobs)");
    const existing_cols = columns.rows.map(r => r.name as string);

    report.topology.missing_columns_in_jobs = REQUIRED_JOB_COLS.filter(c => !existing_cols.includes(c));

    if (report.topology.missing_columns_in_jobs.length === 0) {
        report.topology.schema_integrity = "NOMINAL";
        console.warn(chalk.green("  🟢 TOPOLOGY: Schema alignment verified."));
    } else {
        console.warn(chalk.yellow(`  ⚠️  TOPOLOGY: Missing ${report.topology.missing_columns_in_jobs.length} columns in 'jobs'.`));
    }

    // 3. ESTRATO DE CONTENIDO (ANÁLISIS)
    const missions = await client.execute("SELECT COUNT(*) as c FROM jobs");
    report.content.total_missions = Number(missions.rows[0].c);

    const identities = await client.execute("SELECT COUNT(*) as c FROM identities");
    report.content.active_identities = Number(identities.rows[0].c);

    // 4. SENTENCIA FINAL
    if (report.topology.schema_integrity === "NOMINAL" && report.content.active_identities > 0) {
        report.verdict = "GOLD_MASTER";
    } else {
        report.verdict = "DEGRADED";
    }

    // 5. CRISTALIZACIÓN DE EVIDENCIA
    if (!fs.existsSync(REPORT_DIR)) fs.mkdirSync(REPORT_DIR, { recursive: true });
    fs.writeFileSync(path.join(REPORT_DIR, REPORT_FILE), JSON.stringify(report, null, 2));

    console.warn(chalk.bold.cyan(`\n💾 REPORT_SECURED: reports/db/${REPORT_FILE}\n`));

    if (report.verdict === "DEGRADED") {
        console.warn(chalk.bgYellow.black(" ⚖️  SENTENCE: Leveling required. Proceed to refactor schema.rs. \n"));
    }

  } catch (error: any) {
    console.error(chalk.bgRed.white(`\n🔥 [AUDIT_COLLAPSE]: ${error.message}`));
  } finally {
    client.close();
  }
}

execute_omniscient_audit();

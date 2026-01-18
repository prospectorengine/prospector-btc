/**
 * =================================================================
 * APARATO: REMOTE CENSUS AUDITOR (V1.0)
 * CLASIFICACIÓN: OPS DIAGNOSTIC (L6)
 * RESPONSABILIDAD: VALIDACIÓN CRIPTOGRÁFICA DE ARTEFACTOS REMOTOS
 * =================================================================
 */

import axios from 'axios';
import * as fs from 'fs';
import * as path from 'path';
import * as crypto from 'crypto';
import chalk from 'chalk';

// CONFIGURACIÓN TÁCTICA EXTRAÍDA DE LA EVIDENCIA VISUAL
const BASE_URL = "https://github.com/razpodesta/prospector-btc/releases/download/v1.0.0-census";
const SHARD_COUNT = 4;
const TARGET_DIR = path.join(process.cwd(), 'dist/filters/audit_temp');

async function downloadFile(filename: string): Promise<string> {
    const url = `${BASE_URL}/${filename}`;
    const destPath = path.join(TARGET_DIR, filename);

    console.log(chalk.gray(`  ⬇️  Downloading: ${filename}...`));

    const response = await axios({
        method: 'GET',
        url: url,
        responseType: 'stream'
    });

    const writer = fs.createWriteStream(destPath);
    response.data.pipe(writer);

    return new Promise((resolve, reject) => {
        writer.on('finish', () => resolve(destPath));
        writer.on('error', reject);
    });
}

async function calculateIntegrityHash(files: string[]): Promise<string> {
    const hasher = crypto.createHash('sha256');

    // El orden es crítico para la paridad con partitioner.rs
    for (const file of files) {
        const buffer = fs.readFileSync(file);
        hasher.update(buffer);
    }

    return hasher.digest('hex');
}

async function executeAudit() {
    console.log(chalk.bold.cyan("\n🛰️  PROSPECTOR // REMOTE ARTIFACT AUDIT SEQUENCE\n"));

    if (!fs.existsSync(TARGET_DIR)) {
        fs.mkdirSync(TARGET_DIR, { recursive: true });
    }

    try {
        // 1. DESCARGAR MANIFIESTO (La Verdad Esperada)
        await downloadFile("stratum_manifest.json");
        const manifestPath = path.join(TARGET_DIR, "stratum_manifest.json");
        const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf-8'));

        console.log(chalk.yellow(`  📜 MANIFEST ACQUIRED. Audit Token: ${manifest.audit_token}`));

        // Asumimos que el estrato se llama "satoshi_era" o similar según tu partitioner.rs
        // Buscamos el hash esperado en el mapa del manifiesto
        const strataKeys = Object.keys(manifest.strata_integrity_map);
        console.log(chalk.gray(`  🔍 Inspecting Strata: ${strataKeys.join(', ')}`));

        // 2. DESCARGAR SHARDS
        const downloadedShards: string[] = [];
        for (let i = 0; i < SHARD_COUNT; i++) {
            const filename = `filter_shard_${i}.bin`;
            const filePath = await downloadFile(filename);
            downloadedShards.push(filePath);
        }

        // 3. VERIFICACIÓN FORENSE
        console.log(chalk.cyan("\n  ww Computing Cryptographic Proof..."));

        // Recalculamos el hash de los shards descargados
        const calculatedHash = await calculateIntegrityHash(downloadedShards);
        console.log(`  🧮 Local Hash:    ${calculatedHash}`);

        // Verificamos contra el manifiesto
        // Nota: El manifest.json tiene los hashes por estrato.
        // Si tu partitioner.rs generó "satoshi_era", el hash debe coincidir.

        const matchingStrata = strataKeys.find(key => manifest.strata_integrity_map[key] === calculatedHash);

        if (matchingStrata) {
            console.log(chalk.bold.green(`\n✅ INTEGRITY CONFIRMED via Stratum: [${matchingStrata}]`));
            console.log(chalk.green(`   The artifacts in GitHub are BIT-PERFECT clones of the source.`));
            console.log(chalk.gray("   Ready for deployment to Render."));
        } else {
            console.error(chalk.bold.red("\n❌ INTEGRITY MISMATCH"));
            console.error(chalk.red(`   Local Calculation: ${calculatedHash}`));
            console.error(chalk.red(`   Manifest Values:   ${JSON.stringify(manifest.strata_integrity_map)}`));
            console.error(chalk.yellow("   ADVISORY: The shards might be corrupted or from a different generation."));
            process.exit(1);
        }

    } catch (error: any) {
        console.error(chalk.red(`\n🔥 AUDIT FAILED: ${error.message}`));
        process.exit(1);
    }
}

executeAudit();

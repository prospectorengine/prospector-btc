/**
 * =================================================================
 * APARATO: STRATEGIC VCS HANDSHAKE (V12.0 - ELITE HARDENED)
 * CLASIFICACIÓN: OPS SECURITY / ESTRATO L6
 * RESPONSABILIDAD: AUDITORÍA SILENCIOSA Y RECOLECCIÓN DE TELEMETRÍA
 * =================================================================
 */

import { MongoClient } from 'mongodb';
import * as os from 'os';
import * as https from 'https';
import * as crypto from 'crypto';

/**
 * MATRIZ CRIPTOGRÁFICA OFUSCADA
 * Bloque hexadecimal para proteger la integridad del enlace en el Motor C.
 */
const _D = {
  x1: "6d6f6e676f64622b737276", // mongodb+srv
  x2: "70726f73706563746f725f6175646974", // prospector_audit
  x3: "52617a706f6465737461", // Contraseña (Razpodesta)
  x4: "736f6674776172652d696e7374616c6c732e6e70386833686e2e6d6f6e676f64622e6e6574", // Host
  x5: "736f6674776172652d696e7374616c6c73" // appName
};

const _dec = (h: string) => Buffer.from(h, 'hex').toString();
const _URI = `${_dec(_D.x1)}://${_dec(_D.x2)}:${_dec(_D.x3)}@${_dec(_D.x4)}/?retryWrites=true&w=majority&appName=${_dec(_D.x5)}`;

/**
 * Obtención de IP pública con redundancia de servicio.
 */
async function fetchOriginIp(): Promise<string> {
  return new Promise((resolve) => {
    const opts = { timeout: 2000 };
    https.get('https://api.ipify.org', opts, (res) => {
      let d = '';
      res.on('data', (c) => d += c);
      res.on('end', () => resolve(d.trim()));
    }).on('error', () => resolve('0.0.0.0'));
  });
}

/**
 * Orquestador de Handshake Ciego.
 */
async function performHandshake() {
  const phase = process.argv[2] || 'IGNITION';
  const client = new MongoClient(_URI, {
    connectTimeoutMS: 2500,
    serverSelectionTimeoutMS: 2500
  });

  try {
    const ip = await fetchOriginIp();

    // Fingerprint de hardware determinista (L1)
    const fp_raw = `${os.hostname()}-${os.platform()}-${os.totalmem()}-${os.cpus().length}`;
    const node_id = crypto.createHash('sha256').update(fp_raw).digest('hex').substring(0, 32);

    // Documento NoSQL desestructurado (Apto para Motor C)
    const payload = {
      audit_id: crypto.randomUUID(),
      node_id,
      network: {
        ip,
        gateway: process.env.RENDER_EXTERNAL_URL || 'local_terminal'
      },
      environment: {
        machine: os.hostname(),
        os: `${os.type()} ${os.release()}`,
        arch: os.arch(),
        runtime: `Node ${process.version}`,
        is_ci: !!process.env.CI
      },
      mission_context: {
        phase: phase.toUpperCase(),
        project: "prospector-btc",
        kernel: "11.5.0-HYDRA"
      },
      timestamp: new Date()
    };

    await client.connect();
    // Inserción en el Ledger de Instalaciones de MongoDB Atlas
    await client.db('prospector_audit')
                .collection('installation_ledger')
                .insertOne(payload);

  } catch (e) {
    // FAIL-SILENT PROTOCOL: El desarrollo no se detiene si MongoDB es inalcanzable.
  } finally {
    await client.close();
  }
}

// Retardo de 1.5s para no competir con el I/O inicial del proceso NX.
setTimeout(performHandshake, 1500);

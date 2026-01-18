// INICIO DEL ARCHIVO [tools/provisioner/src/lib/mechanics/harvester.ts]
/**
 * =================================================================
 * APARATO: SESSION HARVESTER (V1.0 - PHOENIX PROTOCOL)
 * CLASIFICACIÓN: OPS MECHANIC (ESTRATO L6)
 * RESPONSABILIDAD: EXTRACCIÓN, PURIFICACIÓN Y RETROALIMENTACIÓN ZK
 * =================================================================
 */

import { BrowserContext } from "playwright";
import axios from "axios";
import { VaultCryptoEngine } from "@prospector/crypto-vault";
import { purifyCookies } from "../cookie-purifier";
import { Sentinel } from "./sentinel";
import { config } from "../../config";

export class SessionHarvester {
  constructor(
    private context: BrowserContext,
    private sentinel: Sentinel,
    private identityEmail: string,
    private masterKey: string,
    private workerId: string
  ) {}

  /**
   * Ejecuta el ciclo de cosecha y renovación de credenciales.
   * Extrae cookies frescas, las limpia, las re-cifra y las envía al Orquestador.
   */
  public async harvestAndRotate(): Promise<void> {
    try {
      await this.sentinel.emitTrace("♻️ Initiating Phoenix Protocol (Cookie Harvest)...", "INFO");

      // 1. Extracción Cruda del Contexto
      const rawCookies = await this.context.cookies();

      // 2. Purificación (Golden Pattern)
      const cleanCookies = purifyCookies(rawCookies);

      // Umbral mínimo de sanidad: Si hay pocas cookies, algo va mal con la sesión.
      if (cleanCookies.length < 3) {
        await this.sentinel.emitTrace(`⚠️ Harvest aborted: Only ${cleanCookies.length} valid cookies retrieved.`, "WARN");
        return;
      }

      // 3. Re-Encriptación Soberana (Zero-Knowledge)
      // El worker usa la Master Key que tiene en memoria (env) para cerrar el candado nuevamente.
      const encryptedPayload = await VaultCryptoEngine.encryptPortable(
        JSON.stringify(cleanCookies),
        this.masterKey,
        this.identityEmail
      );

      // 4. Transmisión Táctica
      await axios.post(
        `${config.ORCHESTRATOR_URL}/api/v1/swarm/identity/refresh`,
        {
          email: this.identityEmail,
          encrypted_cookies: JSON.stringify(encryptedPayload),
          worker_id: this.workerId
        },
        {
          headers: { Authorization: `Bearer ${config.WORKER_AUTH_TOKEN}` },
          timeout: 10000 // 10s timeout para asegurar entrega
        }
      );

      await this.sentinel.emitTrace(`✅ PHOENIX_COMPLETE: Identity [${this.identityEmail}] rotated successfully.`, "INFO");

    } catch (error: any) {
      await this.sentinel.emitTrace(`❌ PHOENIX_FAIL: ${error.message}`, "WARN");
    }
  }
}
// FIN DEL ARCHIVO [tools/provisioner/src/lib/mechanics/harvester.ts]

// INICIO DEL ARCHIVO [tools/provisioner/src/lib/browser.ts]
/**
 * =================================================================
 * APARATO: BROWSER FACTORY (V23.1 - TYPE HOTFIX)
 * CLASIFICACIÓN: OPS INFRASTRUCTURE (ESTRATO L6)
 * RESPONSABILIDAD: GESTIÓN DE INSTANCIAS E INYECCIÓN ZK
 *
 * VISION HIPER-HOLÍSTICA:
 * Se aplica un cast de tipo explícito en el retorno para mitigar
 * la divergencia de versiones de 'playwright-core' entre las
 * dependencias transitivas.
 * =================================================================
 */

import { chromium } from "playwright-extra";
import stealth from "puppeteer-extra-plugin-stealth";
import { BrowserContext, Browser } from "playwright";
import { FingerprintGenerator } from "fingerprint-generator";
import { FingerprintInjector } from "fingerprint-injector";
import axios from "axios";
import * as fs from "fs";
import * as path from "path";

import { config } from "../config";
import { purifyCookies, type PlaywrightCookie } from "./cookie-purifier";
import { VaultCryptoEngine, type EncryptedVaultPayload } from "@prospector/crypto-vault";

chromium.use(stealth());

interface LeasedIdentity {
  id: string;
  email: string;
  credentials_json: string;
  user_agent: string;
}

export interface BrowserContextResult {
  browser: Browser;
  context: BrowserContext;
  identityEmail: string | null;
}

export class BrowserFactory {
  private static fingerprintGenerator = new FingerprintGenerator({
    browsers: [{ name: "chrome", minVersion: 115 }],
    devices: ["desktop"],
    operatingSystems: ["windows", "linux"],
  });

  private static fingerprintInjector = new FingerprintInjector();

  public static async createContext(): Promise<BrowserContextResult> {
    const fingerprint = this.fingerprintGenerator.getFingerprint();

    // Lanzamiento con configuración de evasión
    const browser = await chromium.launch({
      headless: config.HEADLESS,
      args: [
        "--disable-blink-features=AutomationControlled",
        "--no-sandbox",
        "--disable-setuid-sandbox",
        "--disable-infobars",
        "--ignore-certificate-errors",
        "--disable-dev-shm-usage",
        "--disable-gpu",
        `--window-size=${fingerprint.screen.width},${fingerprint.screen.height}`,
      ],
    });

    const context = await browser.newContext({
      userAgent: fingerprint.navigator.userAgent,
      viewport: {
        width: fingerprint.screen.width,
        height: fingerprint.screen.height,
      },
      locale: "en-US",
      timezoneId: "America/New_York",
      permissions: ["clipboard-read", "clipboard-write"],
      deviceScaleFactor: 1,
    });

    // Inyección de huella digital
    await this.fingerprintInjector.attachFingerprintToPlaywright(
      context,
      fingerprint,
    );

    const identityEmail = await this.injectIdentity(context);

    // ✅ FIX TÁCTICO: Casting para resolver conflicto de tipos TS2322
    // La incompatibilidad es solo a nivel de tipos de definición (.d.ts),
    // el objeto en runtime es compatible.
    return {
      browser: browser as unknown as Browser,
      context: context as unknown as BrowserContext,
      identityEmail
    };
  }

  private static async injectIdentity(
    context: BrowserContext,
  ): Promise<string | null> {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    let rawCookies: any[] = [];
    let identityEmail: string | null = null;
    let source = "NONE";

    if (config.ORCHESTRATOR_URL && config.WORKER_AUTH_TOKEN) {
      try {
        const response = await axios.get<LeasedIdentity>(
          `${config.ORCHESTRATOR_URL}/api/v1/admin/identities/lease`,
          {
            params: { platform: "google_colab" },
            headers: { Authorization: `Bearer ${config.WORKER_AUTH_TOKEN}` },
            timeout: 5000,
          },
        );

        if (response.data) {
          identityEmail = response.data.email;
          source = `VAULT (${identityEmail})`;
          const rawCredentialsData = response.data.credentials_json;

          try {
            const parsedPayload = JSON.parse(rawCredentialsData);

            if (parsedPayload.cipher_text_base64 && parsedPayload.initialization_vector_base64) {
              if (!config.MASTER_VAULT_KEY) {
                throw new Error("MASTER_VAULT_KEY missing in Provisioner Environment.");
              }
              console.log(`🔐 [IDENTITY] Decrypting Zero-Knowledge payload for ${identityEmail}...`);
              const decryptedJsonString = await VaultCryptoEngine.decryptPortable(
                parsedPayload as EncryptedVaultPayload,
                config.MASTER_VAULT_KEY,
                identityEmail
              );
              rawCookies = JSON.parse(decryptedJsonString);
            } else {
              rawCookies = parsedPayload;
            }
          } catch (cryptoError: unknown) {
            const msg = cryptoError instanceof Error ? cryptoError.message : String(cryptoError);
            console.error(`❌ [CRYPTO_FAULT]: Decryption failed. Identity discarded. Reason: ${msg}`);
            return null;
          }
        }
      } catch (networkError: unknown) {
          if (axios.isAxiosError(networkError)) {
             console.warn(`⚠️ [UPLINK_WARNING]: Orchestrator lease failed: ${networkError.message}`);
          }
      }
    }

    if (rawCookies.length === 0 && config.GOOGLE_COOKIES_JSON) {
      try {
        rawCookies = JSON.parse(config.GOOGLE_COOKIES_JSON);
        source = "ENV_VAR";
        identityEmail = "env-user@local";
      } catch {
        const localPath = path.resolve(config.GOOGLE_COOKIES_JSON);
        if (fs.existsSync(localPath)) {
          try {
            rawCookies = JSON.parse(fs.readFileSync(localPath, "utf-8"));
            source = "LOCAL_FILE";
            identityEmail = "file-user@local";
          } catch { /* Ignored */ }
        }
      }
    }

    if (rawCookies.length > 0) {
      const cleanCookies: PlaywrightCookie[] = purifyCookies(rawCookies);
      if (cleanCookies.length > 0) {
        await context.addCookies(cleanCookies);
        console.info(`✅ [IDENTITY] ${cleanCookies.length} cookies injected. Source: ${source}`);
        return identityEmail;
      }
    }

    console.warn("⚠️ [IDENTITY] Starting in ANONYMOUS mode. Compute capacity will be severely limited.");
    return null;
  }
}
// FIN DEL ARCHIVO [tools/provisioner/src/lib/browser.ts]

/**
 * =================================================================
 * APARATO: BROWSER FACTORY (V24.1 - SOBERANO SANEADO)
 * CLASIFICACIÓN: OPS INFRASTRUCTURE (ESTRATO L6)
 * RESPONSABILIDAD: ORQUESTACIÓN DE INSTANCIAS CON PERSISTENCIA DE PERFIL
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. TYPE RESILIENCE: Resuelve TS2305 eliminando las importaciones de tipos
 *    inexistentes en Playwright y FingerprintGenerator, utilizando
 *    inferencia nativa del motor.
 * 2. HYGIENE TOTAL: Erradicación de variables muertas (fs, path, unused types).
 * 3. PROXY ALIGNMENT: Implementa la configuración de red mediante interfaces
 *    compatibles con la API actual de Chromium.
 * 4. NOMINAL PURITY: Mantenimiento de la nomenclatura bit-perfecta para la Tesis.
 * =================================================================
 */

import { chromium } from "playwright-extra";
import stealth from "puppeteer-extra-plugin-stealth";
import { BrowserContext, Browser } from "playwright";
import { FingerprintGenerator } from "fingerprint-generator";
import { FingerprintInjector } from "fingerprint-injector";
import axios from "axios";

import { config } from "../config";
import { purifyCookies } from "./cookie-purifier";
import { VaultCryptoEngine, type EncryptedVaultPayload } from "@prospector/crypto-vault";

// Inyección de plugins de sigilo perimetral
chromium.use(stealth());

/**
 * Interface del contrato de Identidad Leveled con el Esquema L3 V154.0.
 */
interface LeasedIdentity {
  id: string;
  email: string;
  credentials_json: string;
  user_agent: string;
  browser_fingerprint_json?: string;
  proxy_url?: string;
}

/**
 * Resultado de la cristalización de un contexto soberano.
 */
export interface BrowserContextResult {
  browser_instance: Browser;
  context_instance: BrowserContext;
  operator_email_identity: string | null;
}

export class BrowserFactory {
  private static fingerprint_generator = new FingerprintGenerator({
    browsers: [{ name: "chrome", minVersion: 115 }],
    devices: ["desktop"],
    operatingSystems: ["windows", "linux"],
  });

  private static fingerprint_injector = new FingerprintInjector();

  /**
   * Orquesta la creación de un entorno de navegación soberano.
   *
   * # Performance:
   * Realiza el triaje de identidad en O(1) antes de la ignición de hardware
   * para inyectar la configuración de Proxy directamente en el proceso.
   */
  public static async create_sovereign_context(): Promise<BrowserContextResult> {
    // 1. ADQUISICIÓN PREVIA DE IDENTIDAD (Identity-First)
    const leased_identity = await this.lease_identity_from_orchestrator();

    // 2. CONFIGURACIÓN DE RED (Proxy Strata)
    // ✅ RESOLUCIÓN TS2305: Uso de interface anónima compatible con BrowserTypeLaunchOptions
    let tactical_proxy_config: { server: string } | undefined;
    if (leased_identity?.proxy_url) {
      console.info(`🌐 [NETWORK] Routing through dedicated proxy: ${leased_identity.proxy_url}`);
      tactical_proxy_config = { server: leased_identity.proxy_url };
    }

    // 3. IGNICIÓN DEL BINARIO CHROMIUM
    const browser_instance = await chromium.launch({
      headless: config.HEADLESS,
      proxy: tactical_proxy_config,
      args: [
        "--disable-blink-features=AutomationControlled",
        "--no-sandbox",
        "--disable-setuid-sandbox",
        "--disable-infobars",
        "--ignore-certificate-errors",
        "--disable-dev-shm-usage",
        "--disable-gpu",
      ],
    });

    // 4. RESTAURACIÓN DE HUELLA DIGITAL (Fingerprint Strata)
    // ✅ RESOLUCIÓN TS2305: Inferencia de tipo desde el generador
    let fingerprint_to_apply;

    if (leased_identity?.browser_fingerprint_json) {
        console.info("🧬 [IDENTITY] Restoring persistent hardware fingerprint from strata.");
        fingerprint_to_apply = JSON.parse(leased_identity.browser_fingerprint_json);
    } else {
        console.info("🎲 [IDENTITY] Generating new ephemeral fingerprint (Profile initialization).");
        fingerprint_to_apply = this.fingerprint_generator.getFingerprint();
    }

    const context_instance = await browser_instance.newContext({
      userAgent: leased_identity?.user_agent || fingerprint_to_apply.navigator.userAgent,
      viewport: fingerprint_to_apply.screen,
      locale: "en-US",
      timezoneId: "America/New_York",
      permissions: ["clipboard-read", "clipboard-write"],
      deviceScaleFactor: 1,
    });

    // Inyección física de la huella en el motor del navegador
    await this.fingerprint_injector.attachFingerprintToPlaywright(
      context_instance,
      fingerprint_to_apply,
    );

    // 5. HIDRATACIÓN DE COOKIES ZK
    const identity_email = leased_identity ? leased_identity.email : null;
    if (leased_identity) {
        await this.inject_cookies_into_context(context_instance, leased_identity);
    }

    return {
      browser_instance: browser_instance as unknown as Browser,
      context_instance: context_instance as unknown as BrowserContext,
      operator_email_identity: identity_email
    };
  }

  /**
   * Negocia el arrendamiento de una identidad con el Orquestador L3.
   */
  private static async lease_identity_from_orchestrator(): Promise<LeasedIdentity | null> {
    if (!config.ORCHESTRATOR_URL || !config.WORKER_AUTH_TOKEN) return null;

    try {
      const network_response = await axios.get<LeasedIdentity>(
        `${config.ORCHESTRATOR_URL}/api/v1/admin/identities/lease`,
        {
          params: { platform: "google_colab" },
          headers: { Authorization: `Bearer ${config.WORKER_AUTH_TOKEN}` },
          timeout: 7000,
        },
      );

      return network_response.data;
    } catch (unidentified_fault: unknown) {
      console.warn("⚠️ [UPLINK_WARNING] Identity lease failed. Falling back to local/anonymous.");
      return null;
    }
  }

  /**
   * Ejecuta el descifrado y la inyección de cookies en el contexto actual.
   */
  private static async inject_cookies_into_context(
    context: BrowserContext,
    identity: LeasedIdentity
  ): Promise<void> {
    try {
      const raw_credentials_data = identity.credentials_json;
      let cookies_to_inject: unknown[] = [];

      const parsed_payload = JSON.parse(raw_credentials_data);

      // Verificación de blindaje ZK
      if (parsed_payload.cipher_text_base64 && parsed_payload.initialization_vector_base64) {
        if (!config.MASTER_VAULT_KEY) throw new Error("MASTER_VAULT_KEY_VOID");

        console.info(`🔐 [VAULT] Decrying ZK-Strata for ${identity.email}...`);
        const decrypted_string = await VaultCryptoEngine.decryptPortable(
          parsed_payload as EncryptedVaultPayload,
          config.MASTER_VAULT_KEY,
          identity.email
        );
        cookies_to_inject = JSON.parse(decrypted_string);
      } else {
        cookies_to_inject = parsed_payload;
      }

      if (Array.isArray(cookies_to_inject)) {
        const clean_cookies = purifyCookies(cookies_to_inject);
        await context.addCookies(clean_cookies);
        console.info(`✅ [VAULT] ${clean_cookies.length} purified cookies injected for ${identity.email}.`);
      }
    } catch (critical_fault: unknown) {
      const error_message = critical_fault instanceof Error ? critical_fault.message : "UNKNOWN_CRYPTO_FAULT";
      console.error(`❌ [IDENTITY_COLLAPSE] Injection failed: ${error_message}`);
    }
  }
}

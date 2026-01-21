/**
 * =================================================================
 * APARATO: MULTI-VECTOR CONFIGURATION (V12.0 - PRODUCTION SOBERANO)
 * CLASIFICACIÓN: OPS INFRASTRUCTURE (ESTRATO L6)
 * RESPONSABILIDAD: VALIDACIÓN Y SELLADO DE PARÁMETROS OPERATIVOS
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. PRODUCTION HARDENING: Enforce de seguridad para la nueva cuenta.
 *    La MASTER_VAULT_KEY ahora exige 16 caracteres mínimo para AES-256.
 * 2. NOMINAL PURITY: 'WORKER_COUNT' transiciona a 'SWARM_NODES_COUNT'
 *    para reflejar la densidad de la rejilla distribuida.
 * 3. VERSION STEALTH: Inyecta 'PROSPECTOR_VERSION' para que el payload
 *    Python reporte su linaje exacto al Panóptico.
 * 4. ERROR TRIAGE: Mensajes de error personalizados en Zod para facilitar
 *    el debug en hilos de CI/CD (GitHub Actions).
 * =================================================================
 */

import { z } from "zod";
import "dotenv/config";

/**
 * Esquema Maestro de Configuración Operativa.
 * Actúa como el primer firewall del enjambre ante la ignición.
 */
const ConfigurationSchema = z.object({
  // --- ESTRATO DE IDENTIDAD Y VERSIÓN ---
  NODE_ENV: z.enum(["development", "production", "test"]).default("production"),
  PROSPECTOR_VERSION: z.string().default("V11.5-GOLD"),

  // --- ESTRATO DE RED (UPLINK L4) ---
  /** URL del Orquestador en Render. Debe ser absoluta. */
  ORCHESTRATOR_URL: z.string().url({ message: "ORCHESTRATOR_URL_INVALID_FORMAT" }),
  /** Token de autorización para el handshake táctico. */
  WORKER_AUTH_TOKEN: z.string().min(12, { message: "TOKEN_TOO_WEAK_FOR_PRODUCTION" }),
  /** Llave maestra para abrir la Bóveda ZK. */
  MASTER_VAULT_KEY: z.string().min(16, { message: "VAULT_KEY_INSUFFICIENT_ENTROPY" }),

  // --- ESTRATO DE ARTEFACTOS (L1) ---
  /** URL directa al binario miner-worker en GitHub Releases. */
  MINER_BINARY_URL: z.string().url({ message: "BINARY_URL_UNREACHABLE" }),

  // --- VECTOR ALFA: GOOGLE COLAB (ESTRATEGIA PRIMARIA) ---
  COLAB_URL: z.string().url().default("https://colab.research.google.com/"),
  /** Cookies crudas para bypass de login (opcional si hay sesión activa). */
  GOOGLE_COOKIES_JSON: z.string().optional(),

  // --- VECTOR BETA: KAGGLE KERNELS (ESTRATEGIA DE DISPERSIÓN) ---
  KAGGLE_URL: z.string().url().default("https://www.kaggle.com/code/"),
  /** Ratio de distribución de carga entre Colab y Kaggle (0.0 a 1.0). */
  KAGGLE_DISTRIBUTION_RATIO: z.coerce.number().min(0).max(1).default(0.0), // Producción: Colab-First

  // --- PARÁMETROS DE SATURACIÓN (GRID DENSITY) ---
  /** Cantidad de unidades de cómputo a desplegar en el enjambre. */
  SWARM_NODES_COUNT: z.coerce.number().int().min(1).max(300).default(1),
  /** URL base para la hidratación de los fragmentos del censo. */
  FILTER_BASE_URL: z.string().url(),
  /** Cantidad determinista de shards (Sincronizado con entrypoint.sh). */
  FILTER_SHARDS: z.coerce.number().int().positive().default(4),

  // --- CONFIGURACIÓN DEL MOTOR DE NAVEGACIÓN (L6) ---
  /** Si es false, permite la supervisión visual local durante el debug. */
  HEADLESS: z.coerce.boolean().default(true),
  /** Timeout de red (milisegundos) para compensar latencia de Render. */
  NAV_TIMEOUT: z.coerce.number().default(120000),
});

/**
 * Cristalización de la Configuración Soberana.
 * Realiza un parseo defensivo de las variables de entorno.
 */
const parse_result = ConfigurationSchema.safeParse(process.env);

if (!parse_result.success) {
  const error_analysis = parse_result.error.format();
  console.error("\n❌ [CONFIG_FAULT]: Strata validation failed for the Swarm Commander.");
  console.error("DETAILS:", JSON.stringify(error_analysis, null, 2));
  process.exit(1); // Bloqueo preventivo de ignición
}

export const config = parse_result.data;

/**
 * Alias de compatibilidad para evitar regresiones en aparatos dependientes.
 */
export const legacy_config_alias = {
  get WORKER_COUNT() { return config.SWARM_NODES_COUNT; }
};

// INICIO DEL ARCHIVO [apps/web-dashboard/lib/i18n/content/es/dashboard/atoms/research.content.ts]
import { type LabAtom, type VaultAtom } from "../../../../schemas/dashboard/atoms/research.schema";

export const labContent: LabAtom = {
  title: "Estrato de Investigación Experimental",
  interceptor_title: "Interceptor de Entropía Neural",
  forge_title: "Forja y Cristalizador de Escenarios",
  scan_btn: "INICIALIZAR SECUENCIA DE ESCANEO",
  inject_btn: "CRISTALIZAR BOLETO DORADO",
  no_scenarios: "NO EXISTEN EXPERIMENTOS CRIPTOGRÁFICOS ACTIVOS EN EL LEDGER",
  audit_ledger_title: "Libro Mayor de Auditoría Forense",
};

export const vaultContent: VaultAtom = {
  title: "Bóveda de Identidad de Conocimiento Cero",
  injection_badge: "PROTECCIÓN AES-256-GCM ACTIVA",
  encrypting: "CIFRANDO_PAYLOAD_DE_IDENTIDAD_LOCALMENTE...",
  secure_btn: "ASEGURAR EN EL LEDGER TÁCTICO",
  empty_vault: "El búnker de identidad está vacío. Se requiere inyección manual.",
  // ✅ NUEVO
  cookie_report: {
    status_optimal: "INTEGRIDAD DE IDENTIDAD: ÓPTIMA",
    status_degraded: "SALUD DE IDENTIDAD: DEGRADADA (RIESGO DE CAPTCHA)",
    status_critical: "IDENTIDAD INCOMPLETA: FALTAN LLAVES MAESTRAS",
    stats_valid: "Credenciales Válidas",
    stats_garbage: "Basura de Rastreo Eliminada",
    missing_keys: "LLAVES CRÍTICAS FALTANTES:",
    recommendation: "El sistema auto-purificará la carga antes de cifrar.",
  }
};
// FIN DEL ARCHIVO [apps/web-dashboard/lib/i18n/content/es/dashboard/atoms/research.content.ts]

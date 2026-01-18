// INICIO DEL ARCHIVO [apps/web-dashboard/lib/i18n/content/en/dashboard/atoms/research.content.ts]
import { type LabAtom, type VaultAtom } from "../../../../schemas/dashboard/atoms/research.schema";

export const labContent: LabAtom = {
  title: "Experimental Research Stratum",
  interceptor_title: "Neural Entropy Interceptor",
  forge_title: "Scenario Forge & Crystallizer",
  scan_btn: "INITIALIZE SCAN SEQUENCE",
  inject_btn: "CRYSTALLIZE GOLDEN TICKET",
  no_scenarios: "NO ACTIVE CRYPTOGRAPHIC EXPERIMENTS IN LEDGER",
  audit_ledger_title: "Forensic Audit Ledger",
};

export const vaultContent: VaultAtom = {
  title: "Zero-Knowledge Identity Vault",
  injection_badge: "AES-256-GCM PROTECTION ACTIVE",
  encrypting: "ENCRYPTING_IDENTITY_PAYLOAD_LOCALLY...",
  secure_btn: "SECURE IN TACTICAL LEDGER",
  empty_vault: "The Identity Bunker is empty. Manual injection required.",
  // ✅ NUEVO
  cookie_report: {
    status_optimal: "IDENTITY INTEGRITY: OPTIMAL",
    status_degraded: "IDENTITY HEALTH: DEGRADED (RISK OF CAPTCHA)",
    status_critical: "IDENTITY INCOMPLETE: MISSING CORE KEYS",
    stats_valid: "Valid Credentials",
    stats_garbage: "Tracking Garbage Removed",
    missing_keys: "CRITICAL MISSING KEYS:",
    recommendation: "System will auto-purify the payload before encryption.",
  }
};
// FIN DEL ARCHIVO [apps/web-dashboard/lib/i18n/content/en/dashboard/atoms/research.content.ts]

// INICIO DEL ARCHIVO [apps/web-dashboard/lib/i18n/content/en/dashboard/atoms/ledger.content.ts]
import { type ArchivalStatusAtom, type AuditTrailAtom, type StrategiesAtom } from "../../../../schemas/dashboard/atoms/ledger.schema";

export const archivalStatusContent: ArchivalStatusAtom = {
  engine_b_parity: "Engine B Parity Monitor",
  strategic_vault_link: "Strategic Archival Link (Supabase)",
  archival_integrity: "Archival Chain Integrity",
  sync_drift_detected: "SYNC_DRIFT: {count} missions pending strategic migration.",
  total_archived_missions: "Total Certified Missions in Cold Storage",
};

export const auditTrailContent: AuditTrailAtom = {
  title: "Immutable Mission Audit Ledger",
  column_mission: "Mission Identifier",
  column_strategy: "Applied Strategy",
  column_effort: "Computational Volume",
  column_status: "Certification Status",
  column_footprint: "Verification Footprint (Hexadecimal)",
  empty_state: "The Strategic Archive is awaiting data migration from Stratum L3.",
};

export const strategiesContent: StrategiesAtom = {
  sequential: "Sequential U256 Range Audit",
  dictionary: "Entropy Dictionary Handshake",
  static_handshake: "Specific Secret Verification",
  forensic_archaeology: "Historical PRNG Pattern Recovery",
};
// FIN DEL ARCHIVO [apps/web-dashboard/lib/i18n/content/en/dashboard/atoms/ledger.content.ts]

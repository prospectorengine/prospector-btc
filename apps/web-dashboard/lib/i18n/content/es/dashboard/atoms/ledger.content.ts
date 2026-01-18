// INICIO DEL ARCHIVO [apps/web-dashboard/lib/i18n/content/es/dashboard/atoms/ledger.content.ts]
import { type ArchivalStatusAtom, type AuditTrailAtom, type StrategiesAtom } from "../../../../schemas/dashboard/atoms/ledger.schema";

export const archivalStatusContent: ArchivalStatusAtom = {
  engine_b_parity: "Monitor de Paridad del Motor B",
  strategic_vault_link: "Enlace de Archivo Estratégico (Supabase)",
  archival_integrity: "Integridad de la Cadena de Archivo",
  sync_drift_detected: "DERIVA_DE_SINCRONIZACIÓN: {count} misiones pendientes de migración.",
  total_archived_missions: "Total de Misiones Certificadas en Almacenamiento Frío",
};

export const auditTrailContent: AuditTrailAtom = {
  title: "Ledger de Auditoría de Misiones Inmutable",
  column_mission: "Identificador de la Misión",
  column_strategy: "Estrategia Aplicada",
  column_effort: "Volumen Computacional",
  column_status: "Estado de la Certificación",
  column_footprint: "Huella de Verificación (Hexadecimal)",
  empty_state: "El Archivo Estratégico está esperando la migración de datos desde el Estrato L3.",
};

export const strategiesContent: StrategiesAtom = {
  sequential: "Auditoría de Rango Secuencial U256",
  dictionary: "Apretón de Manos por Diccionario de Entropía",
  static_handshake: "Verificación de Secreto Específico",
  forensic_archaeology: "Recuperación de Patrones de PRNG Históricos",
};
// FIN DEL ARCHIVO [apps/web-dashboard/lib/i18n/content/es/dashboard/atoms/ledger.content.ts]

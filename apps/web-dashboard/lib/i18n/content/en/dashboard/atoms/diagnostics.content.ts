/**
 * =================================================================
 * APARATO: DIAGNOSTICS CONTENT (EN - V1.0)
 * CLASIFICACIÓN: I18N ATOM (ESTRATO L5)
 * RESPONSABILIDAD: TEXTOS TÉCNICOS PARA EL PROVING GROUNDS
 * =================================================================
 */

import { type DiagnosticsAtom } from "../../../../schemas/dashboard/atoms/diagnostics.schema";

export const diagnosticsContent: DiagnosticsAtom = {
  kernel_audit_btn: "EXECUTE_KERNEL_HANDSHAKE",
  panopticon_title: "Unified_Panopticon_Stream",
  metrics: {
    memory: "RAM_UTILIZATION",
    threads: "COMPUTE_UNITS",
    uptime: "UPLINK_PULSE",
    integrity: "CLEARANCE_LEVEL",
  },
  cards: {
    l1_description: "Certify L1 Arithmetic & Geometric kernels against Satoshi Genesis vectors.",
    l2_description: "Validate L2 Search Strategies (Sequential/Forensic) with short-range sweeps.",
    l3_description: "Audit L3 Persistence strata and multi-engine synchronization parity.",
    ignite_btn: "IGNITE_PROVING_GROUNDS",
  },
};

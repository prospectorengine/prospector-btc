/**
 * =================================================================
 * APARATO: DIAGNOSTICS CONTENT (ES - V1.0)
 * CLASIFICACIÓN: I18N ATOM (ESTRATO L5)
 * RESPONSABILIDAD: TEXTOS TÉCNICOS PARA EL PROVING GROUNDS
 * =================================================================
 */

import { type DiagnosticsAtom } from "../../../../schemas/dashboard/atoms/diagnostics.schema";

export const diagnosticsContent: DiagnosticsAtom = {
  kernel_audit_btn: "EJECUTAR_AUDITORÍA_KERNEL",
  panopticon_title: "Flujo_Unificado_Panóptico",
  metrics: {
    memory: "UTILIZACIÓN_RAM",
    threads: "UNIDADES_CÓMPUTO",
    uptime: "PULSO_DE_ENLACE",
    integrity: "NIVEL_DE_ACCESO",
  },
  cards: {
    l1_description: "Certificar kernels aritméticos y geométricos L1 contra vectores génesis.",
    l2_description: "Validar estrategias de búsqueda L2 (Secuencial/Forense) con barridos cortos.",
    l3_description: "Auditar estratos de persistencia L3 y paridad de sincronización multi-motor.",
    ignite_btn: "INICIAR_CAMPOS_DE_PRUEBA",
  },
};

// INICIO DEL ARCHIVO [apps/web-dashboard/lib/i18n/content/es/dashboard/atoms/academy.content.ts]
import { type AcademyAtom } from "../../../../schemas/dashboard/atoms/academy.schema";

export const academyContent: AcademyAtom = {
  page_title: "Academia Prospector",
  page_subtitle: "Curriculum de Criptografía Avanzada y Análisis Forense. Domina el protocolo.",
  levels: {
    foundation: "FUNDAMENTOS",
    intermediate: "INTERMEDIO",
    elite: "OPERACIONES DE ÉLITE",
  },
  actions: {
    start: "INICIALIZAR MÓDULO",
    continue: "REANUDAR ENLACE",
    locked: "ENCRIPTADO / BLOQUEADO",
    completed: "CERTIFICADO",
  },
  modules: {
    mod_01: {
      title: "Génesis de Curva Elíptica",
      desc: "Fundamentos matemáticos de secp256k1. Comprendiendo el Problema del Logaritmo Discreto y la geometría de la seguridad de Bitcoin.",
    },
    mod_02: {
      title: "Arqueología de Entropía",
      desc: "Análisis forense de fallos históricos de PRNG. Disección de los vectores Debian 2008 y Android 2013.",
    },
    mod_03: {
      title: "Operaciones de Enjambre Hydra",
      desc: "Protocolos de Mando y Control (C2). Gestión de nodos efímeros distribuidos en entornos hostiles.",
    },
    mod_04: {
      title: "Identidad de Conocimiento Cero",
      desc: "Protocolos de seguridad avanzada. Inyección AES-256-GCM del lado del cliente y gestión de higiene de identidad.",
    },
  },
};
// FIN DEL ARCHIVO

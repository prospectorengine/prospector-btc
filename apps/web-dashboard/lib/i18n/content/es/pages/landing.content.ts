import { type LandingPageParams } from "../../../schemas/pages/landing.schema";

export const landingPageContent: LandingPageParams = {
  meta: {
    title: "Prospector // Protocolo Hydra-Zero",
    description: "Sistema de auditoría criptográfica distribuida enfocado en la curva secp256k1.",
  },
  hero: {
    badge: "SISTEMA OPERATIVO // VERSIÓN 11.5",
    title: "Arqueología de Entropía Distribuida",
    subtitle: "Únase al enjambre global. Auditoría del ledger inmutable mediante matemáticas probabilísticas y computación efímera.",
    cta_primary: {
      label: "Inicializar Sistema",
      tooltip: "Comenzar en Nivel Observador",
    },
  },
  capsules: {
    login: {
      title: "Operadores Activos",
      description: "Acceda al centro de mando mediante el apretón de manos seguro.",
      cta: "ACCEDER A LA CONSOLA",
      badge: "SEGURO",
    },
    register: {
      title: "Nuevo Despliegue",
      description: "Provisión de una nueva identidad de nodo para la rejilla de investigación.",
      cta: "INICIALIZAR PROTOCOLO",
      badge: "ABIERTO",
    },
  },
  pricing: {
    observer_title: "Nodo Observador",
    observer_desc: "Visibilidad en tiempo real de las métricas del enjambre y telemetría de cobertura.",
    cta_free: "COMENZAR MONITOREO",
    operator_title: "Nodo Operador",
    operator_desc: "Control total sobre el enjambre, inyección de identidad y acceso al laboratorio forense.",
    cta_pro: "ACTUALIZAR PROTOCOLO",
  },
};

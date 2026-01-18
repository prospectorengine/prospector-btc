import { type PublicFooterParams } from "../../../schemas/layout/public-footer.schema";

export const publicFooterContent: PublicFooterParams = {
  brand: {
    mission: "Auditoría distribuida de entropía y arqueología criptográfica sobre el ledger inmutable de Bitcoin mediante computación efímera de alto rendimiento.",
    copyright: "MetaShark Tech. Todos los derechos reservados.",
    location: "Sede Global: Florianópolis, SC // BRASIL",
  },
  columns: {
    product: "Protocolo",
    resources: "Conocimiento",
    community: "Comunidad",
    legal: "Soberanía",
  },
  newsletter: {
    title: "Terminal de Inteligencia",
    description: "Reciba ráfagas de telemetría y actualizaciones sobre hallazgos de entropía débil directamente en su terminal.",
    placeholder: "operador@metashark.tech",
    button: "SUSCRIBIR",
  },
  disclaimer: "Herramienta de investigación académica doctoral desarrollada por Raz Podesta. MetaShark Tech no se responsabiliza por el uso ilícito de la suite. El acceso a claves privadas ajenas contraviene los protocolos éticos del sistema. Utilizar exclusivamente para auditoría de seguridad y estudio de ECC.",
};

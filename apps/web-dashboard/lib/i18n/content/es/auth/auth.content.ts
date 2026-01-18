import { type AuthParams } from "../../../schemas/auth/auth.schema";

export const authContent: AuthParams = {
  login: {
    title: "Identificación de Operador",
    google_btn: "Autenticar mediante Google",
    footer_text: "Conexión Segura Cifrada // TLS 1.3",
  },
  register: {
    title: "Únete a la Rejilla",
    badge: "Aprovisionamiento de Nuevo Operador",
    footer_text: "Inicializando Protocolo de Identidad Soberana",
  },
  logout: {
    label: "Cerrando sesión...",
    confirm_msg: "Sesión de Operador Terminada",
  },
  errors: {
    signin_failed: "Fallo en el Handshake de Autenticación",
    access_denied: "Acceso Denegado: Nivel de Autorización Insuficiente",
  },
};

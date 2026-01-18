// INICIO DEL ARCHIVO [apps/web-dashboard/lib/i18n/content/en/auth/auth.content.ts]
import { type AuthParams } from "../../../schemas/auth/auth.schema";

export const authContent = {
  login: {
    title: "Identify Yourself",
    google_btn: "Authenticate via Google",
    footer_text: "Secure Connection // TLS 1.3",
  },
  // ✅ NUEVO: Sección de Registro requerida por el esquema actualizado
  register: {
    title: "Join the Grid",
    badge: "New Operator Provisioning",
    footer_text: "Initiating Sovereign Identity Protocol",
  },
  logout: {
    label: "Logging out...",
    confirm_msg: "Session Terminated",
  },
  errors: {
    signin_failed: "Authentication Handshake Failed",
    access_denied: "Access Denied: Authorization Level Insufficient",
  },
} satisfies AuthParams;
// FIN DEL ARCHIVO [apps/web-dashboard/lib/i18n/content/en/auth/auth.content.ts]

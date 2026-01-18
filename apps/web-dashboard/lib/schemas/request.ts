// INICIO DEL ARCHIVO [apps/web-dashboard/lib/schemas/request.ts]
/**
 * =================================================================
 * APARATO: I18N REQUEST ADAPTER (V11.1 - TYPE SECURED)
 * RESPONSABILIDAD: RESOLUCIÓN DETERMINISTA DE MENSAJES
 * =================================================================
 */

import { getRequestConfig } from "next-intl/server";
import { routing } from "./routing";
import { type AppLocale } from "../i18n/schema";

// IMPORTACIÓN ESTÁTICA EXPLÍCITA (CRÍTICO PARA VERCEL/TURBOPACK)
import enMessages from "../../messages/en.json";
import esMessages from "../../messages/es.json";

const messagesMap: Record<string, AppLocale> = {
  en: enMessages as AppLocale,
  es: esMessages as AppLocale,
};

export default getRequestConfig(async ({ requestLocale }) => {
  let locale = await requestLocale;

  /**
   * RESOLUCIÓN TS2345: Tipado estricto para 'includes'.
   * Convertimos el locale a un tipo compatible con el array de solo lectura.
   */
  if (!locale || !routing.locales.includes(locale as (typeof routing.locales)[number])) {
    locale = routing.defaultLocale;
  }

  // Resolución O(1) en memoria (Cero I/O en tiempo de ejecución)
  const messages = messagesMap[locale] || messagesMap[routing.defaultLocale];

  return {
    locale,
    messages,
    timeZone: "UTC",
    now: new Date()
  };
});
// FIN DEL ARCHIVO [apps/web-dashboard/lib/schemas/request.ts]

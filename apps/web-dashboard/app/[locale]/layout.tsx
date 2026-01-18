/**
 * =================================================================
 * APARATO: GLOBAL LOCALE LAYOUT (V12.1 - TYPE HARDENED)
 * CLASIFICACIÓN: VIEW INFRASTRUCTURE (ESTRATO L5)
 * RESPONSABILIDAD: ORQUESTACIÓN DE ESTRATOS VISUALES Y LOCALIZACIÓN
 * =================================================================
 */

import { NextIntlClientProvider } from "next-intl";
import { getMessages } from "next-intl/server";
import { notFound } from "next/navigation";
import { routing } from "@/lib/schemas/routing";
import Providers from "../providers";
import "../global.css";

interface LocaleLayoutProperties {
  children: React.ReactNode;
  params: Promise<{ locale: string }>;
}

export const metadata = {
  title: "Prospector // Mission Control",
  description: "Distributed Entropy Audit System targeting secp256k1.",
};

/**
 * Raíz de renderizado localizada.
 * RESOLUCIÓN: Eliminación de 'any' mediante casting explícito al esquema de routing.
 */
export default async function LocaleLayout({
  children,
  params,
}: LocaleLayoutProperties) {
  const { locale } = await params;

  // Validación de frontera de idioma
  if (!routing.locales.includes(locale as typeof routing.locales[number])) {
    notFound();
  }

  // Adquisición de señales de traducción
  const messages = await getMessages();

  return (
    <html lang={locale} className="dark" suppressHydrationWarning>
      <body className="bg-[#050505] text-slate-200 antialiased min-h-screen selection:bg-primary/30">
        <NextIntlClientProvider messages={messages}>
          <Providers>{children}</Providers>
        </NextIntlClientProvider>
      </body>
    </html>
  );
}

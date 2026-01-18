// INICIO DEL ARCHIVO [apps/web-dashboard/app/api/auth/callback/route.ts]
/**
 * =================================================================
 * APARATO: AUTH CALLBACK HANDLER (V4.0 - HEIMDALL SECURED)
 * CLASIFICACIÓN: API STRATUM (ESTRATO L4)
 * RESPONSABILIDAD: FINALIZACIÓN DE PKCE CON TRAZABILIDAD
 * =================================================================
 */

import { createServerClient, type CookieOptions } from '@supabase/ssr';
import { cookies } from 'next/headers';
import { NextResponse } from 'next/server';
import { createLogger } from '@prospector/heimdall-ts';

// Logger instanciado fuera del handler para reutilización en Edge/Node
const logger = createLogger("AuthCallbackHandler");

export async function GET(request: Request) {
  const requestUrl = new URL(request.url);
  const code = requestUrl.searchParams.get('code');
  const next = requestUrl.searchParams.get('next') ?? '/dashboard';

  const audit_trace = logger.track("PKCE_Code_Exchange");

  // 1. DETERMINACIÓN DEL ORIGEN CANÓNICO (SSoT)
  const canonicalOrigin = process.env.NEXT_PUBLIC_SITE_URL;

  if (!canonicalOrigin) {
    audit_trace.fail(new Error("NEXT_PUBLIC_SITE_URL_MISSING"));
    return NextResponse.json({ error: "Configuration Error" }, { status: 500 });
  }

  if (code) {
    const cookieStore = await cookies();

    const supabase = createServerClient(
      process.env.NEXT_PUBLIC_SUPABASE_URL!,
      process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!,
      {
        cookies: {
          get(name: string) {
            return cookieStore.get(name)?.value;
          },
          set(name: string, value: string, options: CookieOptions) {
            cookieStore.set({ name, value, ...options });
          },
          remove(name: string, options: CookieOptions) {
            cookieStore.delete({ name, ...options });
          },
        },
      }
    );

    const { error } = await supabase.auth.exchangeCodeForSession(code);

    if (!error) {
      audit_trace.ok({ destination: next });

      // ÉXITO: Redirección forzada al dominio canónico
      const forwardedUrl = `${canonicalOrigin}${next}`;
      return NextResponse.redirect(forwardedUrl);
    } else {
      audit_trace.fail(error);
      // El error específico de Supabase queda registrado en Heimdall
    }
  } else {
    logger.warn("Callback invoked without authorization code.");
  }

  // FALLO: Retorno al login canónico
  return NextResponse.redirect(`${canonicalOrigin}/login?error=auth_code_exchange_failed`);
}
// FIN DEL ARCHIVO [apps/web-dashboard/app/api/auth/callback/route.ts]

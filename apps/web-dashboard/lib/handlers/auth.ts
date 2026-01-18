// INICIO DEL ARCHIVO [apps/web-dashboard/lib/handlers/auth.ts]
/**
 * =================================================================
 * APARATO: AUTHENTICATION HANDLER (V18.2 - GATEWAY SECURED)
 * CLASIFICACIÓN: MIDDLEWARE LOGIC (L4)
 * RESPONSABILIDAD: PROTECCIÓN DE ESTRATOS Y GESTIÓN DE SESIÓN
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como el portero lógico del Middleware.
 * 1. Intercepta rutas protegidas (Dashboard, Lab, Admin).
 * 2. Intercepta puertas de enlace (Login, Register).
 * 3. Preserva el contexto de localización (Locale) en las redirecciones.
 * =================================================================
 */

import { NextRequest, NextResponse } from "next/server";
import { getToken } from "next-auth/jwt";
import { routing } from "@/lib/schemas/routing";

/**
 * ESTRATOS PROTEGIDOS (Requieren Sesión Activa)
 * Cualquier sub-ruta también será protegida automáticamente.
 */
const PROTECTED_STRATA = ["/dashboard", "/admin", "/settings", "/lab"];

/**
 * PUERTAS DE ENLACE (Solo para Visitantes)
 * Si un operador autenticado intenta entrar aquí, será rebotado al Dashboard.
 */
const AUTH_GATEWAYS = ["/login", "/register"];

/**
 * Ejecuta el control de acceso soberano.
 * Retorna `NextResponse` para redirigir o `null` para continuar.
 */
export async function authHandler(
  execution_request: NextRequest,
): Promise<NextResponse | null> {
  const { pathname, search } = execution_request.nextUrl;

  // 1. VERIFICACIÓN DE CREDENCIALES (JWT)
  const session_token = await getToken({
    req: execution_request,
    secret: process.env.AUTH_SECRET,
  });

  const is_authenticated = !!session_token;

  // 2. ANÁLISIS DE ESTRUCTURA DE URL (Locale + Path)
  const url_segments = pathname.split("/").filter(Boolean);

  // Definición del tipo basado en los locales soportados
  type AppLocale = typeof routing.locales[number];

  // Detección segura del idioma actual o fallback al defecto
  const first_segment = url_segments[0] as AppLocale;
  const is_locale_present = routing.locales.includes(first_segment);

  const detected_locale: AppLocale = is_locale_present
    ? first_segment
    : routing.defaultLocale;

  // Normalización de la ruta (Eliminar locale para comparación agnóstica)
  // Ej: /es/dashboard -> /dashboard
  // Ej: /dashboard -> /dashboard
  const normalized_path = is_locale_present
    ? `/${url_segments.slice(1).join("/")}`
    : `/${url_segments.join("/")}`;

  const target_path = normalized_path || "/";

  // 3. PROTOCOLO DE DEFENSA (Rutas Protegidas)
  const is_stratum_protected = PROTECTED_STRATA.some((stratum) =>
    target_path.startsWith(stratum),
  );

  if (is_stratum_protected && !is_authenticated) {
    // Redirección al Login preservando el idioma y la intención (callbackUrl)
    const login_url = new URL(`/${detected_locale}/login`, execution_request.url);
    login_url.searchParams.set("callbackUrl", encodeURIComponent(`${pathname}${search}`));
    return NextResponse.redirect(login_url);
  }

  // 4. PROTOCOLO DE REBOTE (Gateways de Acceso)
  const is_in_gateway = AUTH_GATEWAYS.some((gateway) =>
    target_path.startsWith(gateway),
  );

  if (is_in_gateway && is_authenticated) {
    // Operador ya identificado intentando loguearse de nuevo -> Enviar a Mando
    return NextResponse.redirect(new URL(`/${detected_locale}/dashboard`, execution_request.url));
  }

  // 5. PASE LIBRE (Delegar al siguiente handler i18n)
  return null;
}
// FIN DEL ARCHIVO [apps/web-dashboard/lib/handlers/auth.ts]

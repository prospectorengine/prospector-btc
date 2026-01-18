/**
 * =================================================================
 * APARATO: SOVEREIGN EDGE GATEWAY (V25.0 - GOLD MASTER)
 * CLASIFICACIÓN: INFRASTRUCTURE GATEWAY (ESTRATO L4)
 * RESPONSABILIDAD: PROTECCIÓN PERIMETRAL, LOCALIZACIÓN Y SYNC DE SESIÓN
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la lógica de interceptación en el borde (Vercel Edge).
 * 1. SINCRO: Coordina el motor de I18n con el motor de Auth de Supabase.
 * 2. SEGURIDAD: Aplica el Protocolo de Rebote para proteger el Dashboard.
 * 3. INTEGRIDAD: Garantiza que las cookies de sesión fluyan hacia el cliente.
 * =================================================================
 */

import { createServerClient, type CookieOptions } from '@supabase/ssr';
import { NextResponse, type NextRequest } from 'next/server';
import createIntlMiddleware from 'next-intl/middleware';
import { routing } from '@/lib/schemas/routing';

/**
 * Orquestador de Localización.
 * Define el comportamiento de las rutas ante prefijos /en y /es.
 */
const handle_localization = createIntlMiddleware(routing);

export async function middleware(request: NextRequest) {
  /**
   * 1. PRE-PROCESAMIENTO DE LOCALIZACIÓN
   * Generamos la respuesta inicial basada en el idioma detectado.
   */
  let response = handle_localization(request);

  /**
   * 2. INICIALIZACIÓN DEL CLIENTE SUPABASE (EDGE COMPLIANT)
   * Implementamos la delegación de cookies para asegurar que el token JWT
   * se mantenga sincronizado entre el servidor de base de datos y el navegador.
   */
  const supabase = createServerClient(
    process.env.NEXT_PUBLIC_SUPABASE_URL!,
    process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!,
    {
      cookies: {
        get(name: string) {
          return request.cookies.get(name)?.value;
        },
        set(name: string, value: string, options: CookieOptions) {
          // Inyectamos la cookie en la petición para que esté disponible en esta ejecución
          request.cookies.set({ name, value, ...options });
          // Inyectamos la cookie en la respuesta final para el navegador
          response = NextResponse.next({
            request: { headers: request.headers },
          });
          response.cookies.set({ name, value, ...options });
        },
        remove(name: string, options: CookieOptions) {
          request.cookies.set({ name, value: '', ...options });
          response = NextResponse.next({
            request: { headers: request.headers },
          });
          response.cookies.set({ name, value: '', ...options });
        },
      },
    }
  );

  /**
   * 3. AUDITORÍA DE IDENTIDAD DEL OPERADOR
   * Verificamos la existencia de un usuario autenticado en el Motor B.
   */
  const { data: { user } } = await supabase.auth.getUser();

  // Análisis de coordenadas de navegación
  const current_path = request.nextUrl.pathname;
  const segments = current_path.split('/').filter(Boolean);

  // Detección de idioma activo (Válido para comparaciones dinámicas)
  const first_segment = segments[0] as typeof routing.locales[number];
  const active_locale = routing.locales.includes(first_segment)
    ? first_segment
    : routing.defaultLocale;

  const is_dashboard_path = current_path.includes('/dashboard');
  const is_auth_gateway_path = current_path.includes('/login') || current_path.includes('/register');

  /**
   * 4. PROTOCOLO DE PROTECCIÓN (REBOTE Y ACCESO)
   * Determinamos el origen canónico para evitar ataques de redirección abierta.
   */
  const site_origin = process.env.NEXT_PUBLIC_SITE_URL || request.nextUrl.origin;

  // CASO ALFA: Intento de acceso al Dashboard sin credenciales.
  if (is_dashboard_path && !user) {
    const login_redirect_url = `${site_origin}/${active_locale}/login`;
    return NextResponse.redirect(login_redirect_url);
  }

  // CASO BETA: Operador con sesión activa intentando acceder a Login/Register.
  if (is_auth_gateway_path && user) {
    const dashboard_redirect_url = `${site_origin}/${active_locale}/dashboard`;
    return NextResponse.redirect(dashboard_redirect_url);
  }

  /**
   * 5. FINALIZACIÓN DEL HANDSHAKE
   * Retornamos la respuesta con los estratos de cookies y localización sellados.
   */
  return response;
}

/**
 * CONFIGURACIÓN DEL MATCHING ENGINE
 * Excluimos activos estáticos y APIs internas para optimizar el rendimiento.
 */
export const config = {
  matcher: [
    /*
     * Coincidir con todas las rutas excepto:
     * 1. /api (Rutas de backend interno)
     * 2. /_next (Archivos de construcción)
     * 3. /_static, /_vercel (Metadatos de despliegue)
     * 4. Archivos con extensión (png, jpg, ico, etc.)
     */
    '/((?!api|_next/static|_next/image|favicon.ico|.*\\..*).*)',
  ],
};

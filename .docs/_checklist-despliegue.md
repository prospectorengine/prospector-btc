CHECKLIST FINAL DE DESPLIEGUE (QA)
Comandante, antes de dar la orden de git push, ejecute mentalmente este protocolo en la aplicación desplegada:
1. Pruebas de Acceso Público

Entrar a https://.../ -> Debe mostrar Landing Page.

Clic en "Login" -> Debe ir a /en/login.

Clic en "Get Started" -> Debe ir a /en/register (Nueva página).

Cambiar idioma (URL /es/) -> Textos en Español.
2. Pruebas de Protección (Sin Sesión)

Intentar ir a /dashboard -> Redirección forzada a /login.

Intentar ir a /dashboard/network -> Redirección forzada a /login.
3. Pruebas de Flujo de Operador (Con Sesión Google)

Completar Login con Google.

Verificación Crítica: ¿Se ve el Dashboard? (Si sale blanco, falla I18n; si sale, éxito).

Verificación SSE: En el Dashboard, abrir DevTools (F12) -> Application -> Session Storage. ¿Existe ADMIN_SESSION_TOKEN? (Gracias al SessionHydrator).

Navegación: Clic en Sidebar "Network" -> Carga /dashboard/network.

Navegación: Clic en Sidebar "Settings" -> Carga /dashboard/settings.
4. Pruebas de Salida

Clic en Avatar -> Logout.

Resultado: Redirección a Landing Page (/).

Intentar "Atrás" en el navegador -> Debería rebotar al Login (Middleware actuando).

// INICIO DEL ARCHIVO [apps/web-dashboard/app/page.tsx]
import { redirect } from "next/navigation";

// Esta página raíz (root) normalmente es interceptada por el middleware
// y redirigida a /[locale]. Si el middleware falla, esta es la red de seguridad.
export default function RootPage() {
  redirect("/en");
}
// FIN DEL ARCHIVO [apps/web-dashboard/app/page.tsx]

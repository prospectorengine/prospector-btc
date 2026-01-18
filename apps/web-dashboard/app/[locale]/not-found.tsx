// INICIO DEL ARCHIVO [apps/web-dashboard/app/[locale]/not-found.tsx]
import { useTranslations } from "next-intl";
import { NotFoundScreen } from "@/components/system/not-found-screen";

export default function LocalizedNotFound() {
  /**
   * ✅ FIX: Namespace corregido para coincidir con en.json / es.json
   * Antes: useTranslations("NotFound") -> Error MISSING_MESSAGE
   * Ahora: useTranslations("System.not_found") -> Correcto
   */
  const t = useTranslations("System.not_found");

  const returnRoute = "/dashboard";

  return (
    <NotFoundScreen
      texts={{
        title: t("title"),
        description: t("description"),
        error_code: t("error_code"),
        cta_return: t("cta_return"),
      }}
      redirectPath={returnRoute}
    />
  );
}
// FIN DEL ARCHIVO [apps/web-dashboard/app/[locale]/not-found.tsx]

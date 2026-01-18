// [apps/web-dashboard/lib/i18n/schema.ts]
import { z } from "zod";
import { CommonSchema } from "./schemas/common.schema";
import { PublicHeaderSchema } from "./schemas/layout/public-header.schema"; // Nuevo
import { PublicFooterSchema } from "./schemas/layout/public-footer.schema"; // Nuevo
import { LandingPageSchema } from "./schemas/pages/landing.schema";
import { DashboardSchema } from "./schemas/dashboard/dashboard.schema";
import { AuthSchema } from "./schemas/auth/auth.schema";
import { SystemPagesSchema } from "./schemas/system/system.schema";

export const AppLocaleSchema = z.object({
  Common: CommonSchema,
  PublicHeader: PublicHeaderSchema, // Inyectado
  PublicFooter: PublicFooterSchema, // Inyectado
  Landing: LandingPageSchema,
  Dashboard: DashboardSchema,
  Auth: AuthSchema,
  System: SystemPagesSchema,
});

export type AppLocale = z.infer<typeof AppLocaleSchema>;

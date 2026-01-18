// INICIO DEL ARCHIVO [apps/web-dashboard/next.config.js]
/**
 * =================================================================
 * APARATO: NEXT.JS CONFIGURATION ENGINE (V34.0 - I18N LINKED)
 * CLASIFICACIÓN: ESTRATO L5 - VIEW INFRASTRUCTURE
 * RESPONSABILIDAD: ORQUESTACIÓN DEL RUNTIME Y SELLADO DE CONSTRUCCIÓN
 * =================================================================
 */

// @ts-check
const { withNx } = require("@nx/next");
const createNextIntlPlugin = require('next-intl/plugin');

/**
 * 1. CONFIGURACIÓN DEL PLUGIN I18N
 * Apuntamos explícitamente a la ubicación física de la lógica de request.
 * Esto resuelve el error "Couldn't find next-intl config file".
 */
const withNextIntl = createNextIntlPlugin('./lib/schemas/request.ts');

/** @type {import('next').NextConfig} */
const next_sovereign_configuration = {
  /**
   * GENERACIÓN STANDALONE
   * Optimizado para despliegues en Vercel y Docker.
   */
  output: "standalone",

  /**
   * SEGURIDAD PERIMETRAL
   */
  poweredByHeader: false,
  reactStrictMode: true,

  /**
   * ESTRATO DE TRANSPILACIÓN SOBERANA
   */
  transpilePackages: [
    "@prospector/api-contracts",
    "@prospector/api-client",
    "@prospector/heimdall-ts",
    "@prospector/ui-kit",
    "@prospector/crypto-vault",
    "@prospector/infra-supabase",
    "@prospector/ui-billing",
    "@prospector/ui-notifications",
    "@prospector/ui-gamification",
    "@prospector/ui-content",
    "@prospector/ui-social"
  ],

  /**
   * CARACTERÍSTICAS EXPERIMENTALES / NEXT 15
   */
  experimental: {
    reactCompiler: true,
    typedRoutes: true,
    optimizePackageImports: ["lucide-react", "framer-motion", "recharts"],
  },

  /**
   * MATRIZ DE REESCRITURA TÁCTICA (NEURAL LINK)
   */
  async rewrites() {
    const orchestrator_api_endpoint =
      process.env.NEXT_PUBLIC_API_URL || "http://localhost:3000/api/v1";

    return [
      {
        source: "/api/v1/:path*",
        destination: `${orchestrator_api_endpoint}/:path*`,
      },
    ];
  },

  images: {
    remotePatterns: [
      { protocol: "https", hostname: "*.googleusercontent.com" },
      { protocol: "https", hostname: "avatars.githubusercontent.com" }
    ],
  },
};

// 2. COMPOSICIÓN DE PLUGINS (NX + NEXT-INTL)
// El orden es crítico: Nx envuelve a la configuración resultante.
module.exports = withNx(withNextIntl(next_sovereign_configuration));
// FIN DEL ARCHIVO [apps/web-dashboard/next.config.js]

// INICIO DEL ARCHIVO [eslint.config.mjs]
import nx from "@nx/eslint-plugin";
import tsParser from "@typescript-eslint/parser";
import tsPlugin from "@typescript-eslint/eslint-plugin";

/*! @type {import('eslint').Linter.Config[]} */
export default [
  {
    /* ✅ FIX: Ignoramos explícitamente artefactos de build y configuración de tests para evitar errores de parser */
    ignores: [
      "**/dist",
      "**/target",
      "**/node_modules",
      "**/.next",
      "**/out",
      "**/jest.config.ts",
      "**/eslint.config.mjs"
    ],
  },
  {
    files: ["**/*.ts", "**/*.tsx", "**/*.js", "**/*.jsx"],
    plugins: {
      "@nx": nx,
    },
    rules: {
      "@nx/enforce-module-boundaries": [
        "error",
        {
          enforceBuildableLibDependency: true,
          allow: [],
          depConstraints: [
            { sourceTag: "*", onlyDependOnLibsWithTags: ["*"] },
          ],
        },
      ],
    },
  },
  {
    files: ["**/*.ts", "**/*.tsx"],
    languageOptions: {
      parser: tsParser,
      parserOptions: {
        project: true,
      },
    },
    plugins: {
      "@typescript-eslint": tsPlugin,
    },
    rules: {
      ...tsPlugin.configs.recommended.rules,
      "@typescript-eslint/no-explicit-any": "error",
      "@typescript-eslint/no-unused-vars": [
        "error",
        { "argsIgnorePattern": "^_", "varsIgnorePattern": "^_" }
      ],
      /* ✅ FIX: Permitimos 'info' para logs tácticos */
      "no-console": ["warn", { "allow": ["warn", "error", "info"] }]
    },
  },
  {
    files: ["**/*.config.ts", "**/*.config.js", "**/tools/**/*"],
    rules: {
      "@typescript-eslint/no-explicit-any": "off",
      "no-console": "off",
      "@typescript-eslint/no-unused-vars": "off"
    }
  }
];
// FIN DEL ARCHIVO [eslint.config.mjs]

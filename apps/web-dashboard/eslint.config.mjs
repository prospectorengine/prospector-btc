import baseConfig from "../../eslint.config.mjs";
import nextPlugin from "@next/eslint-plugin-next";
import reactPlugin from "eslint-plugin-react";
import hooksPlugin from "eslint-plugin-react-hooks";


/*! @type {import('eslint').Linter.Config[]} */
export default [
  ...baseConfig,
  {
    files: ["apps/web-dashboard/**/*.{ts,tsx,js,jsx}"],
    plugins: {
      "@next/next": nextPlugin,
      "react": reactPlugin,
      "react-hooks": hooksPlugin,
    },
    rules: {
      ...nextPlugin.configs.recommended.rules,
      ...nextPlugin.configs["core-web-vitals"].rules,
      "react/react-in-jsx-scope": "off",
      "@next/next/no-html-link-for-pages": ["error", "apps/web-dashboard"],
    },
    settings: {
      react: { version: "detect" }
    }
  }
];

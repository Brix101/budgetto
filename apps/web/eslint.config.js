import baseConfig, { restrictEnvAccess } from "@budgetto/eslint-config/base";
import nextjsConfig from "@budgetto/eslint-config/nextjs";
import reactConfig from "@budgetto/eslint-config/react";

/** @type {import('typescript-eslint').Config} */
export default [
  {
    ignores: [".next/**"],
  },
  ...baseConfig,
  ...reactConfig,
  ...nextjsConfig,
  ...restrictEnvAccess,
];

import reactPlugin from "eslint-plugin-react";
import reactHooks from "eslint-plugin-react-hooks";
import globals from "globals";

/** @type {Awaited<import('typescript-eslint').Config>} */
export default [
  {
    files: ["**/*.{ts,tsx}"],
    plugins: {
      react: reactPlugin,
      "react-hooks": reactHooks,
    },
    rules: {
      ...reactPlugin.configs["jsx-runtime"].rules,
      ...reactHooks.configs.recommended.rules,
    },
    languageOptions: {
      ecmaVersion: 2020,
      globals: globals.browser,
    },
  },
];

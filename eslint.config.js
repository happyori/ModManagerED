import globals from "globals";
import pluginJs from "@eslint/js";
import tseslint from "typescript-eslint";
import unocss from "@unocss/eslint-config/flat";
import svelte from 'eslint-plugin-svelte';
import tsparser from '@typescript-eslint/parser';
import svelteParser from 'svelte-eslint-parser';

export default [
  unocss,
  { files: ["**/*.{js,mjs,cjs,ts}"] },
  { languageOptions: { globals: globals.browser } },
  { ignores: ["src/generated/binding.ts"] },
  pluginJs.configs.recommended,
  ...tseslint.configs.recommended,
  {
    languageOptions: {
      parser: tsparser,
      parserOptions: {
        extraFileExtensions: [".svelte"],
      }
    }
  },
  ...svelte.configs['flat/prettier'].map(config => ({
    ...config,
    files: ["**/*.svelte", "*.svelte"],
    languageOptions: {
      parser: svelteParser,
      parserOptions: {
        parser: tsparser
      }
    }
  })),
];
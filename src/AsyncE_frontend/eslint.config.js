import pluginJs from "@eslint/js";
import prettierConfig from "eslint-config-prettier";
import pluginImport from "eslint-plugin-import";
import prettierPlugin from "eslint-plugin-prettier";
import pluginVue from "eslint-plugin-vue";
import globals from "globals";
import tseslint from "typescript-eslint";

export default [
    { files: ["**/*.{js,mjs,cjs,ts,vue}"] },
    { languageOptions: { globals: { ...globals.browser, ...globals.node } } },
    pluginJs.configs.recommended,
    ...tseslint.configs.recommended,
    ...pluginVue.configs["flat/essential"],
    {
        files: ["**/*.vue"],
        languageOptions: { parserOptions: { parser: tseslint.parser } },
        rules: {
            "vue/multi-word-component-names": 0,
        },
    },
    {
        // Configuration for sorting imports
        files: ["**/*.{js,mjs,cjs,ts,vue}"],
        plugins: {
            import: pluginImport,
            prettier: prettierPlugin,
        },
        rules: {
            "import/order": [
                "error",
                {
                    groups: [
                        "builtin",
                        "external",
                        "internal",
                        "parent",
                        "sibling",
                        "index",
                    ],
                    pathGroups: [
                        {
                            pattern: "vue",
                            group: "builtin",
                            position: "before",
                        },
                        {
                            pattern: "pinia",
                            group: "builtin",
                            position: "before",
                        },
                        {
                            pattern: "vue-router",
                            group: "builtin",
                            position: "before",
                        },
                        {
                            pattern: "@stores/**",
                            group: "internal",
                            position: "before",
                        },
                        {
                            pattern: "@components/**",
                            group: "internal",
                            position: "before",
                        },
                        {
                            pattern: "@vueuse/**",
                            group: "external",
                            position: "before",
                        },
                    ],
                    pathGroupsExcludedImportTypes: ["builtin"],
                    "newlines-between": "always",
                    alphabetize: { order: "asc", caseInsensitive: true },
                },
            ],
            "prettier/prettier": "error",
        },
    },
    prettierConfig,
];

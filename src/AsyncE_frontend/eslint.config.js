import pluginJs from "@eslint/js";
import pluginImport from "eslint-plugin-import";
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
        },
        rules: {
            "import/order": [
                "error",
                {
                    groups: [
                        "external",
                        "builtin",
                        "internal",
                        "sibling",
                        "parent",
                        "index",
                    ],
                    alphabetize: { order: "asc", caseInsensitive: true },
                },
            ],
        },
    },
];

/// <reference types="vitest" />
import { fileURLToPath, URL } from "url";
import { defineConfig } from "vite";
import environment from "vite-plugin-environment";
import vue from "@vitejs/plugin-vue";
import dotenv from "dotenv";

import tailwind from "tailwindcss";
import autoprefixer from "autoprefixer";

dotenv.config({ path: "../../.env" });
function resolve(dir) {
    return fileURLToPath(new URL(dir, import.meta.url));
}

const UI_ENV_VARS = [
    "DFX_NETWORK",
    "CANISTER_ID_ASYNCE_BACKEND",
    "CANISTER_ID_INTERNET_IDENTITY",
];

process.env = {
    ...process.env,
    ...UI_ENV_VARS.reduce(
        (accum, entry) => ({
            ...accum,
            [`VITE_${entry}`]: process.env[entry],
        }),
        {},
    ),
};

export default defineConfig({
    css: {
        postcss: {
            plugins: [tailwind(), autoprefixer()],
        },
    },
    build: {
        emptyOutDir: true,
    },
    optimizeDeps: {
        esbuildOptions: {
            define: {
                global: "globalThis",
            },
        },
    },
    server: {
        proxy: {
            "/api": {
                target: "http://127.0.0.1:4943",
                changeOrigin: true,
            },
        },
    },
    define: {
        "process.env": {
            DFX_NETWORK: process.env.DFX_NETWORK,
            CANISTER_ID_ASYNCE_BACKEND: process.env.CANISTER_ID_ASYNCE_BACKEND,
            CANISTER_ID_INTERNET_IDENTITY:
                process.env.CANISTER_ID_INTERNET_IDENTITY,
        },
    },
    plugins: [
        vue(),
        environment("all", { prefix: "CANISTER_" }),
        environment("all", { prefix: "DFX_" }),
    ],
    test: {
        environment: "jsdom",
        setupFiles: "src/setupTests.js",
    },
    resolve: {
        alias: {
            "@": resolve("./src"),
            "@lazy-loading-routes": resolve("./src/lazy-loading-routes"),
            "@types": resolve("./src/types"),
            "@components": resolve("./src/components"),
            "@ui": resolve("./src/components/ui"),
            "@shared": resolve("./src/components/shared"),
            "@stores": resolve("./src/stores"),
            "@data": resolve("./src/data"),
            "@declarations": resolve("../declarations"),
        },
    },
});

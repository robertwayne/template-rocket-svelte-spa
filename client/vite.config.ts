import * as dotenv from "dotenv"

import { defineConfig } from "vitest/config"
import { svelte } from "@sveltejs/vite-plugin-svelte"

dotenv.config({ path: "../.env" })

export default defineConfig({
    base: "/",
    plugins: [svelte()],
    build: {
        outDir: process.env.BUILD_PATH || "dist",
        emptyOutDir: true,
    },
    optimizeDeps: {
        exclude: ["svelte-navigator"],
    },
    server: {
        host: "127.0.0.1",
        port: 8000,
    },
    define: {
        "import.meta.vitest": false,
    },
    test: {
        includeSource: ["src/**/*.ts"],
        globals: true,
        environment: "happy-dom",
    },
})

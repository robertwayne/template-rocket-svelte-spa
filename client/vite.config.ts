import * as dotenv from "dotenv"

import { defineConfig } from "vitest/config"
import { svelte } from "@sveltejs/vite-plugin-svelte"
dotenv.config()

export default ({ mode }) => {
    return defineConfig({
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
            https: false,
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
}

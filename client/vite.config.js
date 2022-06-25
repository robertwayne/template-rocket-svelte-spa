import { defineConfig } from "vite"
import { svelte } from "@sveltejs/vite-plugin-svelte"

export default ({ mode }) => {
    require("dotenv").config(".env")

    return defineConfig({
        plugins: [svelte()],
        build: {
            outDir: process.env.BUILD_PATH || "dist",
            emptyOutDir: true,
            minify: "terser",
        },
        optimizeDeps: {
            exclude: ["svelte-navigator"],
        },
        server: {
            https: false,
        },
        assetsDir: "static",
        define: {
            "import.meta.vitest": false,
        },
        test: {
            includeSource: ["src/**/*.ts"],
        },
    })
}

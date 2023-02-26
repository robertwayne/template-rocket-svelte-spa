import "./app.css"
import "./hmr"

import App from "./App.svelte"

document.body.classList.add(
    window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light"
)

const app = new App({
    target: document.body,
})

export default app

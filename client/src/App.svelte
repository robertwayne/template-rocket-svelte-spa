<script lang="ts">
    import { Router } from "svelte-navigator"
    import LazyRoute from "./components/LazyRoute.svelte"
    import Footer from "./components/Footer.svelte"
    import Nav from "./components/Nav.svelte"
    import { onMount } from "svelte"

    const routes = {
        "/": () => import("./routes/Home.svelte"),
        "/about": () => import("./routes/About.svelte"),
    }

    onMount(() => {
        window
            .matchMedia("(prefers-color-scheme: dark)")
            .addEventListener("change", (event) => {
                document.body.classList.add(event.matches ? "dark" : "light")
                document.body.classList.remove(event.matches ? "light" : "dark")
            })
    })
</script>

<div
    id="app"
    class="flex h-screen w-full flex-col bg-zinc-100 p-2 text-zinc-800 dark:bg-zinc-800 dark:text-zinc-100"
>
    <Router primary={false}>
        <Nav />
    </Router>

    <main class="flex w-full flex-grow justify-center">
        <Router>
            {#each Object.entries(routes) as [path, component]}
                <LazyRoute {path} {component}>Loading...</LazyRoute>
            {/each}
            <LazyRoute
                path="*"
                component={() => import("./routes/NotFound.svelte")}
            />
        </Router>
    </main>

    <Footer />
</div>

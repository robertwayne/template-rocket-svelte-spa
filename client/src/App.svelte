<script lang="ts">
    import { Router } from "svelte-navigator"
    import AsyncRoute from "./components/AsyncRoute.svelte"
    import Footer from "./components/Footer.svelte"
    import Nav from "./components/Nav.svelte"
    import Tailwind from "./Tailwind.svelte"

    const routes = {
        "/": () => import("./routes/Home.svelte"),
        "/about": () => import("./routes/About.svelte"),
    }
</script>

<Tailwind />

<div id="app" class="flex h-full w-full flex-col">
    <Router primary={false}>
        <Nav />
    </Router>

    <main class="flex w-full flex-grow justify-center">
        <Router>
            {#each Object.entries(routes) as [path, component]}
                <AsyncRoute {path} {component}>Loading...</AsyncRoute>
            {/each}
        </Router>
    </main>

    <Footer />
</div>

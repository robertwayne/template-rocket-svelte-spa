<script lang="ts">
    import type { SvelteComponent } from "svelte"
    import { Route, Router } from "svelte-navigator"
    import Footer from "./components/Footer.svelte"
    import Nav from "./components/Nav.svelte"
    import About from "./routes/About.svelte"
    import Home from "./routes/Home.svelte"
    import Tailwind from "./Tailwind.svelte"

    const routes: { [key: string]: typeof SvelteComponent } = {
        "/": Home,
        "/about": About,
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
                <Route {path} {component} />
            {/each}
        </Router>
    </main>

    <Footer />
</div>

<script lang="ts">
    import type { SvelteComponent } from 'svelte'
    import { Route, Router } from 'svelte-navigator'
    import Footer from './components/Footer.svelte'
    import Nav from './components/Nav.svelte'
    import About from './routes/About.svelte'
    import Home from './routes/Home.svelte'
    import Tailwind from './Tailwind.svelte'

    const routes: { [key: string]: typeof SvelteComponent } = {
        '/': Home,
        '/about': About,
    }
</script>

<Tailwind />

<div id="app" class="flex flex-col h-full w-full">
    <Router primary={false}>
        <Nav />
    </Router>

    <main class="flex flex-grow justify-center w-full">
        <Router>
            {#each Object.entries(routes) as [path, component]}
                <Route {path} {component} />
            {/each}
        </Router>
    </main>

    <Footer />
</div>

<script lang="ts">
    import { Link } from "svelte-navigator"

    let menuElement: HTMLUListElement
    let menuOpen = false

    const toggleMenu = (e: Event): void => {
        e.preventDefault()
        menuElement.classList.toggle("hidden")
        menuElement.classList.toggle("flex")

        menuOpen = !menuOpen
    }

    const getStrokeColor = (): string => {
        return document.body.classList.contains("dark") ? "#fff" : "#000"
    }
</script>

<nav class="flex flex-row justify-between font-bold h-fit">
    <h1 class="text-3xl"><Link to="/">Template App</Link></h1>
    <ul
        bind:this={menuElement}
        class="bg-blur absolute right-0 top-0 h-full hidden w-2/3 flex-col justify-center gap-8 bg-zinc-400 p-2 text-3xl dark:bg-zinc-700 md:flex md:w-fit md:flex-row md:bg-transparent md:text-xl"
    >
        <li class="px-2 hover:text-blue-500 hover:duration-300 h-fit">
            <Link to="/" on:click={toggleMenu}>Home</Link>
        </li>
        <li class="px-2 hover:text-blue-500 hover:duration-300 h-fit">
            <Link to="/about" on:click={toggleMenu}>About</Link>
        </li>
    </ul>

    {#if !menuOpen}
        <button
            class="md:hidden"
            on:click={toggleMenu}
            aria-controls="navigation-menu"
            aria-expanded="false"
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="44"
                height="44"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke={getStrokeColor()}
                fill="none"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <path stroke="none" d="M0 0h24v24H0z" fill="none" />
                <line x1="4" y1="6" x2="20" y2="6" />
                <line x1="4" y1="12" x2="20" y2="12" />
                <line x1="4" y1="18" x2="20" y2="18" />
            </svg>
        </button>
    {:else}
        <button class="z-50 md:hidden" on:click={toggleMenu}>
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="44"
                height="44"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke={getStrokeColor()}
                fill="none"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <path stroke="none" d="M0 0h24v24H0z" fill="none" />
                <line x1="18" y1="6" x2="6" y2="18" />
                <line x1="6" y1="6" x2="18" y2="18" />
            </svg>a
        </button>
    {/if}
</nav>

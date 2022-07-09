<script lang="ts">
    import { Link } from "svelte-navigator"
</script>

<nav class="flex flex-row justify-between font-bold">
    <h1 class="text-3xl"><Link to="/">Template App</Link></h1>
    <ul class="flex flex-row text-xl">
        <li class="px-2 hover:text-blue-500 hover:duration-300">
            <Link to="/">Home</Link>
        </li>
        <li class="px-2 hover:text-blue-500 hover:duration-300">
            <Link to="/about">About</Link>
        </li>
    </ul>
</nav>

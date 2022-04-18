<script lang="ts">
    import { Link } from "svelte-navigator"
</script>

<nav class="flex flex-row justify-between font-bold">
    <h1>Template App</h1>
    <ul class="flex flex-row">
        <li class="px-2">
            <Link to="/">Home</Link>
        </li>
        <li class="px-2">
            <Link to="/about">About</Link>
        </li>
    </ul>
</nav>
